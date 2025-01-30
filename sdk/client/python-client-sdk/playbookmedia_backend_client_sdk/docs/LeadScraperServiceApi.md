# playbookmedia_backend_client_sdk.LeadScraperServiceApi

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_account**](LeadScraperServiceApi.md#create_account) | **POST** /lead-scraper-microservice/api/v1/accounts | Create a new account
[**create_scraping_job**](LeadScraperServiceApi.md#create_scraping_job) | **POST** /lead-scraper-microservice/api/v1/jobs | Create a new job scraping task
[**delete_account**](LeadScraperServiceApi.md#delete_account) | **DELETE** /lead-scraper-microservice/api/v1/accounts/{id} | Delete account
[**delete_scraping_job**](LeadScraperServiceApi.md#delete_scraping_job) | **DELETE** /lead-scraper-microservice/api/v1/jobs/{jobId} | Delete a specific job
[**download_scraping_results**](LeadScraperServiceApi.md#download_scraping_results) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId}/download | Download job results as CSV
[**get_account**](LeadScraperServiceApi.md#get_account) | **GET** /lead-scraper-microservice/api/v1/accounts/{id} | Get account details
[**get_scraping_job**](LeadScraperServiceApi.md#get_scraping_job) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId} | Get a specific job
[**list_scraping_jobs**](LeadScraperServiceApi.md#list_scraping_jobs) | **GET** /lead-scraper-microservice/api/v1/jobs | Get all jobs
[**update_account**](LeadScraperServiceApi.md#update_account) | **PUT** /lead-scraper-microservice/api/v1/accounts | Update account details


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

