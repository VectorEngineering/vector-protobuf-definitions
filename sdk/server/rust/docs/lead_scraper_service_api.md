# lead_scraper_service_api

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

Method | HTTP request | Description
------------- | ------------- | -------------
**CreateAccount**](lead_scraper_service_api.md#CreateAccount) | **POST** /lead-scraper-microservice/api/v1/accounts | Create a new account
**CreateScrapingJob**](lead_scraper_service_api.md#CreateScrapingJob) | **POST** /lead-scraper-microservice/api/v1/jobs | Create a new job scraping task
**CreateWorkspace**](lead_scraper_service_api.md#CreateWorkspace) | **POST** /lead-scraper-microservice/api/v1/workspaces | Create a new workspace
**DeleteAccount**](lead_scraper_service_api.md#DeleteAccount) | **DELETE** /lead-scraper-microservice/api/v1/accounts/{id} | Delete account
**DeleteScrapingJob**](lead_scraper_service_api.md#DeleteScrapingJob) | **DELETE** /lead-scraper-microservice/api/v1/jobs/{jobId} | Delete a specific job
**DeleteWorkspace**](lead_scraper_service_api.md#DeleteWorkspace) | **DELETE** /lead-scraper-microservice/api/v1/workspace/{id} | Delete a workspace
**DownloadScrapingResults**](lead_scraper_service_api.md#DownloadScrapingResults) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId}/download | Download job results as CSV
**GetAccount**](lead_scraper_service_api.md#GetAccount) | **GET** /lead-scraper-microservice/api/v1/accounts/{id} | Get account details
**GetAccountUsage**](lead_scraper_service_api.md#GetAccountUsage) | **GET** /lead-scraper-microservice/api/v1/accounts/{id}/usage | Get account usage
**GetScrapingJob**](lead_scraper_service_api.md#GetScrapingJob) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId} | Get a specific job
**GetWorkflow**](lead_scraper_service_api.md#GetWorkflow) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id} | Get workflow details
**GetWorkspace**](lead_scraper_service_api.md#GetWorkspace) | **GET** /lead-scraper-microservice/api/v1/workspace/{id} | Get workspace details
**GetWorkspaceAnalytics**](lead_scraper_service_api.md#GetWorkspaceAnalytics) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/analytics | Get workspace analytics
**LeadScraperService_CreateWorkflow**](lead_scraper_service_api.md#LeadScraperService_CreateWorkflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows | Create a new workflow
**ListAccounts**](lead_scraper_service_api.md#ListAccounts) | **GET** /lead-scraper-microservice/api/v1/accounts | List all accounts
**ListScrapingJobs**](lead_scraper_service_api.md#ListScrapingJobs) | **GET** /lead-scraper-microservice/api/v1/jobs | Get all jobs
**ListWorkflows**](lead_scraper_service_api.md#ListWorkflows) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows | List workflows
**ListWorkspaces**](lead_scraper_service_api.md#ListWorkspaces) | **GET** /lead-scraper-microservice/api/v1/workspaces | List workspaces
**PauseWorkflow**](lead_scraper_service_api.md#PauseWorkflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id}/pause | Pause workflow execution
**TriggerWorkflow**](lead_scraper_service_api.md#TriggerWorkflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id}/trigger | Trigger workflow execution
**UpdateAccount**](lead_scraper_service_api.md#UpdateAccount) | **PUT** /lead-scraper-microservice/api/v1/accounts | Update account details
**UpdateAccountSettings**](lead_scraper_service_api.md#UpdateAccountSettings) | **PUT** /lead-scraper-microservice/api/v1/accounts/settings | Update account settings
**UpdateWorkflow**](lead_scraper_service_api.md#UpdateWorkflow) | **PUT** /lead-scraper-microservice/api/v1/workspaces/workflow | Update workflow details
**UpdateWorkspace**](lead_scraper_service_api.md#UpdateWorkspace) | **PUT** /lead-scraper-microservice/api/v1/workspace | Update workspace details


# **CreateAccount**
> models::CreateAccountResponse CreateAccount(create_account_request)
Create a new account

Creates a new user account in the workspace service

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **create_account_request** | [**CreateAccountRequest**](CreateAccountRequest.md)|  | 

### Return type

[**models::CreateAccountResponse**](CreateAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **CreateScrapingJob**
> models::CreateScrapingJobResponse CreateScrapingJob(create_scraping_job_request)
Create a new job scraping task

This endpoint creates a new Google Maps scraping job

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **create_scraping_job_request** | [**CreateScrapingJobRequest**](CreateScrapingJobRequest.md)|  | 

### Return type

[**models::CreateScrapingJobResponse**](CreateScrapingJobResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **CreateWorkspace**
> models::CreateWorkspaceResponse CreateWorkspace(create_workspace_request)
Create a new workspace

Creates a new workspace for a given account

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **create_workspace_request** | [**CreateWorkspaceRequest**](CreateWorkspaceRequest.md)|  | 

### Return type

[**models::CreateWorkspaceResponse**](CreateWorkspaceResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DeleteAccount**
> models::DeleteAccountResponse DeleteAccount(id)
Delete account

Permanently deletes an account and associated resources

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**|  | 

### Return type

[**models::DeleteAccountResponse**](DeleteAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DeleteScrapingJob**
> models::DeleteScrapingJobResponse DeleteScrapingJob(job_id, user_id, org_id, tenant_id)
Delete a specific job

This endpoint deletes a specific Google Maps scraping job

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **job_id** | **String**|  | 
  **user_id** | **String**|  | 
  **org_id** | **String**|  | 
  **tenant_id** | **String**|  | 

### Return type

[**models::DeleteScrapingJobResponse**](DeleteScrapingJobResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DeleteWorkspace**
> models::DeleteWorkspaceResponse DeleteWorkspace(id)
Delete a workspace

Deletes a specific workspace

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**|  | 

### Return type

[**models::DeleteWorkspaceResponse**](DeleteWorkspaceResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DownloadScrapingResults**
> models::DownloadScrapingResultsResponse DownloadScrapingResults(job_id, user_id, org_id, tenant_id)
Download job results as CSV

This endpoint downloads the results of a Google Maps scraping job as CSV

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **job_id** | **String**|  | 
  **user_id** | **String**|  | 
  **org_id** | **String**|  | 
  **tenant_id** | **String**|  | 

### Return type

[**models::DownloadScrapingResultsResponse**](DownloadScrapingResultsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetAccount**
> models::GetAccountResponse GetAccount(id)
Get account details

Retrieves details of a specific account

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**|  | 

### Return type

[**models::GetAccountResponse**](GetAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetAccountUsage**
> models::GetAccountUsageResponse GetAccountUsage(id)
Get account usage

Retrieves usage details for a given account

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**|  | 

### Return type

[**models::GetAccountUsageResponse**](GetAccountUsageResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetScrapingJob**
> models::GetScrapingJobResponse GetScrapingJob(job_id, user_id, org_id, tenant_id)
Get a specific job

This endpoint retrieves a specific Google Maps scraping job

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **job_id** | **String**|  | 
  **user_id** | **String**|  | 
  **org_id** | **String**|  | 
  **tenant_id** | **String**|  | 

### Return type

[**models::GetScrapingJobResponse**](GetScrapingJobResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetWorkflow**
> models::ScrapingWorkflow GetWorkflow(workspace_id, id)
Get workflow details

Retrieves details of a specific workflow

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **workspace_id** | **String**|  | 
  **id** | **String**|  | 

### Return type

[**models::ScrapingWorkflow**](ScrapingWorkflow.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetWorkspace**
> models::Workspace GetWorkspace(id)
Get workspace details

Retrieves details of a specific workspace

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**|  | 

### Return type

[**models::Workspace**](Workspace.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetWorkspaceAnalytics**
> models::GetWorkspaceAnalyticsResponse GetWorkspaceAnalytics(workspace_id, optional)
Get workspace analytics

Retrieves analytics data for a specific workspace

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **workspace_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **workspace_id** | **String**|  | 
 **start_time** | **chrono::DateTime::<chrono::Utc>**|  | 
 **end_time** | **chrono::DateTime::<chrono::Utc>**|  | 

### Return type

[**models::GetWorkspaceAnalyticsResponse**](GetWorkspaceAnalyticsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **LeadScraperService_CreateWorkflow**
> models::CreateWorkflowResponse LeadScraperService_CreateWorkflow(workspace_id, create_workflow_body)
Create a new workflow

This endpoint creates a new workflow configuration for a specific workspace  Required parameters: - workspace_id: Parent workspace ID - name: Human-readable workflow name - description: Optional workflow description - parameters: Job parameters for the workflow  Example: ```json

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **workspace_id** | **String**|  | 
  **create_workflow_body** | [**CreateWorkflowBody**](CreateWorkflowBody.md)|  | 

### Return type

[**models::CreateWorkflowResponse**](CreateWorkflowResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ListAccounts**
> models::ListAccountsResponse ListAccounts(optional)
List all accounts

Retrieves a list of accounts

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **page_size** | **i32**|  | 
 **page_token** | **String**|  | 
 **filter** | **String**|  | 

### Return type

[**models::ListAccountsResponse**](ListAccountsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ListScrapingJobs**
> models::ListScrapingJobsResponse ListScrapingJobs(user_id, org_id, tenant_id)
Get all jobs

This endpoint retrieves all Google Maps scraping jobs

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **user_id** | **String**|  | 
  **org_id** | **String**|  | 
  **tenant_id** | **String**|  | 

### Return type

[**models::ListScrapingJobsResponse**](ListScrapingJobsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ListWorkflows**
> models::ListWorkflowsResponse ListWorkflows(workspace_id, optional)
List workflows

Retrieves a list of workflows for a given workspace

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **workspace_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **workspace_id** | **String**|  | 
 **page_size** | **i32**|  | 
 **page_token** | **String**|  | 
 **filter** | **String**|  | 

### Return type

[**models::ListWorkflowsResponse**](ListWorkflowsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ListWorkspaces**
> models::ListWorkspacesResponse ListWorkspaces(optional)
List workspaces

Retrieves a list of workspaces for a given account

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account_id** | **String**|  | 
 **page_size** | **i32**|  | 
 **page_token** | **String**|  | 

### Return type

[**models::ListWorkspacesResponse**](ListWorkspacesResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **PauseWorkflow**
> models::PauseWorkflowResponse PauseWorkflow(workspace_id, id, pause_workflow_body)
Pause workflow execution

Pauses the execution of a specific workflow

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **workspace_id** | **String**|  | 
  **id** | **String**|  | 
  **pause_workflow_body** | [**PauseWorkflowBody**](PauseWorkflowBody.md)|  | 

### Return type

[**models::PauseWorkflowResponse**](PauseWorkflowResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **TriggerWorkflow**
> models::TriggerWorkflowResponse TriggerWorkflow(workspace_id, id, trigger_workflow_body)
Trigger workflow execution

Triggers the execution of a specific workflow

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **workspace_id** | **String**|  | 
  **id** | **String**|  | 
  **trigger_workflow_body** | [**TriggerWorkflowBody**](TriggerWorkflowBody.md)|  | 

### Return type

[**models::TriggerWorkflowResponse**](TriggerWorkflowResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **UpdateAccount**
> models::UpdateAccountResponse UpdateAccount(update_account_request)
Update account details

Updates specified fields of an existing account

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **update_account_request** | [**UpdateAccountRequest**](UpdateAccountRequest.md)|  | 

### Return type

[**models::UpdateAccountResponse**](UpdateAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **UpdateAccountSettings**
> models::UpdateAccountSettingsResponse UpdateAccountSettings(update_account_settings_request)
Update account settings

Updates the settings for a given account

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **update_account_settings_request** | [**UpdateAccountSettingsRequest**](UpdateAccountSettingsRequest.md)|  | 

### Return type

[**models::UpdateAccountSettingsResponse**](UpdateAccountSettingsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **UpdateWorkflow**
> models::ScrapingWorkflow UpdateWorkflow(update_workflow_request)
Update workflow details

Updates the details of a specific workflow

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **update_workflow_request** | [**UpdateWorkflowRequest**](UpdateWorkflowRequest.md)|  | 

### Return type

[**models::ScrapingWorkflow**](ScrapingWorkflow.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **UpdateWorkspace**
> models::UpdateWorkspaceResponse UpdateWorkspace(update_workspace_request)
Update workspace details

Updates the details of a specific workspace

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **update_workspace_request** | [**UpdateWorkspaceRequest**](UpdateWorkspaceRequest.md)|  | 

### Return type

[**models::UpdateWorkspaceResponse**](UpdateWorkspaceResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

