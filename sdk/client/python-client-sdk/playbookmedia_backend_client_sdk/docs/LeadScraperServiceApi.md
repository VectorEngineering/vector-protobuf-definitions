# playbookmedia_backend_client_sdk.LeadScraperServiceApi

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_account**](LeadScraperServiceApi.md#create_account) | **POST** /lead-scraper-microservice/api/v1/accounts | Create a new account
[**create_scraping_job**](LeadScraperServiceApi.md#create_scraping_job) | **POST** /lead-scraper-microservice/api/v1/jobs | Create a new job scraping task
[**create_workspace**](LeadScraperServiceApi.md#create_workspace) | **POST** /lead-scraper-microservice/api/v1/workspaces | Create a new workspace
[**delete_account**](LeadScraperServiceApi.md#delete_account) | **DELETE** /lead-scraper-microservice/api/v1/accounts/{id} | Delete account
[**delete_scraping_job**](LeadScraperServiceApi.md#delete_scraping_job) | **DELETE** /lead-scraper-microservice/api/v1/jobs/{jobId} | Delete a specific job
[**delete_workspace**](LeadScraperServiceApi.md#delete_workspace) | **DELETE** /lead-scraper-microservice/api/v1/workspace/{id} | Delete a workspace
[**download_scraping_results**](LeadScraperServiceApi.md#download_scraping_results) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId}/download | Download job results as CSV
[**get_account**](LeadScraperServiceApi.md#get_account) | **GET** /lead-scraper-microservice/api/v1/accounts/{id} | Get account details
[**get_account_usage**](LeadScraperServiceApi.md#get_account_usage) | **GET** /lead-scraper-microservice/api/v1/accounts/{id}/usage | Get account usage
[**get_scraping_job**](LeadScraperServiceApi.md#get_scraping_job) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId} | Get a specific job
[**get_workflow**](LeadScraperServiceApi.md#get_workflow) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id} | Get workflow details
[**get_workspace**](LeadScraperServiceApi.md#get_workspace) | **GET** /lead-scraper-microservice/api/v1/workspace/{id} | Get workspace details
[**get_workspace_analytics**](LeadScraperServiceApi.md#get_workspace_analytics) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/analytics | Get workspace analytics
[**lead_scraper_service_create_workflow**](LeadScraperServiceApi.md#lead_scraper_service_create_workflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows | Create a new workflow
[**list_accounts**](LeadScraperServiceApi.md#list_accounts) | **GET** /lead-scraper-microservice/api/v1/accounts | List all accounts
[**list_scraping_jobs**](LeadScraperServiceApi.md#list_scraping_jobs) | **GET** /lead-scraper-microservice/api/v1/jobs | Get all jobs
[**list_workflows**](LeadScraperServiceApi.md#list_workflows) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows | List workflows
[**list_workspaces**](LeadScraperServiceApi.md#list_workspaces) | **GET** /lead-scraper-microservice/api/v1/workspaces | List workspaces
[**pause_workflow**](LeadScraperServiceApi.md#pause_workflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id}/pause | Pause workflow execution
[**trigger_workflow**](LeadScraperServiceApi.md#trigger_workflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id}/trigger | Trigger workflow execution
[**update_account**](LeadScraperServiceApi.md#update_account) | **PUT** /lead-scraper-microservice/api/v1/accounts | Update account details
[**update_account_settings**](LeadScraperServiceApi.md#update_account_settings) | **PUT** /lead-scraper-microservice/api/v1/accounts/settings | Update account settings
[**update_workflow**](LeadScraperServiceApi.md#update_workflow) | **PUT** /lead-scraper-microservice/api/v1/workspaces/workflow | Update workflow details
[**update_workspace**](LeadScraperServiceApi.md#update_workspace) | **PUT** /lead-scraper-microservice/api/v1/workspace | Update workspace details


# **create_account**
> CreateAccountResponse create_account(create_account_request)

Create a new account

Creates a new user account in the workspace service

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
    api_instance = playbookmedia_backend_client_sdk.LeadScraperServiceApi(api_client)
    create_account_request = playbookmedia_backend_client_sdk.CreateAccountRequest() # CreateAccountRequest | 

    try:
        # Create a new account
        api_response = await api_instance.create_account(create_account_request)
        print("The response of LeadScraperServiceApi->create_account:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->create_account: %s\n" % e)
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
**402** | Payment Required - Payment is necessary to proceed |  -  |
**403** | Forbidden - Access denied |  -  |
**404** | Not Found - Resource not found |  -  |
**405** | Method Not Allowed - HTTP method not supported |  -  |
**409** | Conflict - Resource already exists |  -  |
**410** | Gone - Resource is no longer available |  -  |
**412** | Precondition Failed - Preconditions in headers did not match |  -  |
**422** | Unprocessable Entity - Semantic errors in the request |  -  |
**425** | Too Early - Request is being replayed |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**501** | Not Implemented - Functionality not supported |  -  |
**502** | Bad Gateway - Invalid response from upstream server |  -  |
**503** | Service Unavailable - Try again later |  -  |
**504** | Gateway Timeout - Upstream server timed out |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_scraping_job**
> CreateScrapingJobResponse create_scraping_job(create_scraping_job_request)

Create a new job scraping task

This endpoint creates a new Google Maps scraping job

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.create_scraping_job_request import CreateScrapingJobRequest
from playbookmedia_backend_client_sdk.models.create_scraping_job_response import CreateScrapingJobResponse
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
    api_instance = playbookmedia_backend_client_sdk.LeadScraperServiceApi(api_client)
    create_scraping_job_request = playbookmedia_backend_client_sdk.CreateScrapingJobRequest() # CreateScrapingJobRequest | 

    try:
        # Create a new job scraping task
        api_response = await api_instance.create_scraping_job(create_scraping_job_request)
        print("The response of LeadScraperServiceApi->create_scraping_job:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->create_scraping_job: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **create_scraping_job_request** | [**CreateScrapingJobRequest**](CreateScrapingJobRequest.md)|  | 

### Return type

[**CreateScrapingJobResponse**](CreateScrapingJobResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | A successful response. |  -  |
**201** | Job created successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**402** | Payment Required - Payment is necessary to proceed |  -  |
**403** | Forbidden - Access denied |  -  |
**404** | Not Found - Resource not found |  -  |
**405** | Method Not Allowed - HTTP method not supported |  -  |
**409** | Conflict - Resource already exists |  -  |
**410** | Gone - Resource is no longer available |  -  |
**412** | Precondition Failed - Preconditions in headers did not match |  -  |
**422** | Unprocessable Entity - Semantic errors in the request |  -  |
**425** | Too Early - Request is being replayed |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**501** | Not Implemented - Functionality not supported |  -  |
**502** | Bad Gateway - Invalid response from upstream server |  -  |
**503** | Service Unavailable - Try again later |  -  |
**504** | Gateway Timeout - Upstream server timed out |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_workspace**
> CreateWorkspaceResponse create_workspace(create_workspace_request)

Create a new workspace

Creates a new workspace for a given account

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
    api_instance = playbookmedia_backend_client_sdk.LeadScraperServiceApi(api_client)
    create_workspace_request = playbookmedia_backend_client_sdk.CreateWorkspaceRequest() # CreateWorkspaceRequest | 

    try:
        # Create a new workspace
        api_response = await api_instance.create_workspace(create_workspace_request)
        print("The response of LeadScraperServiceApi->create_workspace:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->create_workspace: %s\n" % e)
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
**402** | Payment Required - Payment is necessary to proceed |  -  |
**403** | Forbidden - Access denied |  -  |
**404** | Not Found - Resource not found |  -  |
**405** | Method Not Allowed - HTTP method not supported |  -  |
**409** | Conflict - Resource already exists |  -  |
**410** | Gone - Resource is no longer available |  -  |
**412** | Precondition Failed - Preconditions in headers did not match |  -  |
**422** | Unprocessable Entity - Semantic errors in the request |  -  |
**425** | Too Early - Request is being replayed |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**501** | Not Implemented - Functionality not supported |  -  |
**502** | Bad Gateway - Invalid response from upstream server |  -  |
**503** | Service Unavailable - Try again later |  -  |
**504** | Gateway Timeout - Upstream server timed out |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_account**
> DeleteAccountResponse delete_account(id)

Delete account

Permanently deletes an account and associated resources

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
    api_instance = playbookmedia_backend_client_sdk.LeadScraperServiceApi(api_client)
    id = 'id_example' # str | 

    try:
        # Delete account
        api_response = await api_instance.delete_account(id)
        print("The response of LeadScraperServiceApi->delete_account:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->delete_account: %s\n" % e)
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
**402** | Payment Required - Payment is necessary to proceed |  -  |
**403** | Forbidden - Access denied |  -  |
**404** | Not Found - Resource not found |  -  |
**405** | Method Not Allowed - HTTP method not supported |  -  |
**409** | Conflict - Resource already exists |  -  |
**410** | Gone - Resource is no longer available |  -  |
**412** | Precondition Failed - Preconditions in headers did not match |  -  |
**422** | Unprocessable Entity - Semantic errors in the request |  -  |
**425** | Too Early - Request is being replayed |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**501** | Not Implemented - Functionality not supported |  -  |
**502** | Bad Gateway - Invalid response from upstream server |  -  |
**503** | Service Unavailable - Try again later |  -  |
**504** | Gateway Timeout - Upstream server timed out |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_scraping_job**
> DeleteScrapingJobResponse delete_scraping_job(job_id, user_id, org_id, tenant_id)

Delete a specific job

This endpoint deletes a specific Google Maps scraping job

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.delete_scraping_job_response import DeleteScrapingJobResponse
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
    api_instance = playbookmedia_backend_client_sdk.LeadScraperServiceApi(api_client)
    job_id = 'job_id_example' # str | 
    user_id = 'user_id_example' # str | 
    org_id = 'org_id_example' # str | 
    tenant_id = 'tenant_id_example' # str | 

    try:
        # Delete a specific job
        api_response = await api_instance.delete_scraping_job(job_id, user_id, org_id, tenant_id)
        print("The response of LeadScraperServiceApi->delete_scraping_job:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->delete_scraping_job: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **job_id** | **str**|  | 
 **user_id** | **str**|  | 
 **org_id** | **str**|  | 
 **tenant_id** | **str**|  | 

### Return type

[**DeleteScrapingJobResponse**](DeleteScrapingJobResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Job deleted successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**402** | Payment Required - Payment is necessary to proceed |  -  |
**403** | Forbidden - Access denied |  -  |
**404** | Not Found - Resource not found |  -  |
**405** | Method Not Allowed - HTTP method not supported |  -  |
**409** | Conflict - Resource already exists |  -  |
**410** | Gone - Resource is no longer available |  -  |
**412** | Precondition Failed - Preconditions in headers did not match |  -  |
**422** | Unprocessable Entity - Semantic errors in the request |  -  |
**425** | Too Early - Request is being replayed |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**501** | Not Implemented - Functionality not supported |  -  |
**502** | Bad Gateway - Invalid response from upstream server |  -  |
**503** | Service Unavailable - Try again later |  -  |
**504** | Gateway Timeout - Upstream server timed out |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_workspace**
> DeleteWorkspaceResponse delete_workspace(id)

Delete a workspace

Deletes a specific workspace

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
    api_instance = playbookmedia_backend_client_sdk.LeadScraperServiceApi(api_client)
    id = 'id_example' # str | 

    try:
        # Delete a workspace
        api_response = await api_instance.delete_workspace(id)
        print("The response of LeadScraperServiceApi->delete_workspace:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->delete_workspace: %s\n" % e)
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
**402** | Payment Required - Payment is necessary to proceed |  -  |
**403** | Forbidden - Access denied |  -  |
**404** | Not Found - Resource not found |  -  |
**405** | Method Not Allowed - HTTP method not supported |  -  |
**409** | Conflict - Resource already exists |  -  |
**410** | Gone - Resource is no longer available |  -  |
**412** | Precondition Failed - Preconditions in headers did not match |  -  |
**422** | Unprocessable Entity - Semantic errors in the request |  -  |
**425** | Too Early - Request is being replayed |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**501** | Not Implemented - Functionality not supported |  -  |
**502** | Bad Gateway - Invalid response from upstream server |  -  |
**503** | Service Unavailable - Try again later |  -  |
**504** | Gateway Timeout - Upstream server timed out |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **download_scraping_results**
> DownloadScrapingResultsResponse download_scraping_results(job_id, user_id, org_id, tenant_id)

Download job results as CSV

This endpoint downloads the results of a Google Maps scraping job as CSV

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.download_scraping_results_response import DownloadScrapingResultsResponse
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
    api_instance = playbookmedia_backend_client_sdk.LeadScraperServiceApi(api_client)
    job_id = 'job_id_example' # str | 
    user_id = 'user_id_example' # str | 
    org_id = 'org_id_example' # str | 
    tenant_id = 'tenant_id_example' # str | 

    try:
        # Download job results as CSV
        api_response = await api_instance.download_scraping_results(job_id, user_id, org_id, tenant_id)
        print("The response of LeadScraperServiceApi->download_scraping_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->download_scraping_results: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **job_id** | **str**|  | 
 **user_id** | **str**|  | 
 **org_id** | **str**|  | 
 **tenant_id** | **str**|  | 

### Return type

[**DownloadScrapingResultsResponse**](DownloadScrapingResultsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**402** | Payment Required - Payment is necessary to proceed |  -  |
**403** | Forbidden - Access denied |  -  |
**404** | Not Found - Resource not found |  -  |
**405** | Method Not Allowed - HTTP method not supported |  -  |
**409** | Conflict - Resource already exists |  -  |
**410** | Gone - Resource is no longer available |  -  |
**412** | Precondition Failed - Preconditions in headers did not match |  -  |
**422** | Unprocessable Entity - Semantic errors in the request |  -  |
**425** | Too Early - Request is being replayed |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**501** | Not Implemented - Functionality not supported |  -  |
**502** | Bad Gateway - Invalid response from upstream server |  -  |
**503** | Service Unavailable - Try again later |  -  |
**504** | Gateway Timeout - Upstream server timed out |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_account**
> GetAccountResponse get_account(id)

Get account details

Retrieves details of a specific account

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
    api_instance = playbookmedia_backend_client_sdk.LeadScraperServiceApi(api_client)
    id = 'id_example' # str | 

    try:
        # Get account details
        api_response = await api_instance.get_account(id)
        print("The response of LeadScraperServiceApi->get_account:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->get_account: %s\n" % e)
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
**402** | Payment Required - Payment is necessary to proceed |  -  |
**403** | Forbidden - Access denied |  -  |
**404** | Not Found - Resource not found |  -  |
**405** | Method Not Allowed - HTTP method not supported |  -  |
**409** | Conflict - Resource already exists |  -  |
**410** | Gone - Resource is no longer available |  -  |
**412** | Precondition Failed - Preconditions in headers did not match |  -  |
**422** | Unprocessable Entity - Semantic errors in the request |  -  |
**425** | Too Early - Request is being replayed |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**501** | Not Implemented - Functionality not supported |  -  |
**502** | Bad Gateway - Invalid response from upstream server |  -  |
**503** | Service Unavailable - Try again later |  -  |
**504** | Gateway Timeout - Upstream server timed out |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_account_usage**
> GetAccountUsageResponse get_account_usage(id)

Get account usage

Retrieves usage details for a given account

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.get_account_usage_response import GetAccountUsageResponse
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
    api_instance = playbookmedia_backend_client_sdk.LeadScraperServiceApi(api_client)
    id = 'id_example' # str | 

    try:
        # Get account usage
        api_response = await api_instance.get_account_usage(id)
        print("The response of LeadScraperServiceApi->get_account_usage:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->get_account_usage: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**|  | 

### Return type

[**GetAccountUsageResponse**](GetAccountUsageResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Usage details retrieved successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**402** | Payment Required - Payment is necessary to proceed |  -  |
**403** | Forbidden - Access denied |  -  |
**404** | Not Found - Resource not found |  -  |
**405** | Method Not Allowed - HTTP method not supported |  -  |
**409** | Conflict - Resource already exists |  -  |
**410** | Gone - Resource is no longer available |  -  |
**412** | Precondition Failed - Preconditions in headers did not match |  -  |
**422** | Unprocessable Entity - Semantic errors in the request |  -  |
**425** | Too Early - Request is being replayed |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**501** | Not Implemented - Functionality not supported |  -  |
**502** | Bad Gateway - Invalid response from upstream server |  -  |
**503** | Service Unavailable - Try again later |  -  |
**504** | Gateway Timeout - Upstream server timed out |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_scraping_job**
> GetScrapingJobResponse get_scraping_job(job_id, user_id, org_id, tenant_id)

Get a specific job

This endpoint retrieves a specific Google Maps scraping job

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.get_scraping_job_response import GetScrapingJobResponse
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
    api_instance = playbookmedia_backend_client_sdk.LeadScraperServiceApi(api_client)
    job_id = 'job_id_example' # str | 
    user_id = 'user_id_example' # str | 
    org_id = 'org_id_example' # str | 
    tenant_id = 'tenant_id_example' # str | 

    try:
        # Get a specific job
        api_response = await api_instance.get_scraping_job(job_id, user_id, org_id, tenant_id)
        print("The response of LeadScraperServiceApi->get_scraping_job:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->get_scraping_job: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **job_id** | **str**|  | 
 **user_id** | **str**|  | 
 **org_id** | **str**|  | 
 **tenant_id** | **str**|  | 

### Return type

[**GetScrapingJobResponse**](GetScrapingJobResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**402** | Payment Required - Payment is necessary to proceed |  -  |
**403** | Forbidden - Access denied |  -  |
**404** | Not Found - Resource not found |  -  |
**405** | Method Not Allowed - HTTP method not supported |  -  |
**409** | Conflict - Resource already exists |  -  |
**410** | Gone - Resource is no longer available |  -  |
**412** | Precondition Failed - Preconditions in headers did not match |  -  |
**422** | Unprocessable Entity - Semantic errors in the request |  -  |
**425** | Too Early - Request is being replayed |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**501** | Not Implemented - Functionality not supported |  -  |
**502** | Bad Gateway - Invalid response from upstream server |  -  |
**503** | Service Unavailable - Try again later |  -  |
**504** | Gateway Timeout - Upstream server timed out |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_workflow**
> ScrapingWorkflow get_workflow(workspace_id, id)

Get workflow details

Retrieves details of a specific workflow

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.scraping_workflow import ScrapingWorkflow
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
    api_instance = playbookmedia_backend_client_sdk.LeadScraperServiceApi(api_client)
    workspace_id = 'workspace_id_example' # str | 
    id = 'id_example' # str | 

    try:
        # Get workflow details
        api_response = await api_instance.get_workflow(workspace_id, id)
        print("The response of LeadScraperServiceApi->get_workflow:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->get_workflow: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **workspace_id** | **str**|  | 
 **id** | **str**|  | 

### Return type

[**ScrapingWorkflow**](ScrapingWorkflow.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Workflow retrieved successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**402** | Payment Required - Payment is necessary to proceed |  -  |
**403** | Forbidden - Access denied |  -  |
**404** | Not Found - Resource not found |  -  |
**405** | Method Not Allowed - HTTP method not supported |  -  |
**409** | Conflict - Resource already exists |  -  |
**410** | Gone - Resource is no longer available |  -  |
**412** | Precondition Failed - Preconditions in headers did not match |  -  |
**422** | Unprocessable Entity - Semantic errors in the request |  -  |
**425** | Too Early - Request is being replayed |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**501** | Not Implemented - Functionality not supported |  -  |
**502** | Bad Gateway - Invalid response from upstream server |  -  |
**503** | Service Unavailable - Try again later |  -  |
**504** | Gateway Timeout - Upstream server timed out |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_workspace**
> Workspace get_workspace(id)

Get workspace details

Retrieves details of a specific workspace

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.workspace import Workspace
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
    api_instance = playbookmedia_backend_client_sdk.LeadScraperServiceApi(api_client)
    id = 'id_example' # str | 

    try:
        # Get workspace details
        api_response = await api_instance.get_workspace(id)
        print("The response of LeadScraperServiceApi->get_workspace:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->get_workspace: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**|  | 

### Return type

[**Workspace**](Workspace.md)

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
**402** | Payment Required - Payment is necessary to proceed |  -  |
**403** | Forbidden - Access denied |  -  |
**404** | Not Found - Resource not found |  -  |
**405** | Method Not Allowed - HTTP method not supported |  -  |
**409** | Conflict - Resource already exists |  -  |
**410** | Gone - Resource is no longer available |  -  |
**412** | Precondition Failed - Preconditions in headers did not match |  -  |
**422** | Unprocessable Entity - Semantic errors in the request |  -  |
**425** | Too Early - Request is being replayed |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**501** | Not Implemented - Functionality not supported |  -  |
**502** | Bad Gateway - Invalid response from upstream server |  -  |
**503** | Service Unavailable - Try again later |  -  |
**504** | Gateway Timeout - Upstream server timed out |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_workspace_analytics**
> GetWorkspaceAnalyticsResponse get_workspace_analytics(workspace_id, start_time=start_time, end_time=end_time)

Get workspace analytics

Retrieves analytics data for a specific workspace

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.get_workspace_analytics_response import GetWorkspaceAnalyticsResponse
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
    api_instance = playbookmedia_backend_client_sdk.LeadScraperServiceApi(api_client)
    workspace_id = 'workspace_id_example' # str | 
    start_time = '2013-10-20T19:20:30+01:00' # datetime |  (optional)
    end_time = '2013-10-20T19:20:30+01:00' # datetime |  (optional)

    try:
        # Get workspace analytics
        api_response = await api_instance.get_workspace_analytics(workspace_id, start_time=start_time, end_time=end_time)
        print("The response of LeadScraperServiceApi->get_workspace_analytics:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->get_workspace_analytics: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **workspace_id** | **str**|  | 
 **start_time** | **datetime**|  | [optional] 
 **end_time** | **datetime**|  | [optional] 

### Return type

[**GetWorkspaceAnalyticsResponse**](GetWorkspaceAnalyticsResponse.md)

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
**402** | Payment Required - Payment is necessary to proceed |  -  |
**403** | Forbidden - Access denied |  -  |
**404** | Not Found - Resource not found |  -  |
**405** | Method Not Allowed - HTTP method not supported |  -  |
**409** | Conflict - Resource already exists |  -  |
**410** | Gone - Resource is no longer available |  -  |
**412** | Precondition Failed - Preconditions in headers did not match |  -  |
**422** | Unprocessable Entity - Semantic errors in the request |  -  |
**425** | Too Early - Request is being replayed |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**501** | Not Implemented - Functionality not supported |  -  |
**502** | Bad Gateway - Invalid response from upstream server |  -  |
**503** | Service Unavailable - Try again later |  -  |
**504** | Gateway Timeout - Upstream server timed out |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **lead_scraper_service_create_workflow**
> CreateWorkflowResponse lead_scraper_service_create_workflow(workspace_id, create_workflow_body)

Create a new workflow

This endpoint creates a new workflow configuration for a specific workspace  Required parameters: - workspace_id: Parent workspace ID - name: Human-readable workflow name - description: Optional workflow description - parameters: Job parameters for the workflow  Example: ```json

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.create_workflow_body import CreateWorkflowBody
from playbookmedia_backend_client_sdk.models.create_workflow_response import CreateWorkflowResponse
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
    api_instance = playbookmedia_backend_client_sdk.LeadScraperServiceApi(api_client)
    workspace_id = 'workspace_id_example' # str | 
    create_workflow_body = playbookmedia_backend_client_sdk.CreateWorkflowBody() # CreateWorkflowBody | 

    try:
        # Create a new workflow
        api_response = await api_instance.lead_scraper_service_create_workflow(workspace_id, create_workflow_body)
        print("The response of LeadScraperServiceApi->lead_scraper_service_create_workflow:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->lead_scraper_service_create_workflow: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **workspace_id** | **str**|  | 
 **create_workflow_body** | [**CreateWorkflowBody**](CreateWorkflowBody.md)|  | 

### Return type

[**CreateWorkflowResponse**](CreateWorkflowResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | A successful response. |  -  |
**201** | Workflow created successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**402** | Payment Required - Payment is necessary to proceed |  -  |
**403** | Forbidden - Access denied |  -  |
**404** | Not Found - Resource not found |  -  |
**405** | Method Not Allowed - HTTP method not supported |  -  |
**409** | Conflict - Resource already exists |  -  |
**410** | Gone - Resource is no longer available |  -  |
**412** | Precondition Failed - Preconditions in headers did not match |  -  |
**422** | Unprocessable Entity - Semantic errors in the request |  -  |
**425** | Too Early - Request is being replayed |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**501** | Not Implemented - Functionality not supported |  -  |
**502** | Bad Gateway - Invalid response from upstream server |  -  |
**503** | Service Unavailable - Try again later |  -  |
**504** | Gateway Timeout - Upstream server timed out |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_accounts**
> ListAccountsResponse list_accounts(page_size=page_size, page_number=page_number, filter=filter)

List all accounts

Retrieves a list of accounts

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.list_accounts_response import ListAccountsResponse
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
    api_instance = playbookmedia_backend_client_sdk.LeadScraperServiceApi(api_client)
    page_size = 56 # int |  (optional)
    page_number = 56 # int |  (optional)
    filter = 'filter_example' # str |  (optional)

    try:
        # List all accounts
        api_response = await api_instance.list_accounts(page_size=page_size, page_number=page_number, filter=filter)
        print("The response of LeadScraperServiceApi->list_accounts:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->list_accounts: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **page_size** | **int**|  | [optional] 
 **page_number** | **int**|  | [optional] 
 **filter** | **str**|  | [optional] 

### Return type

[**ListAccountsResponse**](ListAccountsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Accounts retrieved successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**402** | Payment Required - Payment is necessary to proceed |  -  |
**403** | Forbidden - Access denied |  -  |
**404** | Not Found - Resource not found |  -  |
**405** | Method Not Allowed - HTTP method not supported |  -  |
**409** | Conflict - Resource already exists |  -  |
**410** | Gone - Resource is no longer available |  -  |
**412** | Precondition Failed - Preconditions in headers did not match |  -  |
**422** | Unprocessable Entity - Semantic errors in the request |  -  |
**425** | Too Early - Request is being replayed |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**501** | Not Implemented - Functionality not supported |  -  |
**502** | Bad Gateway - Invalid response from upstream server |  -  |
**503** | Service Unavailable - Try again later |  -  |
**504** | Gateway Timeout - Upstream server timed out |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_scraping_jobs**
> ListScrapingJobsResponse list_scraping_jobs(user_id, org_id, tenant_id)

Get all jobs

This endpoint retrieves all Google Maps scraping jobs

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.list_scraping_jobs_response import ListScrapingJobsResponse
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
    api_instance = playbookmedia_backend_client_sdk.LeadScraperServiceApi(api_client)
    user_id = 'user_id_example' # str | 
    org_id = 'org_id_example' # str | 
    tenant_id = 'tenant_id_example' # str | 

    try:
        # Get all jobs
        api_response = await api_instance.list_scraping_jobs(user_id, org_id, tenant_id)
        print("The response of LeadScraperServiceApi->list_scraping_jobs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->list_scraping_jobs: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **user_id** | **str**|  | 
 **org_id** | **str**|  | 
 **tenant_id** | **str**|  | 

### Return type

[**ListScrapingJobsResponse**](ListScrapingJobsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful response |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**402** | Payment Required - Payment is necessary to proceed |  -  |
**403** | Forbidden - Access denied |  -  |
**404** | Not Found - Resource not found |  -  |
**405** | Method Not Allowed - HTTP method not supported |  -  |
**409** | Conflict - Resource already exists |  -  |
**410** | Gone - Resource is no longer available |  -  |
**412** | Precondition Failed - Preconditions in headers did not match |  -  |
**422** | Unprocessable Entity - Semantic errors in the request |  -  |
**425** | Too Early - Request is being replayed |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**501** | Not Implemented - Functionality not supported |  -  |
**502** | Bad Gateway - Invalid response from upstream server |  -  |
**503** | Service Unavailable - Try again later |  -  |
**504** | Gateway Timeout - Upstream server timed out |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_workflows**
> ListWorkflowsResponse list_workflows(workspace_id, page_size=page_size, page_number=page_number, filter=filter)

List workflows

Retrieves a list of workflows for a given workspace

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.list_workflows_response import ListWorkflowsResponse
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
    api_instance = playbookmedia_backend_client_sdk.LeadScraperServiceApi(api_client)
    workspace_id = 'workspace_id_example' # str | 
    page_size = 56 # int |  (optional)
    page_number = 56 # int |  (optional)
    filter = 'filter_example' # str |  (optional)

    try:
        # List workflows
        api_response = await api_instance.list_workflows(workspace_id, page_size=page_size, page_number=page_number, filter=filter)
        print("The response of LeadScraperServiceApi->list_workflows:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->list_workflows: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **workspace_id** | **str**|  | 
 **page_size** | **int**|  | [optional] 
 **page_number** | **int**|  | [optional] 
 **filter** | **str**|  | [optional] 

### Return type

[**ListWorkflowsResponse**](ListWorkflowsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Workflows retrieved successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**402** | Payment Required - Payment is necessary to proceed |  -  |
**403** | Forbidden - Access denied |  -  |
**404** | Not Found - Resource not found |  -  |
**405** | Method Not Allowed - HTTP method not supported |  -  |
**409** | Conflict - Resource already exists |  -  |
**410** | Gone - Resource is no longer available |  -  |
**412** | Precondition Failed - Preconditions in headers did not match |  -  |
**422** | Unprocessable Entity - Semantic errors in the request |  -  |
**425** | Too Early - Request is being replayed |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**501** | Not Implemented - Functionality not supported |  -  |
**502** | Bad Gateway - Invalid response from upstream server |  -  |
**503** | Service Unavailable - Try again later |  -  |
**504** | Gateway Timeout - Upstream server timed out |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_workspaces**
> ListWorkspacesResponse list_workspaces(account_id=account_id, page_size=page_size, page_number=page_number)

List workspaces

Retrieves a list of workspaces for a given account

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.list_workspaces_response import ListWorkspacesResponse
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
    api_instance = playbookmedia_backend_client_sdk.LeadScraperServiceApi(api_client)
    account_id = 'account_id_example' # str |  (optional)
    page_size = 56 # int |  (optional)
    page_number = 56 # int |  (optional)

    try:
        # List workspaces
        api_response = await api_instance.list_workspaces(account_id=account_id, page_size=page_size, page_number=page_number)
        print("The response of LeadScraperServiceApi->list_workspaces:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->list_workspaces: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account_id** | **str**|  | [optional] 
 **page_size** | **int**|  | [optional] 
 **page_number** | **int**|  | [optional] 

### Return type

[**ListWorkspacesResponse**](ListWorkspacesResponse.md)

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
**402** | Payment Required - Payment is necessary to proceed |  -  |
**403** | Forbidden - Access denied |  -  |
**404** | Not Found - Resource not found |  -  |
**405** | Method Not Allowed - HTTP method not supported |  -  |
**409** | Conflict - Resource already exists |  -  |
**410** | Gone - Resource is no longer available |  -  |
**412** | Precondition Failed - Preconditions in headers did not match |  -  |
**422** | Unprocessable Entity - Semantic errors in the request |  -  |
**425** | Too Early - Request is being replayed |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**501** | Not Implemented - Functionality not supported |  -  |
**502** | Bad Gateway - Invalid response from upstream server |  -  |
**503** | Service Unavailable - Try again later |  -  |
**504** | Gateway Timeout - Upstream server timed out |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **pause_workflow**
> PauseWorkflowResponse pause_workflow(workspace_id, id, pause_workflow_body)

Pause workflow execution

Pauses the execution of a specific workflow

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.pause_workflow_body import PauseWorkflowBody
from playbookmedia_backend_client_sdk.models.pause_workflow_response import PauseWorkflowResponse
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
    api_instance = playbookmedia_backend_client_sdk.LeadScraperServiceApi(api_client)
    workspace_id = 'workspace_id_example' # str | 
    id = 'id_example' # str | 
    pause_workflow_body = playbookmedia_backend_client_sdk.PauseWorkflowBody() # PauseWorkflowBody | 

    try:
        # Pause workflow execution
        api_response = await api_instance.pause_workflow(workspace_id, id, pause_workflow_body)
        print("The response of LeadScraperServiceApi->pause_workflow:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->pause_workflow: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **workspace_id** | **str**|  | 
 **id** | **str**|  | 
 **pause_workflow_body** | [**PauseWorkflowBody**](PauseWorkflowBody.md)|  | 

### Return type

[**PauseWorkflowResponse**](PauseWorkflowResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Workflow paused successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**402** | Payment Required - Payment is necessary to proceed |  -  |
**403** | Forbidden - Access denied |  -  |
**404** | Not Found - Resource not found |  -  |
**405** | Method Not Allowed - HTTP method not supported |  -  |
**409** | Conflict - Resource already exists |  -  |
**410** | Gone - Resource is no longer available |  -  |
**412** | Precondition Failed - Preconditions in headers did not match |  -  |
**422** | Unprocessable Entity - Semantic errors in the request |  -  |
**425** | Too Early - Request is being replayed |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**501** | Not Implemented - Functionality not supported |  -  |
**502** | Bad Gateway - Invalid response from upstream server |  -  |
**503** | Service Unavailable - Try again later |  -  |
**504** | Gateway Timeout - Upstream server timed out |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **trigger_workflow**
> TriggerWorkflowResponse trigger_workflow(workspace_id, id, trigger_workflow_body)

Trigger workflow execution

Triggers the execution of a specific workflow

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.trigger_workflow_body import TriggerWorkflowBody
from playbookmedia_backend_client_sdk.models.trigger_workflow_response import TriggerWorkflowResponse
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
    api_instance = playbookmedia_backend_client_sdk.LeadScraperServiceApi(api_client)
    workspace_id = 'workspace_id_example' # str | 
    id = 'id_example' # str | 
    trigger_workflow_body = playbookmedia_backend_client_sdk.TriggerWorkflowBody() # TriggerWorkflowBody | 

    try:
        # Trigger workflow execution
        api_response = await api_instance.trigger_workflow(workspace_id, id, trigger_workflow_body)
        print("The response of LeadScraperServiceApi->trigger_workflow:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->trigger_workflow: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **workspace_id** | **str**|  | 
 **id** | **str**|  | 
 **trigger_workflow_body** | [**TriggerWorkflowBody**](TriggerWorkflowBody.md)|  | 

### Return type

[**TriggerWorkflowResponse**](TriggerWorkflowResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Workflow triggered successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**402** | Payment Required - Payment is necessary to proceed |  -  |
**403** | Forbidden - Access denied |  -  |
**404** | Not Found - Resource not found |  -  |
**405** | Method Not Allowed - HTTP method not supported |  -  |
**409** | Conflict - Resource already exists |  -  |
**410** | Gone - Resource is no longer available |  -  |
**412** | Precondition Failed - Preconditions in headers did not match |  -  |
**422** | Unprocessable Entity - Semantic errors in the request |  -  |
**425** | Too Early - Request is being replayed |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**501** | Not Implemented - Functionality not supported |  -  |
**502** | Bad Gateway - Invalid response from upstream server |  -  |
**503** | Service Unavailable - Try again later |  -  |
**504** | Gateway Timeout - Upstream server timed out |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_account**
> UpdateAccountResponse update_account(update_account_request)

Update account details

Updates specified fields of an existing account

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
    api_instance = playbookmedia_backend_client_sdk.LeadScraperServiceApi(api_client)
    update_account_request = playbookmedia_backend_client_sdk.UpdateAccountRequest() # UpdateAccountRequest | 

    try:
        # Update account details
        api_response = await api_instance.update_account(update_account_request)
        print("The response of LeadScraperServiceApi->update_account:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->update_account: %s\n" % e)
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
**402** | Payment Required - Payment is necessary to proceed |  -  |
**403** | Forbidden - Access denied |  -  |
**404** | Not Found - Resource not found |  -  |
**405** | Method Not Allowed - HTTP method not supported |  -  |
**409** | Conflict - Resource already exists |  -  |
**410** | Gone - Resource is no longer available |  -  |
**412** | Precondition Failed - Preconditions in headers did not match |  -  |
**422** | Unprocessable Entity - Semantic errors in the request |  -  |
**425** | Too Early - Request is being replayed |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**501** | Not Implemented - Functionality not supported |  -  |
**502** | Bad Gateway - Invalid response from upstream server |  -  |
**503** | Service Unavailable - Try again later |  -  |
**504** | Gateway Timeout - Upstream server timed out |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_account_settings**
> UpdateAccountSettingsResponse update_account_settings(update_account_settings_request)

Update account settings

Updates the settings for a given account

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.update_account_settings_request import UpdateAccountSettingsRequest
from playbookmedia_backend_client_sdk.models.update_account_settings_response import UpdateAccountSettingsResponse
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
    api_instance = playbookmedia_backend_client_sdk.LeadScraperServiceApi(api_client)
    update_account_settings_request = playbookmedia_backend_client_sdk.UpdateAccountSettingsRequest() # UpdateAccountSettingsRequest | 

    try:
        # Update account settings
        api_response = await api_instance.update_account_settings(update_account_settings_request)
        print("The response of LeadScraperServiceApi->update_account_settings:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->update_account_settings: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **update_account_settings_request** | [**UpdateAccountSettingsRequest**](UpdateAccountSettingsRequest.md)|  | 

### Return type

[**UpdateAccountSettingsResponse**](UpdateAccountSettingsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Settings updated successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**402** | Payment Required - Payment is necessary to proceed |  -  |
**403** | Forbidden - Access denied |  -  |
**404** | Not Found - Resource not found |  -  |
**405** | Method Not Allowed - HTTP method not supported |  -  |
**409** | Conflict - Resource already exists |  -  |
**410** | Gone - Resource is no longer available |  -  |
**412** | Precondition Failed - Preconditions in headers did not match |  -  |
**422** | Unprocessable Entity - Semantic errors in the request |  -  |
**425** | Too Early - Request is being replayed |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**501** | Not Implemented - Functionality not supported |  -  |
**502** | Bad Gateway - Invalid response from upstream server |  -  |
**503** | Service Unavailable - Try again later |  -  |
**504** | Gateway Timeout - Upstream server timed out |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_workflow**
> ScrapingWorkflow update_workflow(update_workflow_request)

Update workflow details

Updates the details of a specific workflow

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.scraping_workflow import ScrapingWorkflow
from playbookmedia_backend_client_sdk.models.update_workflow_request import UpdateWorkflowRequest
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
    api_instance = playbookmedia_backend_client_sdk.LeadScraperServiceApi(api_client)
    update_workflow_request = playbookmedia_backend_client_sdk.UpdateWorkflowRequest() # UpdateWorkflowRequest | 

    try:
        # Update workflow details
        api_response = await api_instance.update_workflow(update_workflow_request)
        print("The response of LeadScraperServiceApi->update_workflow:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->update_workflow: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **update_workflow_request** | [**UpdateWorkflowRequest**](UpdateWorkflowRequest.md)|  | 

### Return type

[**ScrapingWorkflow**](ScrapingWorkflow.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Workflow updated successfully |  -  |
**400** | Bad Request - Invalid input parameters |  -  |
**401** | Unauthorized - Authentication required |  -  |
**402** | Payment Required - Payment is necessary to proceed |  -  |
**403** | Forbidden - Access denied |  -  |
**404** | Not Found - Resource not found |  -  |
**405** | Method Not Allowed - HTTP method not supported |  -  |
**409** | Conflict - Resource already exists |  -  |
**410** | Gone - Resource is no longer available |  -  |
**412** | Precondition Failed - Preconditions in headers did not match |  -  |
**422** | Unprocessable Entity - Semantic errors in the request |  -  |
**425** | Too Early - Request is being replayed |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**501** | Not Implemented - Functionality not supported |  -  |
**502** | Bad Gateway - Invalid response from upstream server |  -  |
**503** | Service Unavailable - Try again later |  -  |
**504** | Gateway Timeout - Upstream server timed out |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_workspace**
> UpdateWorkspaceResponse update_workspace(update_workspace_request)

Update workspace details

Updates the details of a specific workspace

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
    api_instance = playbookmedia_backend_client_sdk.LeadScraperServiceApi(api_client)
    update_workspace_request = playbookmedia_backend_client_sdk.UpdateWorkspaceRequest() # UpdateWorkspaceRequest | 

    try:
        # Update workspace details
        api_response = await api_instance.update_workspace(update_workspace_request)
        print("The response of LeadScraperServiceApi->update_workspace:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->update_workspace: %s\n" % e)
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
**402** | Payment Required - Payment is necessary to proceed |  -  |
**403** | Forbidden - Access denied |  -  |
**404** | Not Found - Resource not found |  -  |
**405** | Method Not Allowed - HTTP method not supported |  -  |
**409** | Conflict - Resource already exists |  -  |
**410** | Gone - Resource is no longer available |  -  |
**412** | Precondition Failed - Preconditions in headers did not match |  -  |
**422** | Unprocessable Entity - Semantic errors in the request |  -  |
**425** | Too Early - Request is being replayed |  -  |
**429** | Too Many Requests - Rate limit exceeded |  -  |
**500** | Internal Server Error |  -  |
**501** | Not Implemented - Functionality not supported |  -  |
**502** | Bad Gateway - Invalid response from upstream server |  -  |
**503** | Service Unavailable - Try again later |  -  |
**504** | Gateway Timeout - Upstream server timed out |  -  |
**0** | An unexpected error response. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

