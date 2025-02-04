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
    CreateOrganizationResponse,
    CreateScrapingJobResponse,
    CreateTenantResponse,
    CreateWorkspaceResponse,
    DeleteAccountResponse,
    DeleteOrganizationResponse,
    DeleteScrapingJobResponse,
    DeleteTenantResponse,
    DeleteWorkspaceResponse,
    DownloadScrapingResultsResponse,
    GetAccountResponse,
    GetAccountUsageResponse,
    GetOrganizationResponse,
    GetScrapingJobResponse,
    GetTenantResponse,
    GetWorkflowResponse,
    GetWorkspaceResponse,
    GetWorkspaceAnalyticsResponse,
    LeadScraperServiceCreateWorkflowResponse,
    ListAccountsResponse,
    ListOrganizationsResponse,
    ListScrapingJobsResponse,
    ListTenantsResponse,
    ListWorkflowsResponse,
    ListWorkspacesResponse,
    PauseWorkflowResponse,
    TriggerWorkflowResponse,
    UpdateAccountResponse,
    UpdateAccountSettingsResponse,
    UpdateOrganizationResponse,
    UpdateTenantResponse,
    UpdateWorkflowResponse,
    UpdateWorkspaceResponse,
    CreateAccount1Response,
    CreateWorkspace1Response,
    DeleteAccount1Response,
    DeleteWorkspace1Response,
    GetAccount1Response,
    GetWorkspace1Response,
    GetWorkspaceAnalytics1Response,
    GetWorkspaceComplianceReportResponse,
    GetWorkspaceStorageStatsResponse,
    ListWorkspaceSharingsResponse,
    ListWorkspaces1Response,
    RemoveWorkspaceSharingResponse,
    ShareWorkspaceResponse,
    UpdateAccount1Response,
    UpdateWorkspace1Response,
    UpdateWorkspaceSharingResponse,
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

    /// Create a new organization
    async fn create_organization(
        &self,
        create_organization_request: models::CreateOrganizationRequest,
        context: &C) -> Result<CreateOrganizationResponse, ApiError>
    {
        info!("create_organization({:?}) - X-Span-ID: {:?}", create_organization_request, context.get().0.clone());
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

    /// Create a new tenant
    async fn create_tenant(
        &self,
        organization_id: String,
        create_tenant_body: models::CreateTenantBody,
        context: &C) -> Result<CreateTenantResponse, ApiError>
    {
        info!("create_tenant(\"{}\", {:?}) - X-Span-ID: {:?}", organization_id, create_tenant_body, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Create a new workspace
    async fn create_workspace(
        &self,
        create_workspace_request: models::CreateWorkspaceRequest,
        context: &C) -> Result<CreateWorkspaceResponse, ApiError>
    {
        info!("create_workspace({:?}) - X-Span-ID: {:?}", create_workspace_request, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Delete account
    async fn delete_account(
        &self,
        id: String,
        context: &C) -> Result<DeleteAccountResponse, ApiError>
    {
        info!("delete_account(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Delete an organization
    async fn delete_organization(
        &self,
        id: String,
        context: &C) -> Result<DeleteOrganizationResponse, ApiError>
    {
        info!("delete_organization(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
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

    /// Delete a tenant
    async fn delete_tenant(
        &self,
        organization_id: String,
        tenant_id: String,
        context: &C) -> Result<DeleteTenantResponse, ApiError>
    {
        info!("delete_tenant(\"{}\", \"{}\") - X-Span-ID: {:?}", organization_id, tenant_id, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Delete a workspace
    async fn delete_workspace(
        &self,
        id: String,
        context: &C) -> Result<DeleteWorkspaceResponse, ApiError>
    {
        info!("delete_workspace(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
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
        id: String,
        context: &C) -> Result<GetAccountResponse, ApiError>
    {
        info!("get_account(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Get account usage
    async fn get_account_usage(
        &self,
        id: String,
        context: &C) -> Result<GetAccountUsageResponse, ApiError>
    {
        info!("get_account_usage(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Get organization details
    async fn get_organization(
        &self,
        id: String,
        context: &C) -> Result<GetOrganizationResponse, ApiError>
    {
        info!("get_organization(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
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

    /// Get tenant details
    async fn get_tenant(
        &self,
        organization_id: String,
        tenant_id: String,
        context: &C) -> Result<GetTenantResponse, ApiError>
    {
        info!("get_tenant(\"{}\", \"{}\") - X-Span-ID: {:?}", organization_id, tenant_id, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Get workflow details
    async fn get_workflow(
        &self,
        workspace_id: String,
        id: String,
        context: &C) -> Result<GetWorkflowResponse, ApiError>
    {
        info!("get_workflow(\"{}\", \"{}\") - X-Span-ID: {:?}", workspace_id, id, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Get workspace details
    async fn get_workspace(
        &self,
        id: String,
        context: &C) -> Result<GetWorkspaceResponse, ApiError>
    {
        info!("get_workspace(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Get workspace analytics
    async fn get_workspace_analytics(
        &self,
        workspace_id: String,
        start_time: Option<chrono::DateTime::<chrono::Utc>>,
        end_time: Option<chrono::DateTime::<chrono::Utc>>,
        context: &C) -> Result<GetWorkspaceAnalyticsResponse, ApiError>
    {
        info!("get_workspace_analytics(\"{}\", {:?}, {:?}) - X-Span-ID: {:?}", workspace_id, start_time, end_time, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Create a new workflow
    async fn lead_scraper_service_create_workflow(
        &self,
        workspace_id: String,
        create_workflow_body: models::CreateWorkflowBody,
        context: &C) -> Result<LeadScraperServiceCreateWorkflowResponse, ApiError>
    {
        info!("lead_scraper_service_create_workflow(\"{}\", {:?}) - X-Span-ID: {:?}", workspace_id, create_workflow_body, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// List all accounts
    async fn list_accounts(
        &self,
        page_size: Option<i32>,
        page_number: Option<i32>,
        filter: Option<String>,
        context: &C) -> Result<ListAccountsResponse, ApiError>
    {
        info!("list_accounts({:?}, {:?}, {:?}) - X-Span-ID: {:?}", page_size, page_number, filter, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// List all organizations
    async fn list_organizations(
        &self,
        page_size: Option<i32>,
        page_number: Option<i32>,
        context: &C) -> Result<ListOrganizationsResponse, ApiError>
    {
        info!("list_organizations({:?}, {:?}) - X-Span-ID: {:?}", page_size, page_number, context.get().0.clone());
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

    /// List all tenants
    async fn list_tenants(
        &self,
        organization_id: String,
        page_size: Option<i32>,
        page_number: Option<i32>,
        context: &C) -> Result<ListTenantsResponse, ApiError>
    {
        info!("list_tenants(\"{}\", {:?}, {:?}) - X-Span-ID: {:?}", organization_id, page_size, page_number, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// List workflows
    async fn list_workflows(
        &self,
        workspace_id: String,
        page_size: Option<i32>,
        page_number: Option<i32>,
        filter: Option<String>,
        context: &C) -> Result<ListWorkflowsResponse, ApiError>
    {
        info!("list_workflows(\"{}\", {:?}, {:?}, {:?}) - X-Span-ID: {:?}", workspace_id, page_size, page_number, filter, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// List workspaces
    async fn list_workspaces(
        &self,
        account_id: Option<String>,
        page_size: Option<i32>,
        page_number: Option<i32>,
        context: &C) -> Result<ListWorkspacesResponse, ApiError>
    {
        info!("list_workspaces({:?}, {:?}, {:?}) - X-Span-ID: {:?}", account_id, page_size, page_number, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Pause workflow execution
    async fn pause_workflow(
        &self,
        workspace_id: String,
        id: String,
        pause_workflow_body: models::PauseWorkflowBody,
        context: &C) -> Result<PauseWorkflowResponse, ApiError>
    {
        info!("pause_workflow(\"{}\", \"{}\", {:?}) - X-Span-ID: {:?}", workspace_id, id, pause_workflow_body, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Trigger workflow execution
    async fn trigger_workflow(
        &self,
        workspace_id: String,
        id: String,
        trigger_workflow_body: models::TriggerWorkflowBody,
        context: &C) -> Result<TriggerWorkflowResponse, ApiError>
    {
        info!("trigger_workflow(\"{}\", \"{}\", {:?}) - X-Span-ID: {:?}", workspace_id, id, trigger_workflow_body, context.get().0.clone());
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

    /// Update account settings
    async fn update_account_settings(
        &self,
        update_account_settings_request: models::UpdateAccountSettingsRequest,
        context: &C) -> Result<UpdateAccountSettingsResponse, ApiError>
    {
        info!("update_account_settings({:?}) - X-Span-ID: {:?}", update_account_settings_request, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Update organization details
    async fn update_organization(
        &self,
        update_organization_request: models::UpdateOrganizationRequest,
        context: &C) -> Result<UpdateOrganizationResponse, ApiError>
    {
        info!("update_organization({:?}) - X-Span-ID: {:?}", update_organization_request, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Update tenant details
    async fn update_tenant(
        &self,
        update_tenant_request: models::UpdateTenantRequest,
        context: &C) -> Result<UpdateTenantResponse, ApiError>
    {
        info!("update_tenant({:?}) - X-Span-ID: {:?}", update_tenant_request, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Update workflow details
    async fn update_workflow(
        &self,
        update_workflow_request: models::UpdateWorkflowRequest,
        context: &C) -> Result<UpdateWorkflowResponse, ApiError>
    {
        info!("update_workflow({:?}) - X-Span-ID: {:?}", update_workflow_request, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Update workspace details
    async fn update_workspace(
        &self,
        update_workspace_request: models::UpdateWorkspaceRequest,
        context: &C) -> Result<UpdateWorkspaceResponse, ApiError>
    {
        info!("update_workspace({:?}) - X-Span-ID: {:?}", update_workspace_request, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Create a new account
    async fn create_account1(
        &self,
        create_account_request1: models::CreateAccountRequest1,
        context: &C) -> Result<CreateAccount1Response, ApiError>
    {
        info!("create_account1({:?}) - X-Span-ID: {:?}", create_account_request1, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Create workspace
    async fn create_workspace1(
        &self,
        create_workspace_request1: models::CreateWorkspaceRequest1,
        context: &C) -> Result<CreateWorkspace1Response, ApiError>
    {
        info!("create_workspace1({:?}) - X-Span-ID: {:?}", create_workspace_request1, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Delete account
    async fn delete_account1(
        &self,
        id: String,
        context: &C) -> Result<DeleteAccount1Response, ApiError>
    {
        info!("delete_account1(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Delete workspace
    async fn delete_workspace1(
        &self,
        id: String,
        context: &C) -> Result<DeleteWorkspace1Response, ApiError>
    {
        info!("delete_workspace1(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Get account details
    async fn get_account1(
        &self,
        id: String,
        context: &C) -> Result<GetAccount1Response, ApiError>
    {
        info!("get_account1(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Get workspace details
    async fn get_workspace1(
        &self,
        id: String,
        context: &C) -> Result<GetWorkspace1Response, ApiError>
    {
        info!("get_workspace1(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Get workspace analytics
    async fn get_workspace_analytics1(
        &self,
        workspace_id: String,
        start_time: Option<chrono::DateTime::<chrono::Utc>>,
        end_time: Option<chrono::DateTime::<chrono::Utc>>,
        context: &C) -> Result<GetWorkspaceAnalytics1Response, ApiError>
    {
        info!("get_workspace_analytics1(\"{}\", {:?}, {:?}) - X-Span-ID: {:?}", workspace_id, start_time, end_time, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Get compliance report
    async fn get_workspace_compliance_report(
        &self,
        workspace_id: String,
        compliance_type: Option<String>,
        context: &C) -> Result<GetWorkspaceComplianceReportResponse, ApiError>
    {
        info!("get_workspace_compliance_report(\"{}\", {:?}) - X-Span-ID: {:?}", workspace_id, compliance_type, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Get storage statistics
    async fn get_workspace_storage_stats(
        &self,
        workspace_id: String,
        context: &C) -> Result<GetWorkspaceStorageStatsResponse, ApiError>
    {
        info!("get_workspace_storage_stats(\"{}\") - X-Span-ID: {:?}", workspace_id, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// List workspace sharings
    async fn list_workspace_sharings(
        &self,
        workspace_id: String,
        page_size: Option<i32>,
        page_token: Option<String>,
        context: &C) -> Result<ListWorkspaceSharingsResponse, ApiError>
    {
        info!("list_workspace_sharings(\"{}\", {:?}, {:?}) - X-Span-ID: {:?}", workspace_id, page_size, page_token, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// List workspaces
    async fn list_workspaces1(
        &self,
        account_id: Option<String>,
        page_size: Option<i32>,
        page_token: Option<String>,
        filter: Option<String>,
        context: &C) -> Result<ListWorkspaces1Response, ApiError>
    {
        info!("list_workspaces1({:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}", account_id, page_size, page_token, filter, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Remove workspace sharing
    async fn remove_workspace_sharing(
        &self,
        sharing_id: String,
        context: &C) -> Result<RemoveWorkspaceSharingResponse, ApiError>
    {
        info!("remove_workspace_sharing(\"{}\") - X-Span-ID: {:?}", sharing_id, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Share workspace
    async fn share_workspace(
        &self,
        workspace_id: String,
        share_workspace_body: models::ShareWorkspaceBody,
        context: &C) -> Result<ShareWorkspaceResponse, ApiError>
    {
        info!("share_workspace(\"{}\", {:?}) - X-Span-ID: {:?}", workspace_id, share_workspace_body, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Update account details
    async fn update_account1(
        &self,
        update_account_request: models::UpdateAccountRequest,
        context: &C) -> Result<UpdateAccount1Response, ApiError>
    {
        info!("update_account1({:?}) - X-Span-ID: {:?}", update_account_request, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Update workspace
    async fn update_workspace1(
        &self,
        update_workspace_request: models::UpdateWorkspaceRequest,
        context: &C) -> Result<UpdateWorkspace1Response, ApiError>
    {
        info!("update_workspace1({:?}) - X-Span-ID: {:?}", update_workspace_request, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

    /// Update workspace sharing
    async fn update_workspace_sharing(
        &self,
        update_workspace_sharing_request: models::UpdateWorkspaceSharingRequest,
        context: &C) -> Result<UpdateWorkspaceSharingResponse, ApiError>
    {
        info!("update_workspace_sharing({:?}) - X-Span-ID: {:?}", update_workspace_sharing_request, context.get().0.clone());
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
    }

}
