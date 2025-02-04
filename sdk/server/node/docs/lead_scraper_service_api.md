# lead_scraper_service_api

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CreateAccount**](lead_scraper_service_api.md#CreateAccount) | **POST** /lead-scraper-microservice/api/v1/accounts | Create a new account
[**CreateScrapingJob**](lead_scraper_service_api.md#CreateScrapingJob) | **POST** /lead-scraper-microservice/api/v1/jobs | Create a new job scraping task
[**CreateWorkspace**](lead_scraper_service_api.md#CreateWorkspace) | **POST** /lead-scraper-microservice/api/v1/workspaces | Create a new workspace
[**DeleteAccount**](lead_scraper_service_api.md#DeleteAccount) | **DELETE** /lead-scraper-microservice/api/v1/accounts/{id} | Delete account
[**DeleteScrapingJob**](lead_scraper_service_api.md#DeleteScrapingJob) | **DELETE** /lead-scraper-microservice/api/v1/jobs/{jobId} | Delete a specific job
[**DeleteWorkspace**](lead_scraper_service_api.md#DeleteWorkspace) | **DELETE** /lead-scraper-microservice/api/v1/workspace/{id} | Delete a workspace
[**DownloadScrapingResults**](lead_scraper_service_api.md#DownloadScrapingResults) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId}/download | Download job results as CSV
[**GetAccount**](lead_scraper_service_api.md#GetAccount) | **GET** /lead-scraper-microservice/api/v1/accounts/{id} | Get account details
[**GetAccountUsage**](lead_scraper_service_api.md#GetAccountUsage) | **GET** /lead-scraper-microservice/api/v1/accounts/{id}/usage | Get account usage
[**GetScrapingJob**](lead_scraper_service_api.md#GetScrapingJob) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId} | Get a specific job
[**GetWorkflow**](lead_scraper_service_api.md#GetWorkflow) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id} | Get workflow details
[**GetWorkspace**](lead_scraper_service_api.md#GetWorkspace) | **GET** /lead-scraper-microservice/api/v1/workspace/{id} | Get workspace details
[**GetWorkspaceAnalytics**](lead_scraper_service_api.md#GetWorkspaceAnalytics) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/analytics | Get workspace analytics
[**LeadScraperServiceCreateWorkflow**](lead_scraper_service_api.md#LeadScraperServiceCreateWorkflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows | Create a new workflow
[**ListAccounts**](lead_scraper_service_api.md#ListAccounts) | **GET** /lead-scraper-microservice/api/v1/accounts | List all accounts
[**ListScrapingJobs**](lead_scraper_service_api.md#ListScrapingJobs) | **GET** /lead-scraper-microservice/api/v1/jobs | Get all jobs
[**ListWorkflows**](lead_scraper_service_api.md#ListWorkflows) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows | List workflows
[**ListWorkspaces**](lead_scraper_service_api.md#ListWorkspaces) | **GET** /lead-scraper-microservice/api/v1/workspaces | List workspaces
[**PauseWorkflow**](lead_scraper_service_api.md#PauseWorkflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id}/pause | Pause workflow execution
[**TriggerWorkflow**](lead_scraper_service_api.md#TriggerWorkflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id}/trigger | Trigger workflow execution
[**UpdateAccount**](lead_scraper_service_api.md#UpdateAccount) | **PUT** /lead-scraper-microservice/api/v1/accounts | Update account details
[**UpdateAccountSettings**](lead_scraper_service_api.md#UpdateAccountSettings) | **PUT** /lead-scraper-microservice/api/v1/accounts/settings | Update account settings
[**UpdateWorkflow**](lead_scraper_service_api.md#UpdateWorkflow) | **PUT** /lead-scraper-microservice/api/v1/workspaces/workflow | Update workflow details
[**UpdateWorkspace**](lead_scraper_service_api.md#UpdateWorkspace) | **PUT** /lead-scraper-microservice/api/v1/workspace | Update workspace details


<a name="CreateAccount"></a>
# **CreateAccount**
> CreateAccountResponse CreateAccount(createAccountRequest)

Create a new account

Creates a new user account in the workspace service
<a name="CreateScrapingJob"></a>
# **CreateScrapingJob**
> CreateScrapingJobResponse CreateScrapingJob(createScrapingJobRequest)

Create a new job scraping task

This endpoint creates a new Google Maps scraping job
<a name="CreateWorkspace"></a>
# **CreateWorkspace**
> CreateWorkspaceResponse CreateWorkspace(createWorkspaceRequest)

Create a new workspace

Creates a new workspace for a given account
<a name="DeleteAccount"></a>
# **DeleteAccount**
> DeleteAccountResponse DeleteAccount(Id_)

Delete account

Permanently deletes an account and associated resources
<a name="DeleteScrapingJob"></a>
# **DeleteScrapingJob**
> DeleteScrapingJobResponse DeleteScrapingJob(jobId, userId, orgId, tenantId)

Delete a specific job

This endpoint deletes a specific Google Maps scraping job
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
> GetAccountResponse GetAccount(Id_)

Get account details

Retrieves details of a specific account
<a name="GetAccountUsage"></a>
# **GetAccountUsage**
> GetAccountUsageResponse GetAccountUsage(Id_)

Get account usage

Retrieves usage details for a given account
<a name="GetScrapingJob"></a>
# **GetScrapingJob**
> GetScrapingJobResponse GetScrapingJob(jobId, userId, orgId, tenantId)

Get a specific job

This endpoint retrieves a specific Google Maps scraping job
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
> ListAccountsResponse ListAccounts(pageSize, pageNumber, filter)

List all accounts

Retrieves a list of accounts
<a name="ListScrapingJobs"></a>
# **ListScrapingJobs**
> ListScrapingJobsResponse ListScrapingJobs(userId, orgId, tenantId)

Get all jobs

This endpoint retrieves all Google Maps scraping jobs
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
