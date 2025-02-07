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
     CreateOrganizationResponse,
     CreateTenantResponse,
     DeleteOrganizationResponse,
     DeleteTenantResponse,
     GetOrganizationResponse,
     GetTenantResponse,
     ListOrganizationsResponse,
     ListTenantsResponse,
     UpdateOrganizationResponse,
     UpdateTenantResponse,
     CreateAccountResponse,
     CreateWorkspaceResponse,
     DeleteAccountResponse,
     DeleteWorkspaceResponse,
     GetAccountResponse,
     GetWorkspaceResponse,
     GetWorkspaceAnalyticsResponse,
     GetWorkspaceComplianceReportResponse,
     GetWorkspaceStorageStatsResponse,
     ListWorkspaceSharingsResponse,
     ListWorkspacesResponse,
     RemoveWorkspaceSharingResponse,
     ShareWorkspaceResponse,
     UpdateAccountResponse,
     UpdateWorkspaceResponse,
     UpdateWorkspaceSharingResponse
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

    async fn create_organization(
        &self,
        param_create_organization_request: models::CreateOrganizationRequest,
        context: &C) -> Result<CreateOrganizationResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/lead-scraper-microservice/api/v1/organization",
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
        let body = serde_json::to_string(&param_create_organization_request).expect("impossible to fail to serialize");
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
                let body = serde_json::from_str::<models::CreateOrganizationResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateOrganizationResponse::ASuccessfulResponse
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
                let body = serde_json::from_str::<models::CreateOrganizationResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateOrganizationResponse::OrganizationCreatedSuccessfully
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
                Ok(CreateOrganizationResponse::BadRequest
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
                Ok(CreateOrganizationResponse::Unauthorized
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
                Ok(CreateOrganizationResponse::PaymentRequired
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
                Ok(CreateOrganizationResponse::Forbidden
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
                Ok(CreateOrganizationResponse::NotFound
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
                Ok(CreateOrganizationResponse::MethodNotAllowed
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
                Ok(CreateOrganizationResponse::Conflict
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
                Ok(CreateOrganizationResponse::Gone
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
                Ok(CreateOrganizationResponse::PreconditionFailed
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
                Ok(CreateOrganizationResponse::UnprocessableEntity
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
                Ok(CreateOrganizationResponse::TooEarly
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
                Ok(CreateOrganizationResponse::TooManyRequests
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
                Ok(CreateOrganizationResponse::InternalServerError
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
                Ok(CreateOrganizationResponse::NotImplemented
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
                Ok(CreateOrganizationResponse::BadGateway
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
                Ok(CreateOrganizationResponse::ServiceUnavailable
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
                Ok(CreateOrganizationResponse::GatewayTimeout
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
                Ok(CreateOrganizationResponse::AnUnexpectedErrorResponse
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

    async fn create_tenant(
        &self,
        param_organization_id: String,
        param_create_tenant_body: models::CreateTenantBody,
        context: &C) -> Result<CreateTenantResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/lead-scraper-microservice/api/v1/organizations/{organization_id}/tenants",
            self.base_path
            ,organization_id=utf8_percent_encode(&param_organization_id.to_string(), ID_ENCODE_SET)
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

        let body = serde_json::to_string(&param_create_tenant_body).expect("impossible to fail to serialize");
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
                let body = serde_json::from_str::<models::CreateTenantResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateTenantResponse::ASuccessfulResponse
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
                let body = serde_json::from_str::<models::CreateTenantResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateTenantResponse::TenantCreatedSuccessfully
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
                Ok(CreateTenantResponse::BadRequest
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
                Ok(CreateTenantResponse::Unauthorized
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
                Ok(CreateTenantResponse::PaymentRequired
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
                Ok(CreateTenantResponse::Forbidden
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
                Ok(CreateTenantResponse::NotFound
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
                Ok(CreateTenantResponse::MethodNotAllowed
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
                Ok(CreateTenantResponse::Conflict
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
                Ok(CreateTenantResponse::Gone
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
                Ok(CreateTenantResponse::PreconditionFailed
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
                Ok(CreateTenantResponse::UnprocessableEntity
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
                Ok(CreateTenantResponse::TooEarly
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
                Ok(CreateTenantResponse::TooManyRequests
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
                Ok(CreateTenantResponse::InternalServerError
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
                Ok(CreateTenantResponse::NotImplemented
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
                Ok(CreateTenantResponse::BadGateway
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
                Ok(CreateTenantResponse::ServiceUnavailable
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
                Ok(CreateTenantResponse::GatewayTimeout
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
                Ok(CreateTenantResponse::AnUnexpectedErrorResponse
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

    async fn delete_organization(
        &self,
        param_id: String,
        context: &C) -> Result<DeleteOrganizationResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/lead-scraper-microservice/api/v1/organization/{id}",
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
                let body = serde_json::from_str::<models::DeleteOrganizationResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteOrganizationResponse::OrganizationDeletedSuccessfully
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
                Ok(DeleteOrganizationResponse::BadRequest
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
                Ok(DeleteOrganizationResponse::Unauthorized
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
                Ok(DeleteOrganizationResponse::PaymentRequired
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
                Ok(DeleteOrganizationResponse::Forbidden
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
                Ok(DeleteOrganizationResponse::NotFound
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
                Ok(DeleteOrganizationResponse::MethodNotAllowed
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
                Ok(DeleteOrganizationResponse::Conflict
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
                Ok(DeleteOrganizationResponse::Gone
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
                Ok(DeleteOrganizationResponse::PreconditionFailed
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
                Ok(DeleteOrganizationResponse::UnprocessableEntity
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
                Ok(DeleteOrganizationResponse::TooEarly
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
                Ok(DeleteOrganizationResponse::TooManyRequests
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
                Ok(DeleteOrganizationResponse::InternalServerError
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
                Ok(DeleteOrganizationResponse::NotImplemented
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
                Ok(DeleteOrganizationResponse::BadGateway
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
                Ok(DeleteOrganizationResponse::ServiceUnavailable
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
                Ok(DeleteOrganizationResponse::GatewayTimeout
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
                Ok(DeleteOrganizationResponse::AnUnexpectedErrorResponse
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

    async fn delete_tenant(
        &self,
        param_organization_id: String,
        param_tenant_id: String,
        context: &C) -> Result<DeleteTenantResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/lead-scraper-microservice/api/v1/organization/tenants/{organization_id}/{tenant_id}",
            self.base_path
            ,organization_id=utf8_percent_encode(&param_organization_id.to_string(), ID_ENCODE_SET)
            ,tenant_id=utf8_percent_encode(&param_tenant_id.to_string(), ID_ENCODE_SET)
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
                let body = serde_json::from_str::<models::DeleteTenantResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteTenantResponse::TenantDeletedSuccessfully
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
                Ok(DeleteTenantResponse::BadRequest
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
                Ok(DeleteTenantResponse::Unauthorized
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
                Ok(DeleteTenantResponse::PaymentRequired
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
                Ok(DeleteTenantResponse::Forbidden
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
                Ok(DeleteTenantResponse::NotFound
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
                Ok(DeleteTenantResponse::MethodNotAllowed
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
                Ok(DeleteTenantResponse::Conflict
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
                Ok(DeleteTenantResponse::Gone
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
                Ok(DeleteTenantResponse::PreconditionFailed
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
                Ok(DeleteTenantResponse::UnprocessableEntity
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
                Ok(DeleteTenantResponse::TooEarly
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
                Ok(DeleteTenantResponse::TooManyRequests
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
                Ok(DeleteTenantResponse::InternalServerError
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
                Ok(DeleteTenantResponse::NotImplemented
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
                Ok(DeleteTenantResponse::BadGateway
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
                Ok(DeleteTenantResponse::ServiceUnavailable
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
                Ok(DeleteTenantResponse::GatewayTimeout
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
                Ok(DeleteTenantResponse::AnUnexpectedErrorResponse
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

    async fn get_organization(
        &self,
        param_id: String,
        context: &C) -> Result<GetOrganizationResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/lead-scraper-microservice/api/v1/organization/{id}",
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
                let body = serde_json::from_str::<models::GetOrganizationResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetOrganizationResponse::OrganizationRetrievedSuccessfully
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
                Ok(GetOrganizationResponse::BadRequest
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
                Ok(GetOrganizationResponse::Unauthorized
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
                Ok(GetOrganizationResponse::PaymentRequired
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
                Ok(GetOrganizationResponse::Forbidden
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
                Ok(GetOrganizationResponse::NotFound
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
                Ok(GetOrganizationResponse::MethodNotAllowed
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
                Ok(GetOrganizationResponse::Conflict
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
                Ok(GetOrganizationResponse::Gone
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
                Ok(GetOrganizationResponse::PreconditionFailed
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
                Ok(GetOrganizationResponse::UnprocessableEntity
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
                Ok(GetOrganizationResponse::TooEarly
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
                Ok(GetOrganizationResponse::TooManyRequests
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
                Ok(GetOrganizationResponse::InternalServerError
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
                Ok(GetOrganizationResponse::NotImplemented
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
                Ok(GetOrganizationResponse::BadGateway
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
                Ok(GetOrganizationResponse::ServiceUnavailable
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
                Ok(GetOrganizationResponse::GatewayTimeout
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
                Ok(GetOrganizationResponse::AnUnexpectedErrorResponse
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

    async fn get_tenant(
        &self,
        param_organization_id: String,
        param_tenant_id: String,
        context: &C) -> Result<GetTenantResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/lead-scraper-microservice/api/v1/organizations/tenants/{organization_id}/{tenant_id}",
            self.base_path
            ,organization_id=utf8_percent_encode(&param_organization_id.to_string(), ID_ENCODE_SET)
            ,tenant_id=utf8_percent_encode(&param_tenant_id.to_string(), ID_ENCODE_SET)
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
                let body = serde_json::from_str::<models::GetTenantResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetTenantResponse::TenantRetrievedSuccessfully
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
                Ok(GetTenantResponse::BadRequest
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
                Ok(GetTenantResponse::Unauthorized
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
                Ok(GetTenantResponse::PaymentRequired
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
                Ok(GetTenantResponse::Forbidden
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
                Ok(GetTenantResponse::NotFound
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
                Ok(GetTenantResponse::MethodNotAllowed
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
                Ok(GetTenantResponse::Conflict
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
                Ok(GetTenantResponse::Gone
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
                Ok(GetTenantResponse::PreconditionFailed
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
                Ok(GetTenantResponse::UnprocessableEntity
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
                Ok(GetTenantResponse::TooEarly
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
                Ok(GetTenantResponse::TooManyRequests
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
                Ok(GetTenantResponse::InternalServerError
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
                Ok(GetTenantResponse::NotImplemented
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
                Ok(GetTenantResponse::BadGateway
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
                Ok(GetTenantResponse::ServiceUnavailable
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
                Ok(GetTenantResponse::GatewayTimeout
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
                Ok(GetTenantResponse::AnUnexpectedErrorResponse
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

    async fn list_organizations(
        &self,
        param_page_size: Option<i32>,
        param_page_number: Option<i32>,
        context: &C) -> Result<ListOrganizationsResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/lead-scraper-microservice/api/v1/organization",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_page_size) = param_page_size {
                query_string.append_pair("pageSize",
                    &param_page_size.to_string());
            }
            if let Some(param_page_number) = param_page_number {
                query_string.append_pair("pageNumber",
                    &param_page_number.to_string());
            }
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
                let body = serde_json::from_str::<models::ListOrganizationsResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListOrganizationsResponse::OrganizationsRetrievedSuccessfully
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
                Ok(ListOrganizationsResponse::BadRequest
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
                Ok(ListOrganizationsResponse::Unauthorized
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
                Ok(ListOrganizationsResponse::PaymentRequired
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
                Ok(ListOrganizationsResponse::Forbidden
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
                Ok(ListOrganizationsResponse::NotFound
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
                Ok(ListOrganizationsResponse::MethodNotAllowed
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
                Ok(ListOrganizationsResponse::Conflict
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
                Ok(ListOrganizationsResponse::Gone
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
                Ok(ListOrganizationsResponse::PreconditionFailed
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
                Ok(ListOrganizationsResponse::UnprocessableEntity
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
                Ok(ListOrganizationsResponse::TooEarly
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
                Ok(ListOrganizationsResponse::TooManyRequests
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
                Ok(ListOrganizationsResponse::InternalServerError
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
                Ok(ListOrganizationsResponse::NotImplemented
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
                Ok(ListOrganizationsResponse::BadGateway
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
                Ok(ListOrganizationsResponse::ServiceUnavailable
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
                Ok(ListOrganizationsResponse::GatewayTimeout
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
                Ok(ListOrganizationsResponse::AnUnexpectedErrorResponse
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

    async fn list_tenants(
        &self,
        param_organization_id: String,
        param_page_size: Option<i32>,
        param_page_number: Option<i32>,
        context: &C) -> Result<ListTenantsResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/lead-scraper-microservice/api/v1/organization/tenants/{organization_id}",
            self.base_path
            ,organization_id=utf8_percent_encode(&param_organization_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_page_size) = param_page_size {
                query_string.append_pair("pageSize",
                    &param_page_size.to_string());
            }
            if let Some(param_page_number) = param_page_number {
                query_string.append_pair("pageNumber",
                    &param_page_number.to_string());
            }
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
                let body = serde_json::from_str::<models::ListTenantsResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListTenantsResponse::TenantsRetrievedSuccessfully
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
                Ok(ListTenantsResponse::BadRequest
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
                Ok(ListTenantsResponse::Unauthorized
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
                Ok(ListTenantsResponse::PaymentRequired
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
                Ok(ListTenantsResponse::Forbidden
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
                Ok(ListTenantsResponse::NotFound
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
                Ok(ListTenantsResponse::MethodNotAllowed
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
                Ok(ListTenantsResponse::Conflict
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
                Ok(ListTenantsResponse::Gone
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
                Ok(ListTenantsResponse::PreconditionFailed
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
                Ok(ListTenantsResponse::UnprocessableEntity
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
                Ok(ListTenantsResponse::TooEarly
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
                Ok(ListTenantsResponse::TooManyRequests
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
                Ok(ListTenantsResponse::InternalServerError
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
                Ok(ListTenantsResponse::NotImplemented
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
                Ok(ListTenantsResponse::BadGateway
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
                Ok(ListTenantsResponse::ServiceUnavailable
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
                Ok(ListTenantsResponse::GatewayTimeout
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
                Ok(ListTenantsResponse::AnUnexpectedErrorResponse
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

    async fn update_organization(
        &self,
        param_update_organization_request: models::UpdateOrganizationRequest,
        context: &C) -> Result<UpdateOrganizationResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/lead-scraper-microservice/api/v1/organization",
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

        let body = serde_json::to_string(&param_update_organization_request).expect("impossible to fail to serialize");
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
                let body = serde_json::from_str::<models::UpdateOrganizationResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateOrganizationResponse::OrganizationUpdatedSuccessfully
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
                Ok(UpdateOrganizationResponse::BadRequest
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
                Ok(UpdateOrganizationResponse::Unauthorized
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
                Ok(UpdateOrganizationResponse::PaymentRequired
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
                Ok(UpdateOrganizationResponse::Forbidden
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
                Ok(UpdateOrganizationResponse::NotFound
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
                Ok(UpdateOrganizationResponse::MethodNotAllowed
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
                Ok(UpdateOrganizationResponse::Conflict
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
                Ok(UpdateOrganizationResponse::Gone
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
                Ok(UpdateOrganizationResponse::PreconditionFailed
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
                Ok(UpdateOrganizationResponse::UnprocessableEntity
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
                Ok(UpdateOrganizationResponse::TooEarly
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
                Ok(UpdateOrganizationResponse::TooManyRequests
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
                Ok(UpdateOrganizationResponse::InternalServerError
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
                Ok(UpdateOrganizationResponse::NotImplemented
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
                Ok(UpdateOrganizationResponse::BadGateway
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
                Ok(UpdateOrganizationResponse::ServiceUnavailable
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
                Ok(UpdateOrganizationResponse::GatewayTimeout
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
                Ok(UpdateOrganizationResponse::AnUnexpectedErrorResponse
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

    async fn update_tenant(
        &self,
        param_update_tenant_request: models::UpdateTenantRequest,
        context: &C) -> Result<UpdateTenantResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/lead-scraper-microservice/api/v1/organizations/tenants",
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

        let body = serde_json::to_string(&param_update_tenant_request).expect("impossible to fail to serialize");

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
                let body = serde_json::from_str::<models::UpdateTenantResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateTenantResponse::TenantUpdatedSuccessfully
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
                Ok(UpdateTenantResponse::BadRequest
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
                Ok(UpdateTenantResponse::Unauthorized
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
                Ok(UpdateTenantResponse::PaymentRequired
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
                Ok(UpdateTenantResponse::Forbidden
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
                Ok(UpdateTenantResponse::NotFound
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
                Ok(UpdateTenantResponse::MethodNotAllowed
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
                Ok(UpdateTenantResponse::Conflict
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
                Ok(UpdateTenantResponse::Gone
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
                Ok(UpdateTenantResponse::PreconditionFailed
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
                Ok(UpdateTenantResponse::UnprocessableEntity
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
                Ok(UpdateTenantResponse::TooEarly
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
                Ok(UpdateTenantResponse::TooManyRequests
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
                Ok(UpdateTenantResponse::InternalServerError
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
                Ok(UpdateTenantResponse::NotImplemented
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
                Ok(UpdateTenantResponse::BadGateway
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
                Ok(UpdateTenantResponse::ServiceUnavailable
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
                Ok(UpdateTenantResponse::GatewayTimeout
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
                Ok(UpdateTenantResponse::AnUnexpectedErrorResponse
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

    async fn create_account(
        &self,
        param_create_account_request: models::CreateAccountRequest,
        context: &C) -> Result<CreateAccountResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/workspace-service/v1/accounts",
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
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse1>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateAccountResponse::Unauthorized
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
            0 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Status>(body).map_err(|e| {
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

    async fn create_workspace(
        &self,
        param_create_workspace_request: models::CreateWorkspaceRequest,
        context: &C) -> Result<CreateWorkspaceResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/workspace-service/v1/workspaces",
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

        let body = serde_json::to_string(&param_create_workspace_request).expect("impossible to fail to serialize");
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
                let body = serde_json::from_str::<models::CreateWorkspaceResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateWorkspaceResponse::ASuccessfulResponse
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
                let body = serde_json::from_str::<models::CreateWorkspaceResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateWorkspaceResponse::WorkspaceCreatedSuccessfully
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
                Ok(CreateWorkspaceResponse::BadRequest
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
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse1>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateWorkspaceResponse::Unauthorized
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
                Ok(CreateWorkspaceResponse::Forbidden
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
                Ok(CreateWorkspaceResponse::NotFound
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
                Ok(CreateWorkspaceResponse::Conflict
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
                Ok(CreateWorkspaceResponse::TooManyRequests
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
                Ok(CreateWorkspaceResponse::InternalServerError
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
                let body = serde_json::from_str::<models::Status>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateWorkspaceResponse::AnUnexpectedErrorResponse
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
            "{}/workspace-service/v1/accounts/{id}",
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
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse1>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteAccountResponse::Unauthorized
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
            0 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Status>(body).map_err(|e| {
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

    async fn delete_workspace(
        &self,
        param_id: String,
        context: &C) -> Result<DeleteWorkspaceResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/workspace-service/v1/workspaces/{id}",
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
                let body = serde_json::from_str::<models::DeleteWorkspaceResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteWorkspaceResponse::WorkspaceDeletedSuccessfully
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
                Ok(DeleteWorkspaceResponse::BadRequest
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
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse1>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteWorkspaceResponse::Unauthorized
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
                Ok(DeleteWorkspaceResponse::Forbidden
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
                Ok(DeleteWorkspaceResponse::NotFound
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
                Ok(DeleteWorkspaceResponse::Conflict
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
                Ok(DeleteWorkspaceResponse::TooManyRequests
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
                Ok(DeleteWorkspaceResponse::InternalServerError
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
                let body = serde_json::from_str::<models::Status>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(DeleteWorkspaceResponse::AnUnexpectedErrorResponse
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
            "{}/workspace-service/v1/accounts/{id}",
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
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse1>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetAccountResponse::Unauthorized
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
            0 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Status>(body).map_err(|e| {
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

    async fn get_workspace(
        &self,
        param_id: String,
        context: &C) -> Result<GetWorkspaceResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/workspace-service/v1/workspaces/{id}",
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
                let body = serde_json::from_str::<models::GetWorkspaceResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetWorkspaceResponse::WorkspaceRetrievedSuccessfully
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
                Ok(GetWorkspaceResponse::BadRequest
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
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse1>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetWorkspaceResponse::Unauthorized
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
                Ok(GetWorkspaceResponse::Forbidden
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
                Ok(GetWorkspaceResponse::NotFound
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
                Ok(GetWorkspaceResponse::Conflict
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
                Ok(GetWorkspaceResponse::TooManyRequests
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
                Ok(GetWorkspaceResponse::InternalServerError
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
                let body = serde_json::from_str::<models::Status>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetWorkspaceResponse::AnUnexpectedErrorResponse
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

    async fn get_workspace_analytics(
        &self,
        param_workspace_id: String,
        param_start_time: Option<chrono::DateTime::<chrono::Utc>>,
        param_end_time: Option<chrono::DateTime::<chrono::Utc>>,
        context: &C) -> Result<GetWorkspaceAnalyticsResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/workspace-service/v1/workspaces/analytics/{workspace_id}",
            self.base_path
            ,workspace_id=utf8_percent_encode(&param_workspace_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_start_time) = param_start_time {
                query_string.append_pair("startTime",
                    &param_start_time.to_string());
            }
            if let Some(param_end_time) = param_end_time {
                query_string.append_pair("endTime",
                    &param_end_time.to_string());
            }
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
                let body = serde_json::from_str::<models::GetWorkspaceAnalyticsResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetWorkspaceAnalyticsResponse::AnalyticsDataRetrievedSuccessfully
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
                Ok(GetWorkspaceAnalyticsResponse::BadRequest
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
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse1>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetWorkspaceAnalyticsResponse::Unauthorized
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
                Ok(GetWorkspaceAnalyticsResponse::Forbidden
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
                Ok(GetWorkspaceAnalyticsResponse::NotFound
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
                Ok(GetWorkspaceAnalyticsResponse::Conflict
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
                Ok(GetWorkspaceAnalyticsResponse::TooManyRequests
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
                Ok(GetWorkspaceAnalyticsResponse::InternalServerError
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
                let body = serde_json::from_str::<models::Status>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetWorkspaceAnalyticsResponse::AnUnexpectedErrorResponse
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

    async fn get_workspace_compliance_report(
        &self,
        param_workspace_id: String,
        param_compliance_type: Option<String>,
        context: &C) -> Result<GetWorkspaceComplianceReportResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/workspace-service/v1/workspaces/compliance-report/{workspace_id}",
            self.base_path
            ,workspace_id=utf8_percent_encode(&param_workspace_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_compliance_type) = param_compliance_type {
                query_string.append_pair("complianceType",
                    &param_compliance_type);
            }
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
                let body = serde_json::from_str::<models::GetWorkspaceComplianceReportResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetWorkspaceComplianceReportResponse::ComplianceReportRetrievedSuccessfully
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
                Ok(GetWorkspaceComplianceReportResponse::BadRequest
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
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse1>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetWorkspaceComplianceReportResponse::Unauthorized
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
                Ok(GetWorkspaceComplianceReportResponse::Forbidden
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
                Ok(GetWorkspaceComplianceReportResponse::NotFound
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
                Ok(GetWorkspaceComplianceReportResponse::Conflict
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
                Ok(GetWorkspaceComplianceReportResponse::TooManyRequests
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
                Ok(GetWorkspaceComplianceReportResponse::InternalServerError
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
                let body = serde_json::from_str::<models::Status>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetWorkspaceComplianceReportResponse::AnUnexpectedErrorResponse
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

    async fn get_workspace_storage_stats(
        &self,
        param_workspace_id: String,
        context: &C) -> Result<GetWorkspaceStorageStatsResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/workspace-service/v1/workspaces/storage-stats/{workspace_id}",
            self.base_path
            ,workspace_id=utf8_percent_encode(&param_workspace_id.to_string(), ID_ENCODE_SET)
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
                let body = serde_json::from_str::<models::GetWorkspaceStorageStatsResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetWorkspaceStorageStatsResponse::StorageStatisticsRetrievedSuccessfully
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
                Ok(GetWorkspaceStorageStatsResponse::BadRequest
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
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse1>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetWorkspaceStorageStatsResponse::Unauthorized
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
                Ok(GetWorkspaceStorageStatsResponse::Forbidden
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
                Ok(GetWorkspaceStorageStatsResponse::NotFound
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
                Ok(GetWorkspaceStorageStatsResponse::Conflict
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
                Ok(GetWorkspaceStorageStatsResponse::TooManyRequests
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
                Ok(GetWorkspaceStorageStatsResponse::InternalServerError
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
                let body = serde_json::from_str::<models::Status>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetWorkspaceStorageStatsResponse::AnUnexpectedErrorResponse
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

    async fn list_workspace_sharings(
        &self,
        param_workspace_id: String,
        param_page_size: Option<i32>,
        param_page_token: Option<String>,
        context: &C) -> Result<ListWorkspaceSharingsResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/workspace-service/v1/workspaces/sharings/{workspace_id}",
            self.base_path
            ,workspace_id=utf8_percent_encode(&param_workspace_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_page_size) = param_page_size {
                query_string.append_pair("pageSize",
                    &param_page_size.to_string());
            }
            if let Some(param_page_token) = param_page_token {
                query_string.append_pair("pageToken",
                    &param_page_token);
            }
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
                let body = serde_json::from_str::<models::ListWorkspaceSharingsResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListWorkspaceSharingsResponse::WorkspaceSharingsRetrievedSuccessfully
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
                Ok(ListWorkspaceSharingsResponse::BadRequest
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
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse1>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListWorkspaceSharingsResponse::Unauthorized
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
                Ok(ListWorkspaceSharingsResponse::Forbidden
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
                Ok(ListWorkspaceSharingsResponse::NotFound
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
                Ok(ListWorkspaceSharingsResponse::Conflict
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
                Ok(ListWorkspaceSharingsResponse::TooManyRequests
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
                Ok(ListWorkspaceSharingsResponse::InternalServerError
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
                let body = serde_json::from_str::<models::Status>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListWorkspaceSharingsResponse::AnUnexpectedErrorResponse
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

    async fn list_workspaces(
        &self,
        param_account_id: Option<String>,
        param_page_size: Option<i32>,
        param_page_token: Option<String>,
        param_filter: Option<String>,
        context: &C) -> Result<ListWorkspacesResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/workspace-service/v1/workspaces/list",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_account_id) = param_account_id {
                query_string.append_pair("accountId",
                    &param_account_id);
            }
            if let Some(param_page_size) = param_page_size {
                query_string.append_pair("pageSize",
                    &param_page_size.to_string());
            }
            if let Some(param_page_token) = param_page_token {
                query_string.append_pair("pageToken",
                    &param_page_token);
            }
            if let Some(param_filter) = param_filter {
                query_string.append_pair("filter",
                    &param_filter);
            }
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
                let body = serde_json::from_str::<models::ListWorkspacesResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListWorkspacesResponse::WorkspacesRetrievedSuccessfully
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
                Ok(ListWorkspacesResponse::BadRequest
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
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse1>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListWorkspacesResponse::Unauthorized
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
                Ok(ListWorkspacesResponse::Forbidden
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
                Ok(ListWorkspacesResponse::NotFound
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
                Ok(ListWorkspacesResponse::Conflict
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
                Ok(ListWorkspacesResponse::TooManyRequests
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
                Ok(ListWorkspacesResponse::InternalServerError
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
                let body = serde_json::from_str::<models::Status>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ListWorkspacesResponse::AnUnexpectedErrorResponse
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

    async fn remove_workspace_sharing(
        &self,
        param_sharing_id: String,
        context: &C) -> Result<RemoveWorkspaceSharingResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/workspace-service/v1/workspace-sharings/{sharing_id}",
            self.base_path
            ,sharing_id=utf8_percent_encode(&param_sharing_id.to_string(), ID_ENCODE_SET)
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
                let body = serde_json::from_str::<models::RemoveWorkspaceSharingResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(RemoveWorkspaceSharingResponse::WorkspaceSharingRemovedSuccessfully
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
                Ok(RemoveWorkspaceSharingResponse::BadRequest
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
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse1>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(RemoveWorkspaceSharingResponse::Unauthorized
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
                Ok(RemoveWorkspaceSharingResponse::Forbidden
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
                Ok(RemoveWorkspaceSharingResponse::NotFound
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
                Ok(RemoveWorkspaceSharingResponse::Conflict
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
                Ok(RemoveWorkspaceSharingResponse::TooManyRequests
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
                Ok(RemoveWorkspaceSharingResponse::InternalServerError
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
                let body = serde_json::from_str::<models::Status>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(RemoveWorkspaceSharingResponse::AnUnexpectedErrorResponse
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

    async fn share_workspace(
        &self,
        param_workspace_id: String,
        param_share_workspace_body: models::ShareWorkspaceBody,
        context: &C) -> Result<ShareWorkspaceResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/workspace-service/v1/workspaces/{workspace_id}/share",
            self.base_path
            ,workspace_id=utf8_percent_encode(&param_workspace_id.to_string(), ID_ENCODE_SET)
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

        let body = serde_json::to_string(&param_share_workspace_body).expect("impossible to fail to serialize");
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
                let body = serde_json::from_str::<models::ShareWorkspaceResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ShareWorkspaceResponse::WorkspaceSharedSuccessfully
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
                Ok(ShareWorkspaceResponse::BadRequest
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
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse1>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ShareWorkspaceResponse::Unauthorized
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
                Ok(ShareWorkspaceResponse::Forbidden
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
                Ok(ShareWorkspaceResponse::NotFound
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
                Ok(ShareWorkspaceResponse::Conflict
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
                Ok(ShareWorkspaceResponse::TooManyRequests
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
                Ok(ShareWorkspaceResponse::InternalServerError
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
                let body = serde_json::from_str::<models::Status>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ShareWorkspaceResponse::AnUnexpectedErrorResponse
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
            "{}/workspace-service/v1/accounts",
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
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse1>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateAccountResponse::Unauthorized
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
            0 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Status>(body).map_err(|e| {
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

    async fn update_workspace(
        &self,
        param_update_workspace_request: models::UpdateWorkspaceRequest,
        context: &C) -> Result<UpdateWorkspaceResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/workspace-service/v1/workspaces",
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

        let body = serde_json::to_string(&param_update_workspace_request).expect("impossible to fail to serialize");
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
                let body = serde_json::from_str::<models::UpdateWorkspaceResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateWorkspaceResponse::WorkspaceUpdatedSuccessfully
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
                Ok(UpdateWorkspaceResponse::BadRequest
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
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse1>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateWorkspaceResponse::Unauthorized
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
                Ok(UpdateWorkspaceResponse::Forbidden
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
                Ok(UpdateWorkspaceResponse::NotFound
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
                Ok(UpdateWorkspaceResponse::Conflict
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
                Ok(UpdateWorkspaceResponse::TooManyRequests
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
                Ok(UpdateWorkspaceResponse::InternalServerError
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
                let body = serde_json::from_str::<models::Status>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateWorkspaceResponse::AnUnexpectedErrorResponse
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

    async fn update_workspace_sharing(
        &self,
        param_update_workspace_sharing_request: models::UpdateWorkspaceSharingRequest,
        context: &C) -> Result<UpdateWorkspaceSharingResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/workspace-service/v1/workspace-sharings",
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

        let body = serde_json::to_string(&param_update_workspace_sharing_request).expect("impossible to fail to serialize");

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
                let body = serde_json::from_str::<models::UpdateWorkspaceSharingResponse>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateWorkspaceSharingResponse::WorkspaceSharingUpdatedSuccessfully
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
                Ok(UpdateWorkspaceSharingResponse::BadRequest
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
                let body = serde_json::from_str::<models::AuthenticationErrorMessageResponse1>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateWorkspaceSharingResponse::Unauthorized
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
                Ok(UpdateWorkspaceSharingResponse::Forbidden
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
                Ok(UpdateWorkspaceSharingResponse::NotFound
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
                Ok(UpdateWorkspaceSharingResponse::Conflict
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
                Ok(UpdateWorkspaceSharingResponse::TooManyRequests
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
                Ok(UpdateWorkspaceSharingResponse::InternalServerError
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
                let body = serde_json::from_str::<models::Status>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(UpdateWorkspaceSharingResponse::AnUnexpectedErrorResponse
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
