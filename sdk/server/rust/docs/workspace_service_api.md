# workspace_service_api

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

Method | HTTP request | Description
------------- | ------------- | -------------
**CreateAccount**](workspace_service_api.md#CreateAccount) | **POST** /workspace-service/v1/accounts | Create a new account
**CreateWorkspace**](workspace_service_api.md#CreateWorkspace) | **POST** /workspace-service/v1/workspaces | Create workspace
**DeleteAccount**](workspace_service_api.md#DeleteAccount) | **DELETE** /workspace-service/v1/accounts/{id} | Delete account
**DeleteWorkspace1**](workspace_service_api.md#DeleteWorkspace1) | **DELETE** /workspace-service/v1/workspaces/{id} | Delete workspace
**GetAccount**](workspace_service_api.md#GetAccount) | **GET** /workspace-service/v1/accounts/{id} | Get account details
**GetWorkspace**](workspace_service_api.md#GetWorkspace) | **GET** /workspace-service/v1/workspaces/{id} | Get workspace details
**GetWorkspaceAnalytics**](workspace_service_api.md#GetWorkspaceAnalytics) | **GET** /workspace-service/v1/workspaces/analytics/{workspaceId} | Get workspace analytics
**GetWorkspaceComplianceReport**](workspace_service_api.md#GetWorkspaceComplianceReport) | **GET** /workspace-service/v1/workspaces/compliance-report/{workspaceId} | Get compliance report
**GetWorkspaceStorageStats**](workspace_service_api.md#GetWorkspaceStorageStats) | **GET** /workspace-service/v1/workspaces/storage-stats/{workspaceId} | Get storage statistics
**ListWorkspaceSharings**](workspace_service_api.md#ListWorkspaceSharings) | **GET** /workspace-service/v1/workspaces/sharings/{workspaceId} | List workspace sharings
**ListWorkspaces**](workspace_service_api.md#ListWorkspaces) | **GET** /workspace-service/v1/workspaces/list | List workspaces
**RemoveWorkspaceSharing**](workspace_service_api.md#RemoveWorkspaceSharing) | **DELETE** /workspace-service/v1/workspace-sharings/{sharingId} | Remove workspace sharing
**ShareWorkspace**](workspace_service_api.md#ShareWorkspace) | **POST** /workspace-service/v1/workspaces/{workspaceId}/share | Share workspace
**UpdateAccount**](workspace_service_api.md#UpdateAccount) | **PUT** /workspace-service/v1/accounts | Update account details
**UpdateWorkspace1**](workspace_service_api.md#UpdateWorkspace1) | **PUT** /workspace-service/v1/workspaces | Update workspace
**UpdateWorkspaceSharing**](workspace_service_api.md#UpdateWorkspaceSharing) | **PUT** /workspace-service/v1/workspace-sharings | Update workspace sharing


# **CreateAccount**
> models::CreateAccountResponse CreateAccount(create_account_request)
Create a new account

Creates a new user account with initial workspace

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

# **CreateWorkspace**
> models::CreateWorkspaceResponse CreateWorkspace(create_workspace_request)
Create workspace

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

# **DeleteWorkspace1**
> models::DeleteWorkspaceResponse DeleteWorkspace1(id)
Delete workspace

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

# **GetAccount**
> models::GetAccountResponse GetAccount(id)
Get account details

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

# **GetWorkspace**
> models::GetWorkspaceResponse GetWorkspace(id)
Get workspace details

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**|  | 

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

# **GetWorkspaceComplianceReport**
> models::GetWorkspaceComplianceReportResponse GetWorkspaceComplianceReport(workspace_id, optional)
Get compliance report

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
 **compliance_type** | **String**|  | 

### Return type

[**models::GetWorkspaceComplianceReportResponse**](GetWorkspaceComplianceReportResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetWorkspaceStorageStats**
> models::GetWorkspaceStorageStatsResponse GetWorkspaceStorageStats(workspace_id)
Get storage statistics

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **workspace_id** | **String**|  | 

### Return type

[**models::GetWorkspaceStorageStatsResponse**](GetWorkspaceStorageStatsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ListWorkspaceSharings**
> models::ListWorkspaceSharingsResponse ListWorkspaceSharings(workspace_id, optional)
List workspace sharings

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

### Return type

[**models::ListWorkspaceSharingsResponse**](ListWorkspaceSharingsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ListWorkspaces**
> models::ListWorkspacesResponse ListWorkspaces(optional)
List workspaces

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
 **filter** | **String**|  | 

### Return type

[**models::ListWorkspacesResponse**](ListWorkspacesResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **RemoveWorkspaceSharing**
> models::RemoveWorkspaceSharingResponse RemoveWorkspaceSharing(sharing_id)
Remove workspace sharing

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **sharing_id** | **String**|  | 

### Return type

[**models::RemoveWorkspaceSharingResponse**](RemoveWorkspaceSharingResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ShareWorkspace**
> models::ShareWorkspaceResponse ShareWorkspace(workspace_id, share_workspace_body)
Share workspace

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **workspace_id** | **String**|  | 
  **share_workspace_body** | [**ShareWorkspaceBody**](ShareWorkspaceBody.md)|  | 

### Return type

[**models::ShareWorkspaceResponse**](ShareWorkspaceResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **UpdateAccount**
> models::UpdateAccountResponse UpdateAccount(update_account_request)
Update account details

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

# **UpdateWorkspace1**
> models::UpdateWorkspaceResponse UpdateWorkspace1(update_workspace_request1)
Update workspace

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **update_workspace_request1** | [**UpdateWorkspaceRequest1**](UpdateWorkspaceRequest1.md)|  | 

### Return type

[**models::UpdateWorkspaceResponse**](UpdateWorkspaceResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **UpdateWorkspaceSharing**
> models::UpdateWorkspaceSharingResponse UpdateWorkspaceSharing(update_workspace_sharing_request)
Update workspace sharing

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **update_workspace_sharing_request** | [**UpdateWorkspaceSharingRequest**](UpdateWorkspaceSharingRequest.md)|  | 

### Return type

[**models::UpdateWorkspaceSharingResponse**](UpdateWorkspaceSharingResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

