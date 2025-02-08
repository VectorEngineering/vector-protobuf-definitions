# playbookmedia_backend_client_sdk.WorkspaceServiceApi

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_account**](WorkspaceServiceApi.md#create_account) | **POST** /workspace-service/v1/accounts | Create a new account
[**create_workspace**](WorkspaceServiceApi.md#create_workspace) | **POST** /workspace-service/v1/workspaces | Create workspace
[**delete_account**](WorkspaceServiceApi.md#delete_account) | **DELETE** /workspace-service/v1/accounts/{id} | Delete account
[**delete_workspace1**](WorkspaceServiceApi.md#delete_workspace1) | **DELETE** /workspace-service/v1/workspaces/{id} | Delete workspace
[**get_account**](WorkspaceServiceApi.md#get_account) | **GET** /workspace-service/v1/accounts/{id} | Get account details
[**get_workspace1**](WorkspaceServiceApi.md#get_workspace1) | **GET** /workspace-service/v1/workspaces/{id} | Get workspace details
[**get_workspace_analytics1**](WorkspaceServiceApi.md#get_workspace_analytics1) | **GET** /workspace-service/v1/workspaces/analytics/{workspaceId} | Get workspace analytics
[**get_workspace_compliance_report**](WorkspaceServiceApi.md#get_workspace_compliance_report) | **GET** /workspace-service/v1/workspaces/compliance-report/{workspaceId} | Get compliance report
[**get_workspace_storage_stats**](WorkspaceServiceApi.md#get_workspace_storage_stats) | **GET** /workspace-service/v1/workspaces/storage-stats/{workspaceId} | Get storage statistics
[**list_workspace_sharings**](WorkspaceServiceApi.md#list_workspace_sharings) | **GET** /workspace-service/v1/workspaces/sharings/{workspaceId} | List workspace sharings
[**list_workspaces1**](WorkspaceServiceApi.md#list_workspaces1) | **GET** /workspace-service/v1/workspaces/list | List workspaces
[**remove_workspace_sharing**](WorkspaceServiceApi.md#remove_workspace_sharing) | **DELETE** /workspace-service/v1/workspace-sharings/{sharingId} | Remove workspace sharing
[**share_workspace**](WorkspaceServiceApi.md#share_workspace) | **POST** /workspace-service/v1/workspaces/{workspaceId}/share | Share workspace
[**update_account**](WorkspaceServiceApi.md#update_account) | **PUT** /workspace-service/v1/accounts | Update account details
[**update_workspace1**](WorkspaceServiceApi.md#update_workspace1) | **PUT** /workspace-service/v1/workspaces | Update workspace
[**update_workspace_sharing**](WorkspaceServiceApi.md#update_workspace_sharing) | **PUT** /workspace-service/v1/workspace-sharings | Update workspace sharing


# **create_account**
> CreateAccountResponse create_account(create_account_request)

Create a new account

Creates a new user account with initial workspace

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.create_account_request import CreateAccountRequest
from playbookmedia_backend_client_sdk.models.create_account_response import CreateAccountResponse
from playbookmedia_backend_client_sdk.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://lead-scraping-microservice.vector.svc.cluster.local:9896
# See configuration.py for a list of all supported configuration parameters.
configuration = playbookmedia_backend_client_sdk.Configuration(
    host = "http://lead-scraping-microservice.vector.svc.cluster.local:9896"
)


# Enter a context with an instance of the API client
async with playbookmedia_backend_client_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = playbookmedia_backend_client_sdk.WorkspaceServiceApi(api_client)
    create_account_request = playbookmedia_backend_client_sdk.CreateAccountRequest() # CreateAccountRequest | 

    try:
        # Create a new account
        api_response = await api_instance.create_account(create_account_request)
        print("The response of WorkspaceServiceApi->create_account:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling WorkspaceServiceApi->create_account: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **create_account_request** | [**CreateAccountRequest**](CreateAccountRequest.md)|  | 

### Return type

[**CreateAccountResponse**](CreateAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | A successful response. |  -  |
**201** | Account created successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**403** | Forbidden - Insufficient permissions |  -  |
**404** | Not Found - Resource not found |  -  |
**409** | Conflict - Resource already exists |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_workspace**
> CreateWorkspaceResponse create_workspace(create_workspace_request)

Create workspace

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.create_workspace_request import CreateWorkspaceRequest
from playbookmedia_backend_client_sdk.models.create_workspace_response import CreateWorkspaceResponse
from playbookmedia_backend_client_sdk.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://lead-scraping-microservice.vector.svc.cluster.local:9896
# See configuration.py for a list of all supported configuration parameters.
configuration = playbookmedia_backend_client_sdk.Configuration(
    host = "http://lead-scraping-microservice.vector.svc.cluster.local:9896"
)


# Enter a context with an instance of the API client
async with playbookmedia_backend_client_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = playbookmedia_backend_client_sdk.WorkspaceServiceApi(api_client)
    create_workspace_request = playbookmedia_backend_client_sdk.CreateWorkspaceRequest() # CreateWorkspaceRequest | 

    try:
        # Create workspace
        api_response = await api_instance.create_workspace(create_workspace_request)
        print("The response of WorkspaceServiceApi->create_workspace:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling WorkspaceServiceApi->create_workspace: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **create_workspace_request** | [**CreateWorkspaceRequest**](CreateWorkspaceRequest.md)|  | 

### Return type

[**CreateWorkspaceResponse**](CreateWorkspaceResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | A successful response. |  -  |
**201** | Workspace created successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**403** | Forbidden - Insufficient permissions |  -  |
**404** | Not Found - Resource not found |  -  |
**409** | Conflict - Resource already exists |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_account**
> DeleteAccountResponse delete_account(id)

Delete account

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.delete_account_response import DeleteAccountResponse
from playbookmedia_backend_client_sdk.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://lead-scraping-microservice.vector.svc.cluster.local:9896
# See configuration.py for a list of all supported configuration parameters.
configuration = playbookmedia_backend_client_sdk.Configuration(
    host = "http://lead-scraping-microservice.vector.svc.cluster.local:9896"
)


# Enter a context with an instance of the API client
async with playbookmedia_backend_client_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = playbookmedia_backend_client_sdk.WorkspaceServiceApi(api_client)
    id = 'id_example' # str | 

    try:
        # Delete account
        api_response = await api_instance.delete_account(id)
        print("The response of WorkspaceServiceApi->delete_account:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling WorkspaceServiceApi->delete_account: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**|  | 

### Return type

[**DeleteAccountResponse**](DeleteAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Account deleted successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**403** | Forbidden - Insufficient permissions |  -  |
**404** | Not Found - Resource not found |  -  |
**409** | Conflict - Resource already exists |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_workspace1**
> DeleteWorkspaceResponse delete_workspace1(id)

Delete workspace

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.delete_workspace_response import DeleteWorkspaceResponse
from playbookmedia_backend_client_sdk.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://lead-scraping-microservice.vector.svc.cluster.local:9896
# See configuration.py for a list of all supported configuration parameters.
configuration = playbookmedia_backend_client_sdk.Configuration(
    host = "http://lead-scraping-microservice.vector.svc.cluster.local:9896"
)


# Enter a context with an instance of the API client
async with playbookmedia_backend_client_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = playbookmedia_backend_client_sdk.WorkspaceServiceApi(api_client)
    id = 'id_example' # str | 

    try:
        # Delete workspace
        api_response = await api_instance.delete_workspace1(id)
        print("The response of WorkspaceServiceApi->delete_workspace1:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling WorkspaceServiceApi->delete_workspace1: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**|  | 

### Return type

[**DeleteWorkspaceResponse**](DeleteWorkspaceResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Workspace deleted successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**403** | Forbidden - Insufficient permissions |  -  |
**404** | Not Found - Resource not found |  -  |
**409** | Conflict - Resource already exists |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_account**
> GetAccountResponse get_account(id)

Get account details

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.get_account_response import GetAccountResponse
from playbookmedia_backend_client_sdk.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://lead-scraping-microservice.vector.svc.cluster.local:9896
# See configuration.py for a list of all supported configuration parameters.
configuration = playbookmedia_backend_client_sdk.Configuration(
    host = "http://lead-scraping-microservice.vector.svc.cluster.local:9896"
)


# Enter a context with an instance of the API client
async with playbookmedia_backend_client_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = playbookmedia_backend_client_sdk.WorkspaceServiceApi(api_client)
    id = 'id_example' # str | 

    try:
        # Get account details
        api_response = await api_instance.get_account(id)
        print("The response of WorkspaceServiceApi->get_account:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling WorkspaceServiceApi->get_account: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**|  | 

### Return type

[**GetAccountResponse**](GetAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Account details retrieved successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**403** | Forbidden - Insufficient permissions |  -  |
**404** | Not Found - Resource not found |  -  |
**409** | Conflict - Resource already exists |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_workspace1**
> GetWorkspaceResponse1 get_workspace1(id)

Get workspace details

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.get_workspace_response1 import GetWorkspaceResponse1
from playbookmedia_backend_client_sdk.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://lead-scraping-microservice.vector.svc.cluster.local:9896
# See configuration.py for a list of all supported configuration parameters.
configuration = playbookmedia_backend_client_sdk.Configuration(
    host = "http://lead-scraping-microservice.vector.svc.cluster.local:9896"
)


# Enter a context with an instance of the API client
async with playbookmedia_backend_client_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = playbookmedia_backend_client_sdk.WorkspaceServiceApi(api_client)
    id = 'id_example' # str | 

    try:
        # Get workspace details
        api_response = await api_instance.get_workspace1(id)
        print("The response of WorkspaceServiceApi->get_workspace1:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling WorkspaceServiceApi->get_workspace1: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**|  | 

### Return type

[**GetWorkspaceResponse1**](GetWorkspaceResponse1.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Workspace retrieved successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**403** | Forbidden - Insufficient permissions |  -  |
**404** | Not Found - Resource not found |  -  |
**409** | Conflict - Resource already exists |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_workspace_analytics1**
> GetWorkspaceAnalyticsResponse1 get_workspace_analytics1(workspace_id, start_time=start_time, end_time=end_time)

Get workspace analytics

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.get_workspace_analytics_response1 import GetWorkspaceAnalyticsResponse1
from playbookmedia_backend_client_sdk.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://lead-scraping-microservice.vector.svc.cluster.local:9896
# See configuration.py for a list of all supported configuration parameters.
configuration = playbookmedia_backend_client_sdk.Configuration(
    host = "http://lead-scraping-microservice.vector.svc.cluster.local:9896"
)


# Enter a context with an instance of the API client
async with playbookmedia_backend_client_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = playbookmedia_backend_client_sdk.WorkspaceServiceApi(api_client)
    workspace_id = 'workspace_id_example' # str | 
    start_time = '2013-10-20T19:20:30+01:00' # datetime |  (optional)
    end_time = '2013-10-20T19:20:30+01:00' # datetime |  (optional)

    try:
        # Get workspace analytics
        api_response = await api_instance.get_workspace_analytics1(workspace_id, start_time=start_time, end_time=end_time)
        print("The response of WorkspaceServiceApi->get_workspace_analytics1:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling WorkspaceServiceApi->get_workspace_analytics1: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **workspace_id** | **str**|  | 
 **start_time** | **datetime**|  | [optional] 
 **end_time** | **datetime**|  | [optional] 

### Return type

[**GetWorkspaceAnalyticsResponse1**](GetWorkspaceAnalyticsResponse1.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Analytics data retrieved successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**403** | Forbidden - Insufficient permissions |  -  |
**404** | Not Found - Resource not found |  -  |
**409** | Conflict - Resource already exists |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_workspace_compliance_report**
> GetWorkspaceComplianceReportResponse get_workspace_compliance_report(workspace_id, compliance_type=compliance_type)

Get compliance report

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.get_workspace_compliance_report_response import GetWorkspaceComplianceReportResponse
from playbookmedia_backend_client_sdk.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://lead-scraping-microservice.vector.svc.cluster.local:9896
# See configuration.py for a list of all supported configuration parameters.
configuration = playbookmedia_backend_client_sdk.Configuration(
    host = "http://lead-scraping-microservice.vector.svc.cluster.local:9896"
)


# Enter a context with an instance of the API client
async with playbookmedia_backend_client_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = playbookmedia_backend_client_sdk.WorkspaceServiceApi(api_client)
    workspace_id = 'workspace_id_example' # str | 
    compliance_type = 'compliance_type_example' # str |  (optional)

    try:
        # Get compliance report
        api_response = await api_instance.get_workspace_compliance_report(workspace_id, compliance_type=compliance_type)
        print("The response of WorkspaceServiceApi->get_workspace_compliance_report:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling WorkspaceServiceApi->get_workspace_compliance_report: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **workspace_id** | **str**|  | 
 **compliance_type** | **str**|  | [optional] 

### Return type

[**GetWorkspaceComplianceReportResponse**](GetWorkspaceComplianceReportResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Compliance report retrieved successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**403** | Forbidden - Insufficient permissions |  -  |
**404** | Not Found - Resource not found |  -  |
**409** | Conflict - Resource already exists |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_workspace_storage_stats**
> GetWorkspaceStorageStatsResponse get_workspace_storage_stats(workspace_id)

Get storage statistics

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.get_workspace_storage_stats_response import GetWorkspaceStorageStatsResponse
from playbookmedia_backend_client_sdk.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://lead-scraping-microservice.vector.svc.cluster.local:9896
# See configuration.py for a list of all supported configuration parameters.
configuration = playbookmedia_backend_client_sdk.Configuration(
    host = "http://lead-scraping-microservice.vector.svc.cluster.local:9896"
)


# Enter a context with an instance of the API client
async with playbookmedia_backend_client_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = playbookmedia_backend_client_sdk.WorkspaceServiceApi(api_client)
    workspace_id = 'workspace_id_example' # str | 

    try:
        # Get storage statistics
        api_response = await api_instance.get_workspace_storage_stats(workspace_id)
        print("The response of WorkspaceServiceApi->get_workspace_storage_stats:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling WorkspaceServiceApi->get_workspace_storage_stats: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **workspace_id** | **str**|  | 

### Return type

[**GetWorkspaceStorageStatsResponse**](GetWorkspaceStorageStatsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Storage statistics retrieved successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**403** | Forbidden - Insufficient permissions |  -  |
**404** | Not Found - Resource not found |  -  |
**409** | Conflict - Resource already exists |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_workspace_sharings**
> ListWorkspaceSharingsResponse list_workspace_sharings(workspace_id, page_size=page_size, page_token=page_token)

List workspace sharings

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.list_workspace_sharings_response import ListWorkspaceSharingsResponse
from playbookmedia_backend_client_sdk.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://lead-scraping-microservice.vector.svc.cluster.local:9896
# See configuration.py for a list of all supported configuration parameters.
configuration = playbookmedia_backend_client_sdk.Configuration(
    host = "http://lead-scraping-microservice.vector.svc.cluster.local:9896"
)


# Enter a context with an instance of the API client
async with playbookmedia_backend_client_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = playbookmedia_backend_client_sdk.WorkspaceServiceApi(api_client)
    workspace_id = 'workspace_id_example' # str | 
    page_size = 56 # int |  (optional)
    page_token = 'page_token_example' # str |  (optional)

    try:
        # List workspace sharings
        api_response = await api_instance.list_workspace_sharings(workspace_id, page_size=page_size, page_token=page_token)
        print("The response of WorkspaceServiceApi->list_workspace_sharings:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling WorkspaceServiceApi->list_workspace_sharings: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **workspace_id** | **str**|  | 
 **page_size** | **int**|  | [optional] 
 **page_token** | **str**|  | [optional] 

### Return type

[**ListWorkspaceSharingsResponse**](ListWorkspaceSharingsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Workspace sharings retrieved successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**403** | Forbidden - Insufficient permissions |  -  |
**404** | Not Found - Resource not found |  -  |
**409** | Conflict - Resource already exists |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_workspaces1**
> ListWorkspacesResponse1 list_workspaces1(account_id=account_id, page_size=page_size, page_token=page_token, filter=filter)

List workspaces

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.list_workspaces_response1 import ListWorkspacesResponse1
from playbookmedia_backend_client_sdk.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://lead-scraping-microservice.vector.svc.cluster.local:9896
# See configuration.py for a list of all supported configuration parameters.
configuration = playbookmedia_backend_client_sdk.Configuration(
    host = "http://lead-scraping-microservice.vector.svc.cluster.local:9896"
)


# Enter a context with an instance of the API client
async with playbookmedia_backend_client_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = playbookmedia_backend_client_sdk.WorkspaceServiceApi(api_client)
    account_id = 'account_id_example' # str |  (optional)
    page_size = 56 # int |  (optional)
    page_token = 'page_token_example' # str |  (optional)
    filter = 'filter_example' # str |  (optional)

    try:
        # List workspaces
        api_response = await api_instance.list_workspaces1(account_id=account_id, page_size=page_size, page_token=page_token, filter=filter)
        print("The response of WorkspaceServiceApi->list_workspaces1:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling WorkspaceServiceApi->list_workspaces1: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account_id** | **str**|  | [optional] 
 **page_size** | **int**|  | [optional] 
 **page_token** | **str**|  | [optional] 
 **filter** | **str**|  | [optional] 

### Return type

[**ListWorkspacesResponse1**](ListWorkspacesResponse1.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Workspaces retrieved successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**403** | Forbidden - Insufficient permissions |  -  |
**404** | Not Found - Resource not found |  -  |
**409** | Conflict - Resource already exists |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **remove_workspace_sharing**
> RemoveWorkspaceSharingResponse remove_workspace_sharing(sharing_id)

Remove workspace sharing

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.remove_workspace_sharing_response import RemoveWorkspaceSharingResponse
from playbookmedia_backend_client_sdk.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://lead-scraping-microservice.vector.svc.cluster.local:9896
# See configuration.py for a list of all supported configuration parameters.
configuration = playbookmedia_backend_client_sdk.Configuration(
    host = "http://lead-scraping-microservice.vector.svc.cluster.local:9896"
)


# Enter a context with an instance of the API client
async with playbookmedia_backend_client_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = playbookmedia_backend_client_sdk.WorkspaceServiceApi(api_client)
    sharing_id = 'sharing_id_example' # str | 

    try:
        # Remove workspace sharing
        api_response = await api_instance.remove_workspace_sharing(sharing_id)
        print("The response of WorkspaceServiceApi->remove_workspace_sharing:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling WorkspaceServiceApi->remove_workspace_sharing: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sharing_id** | **str**|  | 

### Return type

[**RemoveWorkspaceSharingResponse**](RemoveWorkspaceSharingResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Workspace sharing removed successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**403** | Forbidden - Insufficient permissions |  -  |
**404** | Not Found - Resource not found |  -  |
**409** | Conflict - Resource already exists |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **share_workspace**
> ShareWorkspaceResponse share_workspace(workspace_id, share_workspace_body)

Share workspace

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.share_workspace_body import ShareWorkspaceBody
from playbookmedia_backend_client_sdk.models.share_workspace_response import ShareWorkspaceResponse
from playbookmedia_backend_client_sdk.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://lead-scraping-microservice.vector.svc.cluster.local:9896
# See configuration.py for a list of all supported configuration parameters.
configuration = playbookmedia_backend_client_sdk.Configuration(
    host = "http://lead-scraping-microservice.vector.svc.cluster.local:9896"
)


# Enter a context with an instance of the API client
async with playbookmedia_backend_client_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = playbookmedia_backend_client_sdk.WorkspaceServiceApi(api_client)
    workspace_id = 'workspace_id_example' # str | 
    share_workspace_body = playbookmedia_backend_client_sdk.ShareWorkspaceBody() # ShareWorkspaceBody | 

    try:
        # Share workspace
        api_response = await api_instance.share_workspace(workspace_id, share_workspace_body)
        print("The response of WorkspaceServiceApi->share_workspace:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling WorkspaceServiceApi->share_workspace: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **workspace_id** | **str**|  | 
 **share_workspace_body** | [**ShareWorkspaceBody**](ShareWorkspaceBody.md)|  | 

### Return type

[**ShareWorkspaceResponse**](ShareWorkspaceResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Workspace shared successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**403** | Forbidden - Insufficient permissions |  -  |
**404** | Not Found - Resource not found |  -  |
**409** | Conflict - Resource already exists |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_account**
> UpdateAccountResponse update_account(update_account_request)

Update account details

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.update_account_request import UpdateAccountRequest
from playbookmedia_backend_client_sdk.models.update_account_response import UpdateAccountResponse
from playbookmedia_backend_client_sdk.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://lead-scraping-microservice.vector.svc.cluster.local:9896
# See configuration.py for a list of all supported configuration parameters.
configuration = playbookmedia_backend_client_sdk.Configuration(
    host = "http://lead-scraping-microservice.vector.svc.cluster.local:9896"
)


# Enter a context with an instance of the API client
async with playbookmedia_backend_client_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = playbookmedia_backend_client_sdk.WorkspaceServiceApi(api_client)
    update_account_request = playbookmedia_backend_client_sdk.UpdateAccountRequest() # UpdateAccountRequest | 

    try:
        # Update account details
        api_response = await api_instance.update_account(update_account_request)
        print("The response of WorkspaceServiceApi->update_account:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling WorkspaceServiceApi->update_account: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **update_account_request** | [**UpdateAccountRequest**](UpdateAccountRequest.md)|  | 

### Return type

[**UpdateAccountResponse**](UpdateAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Account updated successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**403** | Forbidden - Insufficient permissions |  -  |
**404** | Not Found - Resource not found |  -  |
**409** | Conflict - Resource already exists |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_workspace1**
> UpdateWorkspaceResponse update_workspace1(update_workspace_request)

Update workspace

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.update_workspace_request import UpdateWorkspaceRequest
from playbookmedia_backend_client_sdk.models.update_workspace_response import UpdateWorkspaceResponse
from playbookmedia_backend_client_sdk.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://lead-scraping-microservice.vector.svc.cluster.local:9896
# See configuration.py for a list of all supported configuration parameters.
configuration = playbookmedia_backend_client_sdk.Configuration(
    host = "http://lead-scraping-microservice.vector.svc.cluster.local:9896"
)


# Enter a context with an instance of the API client
async with playbookmedia_backend_client_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = playbookmedia_backend_client_sdk.WorkspaceServiceApi(api_client)
    update_workspace_request = playbookmedia_backend_client_sdk.UpdateWorkspaceRequest() # UpdateWorkspaceRequest | 

    try:
        # Update workspace
        api_response = await api_instance.update_workspace1(update_workspace_request)
        print("The response of WorkspaceServiceApi->update_workspace1:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling WorkspaceServiceApi->update_workspace1: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **update_workspace_request** | [**UpdateWorkspaceRequest**](UpdateWorkspaceRequest.md)|  | 

### Return type

[**UpdateWorkspaceResponse**](UpdateWorkspaceResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Workspace updated successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**403** | Forbidden - Insufficient permissions |  -  |
**404** | Not Found - Resource not found |  -  |
**409** | Conflict - Resource already exists |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_workspace_sharing**
> UpdateWorkspaceSharingResponse update_workspace_sharing(update_workspace_sharing_request)

Update workspace sharing

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.update_workspace_sharing_request import UpdateWorkspaceSharingRequest
from playbookmedia_backend_client_sdk.models.update_workspace_sharing_response import UpdateWorkspaceSharingResponse
from playbookmedia_backend_client_sdk.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://lead-scraping-microservice.vector.svc.cluster.local:9896
# See configuration.py for a list of all supported configuration parameters.
configuration = playbookmedia_backend_client_sdk.Configuration(
    host = "http://lead-scraping-microservice.vector.svc.cluster.local:9896"
)


# Enter a context with an instance of the API client
async with playbookmedia_backend_client_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = playbookmedia_backend_client_sdk.WorkspaceServiceApi(api_client)
    update_workspace_sharing_request = playbookmedia_backend_client_sdk.UpdateWorkspaceSharingRequest() # UpdateWorkspaceSharingRequest | 

    try:
        # Update workspace sharing
        api_response = await api_instance.update_workspace_sharing(update_workspace_sharing_request)
        print("The response of WorkspaceServiceApi->update_workspace_sharing:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling WorkspaceServiceApi->update_workspace_sharing: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **update_workspace_sharing_request** | [**UpdateWorkspaceSharingRequest**](UpdateWorkspaceSharingRequest.md)|  | 

### Return type

[**UpdateWorkspaceSharingResponse**](UpdateWorkspaceSharingResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Workspace sharing updated successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**403** | Forbidden - Insufficient permissions |  -  |
**404** | Not Found - Resource not found |  -  |
**409** | Conflict - Resource already exists |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

