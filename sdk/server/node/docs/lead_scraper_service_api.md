# lead_scraper_service_api

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CreateAccount**](lead_scraper_service_api.md#CreateAccount) | **POST** /lead-scraper-microservice/api/v1/accounts | Create a new account
[**CreateOrganization**](lead_scraper_service_api.md#CreateOrganization) | **POST** /lead-scraper-microservice/api/v1/organization | Create a new organization
[**CreateScrapingJob**](lead_scraper_service_api.md#CreateScrapingJob) | **POST** /lead-scraper-microservice/api/v1/jobs | Create a new job scraping task
[**CreateTenant**](lead_scraper_service_api.md#CreateTenant) | **POST** /lead-scraper-microservice/api/v1/organizations/{organizationId}/tenants | Create a new tenant
[**CreateWorkspace**](lead_scraper_service_api.md#CreateWorkspace) | **POST** /lead-scraper-microservice/api/v1/workspaces | Create a new workspace
[**DeleteAccount**](lead_scraper_service_api.md#DeleteAccount) | **DELETE** /lead-scraper-microservice/api/v1/accounts/{id} | Delete account
[**DeleteOrganization**](lead_scraper_service_api.md#DeleteOrganization) | **DELETE** /lead-scraper-microservice/api/v1/organization/{id} | Delete an organization
[**DeleteScrapingJob**](lead_scraper_service_api.md#DeleteScrapingJob) | **DELETE** /lead-scraper-microservice/api/v1/jobs/{jobId} | Delete a specific job
[**DeleteTenant**](lead_scraper_service_api.md#DeleteTenant) | **DELETE** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId}/{tenantId} | Delete a tenant
[**DeleteWorkspace**](lead_scraper_service_api.md#DeleteWorkspace) | **DELETE** /lead-scraper-microservice/api/v1/workspace/{id} | Delete a workspace
[**DownloadScrapingResults**](lead_scraper_service_api.md#DownloadScrapingResults) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId}/download | Download job results as CSV
[**GetAccount**](lead_scraper_service_api.md#GetAccount) | **GET** /lead-scraper-microservice/api/v1/accounts/{id} | Get account details
[**GetAccountUsage**](lead_scraper_service_api.md#GetAccountUsage) | **GET** /lead-scraper-microservice/api/v1/accounts/{id}/usage | Get account usage
[**GetOrganization**](lead_scraper_service_api.md#GetOrganization) | **GET** /lead-scraper-microservice/api/v1/organization/{id} | Get organization details
[**GetScrapingJob**](lead_scraper_service_api.md#GetScrapingJob) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId} | Get a specific job
[**GetTenant**](lead_scraper_service_api.md#GetTenant) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/{organizationId}/{tenantId} | Get tenant details
[**GetWorkflow**](lead_scraper_service_api.md#GetWorkflow) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id} | Get workflow details
[**GetWorkspace**](lead_scraper_service_api.md#GetWorkspace) | **GET** /lead-scraper-microservice/api/v1/workspace/{id} | Get workspace details
[**GetWorkspaceAnalytics**](lead_scraper_service_api.md#GetWorkspaceAnalytics) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/analytics | Get workspace analytics
[**LeadScraperServiceCreateWorkflow**](lead_scraper_service_api.md#LeadScraperServiceCreateWorkflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows | Create a new workflow
[**ListAccounts**](lead_scraper_service_api.md#ListAccounts) | **GET** /lead-scraper-microservice/api/v1/accounts | List all accounts
[**ListOrganizations**](lead_scraper_service_api.md#ListOrganizations) | **GET** /lead-scraper-microservice/api/v1/organization | List all organizations
[**ListScrapingJobs**](lead_scraper_service_api.md#ListScrapingJobs) | **GET** /lead-scraper-microservice/api/v1/jobs | Get all jobs
[**ListTenants**](lead_scraper_service_api.md#ListTenants) | **GET** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId} | List all tenants
[**ListWorkflows**](lead_scraper_service_api.md#ListWorkflows) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows | List workflows
[**ListWorkspaces**](lead_scraper_service_api.md#ListWorkspaces) | **GET** /lead-scraper-microservice/api/v1/workspaces | List workspaces
[**PauseWorkflow**](lead_scraper_service_api.md#PauseWorkflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id}/pause | Pause workflow execution
[**TriggerWorkflow**](lead_scraper_service_api.md#TriggerWorkflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id}/trigger | Trigger workflow execution
[**UpdateAccount**](lead_scraper_service_api.md#UpdateAccount) | **PUT** /lead-scraper-microservice/api/v1/accounts | Update account details
[**UpdateAccountSettings**](lead_scraper_service_api.md#UpdateAccountSettings) | **PUT** /lead-scraper-microservice/api/v1/accounts/settings | Update account settings
[**UpdateOrganization**](lead_scraper_service_api.md#UpdateOrganization) | **PUT** /lead-scraper-microservice/api/v1/organization | Update organization details
[**UpdateTenant**](lead_scraper_service_api.md#UpdateTenant) | **PUT** /lead-scraper-microservice/api/v1/organizations/tenants | Update tenant details
[**UpdateWorkflow**](lead_scraper_service_api.md#UpdateWorkflow) | **PUT** /lead-scraper-microservice/api/v1/workspaces/workflow | Update workflow details
[**UpdateWorkspace**](lead_scraper_service_api.md#UpdateWorkspace) | **PUT** /lead-scraper-microservice/api/v1/workspace | Update workspace details


<a name="CreateAccount"></a>
# **CreateAccount**
> CreateAccountResponse CreateAccount(createAccountRequest)

Create a new account

Creates a new user account in the workspace service
<a name="CreateOrganization"></a>
# **CreateOrganization**
> CreateOrganizationResponse CreateOrganization(createOrganizationRequest)

Create a new organization

Creates a new organization within a tenant
<a name="CreateScrapingJob"></a>
# **CreateScrapingJob**
> CreateScrapingJobResponse CreateScrapingJob(createScrapingJobRequest)

Create a new job scraping task

This endpoint creates a new Google Maps scraping job
<a name="CreateTenant"></a>
# **CreateTenant**
> CreateTenantResponse CreateTenant(organizationId, createTenantBody)

Create a new tenant

Creates a new tenant in the system
<a name="CreateWorkspace"></a>
# **CreateWorkspace**
> CreateWorkspaceResponse CreateWorkspace(createWorkspaceRequest)

Create a new workspace

Creates a new workspace for a given account
<a name="DeleteAccount"></a>
# **DeleteAccount**
> DeleteAccountResponse DeleteAccount(Id_, organizationId, tenantId)

Delete account

Permanently deletes an account and associated resources
<a name="DeleteOrganization"></a>
# **DeleteOrganization**
> DeleteOrganizationResponse DeleteOrganization(Id_)

Delete an organization

Permanently deletes an organization and all associated resources
<a name="DeleteScrapingJob"></a>
# **DeleteScrapingJob**
> DeleteScrapingJobResponse DeleteScrapingJob(jobId, userId, orgId, tenantId)

Delete a specific job

This endpoint deletes a specific Google Maps scraping job
<a name="DeleteTenant"></a>
# **DeleteTenant**
> DeleteTenantResponse DeleteTenant(organizationId, tenantId)

Delete a tenant

Permanently deletes a tenant and all associated resources
<a name="DeleteWorkspace"></a>
# **DeleteWorkspace**
> DeleteWorkspaceResponse DeleteWorkspace(Id_)

Delete a workspace

Deletes a specific workspace
<a name="DownloadScrapingResults"></a>
# **DownloadScrapingResults**
> DownloadScrapingResultsResponse DownloadScrapingResults(jobId, userId, orgId, tenantId)

Download job results as CSV

This endpoint downloads the results of a Google Maps scraping job as CSV
<a name="GetAccount"></a>
# **GetAccount**
> GetAccountResponse GetAccount(Id_, organizationId, tenantId)

Get account details

Retrieves details of a specific account
<a name="GetAccountUsage"></a>
# **GetAccountUsage**
> GetAccountUsageResponse GetAccountUsage(Id_)

Get account usage

Retrieves usage details for a given account
<a name="GetOrganization"></a>
# **GetOrganization**
> GetOrganizationResponse GetOrganization(Id_)

Get organization details

Retrieves details of a specific organization
<a name="GetScrapingJob"></a>
# **GetScrapingJob**
> GetScrapingJobResponse GetScrapingJob(jobId, userId, orgId, tenantId)

Get a specific job

This endpoint retrieves a specific Google Maps scraping job
<a name="GetTenant"></a>
# **GetTenant**
> GetTenantResponse GetTenant(organizationId, tenantId)

Get tenant details

Retrieves details of a specific tenant
<a name="GetWorkflow"></a>
# **GetWorkflow**
> ScrapingWorkflow GetWorkflow(workspaceId, Id_)

Get workflow details

Retrieves details of a specific workflow
<a name="GetWorkspace"></a>
# **GetWorkspace**
> Workspace GetWorkspace(Id_)

Get workspace details

Retrieves details of a specific workspace
<a name="GetWorkspaceAnalytics"></a>
# **GetWorkspaceAnalytics**
> GetWorkspaceAnalyticsResponse GetWorkspaceAnalytics(workspaceId, startTime, endTime)

Get workspace analytics

Retrieves analytics data for a specific workspace
<a name="LeadScraperServiceCreateWorkflow"></a>
# **LeadScraperServiceCreateWorkflow**
> CreateWorkflowResponse LeadScraperServiceCreateWorkflow(workspaceId, createWorkflowBody)

Create a new workflow

This endpoint creates a new workflow configuration for a specific workspace  Required parameters: - workspace_id: Parent workspace ID - name: Human-readable workflow name - description: Optional workflow description - parameters: Job parameters for the workflow  Example: &#x60;&#x60;&#x60;json
<a name="ListAccounts"></a>
# **ListAccounts**
> ListAccountsResponse ListAccounts(pageSize, pageNumber, filter, organizationId, tenantId)

List all accounts

Retrieves a list of accounts
<a name="ListOrganizations"></a>
# **ListOrganizations**
> ListOrganizationsResponse ListOrganizations(pageSize, pageNumber)

List all organizations

Retrieves a list of all organizations in a tenant
<a name="ListScrapingJobs"></a>
# **ListScrapingJobs**
> ListScrapingJobsResponse ListScrapingJobs(authPlatformUserId, orgId, tenantId)

Get all jobs

This endpoint retrieves all Google Maps scraping jobs
<a name="ListTenants"></a>
# **ListTenants**
> ListTenantsResponse ListTenants(organizationId, pageSize, pageNumber)

List all tenants

Retrieves a list of all tenants in the system
<a name="ListWorkflows"></a>
# **ListWorkflows**
> ListWorkflowsResponse ListWorkflows(workspaceId, pageSize, pageNumber, filter)

List workflows

Retrieves a list of workflows for a given workspace
<a name="ListWorkspaces"></a>
# **ListWorkspaces**
> ListWorkspacesResponse ListWorkspaces(accountId, pageSize, pageNumber)

List workspaces

Retrieves a list of workspaces for a given account
<a name="PauseWorkflow"></a>
# **PauseWorkflow**
> PauseWorkflowResponse PauseWorkflow(workspaceId, Id_, pauseWorkflowBody)

Pause workflow execution

Pauses the execution of a specific workflow
<a name="TriggerWorkflow"></a>
# **TriggerWorkflow**
> TriggerWorkflowResponse TriggerWorkflow(workspaceId, Id_, triggerWorkflowBody)

Trigger workflow execution

Triggers the execution of a specific workflow
<a name="UpdateAccount"></a>
# **UpdateAccount**
> UpdateAccountResponse UpdateAccount(updateAccountRequest)

Update account details

Updates specified fields of an existing account
<a name="UpdateAccountSettings"></a>
# **UpdateAccountSettings**
> UpdateAccountSettingsResponse UpdateAccountSettings(updateAccountSettingsRequest)

Update account settings

Updates the settings for a given account
<a name="UpdateOrganization"></a>
# **UpdateOrganization**
> UpdateOrganizationResponse UpdateOrganization(updateOrganizationRequest)

Update organization details

Updates the configuration of a specific organization
<a name="UpdateTenant"></a>
# **UpdateTenant**
> UpdateTenantResponse UpdateTenant(updateTenantRequest)

Update tenant details

Updates the configuration of a specific tenant
<a name="UpdateWorkflow"></a>
# **UpdateWorkflow**
> ScrapingWorkflow UpdateWorkflow(updateWorkflowRequest)

Update workflow details

Updates the details of a specific workflow
<a name="UpdateWorkspace"></a>
# **UpdateWorkspace**
> UpdateWorkspaceResponse UpdateWorkspace(updateWorkspaceRequest)

Update workspace details

Updates the details of a specific workspace
