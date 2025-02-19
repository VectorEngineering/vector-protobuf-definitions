#![allow(missing_docs, unused_variables, trivial_casts)]


#[allow(unused_imports)]
use futures::{future, Stream, stream};
#[allow(unused_imports)]
use openapi_client::{Api, ApiNoContext, Claims, Client, ContextWrapperExt, models,
                      CreateAccountResponse,
                      CreateApiKeyResponse,
                      CreateOrganizationResponse,
                      CreateScrapingJobResponse,
                      CreateTenantResponse,
                      CreateTenantApiKeyResponse,
                      CreateWebhookResponse,
                      CreateWorkflowResponse,
                      CreateWorkspaceResponse,
                      DeleteAccountResponse,
                      DeleteApiKeyResponse,
                      DeleteOrganizationResponse,
                      DeleteScrapingJobResponse,
                      DeleteTenantResponse,
                      DeleteTenantApiKeyResponse,
                      DeleteWebhookResponse,
                      DeleteWorkflowResponse,
                      DeleteWorkspaceResponse,
                      DownloadScrapingResultsResponse,
                      GetAccountResponse,
                      GetAccountUsageResponse,
                      GetApiKeyResponse,
                      GetLeadResponse,
                      GetOrganizationResponse,
                      GetScrapingJobResponse,
                      GetTenantResponse,
                      GetTenantApiKeyResponse,
                      GetWebhookResponse,
                      GetWorkflowResponse,
                      GetWorkspaceResponse,
                      GetWorkspaceAnalyticsResponse,
                      ListAccountsResponse,
                      ListApiKeysResponse,
                      ListLeadsResponse,
                      ListOrganizationsResponse,
                      ListScrapingJobsResponse,
                      ListTenantApiKeysResponse,
                      ListTenantsResponse,
                      ListWebhooksResponse,
                      ListWorkflowsResponse,
                      ListWorkspacesResponse,
                      PauseWorkflowResponse,
                      RotateApiKeyResponse,
                      RotateTenantApiKeyResponse,
                      TriggerWorkflowResponse,
                      UpdateAccountResponse,
                      UpdateAccountSettingsResponse,
                      UpdateApiKeyResponse,
                      UpdateOrganizationResponse,
                      UpdateTenantResponse,
                      UpdateTenantApiKeyResponse,
                      UpdateWebhookResponse,
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
use clap::{App, Arg};

// NOTE: Set environment variable RUST_LOG to the name of the executable (or "cargo run") to activate console logging for all loglevels.
//     See https://docs.rs/env_logger/latest/env_logger/  for more details

#[allow(unused_imports)]
use log::info;

// swagger::Has may be unused if there are no examples
#[allow(unused_imports)]
use swagger::{AuthData, ContextBuilder, EmptyContext, Has, Push, XSpanIdString};

type ClientContext = swagger::make_context_ty!(ContextBuilder, EmptyContext, Option<AuthData>, XSpanIdString);

mod client_auth;
use client_auth::build_token;


// rt may be unused if there are no examples
#[allow(unused_mut)]
fn main() {
    env_logger::init();

    let matches = App::new("client")
        .arg(Arg::with_name("operation")
            .help("Sets the operation to run")
            .possible_values(&[
                "DeleteAccount", 
                "DeleteApiKey", 
                "DeleteOrganization", 
                "DeleteScrapingJob", 
                "DeleteTenant", 
                "DeleteTenantApiKey", 
                "DeleteWebhook", 
                "DeleteWorkflow", 
                "DeleteWorkspace", 
                "DownloadScrapingResults", 
                "GetAccount", 
                "GetAccountUsage", 
                "GetApiKey", 
                "GetLead", 
                "GetOrganization", 
                "GetScrapingJob", 
                "GetTenant", 
                "GetTenantApiKey", 
                "GetWebhook", 
                "GetWorkflow", 
                "GetWorkspace", 
                "GetWorkspaceAnalytics", 
                "ListAccounts", 
                "ListApiKeys", 
                "ListLeads", 
                "ListOrganizations", 
                "ListScrapingJobs", 
                "ListTenantApiKeys", 
                "ListTenants", 
                "ListWebhooks", 
                "ListWorkflows", 
                "ListWorkspaces", 
                "DeleteAccount1", 
                "DeleteWorkspace1", 
                "GetAccount1", 
                "GetWorkspace1", 
                "GetWorkspaceAnalytics1", 
                "GetWorkspaceComplianceReport", 
                "GetWorkspaceStorageStats", 
                "ListWorkspaceSharings", 
                "ListWorkspaces1", 
                "RemoveWorkspaceSharing", 
            ])
            .required(true)
            .index(1))
        .arg(Arg::with_name("https")
            .long("https")
            .help("Whether to use HTTPS or not"))
        .arg(Arg::with_name("host")
            .long("host")
            .takes_value(true)
            .default_value("lead-scraping-microservice.vector.svc.cluster.local")
            .help("Hostname to contact"))
        .arg(Arg::with_name("port")
            .long("port")
            .takes_value(true)
            .default_value("9896")
            .help("Port to contact"))
        .get_matches();

    // Create Bearer-token with a fixed key (secret) for test purposes.
    // In a real (production) system this Bearer token should be obtained via an external Identity/Authentication-server
    // Ensure that you set the correct algorithm and encodingkey that matches what is used on the server side.
    // See https://github.com/Keats/jsonwebtoken for more information

    let auth_token = build_token(
            Claims {
                sub: "tester@acme.com".to_owned(), 
                company: "ACME".to_owned(),
                iss: "my_identity_provider".to_owned(),
                // added a very long expiry time
                aud: "org.acme.Resource_Server".to_string(),
                exp: 10000000000,
                // In this example code all available Scopes are added, so the current Bearer Token gets fully authorization.
                scopes: [
                ].join(", ")
            }, 
            b"secret").unwrap();

    let auth_data = if !auth_token.is_empty() {
        Some(AuthData::Bearer(swagger::auth::Bearer { token: auth_token}))
    } else {
        // No Bearer-token available, so return None
        None
    };

    let is_https = matches.is_present("https");
    let base_url = format!("{}://{}:{}",
        if is_https { "https" } else { "http" },
        matches.value_of("host").unwrap(),
        matches.value_of("port").unwrap());

    let context: ClientContext =
        swagger::make_context!(ContextBuilder, EmptyContext, auth_data, XSpanIdString::default());

    let mut client : Box<dyn ApiNoContext<ClientContext>> = if matches.is_present("https") {
        // Using Simple HTTPS
        let client = Box::new(Client::try_new_https(&base_url)
            .expect("Failed to create HTTPS client"));
        Box::new(client.with_context(context))
    } else {
        // Using HTTP
        let client = Box::new(Client::try_new_http(
            &base_url)
            .expect("Failed to create HTTP client"));
        Box::new(client.with_context(context))
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    match matches.value_of("operation") {
        /* Disabled because there's no example.
        Some("CreateAccount") => {
            let result = rt.block_on(client.create_account(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("CreateApiKey") => {
            let result = rt.block_on(client.create_api_key(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("CreateOrganization") => {
            let result = rt.block_on(client.create_organization(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("CreateScrapingJob") => {
            let result = rt.block_on(client.create_scraping_job(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("CreateTenant") => {
            let result = rt.block_on(client.create_tenant(
                  "organization_id_example".to_string(),
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("CreateTenantApiKey") => {
            let result = rt.block_on(client.create_tenant_api_key(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("CreateWebhook") => {
            let result = rt.block_on(client.create_webhook(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("CreateWorkflow") => {
            let result = rt.block_on(client.create_workflow(
                  "workspace_id_example".to_string(),
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("CreateWorkspace") => {
            let result = rt.block_on(client.create_workspace(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("DeleteAccount") => {
            let result = rt.block_on(client.delete_account(
                  "id_example".to_string(),
                  Some("organization_id_example".to_string()),
                  Some("tenant_id_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("DeleteApiKey") => {
            let result = rt.block_on(client.delete_api_key(
                  "key_id_example".to_string(),
                  Some("organization_id_example".to_string()),
                  Some("workspace_id_example".to_string()),
                  Some("tenant_id_example".to_string()),
                  Some("account_id_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("DeleteOrganization") => {
            let result = rt.block_on(client.delete_organization(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("DeleteScrapingJob") => {
            let result = rt.block_on(client.delete_scraping_job(
                  "job_id_example".to_string(),
                  "user_id_example".to_string(),
                  "org_id_example".to_string(),
                  "tenant_id_example".to_string(),
                  Some("workspace_id_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("DeleteTenant") => {
            let result = rt.block_on(client.delete_tenant(
                  "organization_id_example".to_string(),
                  "tenant_id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("DeleteTenantApiKey") => {
            let result = rt.block_on(client.delete_tenant_api_key(
                  "key_id_example".to_string(),
                  Some("organization_id_example".to_string()),
                  Some("tenant_id_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("DeleteWebhook") => {
            let result = rt.block_on(client.delete_webhook(
                  "webhook_id_example".to_string(),
                  Some("organization_id_example".to_string()),
                  Some("workspace_id_example".to_string()),
                  Some("tenant_id_example".to_string()),
                  Some("account_id_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("DeleteWorkflow") => {
            let result = rt.block_on(client.delete_workflow(
                  "workspace_id_example".to_string(),
                  "id_example".to_string(),
                  "org_id_example".to_string(),
                  "tenant_id_example".to_string(),
                  Some("account_id_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("DeleteWorkspace") => {
            let result = rt.block_on(client.delete_workspace(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("DownloadScrapingResults") => {
            let result = rt.block_on(client.download_scraping_results(
                  "job_id_example".to_string(),
                  "user_id_example".to_string(),
                  "org_id_example".to_string(),
                  "tenant_id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetAccount") => {
            let result = rt.block_on(client.get_account(
                  "id_example".to_string(),
                  Some("organization_id_example".to_string()),
                  Some("tenant_id_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetAccountUsage") => {
            let result = rt.block_on(client.get_account_usage(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetApiKey") => {
            let result = rt.block_on(client.get_api_key(
                  "key_id_example".to_string(),
                  Some("organization_id_example".to_string()),
                  Some("tenant_id_example".to_string()),
                  Some("account_id_example".to_string()),
                  Some("workspace_id_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetLead") => {
            let result = rt.block_on(client.get_lead(
                  "lead_id_example".to_string(),
                  Some("organization_id_example".to_string()),
                  Some("workspace_id_example".to_string()),
                  Some("tenant_id_example".to_string()),
                  Some("account_id_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetOrganization") => {
            let result = rt.block_on(client.get_organization(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetScrapingJob") => {
            let result = rt.block_on(client.get_scraping_job(
                  "job_id_example".to_string(),
                  "user_id_example".to_string(),
                  "org_id_example".to_string(),
                  "tenant_id_example".to_string(),
                  Some("workspace_id_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetTenant") => {
            let result = rt.block_on(client.get_tenant(
                  "organization_id_example".to_string(),
                  "tenant_id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetTenantApiKey") => {
            let result = rt.block_on(client.get_tenant_api_key(
                  "key_id_example".to_string(),
                  Some("organization_id_example".to_string()),
                  Some("tenant_id_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetWebhook") => {
            let result = rt.block_on(client.get_webhook(
                  "webhook_id_example".to_string(),
                  Some("organization_id_example".to_string()),
                  Some("workspace_id_example".to_string()),
                  Some("tenant_id_example".to_string()),
                  Some("account_id_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetWorkflow") => {
            let result = rt.block_on(client.get_workflow(
                  "workspace_id_example".to_string(),
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetWorkspace") => {
            let result = rt.block_on(client.get_workspace(
                  "id_example".to_string(),
                  Some("organization_id_example".to_string()),
                  Some("tenant_id_example".to_string()),
                  Some("account_id_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetWorkspaceAnalytics") => {
            let result = rt.block_on(client.get_workspace_analytics(
                  "workspace_id_example".to_string(),
                  None,
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("ListAccounts") => {
            let result = rt.block_on(client.list_accounts(
                  Some(56),
                  Some(56),
                  Some("filter_example".to_string()),
                  Some("organization_id_example".to_string()),
                  Some("tenant_id_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("ListApiKeys") => {
            let result = rt.block_on(client.list_api_keys(
                  Some("organization_id_example".to_string()),
                  Some("tenant_id_example".to_string()),
                  Some("account_id_example".to_string()),
                  Some("workspace_id_example".to_string()),
                  Some(56),
                  Some(56),
                  Some("status_example".to_string()),
                  Some("search_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("ListLeads") => {
            let result = rt.block_on(client.list_leads(
                  Some("organization_id_example".to_string()),
                  Some("workspace_id_example".to_string()),
                  Some("tenant_id_example".to_string()),
                  Some("account_id_example".to_string()),
                  Some(56),
                  Some(56)
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("ListOrganizations") => {
            let result = rt.block_on(client.list_organizations(
                  Some(56),
                  Some(56)
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("ListScrapingJobs") => {
            let result = rt.block_on(client.list_scraping_jobs(
                  "auth_platform_user_id_example".to_string(),
                  "org_id_example".to_string(),
                  "tenant_id_example".to_string(),
                  Some(56),
                  Some(56),
                  Some("workspace_id_example".to_string()),
                  Some("workflow_id_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("ListTenantApiKeys") => {
            let result = rt.block_on(client.list_tenant_api_keys(
                  Some("organization_id_example".to_string()),
                  Some("tenant_id_example".to_string()),
                  Some(56),
                  Some(56),
                  Some("status_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("ListTenants") => {
            let result = rt.block_on(client.list_tenants(
                  "organization_id_example".to_string(),
                  Some(56),
                  Some(56)
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("ListWebhooks") => {
            let result = rt.block_on(client.list_webhooks(
                  Some("organization_id_example".to_string()),
                  Some("workspace_id_example".to_string()),
                  Some("tenant_id_example".to_string()),
                  Some("account_id_example".to_string()),
                  Some(56),
                  Some(56),
                  Some("status_example".to_string()),
                  Some("search_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("ListWorkflows") => {
            let result = rt.block_on(client.list_workflows(
                  "workspace_id_example".to_string(),
                  Some(56),
                  Some(56),
                  Some("filter_example".to_string()),
                  Some("organization_id_example".to_string()),
                  Some("tenant_id_example".to_string()),
                  Some("account_id_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("ListWorkspaces") => {
            let result = rt.block_on(client.list_workspaces(
                  Some("account_id_example".to_string()),
                  Some(56),
                  Some(56),
                  Some("organization_id_example".to_string()),
                  Some("tenant_id_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        /* Disabled because there's no example.
        Some("PauseWorkflow") => {
            let result = rt.block_on(client.pause_workflow(
                  "workspace_id_example".to_string(),
                  "id_example".to_string(),
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("RotateApiKey") => {
            let result = rt.block_on(client.rotate_api_key(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("RotateTenantApiKey") => {
            let result = rt.block_on(client.rotate_tenant_api_key(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("TriggerWorkflow") => {
            let result = rt.block_on(client.trigger_workflow(
                  "workspace_id_example".to_string(),
                  "id_example".to_string(),
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("UpdateAccount") => {
            let result = rt.block_on(client.update_account(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("UpdateAccountSettings") => {
            let result = rt.block_on(client.update_account_settings(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("UpdateApiKey") => {
            let result = rt.block_on(client.update_api_key(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("UpdateOrganization") => {
            let result = rt.block_on(client.update_organization(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("UpdateTenant") => {
            let result = rt.block_on(client.update_tenant(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("UpdateTenantApiKey") => {
            let result = rt.block_on(client.update_tenant_api_key(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("UpdateWebhook") => {
            let result = rt.block_on(client.update_webhook(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("UpdateWorkflow") => {
            let result = rt.block_on(client.update_workflow(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("UpdateWorkspace") => {
            let result = rt.block_on(client.update_workspace(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("CreateAccount1") => {
            let result = rt.block_on(client.create_account1(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("CreateWorkspace1") => {
            let result = rt.block_on(client.create_workspace1(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("DeleteAccount1") => {
            let result = rt.block_on(client.delete_account1(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("DeleteWorkspace1") => {
            let result = rt.block_on(client.delete_workspace1(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetAccount1") => {
            let result = rt.block_on(client.get_account1(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetWorkspace1") => {
            let result = rt.block_on(client.get_workspace1(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetWorkspaceAnalytics1") => {
            let result = rt.block_on(client.get_workspace_analytics1(
                  "workspace_id_example".to_string(),
                  None,
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetWorkspaceComplianceReport") => {
            let result = rt.block_on(client.get_workspace_compliance_report(
                  "workspace_id_example".to_string(),
                  Some("compliance_type_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetWorkspaceStorageStats") => {
            let result = rt.block_on(client.get_workspace_storage_stats(
                  "workspace_id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("ListWorkspaceSharings") => {
            let result = rt.block_on(client.list_workspace_sharings(
                  "workspace_id_example".to_string(),
                  Some(56),
                  Some("page_token_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("ListWorkspaces1") => {
            let result = rt.block_on(client.list_workspaces1(
                  Some("account_id_example".to_string()),
                  Some(56),
                  Some("page_token_example".to_string()),
                  Some("filter_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("RemoveWorkspaceSharing") => {
            let result = rt.block_on(client.remove_workspace_sharing(
                  "sharing_id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        /* Disabled because there's no example.
        Some("ShareWorkspace") => {
            let result = rt.block_on(client.share_workspace(
                  "workspace_id_example".to_string(),
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("UpdateAccount1") => {
            let result = rt.block_on(client.update_account1(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("UpdateWorkspace1") => {
            let result = rt.block_on(client.update_workspace1(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("UpdateWorkspaceSharing") => {
            let result = rt.block_on(client.update_workspace_sharing(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        _ => {
            panic!("Invalid operation provided")
        }
    }
}
