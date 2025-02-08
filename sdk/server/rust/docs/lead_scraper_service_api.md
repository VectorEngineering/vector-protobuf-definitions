# lead_scraper_service_api

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

Method | HTTP request | Description
------------- | ------------- | -------------
**CreateAPIKey**](lead_scraper_service_api.md#CreateAPIKey) | **POST** /lead-scraper-microservice/api/v1/api-keys | Create a new API key
**CreateOrganization**](lead_scraper_service_api.md#CreateOrganization) | **POST** /lead-scraper-microservice/api/v1/organization | Create a new organization
**CreateTenant**](lead_scraper_service_api.md#CreateTenant) | **POST** /lead-scraper-microservice/api/v1/organizations/{organizationId}/tenants | Create a new tenant
**CreateTenantAPIKey**](lead_scraper_service_api.md#CreateTenantAPIKey) | **POST** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys | Create a new tenant API key
**CreateWebhook**](lead_scraper_service_api.md#CreateWebhook) | **POST** /lead-scraper-microservice/api/v1/webhooks | Create webhook
**DeleteAPIKey**](lead_scraper_service_api.md#DeleteAPIKey) | **DELETE** /lead-scraper-microservice/api/v1/api-keys/{keyId} | Delete API key
**DeleteOrganization**](lead_scraper_service_api.md#DeleteOrganization) | **DELETE** /lead-scraper-microservice/api/v1/organization/{id} | Delete an organization
**DeleteTenant**](lead_scraper_service_api.md#DeleteTenant) | **DELETE** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId}/{tenantId} | Delete a tenant
**DeleteTenantAPIKey**](lead_scraper_service_api.md#DeleteTenantAPIKey) | **DELETE** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/{keyId} | Delete tenant API key
**DeleteWebhook**](lead_scraper_service_api.md#DeleteWebhook) | **DELETE** /lead-scraper-microservice/api/v1/webhooks/{webhookId} | Delete webhook
**DeleteWorkspace**](lead_scraper_service_api.md#DeleteWorkspace) | **DELETE** /lead-scraper-microservice/api/v1/workspace/{id} | Delete a workspace
**GetAPIKey**](lead_scraper_service_api.md#GetAPIKey) | **GET** /lead-scraper-microservice/api/v1/api-keys/{keyId} | Get API key details
**GetLead**](lead_scraper_service_api.md#GetLead) | **GET** /lead-scraper-microservice/api/v1/leads/{leadId} | Get lead details
**GetOrganization**](lead_scraper_service_api.md#GetOrganization) | **GET** /lead-scraper-microservice/api/v1/organization/{id} | Get organization details
**GetTenant**](lead_scraper_service_api.md#GetTenant) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/{organizationId}/{tenantId} | Get tenant details
**GetTenantAPIKey**](lead_scraper_service_api.md#GetTenantAPIKey) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/{keyId} | Get tenant API key details
**GetWebhook**](lead_scraper_service_api.md#GetWebhook) | **GET** /lead-scraper-microservice/api/v1/webhooks/{webhookId} | Get webhook
**GetWorkflow**](lead_scraper_service_api.md#GetWorkflow) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id} | Get workflow details
**GetWorkspace**](lead_scraper_service_api.md#GetWorkspace) | **GET** /lead-scraper-microservice/api/v1/workspace/{id} | Get workspace details
**GetWorkspaceAnalytics**](lead_scraper_service_api.md#GetWorkspaceAnalytics) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/analytics | Get workspace analytics
**ListAPIKeys**](lead_scraper_service_api.md#ListAPIKeys) | **GET** /lead-scraper-microservice/api/v1/api-keys/list | List API keys
**ListLeads**](lead_scraper_service_api.md#ListLeads) | **GET** /lead-scraper-microservice/api/v1/leads | List leads
**ListOrganizations**](lead_scraper_service_api.md#ListOrganizations) | **GET** /lead-scraper-microservice/api/v1/organization | List all organizations
**ListTenantAPIKeys**](lead_scraper_service_api.md#ListTenantAPIKeys) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/list | List tenant API keys
**ListTenants**](lead_scraper_service_api.md#ListTenants) | **GET** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId} | List all tenants
**ListWebhooks**](lead_scraper_service_api.md#ListWebhooks) | **GET** /lead-scraper-microservice/api/v1/webhooks | List webhooks
**ListWorkflows**](lead_scraper_service_api.md#ListWorkflows) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows | List workflows
**PauseWorkflow**](lead_scraper_service_api.md#PauseWorkflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id}/pause | Pause workflow execution
**RotateAPIKey**](lead_scraper_service_api.md#RotateAPIKey) | **POST** /lead-scraper-microservice/api/v1/api-keys/rotate | Rotate API key
**RotateTenantAPIKey**](lead_scraper_service_api.md#RotateTenantAPIKey) | **POST** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/rotate | Rotate tenant API key
**TriggerWorkflow**](lead_scraper_service_api.md#TriggerWorkflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id}/trigger | Trigger workflow execution
**UpdateAPIKey**](lead_scraper_service_api.md#UpdateAPIKey) | **PUT** /lead-scraper-microservice/api/v1/api-keys | Update API key
**UpdateOrganization**](lead_scraper_service_api.md#UpdateOrganization) | **PUT** /lead-scraper-microservice/api/v1/organization | Update organization details
**UpdateTenant**](lead_scraper_service_api.md#UpdateTenant) | **PUT** /lead-scraper-microservice/api/v1/organizations/tenants | Update tenant details
**UpdateTenantAPIKey**](lead_scraper_service_api.md#UpdateTenantAPIKey) | **PUT** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys | Update tenant API key
**UpdateWebhook**](lead_scraper_service_api.md#UpdateWebhook) | **PUT** /lead-scraper-microservice/api/v1/webhooks | Update webhook
**UpdateWorkflow**](lead_scraper_service_api.md#UpdateWorkflow) | **PUT** /lead-scraper-microservice/api/v1/workspaces/workflow | Update workflow details
**UpdateWorkspace**](lead_scraper_service_api.md#UpdateWorkspace) | **PUT** /lead-scraper-microservice/api/v1/workspace | Update workspace details


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
> models::CreateWebhookResponse CreateWebhook(create_webhook_request)
Create webhook

Creates a new webhook configuration for event notifications

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **create_webhook_request** | [**CreateWebhookRequest**](CreateWebhookRequest.md)|  | 

### Return type

[**models::CreateWebhookResponse**](CreateWebhookResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DeleteAPIKey**
> models::DeleteApiKeyResponse DeleteAPIKey(key_id, optional)
Delete API key

Permanently deletes an API key

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
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

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DeleteOrganization**
> models::DeleteOrganizationResponse DeleteOrganization(id)
Delete an organization

Permanently deletes an organization and all associated resources

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**|  | 

### Return type

[**models::DeleteOrganizationResponse**](DeleteOrganizationResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DeleteTenant**
> models::DeleteTenantResponse DeleteTenant(organization_id, tenant_id)
Delete a tenant

Permanently deletes a tenant and all associated resources

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **organization_id** | **String**|  | 
  **tenant_id** | **String**|  | 

### Return type

[**models::DeleteTenantResponse**](DeleteTenantResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DeleteTenantAPIKey**
> models::DeleteTenantApiKeyResponse DeleteTenantAPIKey(key_id, optional)
Delete tenant API key

Permanently deletes a tenant API key

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
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

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DeleteWebhook**
> models::DeleteWebhookResponse DeleteWebhook(webhook_id, optional)
Delete webhook

Deletes a webhook configuration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
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

# **GetAPIKey**
> models::GetApiKeyResponse GetAPIKey(key_id, optional)
Get API key details

Retrieves details of a specific API key

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
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

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetLead**
> models::GetLeadResponse GetLead(lead_id, optional)
Get lead details

Retrieves detailed information about a specific lead

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
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

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetOrganization**
> models::GetOrganizationResponse GetOrganization(id)
Get organization details

Retrieves details of a specific organization

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**|  | 

### Return type

[**models::GetOrganizationResponse**](GetOrganizationResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetTenant**
> models::GetTenantResponse GetTenant(organization_id, tenant_id)
Get tenant details

Retrieves details of a specific tenant

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **organization_id** | **String**|  | 
  **tenant_id** | **String**|  | 

### Return type

[**models::GetTenantResponse**](GetTenantResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetTenantAPIKey**
> models::GetTenantApiKeyResponse GetTenantAPIKey(key_id, optional)
Get tenant API key details

Retrieves details of a specific tenant API key

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
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

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetWebhook**
> models::GetWebhookResponse GetWebhook(webhook_id, optional)
Get webhook

Retrieves details of a specific webhook configuration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
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

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetWorkflow**
> models::GetWorkflowResponse GetWorkflow(workspace_id, id)
Get workflow details

Retrieves details of a specific workflow

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **workspace_id** | **String**|  | 
  **id** | **String**|  | 

### Return type

[**models::GetWorkflowResponse**](GetWorkflowResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetWorkspace**
> models::GetWorkspaceResponse GetWorkspace(id, optional)
Get workspace details

Retrieves details of a specific workspace

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
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

# **ListAPIKeys**
> models::ListApiKeysResponse ListAPIKeys(optional)
List API keys

Retrieves a list of API keys with optional filtering

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
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

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ListLeads**
> models::ListLeadsResponse ListLeads(optional)
List leads

Retrieves a paginated list of leads with comprehensive filtering options

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
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

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ListOrganizations**
> models::ListOrganizationsResponse ListOrganizations(optional)
List all organizations

Retrieves a list of all organizations in a tenant

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
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

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ListTenantAPIKeys**
> models::ListTenantApiKeysResponse ListTenantAPIKeys(optional)
List tenant API keys

Retrieves all API keys for a specific tenant

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
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

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ListTenants**
> models::ListTenantsResponse ListTenants(organization_id, optional)
List all tenants

Retrieves a list of all tenants in the system

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
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

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ListWebhooks**
> models::ListWebhooksResponse ListWebhooks(optional)
List webhooks

Lists all webhook configurations with pagination

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
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
 **page_number** | **i32**|  | 
 **filter** | **String**|  | 
 **organization_id** | **String**|  | 
 **tenant_id** | **String**|  | 
 **account_id** | **String**|  | 

### Return type

[**models::ListWorkflowsResponse**](ListWorkflowsResponse.md)

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

# **RotateAPIKey**
> models::RotateApiKeyResponse RotateAPIKey(rotate_api_key_request)
Rotate API key

Rotates an API key while maintaining its configuration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **rotate_api_key_request** | [**RotateApiKeyRequest**](RotateApiKeyRequest.md)|  | 

### Return type

[**models::RotateApiKeyResponse**](RotateAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **RotateTenantAPIKey**
> models::RotateTenantApiKeyResponse RotateTenantAPIKey(rotate_tenant_api_key_request)
Rotate tenant API key

Rotates a tenant API key while maintaining its configuration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **rotate_tenant_api_key_request** | [**RotateTenantApiKeyRequest**](RotateTenantApiKeyRequest.md)|  | 

### Return type

[**models::RotateTenantApiKeyResponse**](RotateTenantAPIKeyResponse.md)

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

# **UpdateAPIKey**
> models::UpdateApiKeyResponse UpdateAPIKey(update_api_key_request)
Update API key

Updates an existing API key's configuration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **update_api_key_request** | [**UpdateApiKeyRequest**](UpdateApiKeyRequest.md)|  | 

### Return type

[**models::UpdateApiKeyResponse**](UpdateAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **UpdateOrganization**
> models::UpdateOrganizationResponse UpdateOrganization(update_organization_request)
Update organization details

Updates the configuration of a specific organization

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **update_organization_request** | [**UpdateOrganizationRequest**](UpdateOrganizationRequest.md)|  | 

### Return type

[**models::UpdateOrganizationResponse**](UpdateOrganizationResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **UpdateTenant**
> models::UpdateTenantResponse UpdateTenant(update_tenant_request)
Update tenant details

Updates the configuration of a specific tenant

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **update_tenant_request** | [**UpdateTenantRequest**](UpdateTenantRequest.md)|  | 

### Return type

[**models::UpdateTenantResponse**](UpdateTenantResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **UpdateTenantAPIKey**
> models::UpdateTenantApiKeyResponse UpdateTenantAPIKey(update_tenant_api_key_request)
Update tenant API key

Updates an existing tenant API key's configuration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **update_tenant_api_key_request** | [**UpdateTenantApiKeyRequest**](UpdateTenantApiKeyRequest.md)|  | 

### Return type

[**models::UpdateTenantApiKeyResponse**](UpdateTenantAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **UpdateWebhook**
> models::UpdateWebhookResponse UpdateWebhook(update_webhook_request)
Update webhook

Updates an existing webhook configuration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **update_webhook_request** | [**UpdateWebhookRequest**](UpdateWebhookRequest.md)|  | 

### Return type

[**models::UpdateWebhookResponse**](UpdateWebhookResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **UpdateWorkflow**
> models::UpdateWorkflowResponse UpdateWorkflow(update_workflow_request)
Update workflow details

Updates the details of a specific workflow

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **update_workflow_request** | [**UpdateWorkflowRequest**](UpdateWorkflowRequest.md)|  | 

### Return type

[**models::UpdateWorkflowResponse**](UpdateWorkflowResponse.md)

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

