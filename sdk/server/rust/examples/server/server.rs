//! Main library entry point for openapi_client implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{Ssl, SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

use openapi_client::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    // This pushes a fourth layer of the middleware-stack even though Swagger assumes only three levels.
    // This fourth layer creates an accept-all policy, hower the example-code already acchieves the same via a Bearer-token with full permissions, so next line is not needed (anymore).  
    // let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    #[allow(unused_mut)]
    let mut service =
        openapi_client::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set certificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = ssl.build();
            let tcp_listener = TcpListener::bind(&addr).await.unwrap();

            info!("Starting a server (with https)");
            loop {
                if let Ok((tcp, _)) = tcp_listener.accept().await {
                    let ssl = Ssl::new(tls_acceptor.context()).unwrap();
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::SslStream::new(ssl, tcp).map_err(|_| ())?;
                        let service = service.await.map_err(|_| ())?;

                        Http::new()
                            .serve_connection(tls, service)
                            .await
                            .map_err(|_| ())
                    });
                }
            }
        }
    } else {
        info!("Starting a server (over http, so no TLS)");
        // Using HTTP
        hyper::server::Server::bind(&addr).serve(service).await.unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}


use jsonwebtoken::{decode, encode, errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use swagger::auth::Authorization;
use crate::server_auth;


use openapi_client::{
    Api,
    CreateAccountResponse,
    CreateScrapingJobResponse,
    DeleteAccountResponse,
    DeleteScrapingJobResponse,
    DownloadScrapingResultsResponse,
    GetAccountResponse,
    GetScrapingJobResponse,
    ListAccountsResponse,
    ListScrapingJobsResponse,
    UpdateAccountResponse,
};
use openapi_client::server::MakeService;
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    /// Create a new account
    async fn create_account(
        &self,
        create_account_request: models::CreateAccountRequest,
        context: &C) -> Result<CreateAccountResponse, ApiError>
    {
        info!("create_account({:?}) - X-Span-ID: {:?}", create_account_request, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Create a new job scraping task
    async fn create_scraping_job(
        &self,
        create_scraping_job_request: models::CreateScrapingJobRequest,
        context: &C) -> Result<CreateScrapingJobResponse, ApiError>
    {
        info!("create_scraping_job({:?}) - X-Span-ID: {:?}", create_scraping_job_request, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Delete account
    async fn delete_account(
        &self,
        account_id: String,
        org_id: String,
        tenant_id: String,
        context: &C) -> Result<DeleteAccountResponse, ApiError>
    {
        info!("delete_account(\"{}\", \"{}\", \"{}\") - X-Span-ID: {:?}", account_id, org_id, tenant_id, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Delete a specific job
    async fn delete_scraping_job(
        &self,
        job_id: String,
        user_id: String,
        org_id: String,
        tenant_id: String,
        context: &C) -> Result<DeleteScrapingJobResponse, ApiError>
    {
        info!("delete_scraping_job(\"{}\", \"{}\", \"{}\", \"{}\") - X-Span-ID: {:?}", job_id, user_id, org_id, tenant_id, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Download job results as CSV
    async fn download_scraping_results(
        &self,
        job_id: String,
        user_id: String,
        org_id: String,
        tenant_id: String,
        context: &C) -> Result<DownloadScrapingResultsResponse, ApiError>
    {
        info!("download_scraping_results(\"{}\", \"{}\", \"{}\", \"{}\") - X-Span-ID: {:?}", job_id, user_id, org_id, tenant_id, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Get account details
    async fn get_account(
        &self,
        account_id: String,
        org_id: String,
        tenant_id: String,
        context: &C) -> Result<GetAccountResponse, ApiError>
    {
        info!("get_account(\"{}\", \"{}\", \"{}\") - X-Span-ID: {:?}", account_id, org_id, tenant_id, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Get a specific job
    async fn get_scraping_job(
        &self,
        job_id: String,
        user_id: String,
        org_id: String,
        tenant_id: String,
        context: &C) -> Result<GetScrapingJobResponse, ApiError>
    {
        info!("get_scraping_job(\"{}\", \"{}\", \"{}\", \"{}\") - X-Span-ID: {:?}", job_id, user_id, org_id, tenant_id, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// List accounts
    async fn list_accounts(
        &self,
        org_id: String,
        tenant_id: String,
        offset: Option<i32>,
        limit: Option<i32>,
        context: &C) -> Result<ListAccountsResponse, ApiError>
    {
        info!("list_accounts(\"{}\", \"{}\", {:?}, {:?}) - X-Span-ID: {:?}", org_id, tenant_id, offset, limit, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Get all jobs
    async fn list_scraping_jobs(
        &self,
        user_id: String,
        org_id: String,
        tenant_id: String,
        context: &C) -> Result<ListScrapingJobsResponse, ApiError>
    {
        info!("list_scraping_jobs(\"{}\", \"{}\", \"{}\") - X-Span-ID: {:?}", user_id, org_id, tenant_id, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Update account details
    async fn update_account(
        &self,
        update_account_request: models::UpdateAccountRequest,
        context: &C) -> Result<UpdateAccountResponse, ApiError>
    {
        info!("update_account({:?}) - X-Span-ID: {:?}", update_account_request, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

}
