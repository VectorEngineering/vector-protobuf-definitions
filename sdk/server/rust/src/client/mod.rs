use async_trait::async_trait;
use futures::{Stream, future, future::BoxFuture, stream, future::TryFutureExt, future::FutureExt, stream::StreamExt};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use hyper::{Body, Request, Response, service::Service, Uri};
use percent_encoding::{utf8_percent_encode, AsciiSet};
use std::borrow::Cow;
use std::convert::TryInto;
use std::io::{ErrorKind, Read};
use std::error::Error;
use std::future::Future;
use std::fmt;
use std::marker::PhantomData;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::str;
use std::str::FromStr;
use std::string::ToString;
use std::task::{Context, Poll};
use swagger::{ApiError, AuthData, BodyExt, Connector, DropContextService, Has, XSpanIdString};
use url::form_urlencoded;


use crate::models;
use crate::header;

/// https://url.spec.whatwg.org/#fragment-percent-encode-set
#[allow(dead_code)]
const FRAGMENT_ENCODE_SET: &AsciiSet = &percent_encoding::CONTROLS
    .add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

/// This encode set is used for object IDs
///
/// Aside from the special characters defined in the `PATH_SEGMENT_ENCODE_SET`,
/// the vertical bar (|) is encoded.
#[allow(dead_code)]
const ID_ENCODE_SET: &AsciiSet = &FRAGMENT_ENCODE_SET.add(b'|');

use crate::{Api,
     CreateAccountResponse,
     CreateScrapingJobResponse,
     DeleteAccountResponse,
     DeleteScrapingJobResponse,
     DownloadScrapingResultsResponse,
     GetAccountResponse,
     GetScrapingJobResponse,
     ListScrapingJobsResponse,
     UpdateAccountResponse
     };

/// Convert input into a base path, e.g. "http://example:123". Also checks the scheme as it goes.
fn into_base_path(input: impl TryInto<Uri, Error=hyper::http::uri::InvalidUri>, correct_scheme: Option<&'static str>) -> Result<String, ClientInitError> {
    // First convert to Uri, since a base path is a subset of Uri.
    let uri = input.try_into()?;

    let scheme = uri.scheme_str().ok_or(ClientInitError::InvalidScheme)?;

    // Check the scheme if necessary
    if let Some(correct_scheme) = correct_scheme {
        if scheme != correct_scheme {
            return Err(ClientInitError::InvalidScheme);
        }
    }

    let host = uri.host().ok_or(ClientInitError::MissingHost)?;
    let port = uri.port_u16().map(|x| format!(":{}", x)).unwrap_or_default();
    Ok(format!("{}://{}{}{}", scheme, host, port, uri.path().trim_end_matches('/')))
}

/// A client that implements the API by making HTTP calls out to a server.
pub struct Client<S, C> where
    S: Service<
           (Request<Body>, C),
           Response=Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Clone + Send + Sync + 'static
{
    /// Inner service
    client_service: S,

    /// Base path of the API
    base_path: String,

    /// Marker
    marker: PhantomData<fn(C)>,
}

impl<S, C> fmt::Debug for Client<S, C> where
    S: Service<
           (Request<Body>, C),
           Response=Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Clone + Send + Sync + 'static
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Client {{ base_path: {} }}", self.base_path)
    }
}

impl<S, C> Clone for Client<S, C> where
    S: Service<
           (Request<Body>, C),
           Response=Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Clone + Send + Sync + 'static
{
    fn clone(&self) -> Self {
        Self {
            client_service: self.client_service.clone(),
            base_path: self.base_path.clone(),
            marker: PhantomData,
        }
    }
}

impl<Connector, C> Client<DropContextService<hyper::client::Client<Connector, Body>, C>, C> where
    Connector: hyper::client::connect::Connect + Clone + Send + Sync + 'static,
    C: Clone + Send + Sync + 'static,
{
    /// Create a client with a custom implementation of hyper::client::Connect.
    ///
    /// Intended for use with custom implementations of connect for e.g. protocol logging
    /// or similar functionality which requires wrapping the transport layer. When wrapping a TCP connection,
    /// this function should be used in conjunction with `swagger::Connector::builder()`.
    ///
    /// For ordinary tcp connections, prefer the use of `try_new_http`, `try_new_https`
    /// and `try_new_https_mutual`, to avoid introducing a dependency on the underlying transport layer.
    ///
    /// # Arguments
    ///
    /// * `base_path` - base path of the client API, i.e. "http://www.my-api-implementation.com"
    /// * `protocol` - Which protocol to use when constructing the request url, e.g. `Some("http")`
    /// * `connector` - Implementation of `hyper::client::Connect` to use for the client
    pub fn try_new_with_connector(
        base_path: &str,
        protocol: Option<&'static str>,
        connector: Connector,
    ) -> Result<Self, ClientInitError>
    {
        let client_service = hyper::client::Client::builder().build(connector);
        let client_service = DropContextService::new(client_service);

        Ok(Self {
            client_service,
            base_path: into_base_path(base_path, protocol)?,
            marker: PhantomData,
        })
    }
}

#[derive(Debug, Clone)]
pub enum HyperClient {
    Http(hyper::client::Client<hyper::client::HttpConnector, Body>),
    Https(hyper::client::Client<HttpsConnector, Body>),
}

impl Service<Request<Body>> for HyperClient {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = hyper::client::ResponseFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
       match self {
          HyperClient::Http(client) => client.poll_ready(cx),
          HyperClient::Https(client) => client.poll_ready(cx),
       }
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
       match self {
          HyperClient::Http(client) => client.call(req),
          HyperClient::Https(client) => client.call(req)
       }
    }
}

impl<C> Client<DropContextService<HyperClient, C>, C> where
    C: Clone + Send + Sync + 'static,
{
    /// Create an HTTP client.
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "http://www.my-api-implementation.com"
    pub fn try_new(
        base_path: &str,
    ) -> Result<Self, ClientInitError> {
        let uri = Uri::from_str(base_path)?;

        let scheme = uri.scheme_str().ok_or(ClientInitError::InvalidScheme)?;
        let scheme = scheme.to_ascii_lowercase();

        let connector = Connector::builder();

        let client_service = match scheme.as_str() {
            "http" => {
                HyperClient::Http(hyper::client::Client::builder().build(connector.build()))
            },
            "https" => {
                let connector = connector.https()
                   .build()
                   .map_err(ClientInitError::SslError)?;
                HyperClient::Https(hyper::client::Client::builder().build(connector))
            },
            _ => {
                return Err(ClientInitError::InvalidScheme);
            }
        };

        let client_service = DropContextService::new(client_service);

        Ok(Self {
            client_service,
            base_path: into_base_path(base_path, None)?,
            marker: PhantomData,
        })
    }
}

impl<C> Client<DropContextService<hyper::client::Client<hyper::client::HttpConnector, Body>, C>, C> where
    C: Clone + Send + Sync + 'static
{
    /// Create an HTTP client.
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "http://www.my-api-implementation.com"
    pub fn try_new_http(
        base_path: &str,
    ) -> Result<Self, ClientInitError> {
        let http_connector = Connector::builder().build();

        Self::try_new_with_connector(base_path, Some("http"), http_connector)
    }
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
type HttpsConnector = hyper_openssl::HttpsConnector<hyper::client::HttpConnector>;

impl<C> Client<DropContextService<hyper::client::Client<HttpsConnector, Body>, C>, C> where
    C: Clone + Send + Sync + 'static
{
    /// Create a client with a TLS connection to the server
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "https://www.my-api-implementation.com"
    pub fn try_new_https(base_path: &str) -> Result<Self, ClientInitError>
    {
        let https_connector = Connector::builder()
            .https()
            .build()
            .map_err(ClientInitError::SslError)?;
        Self::try_new_with_connector(base_path, Some("https"), https_connector)
    }

    /// Create a client with a TLS connection to the server using a pinned certificate
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "https://www.my-api-implementation.com"
    /// * `ca_certificate` - Path to CA certificate used to authenticate the server
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    pub fn try_new_https_pinned<CA>(
        base_path: &str,
        ca_certificate: CA,
    ) -> Result<Self, ClientInitError>
    where
        CA: AsRef<Path>,
    {
        let https_connector = Connector::builder()
            .https()
            .pin_server_certificate(ca_certificate)
            .build()
            .map_err(ClientInitError::SslError)?;
        Self::try_new_with_connector(base_path, Some("https"), https_connector)
    }

    /// Create a client with a mutually authenticated TLS connection to the server.
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "https://www.my-api-implementation.com"
    /// * `ca_certificate` - Path to CA certificate used to authenticate the server
    /// * `client_key` - Path to the client private key
    /// * `client_certificate` - Path to the client's public certificate associated with the private key
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    pub fn try_new_https_mutual<CA, K, D>(
        base_path: &str,
        ca_certificate: CA,
        client_key: K,
        client_certificate: D,
    ) -> Result<Self, ClientInitError>
    where
        CA: AsRef<Path>,
        K: AsRef<Path>,
        D: AsRef<Path>,
    {
        let https_connector = Connector::builder()
            .https()
            .pin_server_certificate(ca_certificate)
            .client_authentication(client_key, client_certificate)
            .build()
            .map_err(ClientInitError::SslError)?;
        Self::try_new_with_connector(base_path, Some("https"), https_connector)
    }
}

impl<S, C> Client<S, C> where
    S: Service<
           (Request<Body>, C),
           Response=Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Clone + Send + Sync + 'static
{
    /// Constructor for creating a `Client` by passing in a pre-made `hyper::service::Service` /
    /// `tower::Service`
    ///
    /// This allows adding custom wrappers around the underlying transport, for example for logging.
    pub fn try_new_with_client_service(
        client_service: S,
        base_path: &str,
    ) -> Result<Self, ClientInitError>
    {
        Ok(Self {
            client_service,
            base_path: into_base_path(base_path, None)?,
            marker: PhantomData,
        })
    }
}

/// Error type failing to create a Client
#[derive(Debug)]
pub enum ClientInitError {
    /// Invalid URL Scheme
    InvalidScheme,

    /// Invalid URI
    InvalidUri(hyper::http::uri::InvalidUri),

    /// Missing Hostname
    MissingHost,

    /// SSL Connection Error
    #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
    SslError(native_tls::Error),

    /// SSL Connection Error
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    SslError(openssl::error::ErrorStack),
}

impl From<hyper::http::uri::InvalidUri> for ClientInitError {
    fn from(err: hyper::http::uri::InvalidUri) -> ClientInitError {
        ClientInitError::InvalidUri(err)
    }
}

impl fmt::Display for ClientInitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: &dyn fmt::Debug = self;
        s.fmt(f)
    }
}

impl Error for ClientInitError {
    fn description(&self) -> &str {
        "Failed to produce a hyper client."
    }
}

#[async_trait]
impl<S, C> Api<C> for Client<S, C> where
    S: Service<
       (Request<Body>, C),
       Response=Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Has<XSpanIdString> + Has<Option<AuthData>> + Clone + Send + Sync + 'static,
{
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), crate::ServiceError>> {
        match self.client_service.clone().poll_ready(cx) {
            Poll::Ready(Err(e)) => Poll::Ready(Err(e.into())),
            Poll::Ready(Ok(o)) => Poll::Ready(Ok(o)),
            Poll::Pending => Poll::Pending,
        }
    }

    async fn create_account(
        &self,
        param_create_account_request: models::CreateAccountRequest,
        context: &C) -> Result<CreateAccountResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/lead-scraper-microservice/api/v1/accounts",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        // Body parameter
        let body = serde_json::to_string(&param_create_account_request).expect("impossible to fail to serialize");
                *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::CreateAccountResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateAccountResponse::ASuccessfulResponse
                    (body)
                )
            }
            201 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::CreateAccountResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateAccountResponse::AccountCreatedSuccessfully
                    (body)
                )
            }
            400 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ValidationErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateAccountResponse::BadRequest
                    (body)
                )
            }
            401 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateAccountResponse::Unauthorized
                    (body)
                )
            }
            402 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::PaymentRequiredErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateAccountResponse::PaymentRequired
                    (body)
                )
            }
            403 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ForbiddenErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateAccountResponse::Forbidden
                    (body)
                )
            }
            404 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::NotFoundErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateAccountResponse::NotFound
                    (body)
                )
            }
            405 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::MethodNotAllowedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateAccountResponse::MethodNotAllowed
                    (body)
                )
            }
            409 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ConflictErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateAccountResponse::Conflict
                    (body)
                )
            }
            410 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::GoneErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateAccountResponse::Gone
                    (body)
                )
            }
            412 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::PreconditionFailedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateAccountResponse::PreconditionFailed
                    (body)
                )
            }
            422 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::UnprocessableEntityErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateAccountResponse::UnprocessableEntity
                    (body)
                )
            }
            425 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::TooEarlyErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateAccountResponse::TooEarly
                    (body)
                )
            }
            429 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::RateLimitErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateAccountResponse::TooManyRequests
                    (body)
                )
            }
            500 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InternalErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateAccountResponse::InternalServerError
                    (body)
                )
            }
            501 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::NotImplementedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateAccountResponse::NotImplemented
                    (body)
                )
            }
            502 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::BadGatewayErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateAccountResponse::BadGateway
                    (body)
                )
            }
            503 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ServiceUnavailableErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateAccountResponse::ServiceUnavailable
                    (body)
                )
            }
            504 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::GatewayTimeoutErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateAccountResponse::GatewayTimeout
                    (body)
                )
            }
            0 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::RpcPeriodStatus>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateAccountResponse::AnUnexpectedErrorResponse
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn create_scraping_job(
        &self,
        param_create_scraping_job_request: models::CreateScrapingJobRequest,
        context: &C) -> Result<CreateScrapingJobResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/lead-scraper-microservice/api/v1/jobs",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let body = serde_json::to_string(&param_create_scraping_job_request).expect("impossible to fail to serialize");
                *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::CreateScrapingJobResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateScrapingJobResponse::ASuccessfulResponse
                    (body)
                )
            }
            201 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::CreateScrapingJobResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateScrapingJobResponse::JobCreatedSuccessfully
                    (body)
                )
            }
            400 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ValidationErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateScrapingJobResponse::BadRequest
                    (body)
                )
            }
            401 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateScrapingJobResponse::Unauthorized
                    (body)
                )
            }
            402 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::PaymentRequiredErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateScrapingJobResponse::PaymentRequired
                    (body)
                )
            }
            403 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ForbiddenErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateScrapingJobResponse::Forbidden
                    (body)
                )
            }
            404 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::NotFoundErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateScrapingJobResponse::NotFound
                    (body)
                )
            }
            405 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::MethodNotAllowedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateScrapingJobResponse::MethodNotAllowed
                    (body)
                )
            }
            409 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ConflictErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateScrapingJobResponse::Conflict
                    (body)
                )
            }
            410 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::GoneErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateScrapingJobResponse::Gone
                    (body)
                )
            }
            412 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::PreconditionFailedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateScrapingJobResponse::PreconditionFailed
                    (body)
                )
            }
            422 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::UnprocessableEntityErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateScrapingJobResponse::UnprocessableEntity
                    (body)
                )
            }
            425 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::TooEarlyErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateScrapingJobResponse::TooEarly
                    (body)
                )
            }
            429 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::RateLimitErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateScrapingJobResponse::TooManyRequests
                    (body)
                )
            }
            500 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InternalErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateScrapingJobResponse::InternalServerError
                    (body)
                )
            }
            501 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::NotImplementedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateScrapingJobResponse::NotImplemented
                    (body)
                )
            }
            502 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::BadGatewayErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateScrapingJobResponse::BadGateway
                    (body)
                )
            }
            503 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ServiceUnavailableErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateScrapingJobResponse::ServiceUnavailable
                    (body)
                )
            }
            504 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::GatewayTimeoutErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateScrapingJobResponse::GatewayTimeout
                    (body)
                )
            }
            0 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::RpcPeriodStatus>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateScrapingJobResponse::AnUnexpectedErrorResponse
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_account(
        &self,
        param_id: String,
        context: &C) -> Result<DeleteAccountResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/lead-scraper-microservice/api/v1/accounts/{id}",
            self.base_path
            ,id=utf8_percent_encode(&param_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::DeleteAccountResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteAccountResponse::AccountDeletedSuccessfully
                    (body)
                )
            }
            400 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ValidationErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteAccountResponse::BadRequest
                    (body)
                )
            }
            401 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteAccountResponse::Unauthorized
                    (body)
                )
            }
            402 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::PaymentRequiredErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteAccountResponse::PaymentRequired
                    (body)
                )
            }
            403 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ForbiddenErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteAccountResponse::Forbidden
                    (body)
                )
            }
            404 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::NotFoundErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteAccountResponse::NotFound
                    (body)
                )
            }
            405 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::MethodNotAllowedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteAccountResponse::MethodNotAllowed
                    (body)
                )
            }
            409 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ConflictErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteAccountResponse::Conflict
                    (body)
                )
            }
            410 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::GoneErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteAccountResponse::Gone
                    (body)
                )
            }
            412 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::PreconditionFailedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteAccountResponse::PreconditionFailed
                    (body)
                )
            }
            422 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::UnprocessableEntityErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteAccountResponse::UnprocessableEntity
                    (body)
                )
            }
            425 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::TooEarlyErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteAccountResponse::TooEarly
                    (body)
                )
            }
            429 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::RateLimitErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteAccountResponse::TooManyRequests
                    (body)
                )
            }
            500 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InternalErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteAccountResponse::InternalServerError
                    (body)
                )
            }
            501 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::NotImplementedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteAccountResponse::NotImplemented
                    (body)
                )
            }
            502 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::BadGatewayErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteAccountResponse::BadGateway
                    (body)
                )
            }
            503 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ServiceUnavailableErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteAccountResponse::ServiceUnavailable
                    (body)
                )
            }
            504 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::GatewayTimeoutErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteAccountResponse::GatewayTimeout
                    (body)
                )
            }
            0 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::RpcPeriodStatus>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteAccountResponse::AnUnexpectedErrorResponse
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_scraping_job(
        &self,
        param_job_id: String,
        param_user_id: String,
        param_org_id: String,
        param_tenant_id: String,
        context: &C) -> Result<DeleteScrapingJobResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/lead-scraper-microservice/api/v1/jobs/{job_id}",
            self.base_path
            ,job_id=utf8_percent_encode(&param_job_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
                query_string.append_pair("userId",
                    &param_user_id);
                query_string.append_pair("orgId",
                    &param_org_id);
                query_string.append_pair("tenantId",
                    &param_tenant_id);
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::DeleteScrapingJobResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteScrapingJobResponse::JobDeletedSuccessfully
                    (body)
                )
            }
            400 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ValidationErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteScrapingJobResponse::BadRequest
                    (body)
                )
            }
            401 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteScrapingJobResponse::Unauthorized
                    (body)
                )
            }
            402 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::PaymentRequiredErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteScrapingJobResponse::PaymentRequired
                    (body)
                )
            }
            403 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ForbiddenErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteScrapingJobResponse::Forbidden
                    (body)
                )
            }
            404 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::NotFoundErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteScrapingJobResponse::NotFound
                    (body)
                )
            }
            405 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::MethodNotAllowedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteScrapingJobResponse::MethodNotAllowed
                    (body)
                )
            }
            409 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ConflictErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteScrapingJobResponse::Conflict
                    (body)
                )
            }
            410 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::GoneErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteScrapingJobResponse::Gone
                    (body)
                )
            }
            412 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::PreconditionFailedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteScrapingJobResponse::PreconditionFailed
                    (body)
                )
            }
            422 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::UnprocessableEntityErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteScrapingJobResponse::UnprocessableEntity
                    (body)
                )
            }
            425 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::TooEarlyErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteScrapingJobResponse::TooEarly
                    (body)
                )
            }
            429 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::RateLimitErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteScrapingJobResponse::TooManyRequests
                    (body)
                )
            }
            500 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InternalErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteScrapingJobResponse::InternalServerError
                    (body)
                )
            }
            501 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::NotImplementedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteScrapingJobResponse::NotImplemented
                    (body)
                )
            }
            502 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::BadGatewayErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteScrapingJobResponse::BadGateway
                    (body)
                )
            }
            503 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ServiceUnavailableErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteScrapingJobResponse::ServiceUnavailable
                    (body)
                )
            }
            504 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::GatewayTimeoutErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteScrapingJobResponse::GatewayTimeout
                    (body)
                )
            }
            0 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::RpcPeriodStatus>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteScrapingJobResponse::AnUnexpectedErrorResponse
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn download_scraping_results(
        &self,
        param_job_id: String,
        param_user_id: String,
        param_org_id: String,
        param_tenant_id: String,
        context: &C) -> Result<DownloadScrapingResultsResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/lead-scraper-microservice/api/v1/jobs/{job_id}/download",
            self.base_path
            ,job_id=utf8_percent_encode(&param_job_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
                query_string.append_pair("userId",
                    &param_user_id);
                query_string.append_pair("orgId",
                    &param_org_id);
                query_string.append_pair("tenantId",
                    &param_tenant_id);
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::DownloadScrapingResultsResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DownloadScrapingResultsResponse::SuccessfulResponse
                    (body)
                )
            }
            400 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ValidationErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DownloadScrapingResultsResponse::BadRequest
                    (body)
                )
            }
            401 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DownloadScrapingResultsResponse::Unauthorized
                    (body)
                )
            }
            402 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::PaymentRequiredErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DownloadScrapingResultsResponse::PaymentRequired
                    (body)
                )
            }
            403 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ForbiddenErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DownloadScrapingResultsResponse::Forbidden
                    (body)
                )
            }
            404 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::NotFoundErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DownloadScrapingResultsResponse::NotFound
                    (body)
                )
            }
            405 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::MethodNotAllowedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DownloadScrapingResultsResponse::MethodNotAllowed
                    (body)
                )
            }
            409 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ConflictErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DownloadScrapingResultsResponse::Conflict
                    (body)
                )
            }
            410 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::GoneErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DownloadScrapingResultsResponse::Gone
                    (body)
                )
            }
            412 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::PreconditionFailedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DownloadScrapingResultsResponse::PreconditionFailed
                    (body)
                )
            }
            422 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::UnprocessableEntityErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DownloadScrapingResultsResponse::UnprocessableEntity
                    (body)
                )
            }
            425 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::TooEarlyErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DownloadScrapingResultsResponse::TooEarly
                    (body)
                )
            }
            429 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::RateLimitErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DownloadScrapingResultsResponse::TooManyRequests
                    (body)
                )
            }
            500 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InternalErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DownloadScrapingResultsResponse::InternalServerError
                    (body)
                )
            }
            501 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::NotImplementedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DownloadScrapingResultsResponse::NotImplemented
                    (body)
                )
            }
            502 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::BadGatewayErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DownloadScrapingResultsResponse::BadGateway
                    (body)
                )
            }
            503 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ServiceUnavailableErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DownloadScrapingResultsResponse::ServiceUnavailable
                    (body)
                )
            }
            504 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::GatewayTimeoutErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DownloadScrapingResultsResponse::GatewayTimeout
                    (body)
                )
            }
            0 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::RpcPeriodStatus>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DownloadScrapingResultsResponse::AnUnexpectedErrorResponse
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_account(
        &self,
        param_id: String,
        context: &C) -> Result<GetAccountResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/lead-scraper-microservice/api/v1/accounts/{id}",
            self.base_path
            ,id=utf8_percent_encode(&param_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::GetAccountResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetAccountResponse::AccountDetailsRetrievedSuccessfully
                    (body)
                )
            }
            400 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ValidationErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetAccountResponse::BadRequest
                    (body)
                )
            }
            401 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetAccountResponse::Unauthorized
                    (body)
                )
            }
            402 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::PaymentRequiredErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetAccountResponse::PaymentRequired
                    (body)
                )
            }
            403 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ForbiddenErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetAccountResponse::Forbidden
                    (body)
                )
            }
            404 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::NotFoundErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetAccountResponse::NotFound
                    (body)
                )
            }
            405 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::MethodNotAllowedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetAccountResponse::MethodNotAllowed
                    (body)
                )
            }
            409 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ConflictErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetAccountResponse::Conflict
                    (body)
                )
            }
            410 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::GoneErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetAccountResponse::Gone
                    (body)
                )
            }
            412 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::PreconditionFailedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetAccountResponse::PreconditionFailed
                    (body)
                )
            }
            422 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::UnprocessableEntityErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetAccountResponse::UnprocessableEntity
                    (body)
                )
            }
            425 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::TooEarlyErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetAccountResponse::TooEarly
                    (body)
                )
            }
            429 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::RateLimitErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetAccountResponse::TooManyRequests
                    (body)
                )
            }
            500 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InternalErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetAccountResponse::InternalServerError
                    (body)
                )
            }
            501 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::NotImplementedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetAccountResponse::NotImplemented
                    (body)
                )
            }
            502 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::BadGatewayErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetAccountResponse::BadGateway
                    (body)
                )
            }
            503 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ServiceUnavailableErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetAccountResponse::ServiceUnavailable
                    (body)
                )
            }
            504 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::GatewayTimeoutErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetAccountResponse::GatewayTimeout
                    (body)
                )
            }
            0 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::RpcPeriodStatus>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetAccountResponse::AnUnexpectedErrorResponse
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_scraping_job(
        &self,
        param_job_id: String,
        param_user_id: String,
        param_org_id: String,
        param_tenant_id: String,
        context: &C) -> Result<GetScrapingJobResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/lead-scraper-microservice/api/v1/jobs/{job_id}",
            self.base_path
            ,job_id=utf8_percent_encode(&param_job_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
                query_string.append_pair("userId",
                    &param_user_id);
                query_string.append_pair("orgId",
                    &param_org_id);
                query_string.append_pair("tenantId",
                    &param_tenant_id);
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::GetScrapingJobResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetScrapingJobResponse::SuccessfulResponse
                    (body)
                )
            }
            400 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ValidationErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetScrapingJobResponse::BadRequest
                    (body)
                )
            }
            401 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetScrapingJobResponse::Unauthorized
                    (body)
                )
            }
            402 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::PaymentRequiredErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetScrapingJobResponse::PaymentRequired
                    (body)
                )
            }
            403 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ForbiddenErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetScrapingJobResponse::Forbidden
                    (body)
                )
            }
            404 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::NotFoundErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetScrapingJobResponse::NotFound
                    (body)
                )
            }
            405 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::MethodNotAllowedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetScrapingJobResponse::MethodNotAllowed
                    (body)
                )
            }
            409 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ConflictErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetScrapingJobResponse::Conflict
                    (body)
                )
            }
            410 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::GoneErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetScrapingJobResponse::Gone
                    (body)
                )
            }
            412 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::PreconditionFailedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetScrapingJobResponse::PreconditionFailed
                    (body)
                )
            }
            422 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::UnprocessableEntityErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetScrapingJobResponse::UnprocessableEntity
                    (body)
                )
            }
            425 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::TooEarlyErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetScrapingJobResponse::TooEarly
                    (body)
                )
            }
            429 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::RateLimitErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetScrapingJobResponse::TooManyRequests
                    (body)
                )
            }
            500 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InternalErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetScrapingJobResponse::InternalServerError
                    (body)
                )
            }
            501 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::NotImplementedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetScrapingJobResponse::NotImplemented
                    (body)
                )
            }
            502 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::BadGatewayErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetScrapingJobResponse::BadGateway
                    (body)
                )
            }
            503 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ServiceUnavailableErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetScrapingJobResponse::ServiceUnavailable
                    (body)
                )
            }
            504 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::GatewayTimeoutErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetScrapingJobResponse::GatewayTimeout
                    (body)
                )
            }
            0 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::RpcPeriodStatus>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetScrapingJobResponse::AnUnexpectedErrorResponse
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn list_scraping_jobs(
        &self,
        param_user_id: String,
        param_org_id: String,
        param_tenant_id: String,
        context: &C) -> Result<ListScrapingJobsResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/lead-scraper-microservice/api/v1/jobs",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
                query_string.append_pair("userId",
                    &param_user_id);
                query_string.append_pair("orgId",
                    &param_org_id);
                query_string.append_pair("tenantId",
                    &param_tenant_id);
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ListScrapingJobsResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListScrapingJobsResponse::SuccessfulResponse
                    (body)
                )
            }
            400 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ValidationErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListScrapingJobsResponse::BadRequest
                    (body)
                )
            }
            401 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListScrapingJobsResponse::Unauthorized
                    (body)
                )
            }
            402 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::PaymentRequiredErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListScrapingJobsResponse::PaymentRequired
                    (body)
                )
            }
            403 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ForbiddenErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListScrapingJobsResponse::Forbidden
                    (body)
                )
            }
            404 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::NotFoundErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListScrapingJobsResponse::NotFound
                    (body)
                )
            }
            405 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::MethodNotAllowedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListScrapingJobsResponse::MethodNotAllowed
                    (body)
                )
            }
            409 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ConflictErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListScrapingJobsResponse::Conflict
                    (body)
                )
            }
            410 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::GoneErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListScrapingJobsResponse::Gone
                    (body)
                )
            }
            412 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::PreconditionFailedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListScrapingJobsResponse::PreconditionFailed
                    (body)
                )
            }
            422 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::UnprocessableEntityErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListScrapingJobsResponse::UnprocessableEntity
                    (body)
                )
            }
            425 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::TooEarlyErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListScrapingJobsResponse::TooEarly
                    (body)
                )
            }
            429 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::RateLimitErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListScrapingJobsResponse::TooManyRequests
                    (body)
                )
            }
            500 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InternalErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListScrapingJobsResponse::InternalServerError
                    (body)
                )
            }
            501 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::NotImplementedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListScrapingJobsResponse::NotImplemented
                    (body)
                )
            }
            502 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::BadGatewayErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListScrapingJobsResponse::BadGateway
                    (body)
                )
            }
            503 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ServiceUnavailableErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListScrapingJobsResponse::ServiceUnavailable
                    (body)
                )
            }
            504 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::GatewayTimeoutErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListScrapingJobsResponse::GatewayTimeout
                    (body)
                )
            }
            0 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::RpcPeriodStatus>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListScrapingJobsResponse::AnUnexpectedErrorResponse
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn update_account(
        &self,
        param_update_account_request: models::UpdateAccountRequest,
        context: &C) -> Result<UpdateAccountResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/lead-scraper-microservice/api/v1/accounts",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("PUT")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let body = serde_json::to_string(&param_update_account_request).expect("impossible to fail to serialize");

                *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::UpdateAccountResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateAccountResponse::AccountUpdatedSuccessfully
                    (body)
                )
            }
            400 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ValidationErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateAccountResponse::BadRequest
                    (body)
                )
            }
            401 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateAccountResponse::Unauthorized
                    (body)
                )
            }
            402 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::PaymentRequiredErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateAccountResponse::PaymentRequired
                    (body)
                )
            }
            403 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ForbiddenErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateAccountResponse::Forbidden
                    (body)
                )
            }
            404 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::NotFoundErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateAccountResponse::NotFound
                    (body)
                )
            }
            405 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::MethodNotAllowedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateAccountResponse::MethodNotAllowed
                    (body)
                )
            }
            409 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ConflictErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateAccountResponse::Conflict
                    (body)
                )
            }
            410 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::GoneErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateAccountResponse::Gone
                    (body)
                )
            }
            412 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::PreconditionFailedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateAccountResponse::PreconditionFailed
                    (body)
                )
            }
            422 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::UnprocessableEntityErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateAccountResponse::UnprocessableEntity
                    (body)
                )
            }
            425 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::TooEarlyErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateAccountResponse::TooEarly
                    (body)
                )
            }
            429 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::RateLimitErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateAccountResponse::TooManyRequests
                    (body)
                )
            }
            500 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InternalErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateAccountResponse::InternalServerError
                    (body)
                )
            }
            501 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::NotImplementedErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateAccountResponse::NotImplemented
                    (body)
                )
            }
            502 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::BadGatewayErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateAccountResponse::BadGateway
                    (body)
                )
            }
            503 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ServiceUnavailableErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateAccountResponse::ServiceUnavailable
                    (body)
                )
            }
            504 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::GatewayTimeoutErrorMessageResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateAccountResponse::GatewayTimeout
                    (body)
                )
            }
            0 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::RpcPeriodStatus>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateAccountResponse::AnUnexpectedErrorResponse
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

}
