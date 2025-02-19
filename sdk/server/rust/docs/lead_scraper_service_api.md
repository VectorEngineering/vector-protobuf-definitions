# lead_scraper_service_api

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

Method | HTTP request | Description
------------- | ------------- | -------------
**CreateAccount**](lead_scraper_service_api.md#CreateAccount) | **POST** /lead-scraper-microservice/api/v1/accounts | Create a new account
**CreateAPIKey**](lead_scraper_service_api.md#CreateAPIKey) | **POST** /lead-scraper-microservice/api/v1/api-keys | Create a new API key
**CreateOrganization**](lead_scraper_service_api.md#CreateOrganization) | **POST** /lead-scraper-microservice/api/v1/organization | Create a new organization
**CreateScrapingJob**](lead_scraper_service_api.md#CreateScrapingJob) | **POST** /lead-scraper-microservice/api/v1/jobs | Create a new job scraping task
**CreateTenant**](lead_scraper_service_api.md#CreateTenant) | **POST** /lead-scraper-microservice/api/v1/organizations/{organizationId}/tenants | Create a new tenant
**CreateTenantAPIKey**](lead_scraper_service_api.md#CreateTenantAPIKey) | **POST** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys | Create a new tenant API key
**CreateWebhook**](lead_scraper_service_api.md#CreateWebhook) | **POST** /lead-scraper-microservice/api/v1/webhooks | Create webhook
**CreateWorkflow**](lead_scraper_service_api.md#CreateWorkflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows | Create a new workflow
**CreateWorkspace**](lead_scraper_service_api.md#CreateWorkspace) | **POST** /lead-scraper-microservice/api/v1/workspaces | Create a new workspace
**DeleteAccount**](lead_scraper_service_api.md#DeleteAccount) | **DELETE** /lead-scraper-microservice/api/v1/accounts/{id} | Delete account
**DeleteAPIKey**](lead_scraper_service_api.md#DeleteAPIKey) | **DELETE** /lead-scraper-microservice/api/v1/api-keys/{keyId} | Delete API key
**DeleteOrganization**](lead_scraper_service_api.md#DeleteOrganization) | **DELETE** /lead-scraper-microservice/api/v1/organization/{id} | Delete an organization
**DeleteScrapingJob**](lead_scraper_service_api.md#DeleteScrapingJob) | **DELETE** /lead-scraper-microservice/api/v1/jobs/{jobId} | Delete a specific job
**DeleteTenant**](lead_scraper_service_api.md#DeleteTenant) | **DELETE** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId}/{tenantId} | Delete a tenant
**DeleteTenantAPIKey**](lead_scraper_service_api.md#DeleteTenantAPIKey) | **DELETE** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/{keyId} | Delete tenant API key
**DeleteWebhook**](lead_scraper_service_api.md#DeleteWebhook) | **DELETE** /lead-scraper-microservice/api/v1/webhooks/{webhookId} | Delete webhook
**DeleteWorkflow**](lead_scraper_service_api.md#DeleteWorkflow) | **DELETE** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id} | Delete workflow
**DeleteWorkspace**](lead_scraper_service_api.md#DeleteWorkspace) | **DELETE** /lead-scraper-microservice/api/v1/workspace/{id} | Delete a workspace
**DownloadScrapingResults**](lead_scraper_service_api.md#DownloadScrapingResults) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId}/download | Download job results as CSV
**GetAccount**](lead_scraper_service_api.md#GetAccount) | **GET** /lead-scraper-microservice/api/v1/accounts/{id} | Get account details
**GetAccountUsage**](lead_scraper_service_api.md#GetAccountUsage) | **GET** /lead-scraper-microservice/api/v1/accounts/{id}/usage | Get account usage
**GetAPIKey**](lead_scraper_service_api.md#GetAPIKey) | **GET** /lead-scraper-microservice/api/v1/api-keys/{keyId} | Get API key details
**GetLead**](lead_scraper_service_api.md#GetLead) | **GET** /lead-scraper-microservice/api/v1/leads/{leadId} | Get lead details
**GetOrganization**](lead_scraper_service_api.md#GetOrganization) | **GET** /lead-scraper-microservice/api/v1/organization/{id} | Get organization details
**GetScrapingJob**](lead_scraper_service_api.md#GetScrapingJob) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId} | Get a specific job
**GetTenant**](lead_scraper_service_api.md#GetTenant) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/{organizationId}/{tenantId} | Get tenant details
**GetTenantAPIKey**](lead_scraper_service_api.md#GetTenantAPIKey) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/{keyId} | Get tenant API key details
**GetWebhook**](lead_scraper_service_api.md#GetWebhook) | **GET** /lead-scraper-microservice/api/v1/webhooks/{webhookId} | Get webhook
**GetWorkflow**](lead_scraper_service_api.md#GetWorkflow) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id} | Get workflow details
**GetWorkspace**](lead_scraper_service_api.md#GetWorkspace) | **GET** /lead-scraper-microservice/api/v1/workspace/{id} | Get workspace details
**GetWorkspaceAnalytics**](lead_scraper_service_api.md#GetWorkspaceAnalytics) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/analytics | Get workspace analytics
**ListAccounts**](lead_scraper_service_api.md#ListAccounts) | **GET** /lead-scraper-microservice/api/v1/accounts/list | List all accounts
**ListAPIKeys**](lead_scraper_service_api.md#ListAPIKeys) | **GET** /lead-scraper-microservice/api/v1/api-keys/list | List API keys
**ListLeads**](lead_scraper_service_api.md#ListLeads) | **GET** /lead-scraper-microservice/api/v1/leads | List leads
**ListOrganizations**](lead_scraper_service_api.md#ListOrganizations) | **GET** /lead-scraper-microservice/api/v1/organization | List all organizations
**ListScrapingJobs**](lead_scraper_service_api.md#ListScrapingJobs) | **GET** /lead-scraper-microservice/api/v1/jobs | Get all jobs
**ListTenantAPIKeys**](lead_scraper_service_api.md#ListTenantAPIKeys) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/list | List tenant API keys
**ListTenants**](lead_scraper_service_api.md#ListTenants) | **GET** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId} | List all tenants
**ListWebhooks**](lead_scraper_service_api.md#ListWebhooks) | **GET** /lead-scraper-microservice/api/v1/webhooks | List webhooks
**ListWorkflows**](lead_scraper_service_api.md#ListWorkflows) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows | List workflows
**ListWorkspaces**](lead_scraper_service_api.md#ListWorkspaces) | **GET** /lead-scraper-microservice/api/v1/workspaces | List workspaces
**PauseWorkflow**](lead_scraper_service_api.md#PauseWorkflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id}/pause | Pause workflow execution
**RotateAPIKey**](lead_scraper_service_api.md#RotateAPIKey) | **POST** /lead-scraper-microservice/api/v1/api-keys/rotate | Rotate API key
**RotateTenantAPIKey**](lead_scraper_service_api.md#RotateTenantAPIKey) | **POST** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/rotate | Rotate tenant API key
**TriggerWorkflow**](lead_scraper_service_api.md#TriggerWorkflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id}/trigger | Trigger workflow execution
**UpdateAccount**](lead_scraper_service_api.md#UpdateAccount) | **PUT** /lead-scraper-microservice/api/v1/accounts/update | Update account details
**UpdateAccountSettings**](lead_scraper_service_api.md#UpdateAccountSettings) | **PUT** /lead-scraper-microservice/api/v1/accounts/settings | Update account settings
**UpdateAPIKey**](lead_scraper_service_api.md#UpdateAPIKey) | **PUT** /lead-scraper-microservice/api/v1/api-keys | Update API key
**UpdateOrganization**](lead_scraper_service_api.md#UpdateOrganization) | **PUT** /lead-scraper-microservice/api/v1/organization | Update organization details
**UpdateTenant**](lead_scraper_service_api.md#UpdateTenant) | **PUT** /lead-scraper-microservice/api/v1/organizations/tenants | Update tenant details
**UpdateTenantAPIKey**](lead_scraper_service_api.md#UpdateTenantAPIKey) | **PUT** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys | Update tenant API key
**UpdateWebhook**](lead_scraper_service_api.md#UpdateWebhook) | **PUT** /lead-scraper-microservice/api/v1/webhooks | Update webhook
**UpdateWorkflow**](lead_scraper_service_api.md#UpdateWorkflow) | **PUT** /lead-scraper-microservice/api/v1/workspaces/workflow | Update workflow details
**UpdateWorkspace**](lead_scraper_service_api.md#UpdateWorkspace) | **PUT** /lead-scraper-microservice/api/v1/workspace | Update workspace details


# **CreateAccount**
> models::CreateAccountResponse CreateAccount(ctx, create_account_request)
Create a new account

Creates a new user account in the workspace service

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **create_account_request** | [**CreateAccountRequest**](CreateAccountRequest.md)|  | 

### Return type

[**models::CreateAccountResponse**](CreateAccountResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **CreateAPIKey**
> models::CreateApiKeyResponse CreateAPIKey(create_api_key_request)
Create a new API key

Creates a new API key with specified permissions and settings

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **create_api_key_request** | [**CreateApiKeyRequest**](CreateApiKeyRequest.md)|  | 

### Return type

[**models::CreateApiKeyResponse**](CreateAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **CreateOrganization**
> models::CreateOrganizationResponse CreateOrganization(create_organization_request)
Create a new organization

Creates a new organization within a tenant

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **create_organization_request** | [**CreateOrganizationRequest**](CreateOrganizationRequest.md)|  | 

### Return type

[**models::CreateOrganizationResponse**](CreateOrganizationResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **CreateScrapingJob**
> models::CreateScrapingJobResponse CreateScrapingJob(ctx, create_scraping_job_request)
Create a new job scraping task

This endpoint creates a new Google Maps scraping job

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **create_scraping_job_request** | [**CreateScrapingJobRequest**](CreateScrapingJobRequest.md)|  | 

### Return type

[**models::CreateScrapingJobResponse**](CreateScrapingJobResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **CreateTenant**
> models::CreateTenantResponse CreateTenant(organization_id, create_tenant_body)
Create a new tenant

Creates a new tenant in the system

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **organization_id** | **String**|  | 
  **create_tenant_body** | [**CreateTenantBody**](CreateTenantBody.md)|  | 

### Return type

[**models::CreateTenantResponse**](CreateTenantResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **CreateTenantAPIKey**
> models::CreateTenantApiKeyResponse CreateTenantAPIKey(create_tenant_api_key_request)
Create a new tenant API key

Creates a new API key for a tenant with specified permissions

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **create_tenant_api_key_request** | [**CreateTenantApiKeyRequest**](CreateTenantApiKeyRequest.md)|  | 

### Return type

[**models::CreateTenantApiKeyResponse**](CreateTenantAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **CreateWebhook**
> models::CreateWebhookResponse CreateWebhook(ctx, create_webhook_request)
Create webhook

Creates a new webhook configuration for event notifications

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **create_webhook_request** | [**CreateWebhookRequest**](CreateWebhookRequest.md)|  | 

### Return type

[**models::CreateWebhookResponse**](CreateWebhookResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **CreateWorkflow**
> models::CreateWorkflowResponse CreateWorkflow(ctx, workspace_id, create_workflow_body)
Create a new workflow

Creates a new workflow for a specific workspace

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **workspace_id** | **String**|  | 
  **create_workflow_body** | [**CreateWorkflowBody**](CreateWorkflowBody.md)|  | 

### Return type

[**models::CreateWorkflowResponse**](CreateWorkflowResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **CreateWorkspace**
> models::CreateWorkspaceResponse CreateWorkspace(ctx, create_workspace_request)
Create a new workspace

Creates a new workspace for a given account

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **create_workspace_request** | [**CreateWorkspaceRequest**](CreateWorkspaceRequest.md)|  | 

### Return type

[**models::CreateWorkspaceResponse**](CreateWorkspaceResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DeleteAccount**
> models::DeleteAccountResponse DeleteAccount(ctx, id, optional)
Delete account

Permanently deletes an account and associated resources

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**|  | 
 **organization_id** | **String**|  | 
 **tenant_id** | **String**|  | 

### Return type

[**models::DeleteAccountResponse**](DeleteAccountResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DeleteAPIKey**
> models::DeleteApiKeyResponse DeleteAPIKey(ctx, key_id, optional)
Delete API key

Permanently deletes an API key

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **key_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **key_id** | **String**|  | 
 **organization_id** | **String**|  | 
 **workspace_id** | **String**|  | 
 **tenant_id** | **String**|  | 
 **account_id** | **String**|  | 

### Return type

[**models::DeleteApiKeyResponse**](DeleteAPIKeyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DeleteOrganization**
> models::DeleteOrganizationResponse DeleteOrganization(ctx, id)
Delete an organization

Permanently deletes an organization and all associated resources

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**|  | 

### Return type

[**models::DeleteOrganizationResponse**](DeleteOrganizationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DeleteScrapingJob**
> models::DeleteScrapingJobResponse DeleteScrapingJob(ctx, job_id, user_id, org_id, tenant_id)
Delete a specific job

This endpoint deletes a specific Google Maps scraping job

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **job_id** | **String**|  | 
  **user_id** | **String**|  | 
  **org_id** | **String**|  | 
  **tenant_id** | **String**|  | 

### Return type

[**models::DeleteScrapingJobResponse**](DeleteScrapingJobResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DeleteTenant**
> models::DeleteTenantResponse DeleteTenant(ctx, organization_id, tenant_id)
Delete a tenant

Permanently deletes a tenant and all associated resources

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **organization_id** | **String**|  | 
  **tenant_id** | **String**|  | 

### Return type

[**models::DeleteTenantResponse**](DeleteTenantResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DeleteTenantAPIKey**
> models::DeleteTenantApiKeyResponse DeleteTenantAPIKey(ctx, key_id, optional)
Delete tenant API key

Permanently deletes a tenant API key

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **key_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **key_id** | **String**|  | 
 **organization_id** | **String**|  | 
 **tenant_id** | **String**|  | 

### Return type

[**models::DeleteTenantApiKeyResponse**](DeleteTenantAPIKeyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DeleteWebhook**
> models::DeleteWebhookResponse DeleteWebhook(ctx, webhook_id, optional)
Delete webhook

Deletes a webhook configuration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **webhook_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **webhook_id** | **String**|  | 
 **organization_id** | **String**|  | 
 **workspace_id** | **String**|  | 
 **tenant_id** | **String**|  | 
 **account_id** | **String**|  | 

### Return type

[**models::DeleteWebhookResponse**](DeleteWebhookResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DeleteWorkflow**
> models::DeleteWorkflowResponse DeleteWorkflow(ctx, workspace_id, id, org_id, tenant_id, optional)
Delete workflow

Deletes a specific workflow

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **workspace_id** | **String**|  | 
  **id** | **String**|  | 
  **org_id** | **String**|  | 
  **tenant_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **workspace_id** | **String**|  | 
 **id** | **String**|  | 
 **org_id** | **String**|  | 
 **tenant_id** | **String**|  | 
 **account_id** | **String**|  | 

### Return type

[**models::DeleteWorkflowResponse**](DeleteWorkflowResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DeleteWorkspace**
> models::DeleteWorkspaceResponse DeleteWorkspace(ctx, id)
Delete a workspace

Deletes a specific workspace

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**|  | 

### Return type

[**models::DeleteWorkspaceResponse**](DeleteWorkspaceResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DownloadScrapingResults**
> models::DownloadScrapingResultsResponse DownloadScrapingResults(ctx, job_id, user_id, org_id, tenant_id)
Download job results as CSV

This endpoint downloads the results of a Google Maps scraping job as CSV

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **job_id** | **String**|  | 
  **user_id** | **String**|  | 
  **org_id** | **String**|  | 
  **tenant_id** | **String**|  | 

### Return type

[**models::DownloadScrapingResultsResponse**](DownloadScrapingResultsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetAccount**
> models::GetAccountResponse GetAccount(ctx, id, optional)
Get account details

Retrieves details of a specific account

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**|  | 
 **organization_id** | **String**|  | 
 **tenant_id** | **String**|  | 

### Return type

[**models::GetAccountResponse**](GetAccountResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetAccountUsage**
> models::GetAccountUsageResponse GetAccountUsage(ctx, id)
Get account usage

Retrieves usage details for a given account

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**|  | 

### Return type

[**models::GetAccountUsageResponse**](GetAccountUsageResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetAPIKey**
> models::GetApiKeyResponse GetAPIKey(ctx, key_id, optional)
Get API key details

Retrieves details of a specific API key

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **key_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **key_id** | **String**|  | 
 **organization_id** | **String**|  | 
 **tenant_id** | **String**|  | 
 **account_id** | **String**|  | 
 **workspace_id** | **String**|  | 

### Return type

[**models::GetApiKeyResponse**](GetAPIKeyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetLead**
> models::GetLeadResponse GetLead(ctx, lead_id, optional)
Get lead details

Retrieves detailed information about a specific lead

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **lead_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **lead_id** | **String**|  | 
 **organization_id** | **String**|  | 
 **workspace_id** | **String**|  | 
 **tenant_id** | **String**|  | 
 **account_id** | **String**|  | 

### Return type

[**models::GetLeadResponse**](GetLeadResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetOrganization**
> models::GetOrganizationResponse GetOrganization(ctx, id)
Get organization details

Retrieves details of a specific organization

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**|  | 

### Return type

[**models::GetOrganizationResponse**](GetOrganizationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetScrapingJob**
> models::GetScrapingJobResponse GetScrapingJob(ctx, job_id, user_id, org_id, tenant_id)
Get a specific job

This endpoint retrieves a specific Google Maps scraping job

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **job_id** | **String**|  | 
  **user_id** | **String**|  | 
  **org_id** | **String**|  | 
  **tenant_id** | **String**|  | 

### Return type

[**models::GetScrapingJobResponse**](GetScrapingJobResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetTenant**
> models::GetTenantResponse GetTenant(ctx, organization_id, tenant_id)
Get tenant details

Retrieves details of a specific tenant

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **organization_id** | **String**|  | 
  **tenant_id** | **String**|  | 

### Return type

[**models::GetTenantResponse**](GetTenantResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetTenantAPIKey**
> models::GetTenantApiKeyResponse GetTenantAPIKey(ctx, key_id, optional)
Get tenant API key details

Retrieves details of a specific tenant API key

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **key_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **key_id** | **String**|  | 
 **organization_id** | **String**|  | 
 **tenant_id** | **String**|  | 

### Return type

[**models::GetTenantApiKeyResponse**](GetTenantAPIKeyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetWebhook**
> models::GetWebhookResponse GetWebhook(ctx, webhook_id, optional)
Get webhook

Retrieves details of a specific webhook configuration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **webhook_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **webhook_id** | **String**|  | 
 **organization_id** | **String**|  | 
 **workspace_id** | **String**|  | 
 **tenant_id** | **String**|  | 
 **account_id** | **String**|  | 

### Return type

[**models::GetWebhookResponse**](GetWebhookResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetWorkflow**
> models::GetWorkflowResponse GetWorkflow(ctx, workspace_id, id)
Get workflow details

Retrieves details of a specific workflow

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **workspace_id** | **String**|  | 
  **id** | **String**|  | 

### Return type

[**models::GetWorkflowResponse**](GetWorkflowResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetWorkspace**
> models::GetWorkspaceResponse GetWorkspace(ctx, id, optional)
Get workspace details

Retrieves details of a specific workspace

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**|  | 
 **organization_id** | **String**|  | 
 **tenant_id** | **String**|  | 
 **account_id** | **String**|  | 

### Return type

[**models::GetWorkspaceResponse**](GetWorkspaceResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetWorkspaceAnalytics**
> models::GetWorkspaceAnalyticsResponse GetWorkspaceAnalytics(ctx, workspace_id, optional)
Get workspace analytics

Retrieves analytics data for a specific workspace

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
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

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ListAccounts**
> models::ListAccountsResponse ListAccounts(ctx, optional)
List all accounts

Retrieves a list of accounts

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **page_size** | **i32**|  | 
 **page_number** | **i32**|  | 
 **filter** | **String**|  | 
 **organization_id** | **String**|  | 
 **tenant_id** | **String**|  | 

### Return type

[**models::ListAccountsResponse**](ListAccountsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ListAPIKeys**
> models::ListApiKeysResponse ListAPIKeys(ctx, optional)
List API keys

Retrieves a list of API keys with optional filtering

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **organization_id** | **String**|  | 
 **tenant_id** | **String**|  | 
 **account_id** | **String**|  | 
 **workspace_id** | **String**|  | 
 **page_size** | **i32**|  | 
 **page_number** | **i32**|  | 
 **status** | **String**|  | 
 **search** | **String**|  | 

### Return type

[**models::ListApiKeysResponse**](ListAPIKeysResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ListLeads**
> models::ListLeadsResponse ListLeads(ctx, optional)
List leads

Retrieves a paginated list of leads with comprehensive filtering options

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **organization_id** | **String**| Context identifiers | 
 **workspace_id** | **String**|  | 
 **tenant_id** | **String**|  | 
 **account_id** | **String**|  | 
 **page_size** | **i32**| Pagination | 
 **page_number** | **i32**|  | 

### Return type

[**models::ListLeadsResponse**](ListLeadsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ListOrganizations**
> models::ListOrganizationsResponse ListOrganizations(ctx, optional)
List all organizations

Retrieves a list of all organizations in a tenant

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **page_size** | **i32**|  | 
 **page_number** | **i32**|  | 

### Return type

[**models::ListOrganizationsResponse**](ListOrganizationsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ListScrapingJobs**
> models::ListScrapingJobsResponse ListScrapingJobs(ctx, auth_platform_user_id, org_id, tenant_id, optional)
Get all jobs

This endpoint retrieves all Google Maps scraping jobs

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **auth_platform_user_id** | **String**|  | 
  **org_id** | **String**|  | 
  **tenant_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **auth_platform_user_id** | **String**|  | 
 **org_id** | **String**|  | 
 **tenant_id** | **String**|  | 
 **page_size** | **i32**|  | 
 **page_number** | **i32**|  | 
 **workspace_id** | **String**|  | 
 **workflow_id** | **String**|  | 

### Return type

[**models::ListScrapingJobsResponse**](ListScrapingJobsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ListTenantAPIKeys**
> models::ListTenantApiKeysResponse ListTenantAPIKeys(ctx, optional)
List tenant API keys

Retrieves all API keys for a specific tenant

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **organization_id** | **String**|  | 
 **tenant_id** | **String**|  | 
 **page_size** | **i32**|  | 
 **page_number** | **i32**|  | 
 **status** | **String**|  | 

### Return type

[**models::ListTenantApiKeysResponse**](ListTenantAPIKeysResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ListTenants**
> models::ListTenantsResponse ListTenants(ctx, organization_id, optional)
List all tenants

Retrieves a list of all tenants in the system

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **organization_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **organization_id** | **String**|  | 
 **page_size** | **i32**|  | 
 **page_number** | **i32**|  | 

### Return type

[**models::ListTenantsResponse**](ListTenantsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ListWebhooks**
> models::ListWebhooksResponse ListWebhooks(ctx, optional)
List webhooks

Lists all webhook configurations with pagination

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **organization_id** | **String**|  | 
 **workspace_id** | **String**|  | 
 **tenant_id** | **String**|  | 
 **account_id** | **String**|  | 
 **page_size** | **i32**|  | 
 **page_number** | **i32**|  | 
 **status** | **String**|  | 
 **search** | **String**|  | 

### Return type

[**models::ListWebhooksResponse**](ListWebhooksResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ListWorkflows**
> models::ListWorkflowsResponse ListWorkflows(ctx, workspace_id, optional)
List workflows

Retrieves a list of workflows for a given workspace

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **workspace_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **workspace_id** | **String**|  | 
 **page_size** | **i32**|  | 
 **page_number** | **i32**|  | 
 **filter** | **String**|  | 
 **organization_id** | **String**|  | 
 **tenant_id** | **String**|  | 
 **account_id** | **String**|  | 

### Return type

[**models::ListWorkflowsResponse**](ListWorkflowsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ListWorkspaces**
> models::ListWorkspacesResponse ListWorkspaces(ctx, optional)
List workspaces

Retrieves a list of workspaces for a given account

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account_id** | **String**|  | 
 **page_size** | **i32**|  | 
 **page_number** | **i32**|  | 
 **organization_id** | **String**|  | 
 **tenant_id** | **String**|  | 

### Return type

[**models::ListWorkspacesResponse**](ListWorkspacesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **PauseWorkflow**
> models::PauseWorkflowResponse PauseWorkflow(ctx, workspace_id, id, pause_workflow_body)
Pause workflow execution

Pauses the execution of a specific workflow

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **workspace_id** | **String**|  | 
  **id** | **String**|  | 
  **pause_workflow_body** | [**PauseWorkflowBody**](PauseWorkflowBody.md)|  | 

### Return type

[**models::PauseWorkflowResponse**](PauseWorkflowResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **RotateAPIKey**
> models::RotateApiKeyResponse RotateAPIKey(ctx, rotate_api_key_request)
Rotate API key

Rotates an API key while maintaining its configuration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **rotate_api_key_request** | [**RotateApiKeyRequest**](RotateApiKeyRequest.md)|  | 

### Return type

[**models::RotateApiKeyResponse**](RotateAPIKeyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **RotateTenantAPIKey**
> models::RotateTenantApiKeyResponse RotateTenantAPIKey(ctx, rotate_tenant_api_key_request)
Rotate tenant API key

Rotates a tenant API key while maintaining its configuration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **rotate_tenant_api_key_request** | [**RotateTenantApiKeyRequest**](RotateTenantApiKeyRequest.md)|  | 

### Return type

[**models::RotateTenantApiKeyResponse**](RotateTenantAPIKeyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **TriggerWorkflow**
> models::TriggerWorkflowResponse TriggerWorkflow(ctx, workspace_id, id, trigger_workflow_body)
Trigger workflow execution

Triggers the execution of a specific workflow

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **workspace_id** | **String**|  | 
  **id** | **String**|  | 
  **trigger_workflow_body** | [**TriggerWorkflowBody**](TriggerWorkflowBody.md)|  | 

### Return type

[**models::TriggerWorkflowResponse**](TriggerWorkflowResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **UpdateAccount**
> models::UpdateAccountResponse UpdateAccount(ctx, update_account_request)
Update account details

Updates specified fields of an existing account

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **update_account_request** | [**UpdateAccountRequest**](UpdateAccountRequest.md)|  | 

### Return type

[**models::UpdateAccountResponse**](UpdateAccountResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **UpdateAccountSettings**
> models::UpdateAccountSettingsResponse UpdateAccountSettings(ctx, update_account_settings_request)
Update account settings

Updates the settings for a given account

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **update_account_settings_request** | [**UpdateAccountSettingsRequest**](UpdateAccountSettingsRequest.md)|  | 

### Return type

[**models::UpdateAccountSettingsResponse**](UpdateAccountSettingsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **UpdateAPIKey**
> models::UpdateApiKeyResponse UpdateAPIKey(ctx, update_api_key_request)
Update API key

Updates an existing API key's configuration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **update_api_key_request** | [**UpdateApiKeyRequest**](UpdateApiKeyRequest.md)|  | 

### Return type

[**models::UpdateApiKeyResponse**](UpdateAPIKeyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **UpdateOrganization**
> models::UpdateOrganizationResponse UpdateOrganization(ctx, update_organization_request)
Update organization details

Updates the configuration of a specific organization

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **update_organization_request** | [**UpdateOrganizationRequest**](UpdateOrganizationRequest.md)|  | 

### Return type

[**models::UpdateOrganizationResponse**](UpdateOrganizationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **UpdateTenant**
> models::UpdateTenantResponse UpdateTenant(ctx, update_tenant_request)
Update tenant details

Updates the configuration of a specific tenant

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **update_tenant_request** | [**UpdateTenantRequest**](UpdateTenantRequest.md)|  | 

### Return type

[**models::UpdateTenantResponse**](UpdateTenantResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **UpdateTenantAPIKey**
> models::UpdateTenantApiKeyResponse UpdateTenantAPIKey(ctx, update_tenant_api_key_request)
Update tenant API key

Updates an existing tenant API key's configuration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **update_tenant_api_key_request** | [**UpdateTenantApiKeyRequest**](UpdateTenantApiKeyRequest.md)|  | 

### Return type

[**models::UpdateTenantApiKeyResponse**](UpdateTenantAPIKeyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **UpdateWebhook**
> models::UpdateWebhookResponse UpdateWebhook(ctx, update_webhook_request)
Update webhook

Updates an existing webhook configuration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **update_webhook_request** | [**UpdateWebhookRequest**](UpdateWebhookRequest.md)|  | 

### Return type

[**models::UpdateWebhookResponse**](UpdateWebhookResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **UpdateWorkflow**
> models::UpdateWorkflowResponse UpdateWorkflow(ctx, update_workflow_request)
Update workflow details

Updates the details of a specific workflow

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **update_workflow_request** | [**UpdateWorkflowRequest**](UpdateWorkflowRequest.md)|  | 

### Return type

[**models::UpdateWorkflowResponse**](UpdateWorkflowResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **UpdateWorkspace**
> models::UpdateWorkspaceResponse UpdateWorkspace(ctx, update_workspace_request)
Update workspace details

Updates the details of a specific workspace

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **update_workspace_request** | [**UpdateWorkspaceRequest**](UpdateWorkspaceRequest.md)|  | 

### Return type

[**models::UpdateWorkspaceResponse**](UpdateWorkspaceResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

