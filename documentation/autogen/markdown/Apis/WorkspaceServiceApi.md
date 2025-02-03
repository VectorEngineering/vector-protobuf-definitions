# WorkspaceServiceApi

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**createAccount1**](WorkspaceServiceApi.md#createAccount1) | **POST** /workspace-service/v1/accounts | Create a new account |
| [**createWorkspace1**](WorkspaceServiceApi.md#createWorkspace1) | **POST** /workspace-service/v1/workspaces | Create workspace |
| [**deleteAccount1**](WorkspaceServiceApi.md#deleteAccount1) | **DELETE** /workspace-service/v1/accounts/{id} | Delete account |
| [**deleteWorkspace1**](WorkspaceServiceApi.md#deleteWorkspace1) | **DELETE** /workspace-service/v1/workspaces/{id} | Delete workspace |
| [**getAccount1**](WorkspaceServiceApi.md#getAccount1) | **GET** /workspace-service/v1/accounts/{id} | Get account details |
| [**getWorkspace1**](WorkspaceServiceApi.md#getWorkspace1) | **GET** /workspace-service/v1/workspaces/{id} | Get workspace details |
| [**getWorkspaceAnalytics1**](WorkspaceServiceApi.md#getWorkspaceAnalytics1) | **GET** /workspace-service/v1/workspaces/analytics/{workspaceId} | Get workspace analytics |
| [**getWorkspaceComplianceReport**](WorkspaceServiceApi.md#getWorkspaceComplianceReport) | **GET** /workspace-service/v1/workspaces/compliance-report/{workspaceId} | Get compliance report |
| [**getWorkspaceStorageStats**](WorkspaceServiceApi.md#getWorkspaceStorageStats) | **GET** /workspace-service/v1/workspaces/storage-stats/{workspaceId} | Get storage statistics |
| [**listWorkspaceSharings**](WorkspaceServiceApi.md#listWorkspaceSharings) | **GET** /workspace-service/v1/workspaces/sharings/{workspaceId} | List workspace sharings |
| [**listWorkspaces1**](WorkspaceServiceApi.md#listWorkspaces1) | **GET** /workspace-service/v1/workspaces/list | List workspaces |
| [**removeWorkspaceSharing**](WorkspaceServiceApi.md#removeWorkspaceSharing) | **DELETE** /workspace-service/v1/workspace-sharings/{sharingId} | Remove workspace sharing |
| [**shareWorkspace**](WorkspaceServiceApi.md#shareWorkspace) | **POST** /workspace-service/v1/workspaces/{workspaceId}/share | Share workspace |
| [**updateAccount1**](WorkspaceServiceApi.md#updateAccount1) | **PUT** /workspace-service/v1/accounts | Update account details |
| [**updateWorkspace1**](WorkspaceServiceApi.md#updateWorkspace1) | **PUT** /workspace-service/v1/workspaces | Update workspace |
| [**updateWorkspaceSharing**](WorkspaceServiceApi.md#updateWorkspaceSharing) | **PUT** /workspace-service/v1/workspace-sharings | Update workspace sharing |


<a name="createAccount1"></a>
# **createAccount1**
> CreateAccountResponse1 createAccount1(CreateAccountRequest1)

Create a new account

    Creates a new user account with initial workspace

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **CreateAccountRequest1** | [**CreateAccountRequest1**](../Models/CreateAccountRequest1.md)|  | |

### Return type

[**CreateAccountResponse1**](../Models/CreateAccountResponse1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, 

<a name="createWorkspace1"></a>
# **createWorkspace1**
> CreateWorkspaceResponse1 createWorkspace1(CreateWorkspaceRequest1)

Create workspace

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **CreateWorkspaceRequest1** | [**CreateWorkspaceRequest1**](../Models/CreateWorkspaceRequest1.md)|  | |

### Return type

[**CreateWorkspaceResponse1**](../Models/CreateWorkspaceResponse1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, 

<a name="deleteAccount1"></a>
# **deleteAccount1**
> DeleteAccountResponse deleteAccount1(id)

Delete account

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | **String**|  | [default to null] |

### Return type

[**DeleteAccountResponse**](../Models/DeleteAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="deleteWorkspace1"></a>
# **deleteWorkspace1**
> DeleteWorkspaceResponse deleteWorkspace1(id)

Delete workspace

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | **String**|  | [default to null] |

### Return type

[**DeleteWorkspaceResponse**](../Models/DeleteWorkspaceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="getAccount1"></a>
# **getAccount1**
> GetAccountResponse1 getAccount1(id)

Get account details

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | **String**|  | [default to null] |

### Return type

[**GetAccountResponse1**](../Models/GetAccountResponse1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="getWorkspace1"></a>
# **getWorkspace1**
> GetWorkspaceResponse getWorkspace1(id)

Get workspace details

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | **String**|  | [default to null] |

### Return type

[**GetWorkspaceResponse**](../Models/GetWorkspaceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="getWorkspaceAnalytics1"></a>
# **getWorkspaceAnalytics1**
> GetWorkspaceAnalyticsResponse1 getWorkspaceAnalytics1(workspaceId, startTime, endTime)

Get workspace analytics

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **workspaceId** | **String**|  | [default to null] |
| **startTime** | **Date**|  | [optional] [default to null] |
| **endTime** | **Date**|  | [optional] [default to null] |

### Return type

[**GetWorkspaceAnalyticsResponse1**](../Models/GetWorkspaceAnalyticsResponse1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="getWorkspaceComplianceReport"></a>
# **getWorkspaceComplianceReport**
> GetWorkspaceComplianceReportResponse getWorkspaceComplianceReport(workspaceId, complianceType)

Get compliance report

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **workspaceId** | **String**|  | [default to null] |
| **complianceType** | **String**|  | [optional] [default to null] |

### Return type

[**GetWorkspaceComplianceReportResponse**](../Models/GetWorkspaceComplianceReportResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="getWorkspaceStorageStats"></a>
# **getWorkspaceStorageStats**
> GetWorkspaceStorageStatsResponse getWorkspaceStorageStats(workspaceId)

Get storage statistics

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **workspaceId** | **String**|  | [default to null] |

### Return type

[**GetWorkspaceStorageStatsResponse**](../Models/GetWorkspaceStorageStatsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="listWorkspaceSharings"></a>
# **listWorkspaceSharings**
> ListWorkspaceSharingsResponse listWorkspaceSharings(workspaceId, pageSize, pageToken)

List workspace sharings

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **workspaceId** | **String**|  | [default to null] |
| **pageSize** | **Integer**|  | [optional] [default to null] |
| **pageToken** | **String**|  | [optional] [default to null] |

### Return type

[**ListWorkspaceSharingsResponse**](../Models/ListWorkspaceSharingsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="listWorkspaces1"></a>
# **listWorkspaces1**
> ListWorkspacesResponse1 listWorkspaces1(accountId, pageSize, pageToken, filter)

List workspaces

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **accountId** | **String**|  | [optional] [default to null] |
| **pageSize** | **Integer**|  | [optional] [default to null] |
| **pageToken** | **String**|  | [optional] [default to null] |
| **filter** | **String**|  | [optional] [default to null] |

### Return type

[**ListWorkspacesResponse1**](../Models/ListWorkspacesResponse1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="removeWorkspaceSharing"></a>
# **removeWorkspaceSharing**
> RemoveWorkspaceSharingResponse removeWorkspaceSharing(sharingId)

Remove workspace sharing

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **sharingId** | **String**|  | [default to null] |

### Return type

[**RemoveWorkspaceSharingResponse**](../Models/RemoveWorkspaceSharingResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="shareWorkspace"></a>
# **shareWorkspace**
> ShareWorkspaceResponse shareWorkspace(workspaceId, ShareWorkspaceBody)

Share workspace

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **workspaceId** | **String**|  | [default to null] |
| **ShareWorkspaceBody** | [**ShareWorkspaceBody**](../Models/ShareWorkspaceBody.md)|  | |

### Return type

[**ShareWorkspaceResponse**](../Models/ShareWorkspaceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, 

<a name="updateAccount1"></a>
# **updateAccount1**
> UpdateAccountResponse updateAccount1(UpdateAccountRequest)

Update account details

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **UpdateAccountRequest** | [**UpdateAccountRequest**](../Models/UpdateAccountRequest.md)|  | |

### Return type

[**UpdateAccountResponse**](../Models/UpdateAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, 

<a name="updateWorkspace1"></a>
# **updateWorkspace1**
> UpdateWorkspaceResponse updateWorkspace1(UpdateWorkspaceRequest)

Update workspace

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **UpdateWorkspaceRequest** | [**UpdateWorkspaceRequest**](../Models/UpdateWorkspaceRequest.md)|  | |

### Return type

[**UpdateWorkspaceResponse**](../Models/UpdateWorkspaceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, 

<a name="updateWorkspaceSharing"></a>
# **updateWorkspaceSharing**
> UpdateWorkspaceSharingResponse updateWorkspaceSharing(UpdateWorkspaceSharingRequest)

Update workspace sharing

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **UpdateWorkspaceSharingRequest** | [**UpdateWorkspaceSharingRequest**](../Models/UpdateWorkspaceSharingRequest.md)|  | |

### Return type

[**UpdateWorkspaceSharingResponse**](../Models/UpdateWorkspaceSharingResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, 

