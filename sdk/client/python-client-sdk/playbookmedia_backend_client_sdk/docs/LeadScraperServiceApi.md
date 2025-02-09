# playbookmedia_backend_client_sdk.LeadScraperServiceApi

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_account**](LeadScraperServiceApi.md#create_account) | **POST** /lead-scraper-microservice/api/v1/account | Create a new account
[**create_api_key**](LeadScraperServiceApi.md#create_api_key) | **POST** /lead-scraper-microservice/api/v1/api-keys | Create a new API key
[**create_organization**](LeadScraperServiceApi.md#create_organization) | **POST** /lead-scraper-microservice/api/v1/organization | Create a new organization
[**create_scraping_job**](LeadScraperServiceApi.md#create_scraping_job) | **POST** /lead-scraper-microservice/api/v1/jobs | Create a new job scraping task
[**create_tenant**](LeadScraperServiceApi.md#create_tenant) | **POST** /lead-scraper-microservice/api/v1/organizations/{organizationId}/tenants | Create a new tenant
[**create_tenant_api_key**](LeadScraperServiceApi.md#create_tenant_api_key) | **POST** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys | Create a new tenant API key
[**create_webhook**](LeadScraperServiceApi.md#create_webhook) | **POST** /lead-scraper-microservice/api/v1/webhooks | Create webhook
[**create_workflow**](LeadScraperServiceApi.md#create_workflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows | Create a new workflow
[**create_workspace**](LeadScraperServiceApi.md#create_workspace) | **POST** /lead-scraper-microservice/api/v1/workspaces | Create a new workspace
[**delete_account**](LeadScraperServiceApi.md#delete_account) | **DELETE** /lead-scraper-microservice/api/v1/accounts/{id} | Delete account
[**delete_api_key**](LeadScraperServiceApi.md#delete_api_key) | **DELETE** /lead-scraper-microservice/api/v1/api-keys/{keyId} | Delete API key
[**delete_organization**](LeadScraperServiceApi.md#delete_organization) | **DELETE** /lead-scraper-microservice/api/v1/organization/{id} | Delete an organization
[**delete_scraping_job**](LeadScraperServiceApi.md#delete_scraping_job) | **DELETE** /lead-scraper-microservice/api/v1/jobs/{jobId} | Delete a specific job
[**delete_tenant**](LeadScraperServiceApi.md#delete_tenant) | **DELETE** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId}/{tenantId} | Delete a tenant
[**delete_tenant_api_key**](LeadScraperServiceApi.md#delete_tenant_api_key) | **DELETE** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/{keyId} | Delete tenant API key
[**delete_webhook**](LeadScraperServiceApi.md#delete_webhook) | **DELETE** /lead-scraper-microservice/api/v1/webhooks/{webhookId} | Delete webhook
[**delete_workflow**](LeadScraperServiceApi.md#delete_workflow) | **DELETE** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id} | Delete workflow
[**delete_workspace**](LeadScraperServiceApi.md#delete_workspace) | **DELETE** /lead-scraper-microservice/api/v1/workspace/{id} | Delete a workspace
[**download_scraping_results**](LeadScraperServiceApi.md#download_scraping_results) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId}/download | Download job results as CSV
[**get_account**](LeadScraperServiceApi.md#get_account) | **GET** /lead-scraper-microservice/api/v1/accounts/{id} | Get account details
[**get_account_usage**](LeadScraperServiceApi.md#get_account_usage) | **GET** /lead-scraper-microservice/api/v1/accounts/{id}/usage | Get account usage
[**get_api_key**](LeadScraperServiceApi.md#get_api_key) | **GET** /lead-scraper-microservice/api/v1/api-keys/{keyId} | Get API key details
[**get_lead**](LeadScraperServiceApi.md#get_lead) | **GET** /lead-scraper-microservice/api/v1/leads/{leadId} | Get lead details
[**get_organization**](LeadScraperServiceApi.md#get_organization) | **GET** /lead-scraper-microservice/api/v1/organization/{id} | Get organization details
[**get_scraping_job**](LeadScraperServiceApi.md#get_scraping_job) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId} | Get a specific job
[**get_tenant**](LeadScraperServiceApi.md#get_tenant) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/{organizationId}/{tenantId} | Get tenant details
[**get_tenant_api_key**](LeadScraperServiceApi.md#get_tenant_api_key) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/{keyId} | Get tenant API key details
[**get_webhook**](LeadScraperServiceApi.md#get_webhook) | **GET** /lead-scraper-microservice/api/v1/webhooks/{webhookId} | Get webhook
[**get_workflow**](LeadScraperServiceApi.md#get_workflow) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id} | Get workflow details
[**get_workspace**](LeadScraperServiceApi.md#get_workspace) | **GET** /lead-scraper-microservice/api/v1/workspace/{id} | Get workspace details
[**get_workspace_analytics**](LeadScraperServiceApi.md#get_workspace_analytics) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/analytics | Get workspace analytics
[**list_accounts**](LeadScraperServiceApi.md#list_accounts) | **GET** /lead-scraper-microservice/api/v1/accounts/list | List all accounts
[**list_api_keys**](LeadScraperServiceApi.md#list_api_keys) | **GET** /lead-scraper-microservice/api/v1/api-keys/list | List API keys
[**list_leads**](LeadScraperServiceApi.md#list_leads) | **GET** /lead-scraper-microservice/api/v1/leads | List leads
[**list_organizations**](LeadScraperServiceApi.md#list_organizations) | **GET** /lead-scraper-microservice/api/v1/organization | List all organizations
[**list_scraping_jobs**](LeadScraperServiceApi.md#list_scraping_jobs) | **GET** /lead-scraper-microservice/api/v1/jobs | Get all jobs
[**list_tenant_api_keys**](LeadScraperServiceApi.md#list_tenant_api_keys) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/list | List tenant API keys
[**list_tenants**](LeadScraperServiceApi.md#list_tenants) | **GET** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId} | List all tenants
[**list_webhooks**](LeadScraperServiceApi.md#list_webhooks) | **GET** /lead-scraper-microservice/api/v1/webhooks | List webhooks
[**list_workflows**](LeadScraperServiceApi.md#list_workflows) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows | List workflows
[**list_workspaces**](LeadScraperServiceApi.md#list_workspaces) | **GET** /lead-scraper-microservice/api/v1/workspaces | List workspaces
[**pause_workflow**](LeadScraperServiceApi.md#pause_workflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id}/pause | Pause workflow execution
[**rotate_api_key**](LeadScraperServiceApi.md#rotate_api_key) | **POST** /lead-scraper-microservice/api/v1/api-keys/rotate | Rotate API key
[**rotate_tenant_api_key**](LeadScraperServiceApi.md#rotate_tenant_api_key) | **POST** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/rotate | Rotate tenant API key
[**trigger_workflow**](LeadScraperServiceApi.md#trigger_workflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id}/trigger | Trigger workflow execution
[**update_account**](LeadScraperServiceApi.md#update_account) | **PUT** /lead-scraper-microservice/api/v1/accounts/update | Update account details
[**update_account_settings**](LeadScraperServiceApi.md#update_account_settings) | **PUT** /lead-scraper-microservice/api/v1/accounts/settings | Update account settings
[**update_api_key**](LeadScraperServiceApi.md#update_api_key) | **PUT** /lead-scraper-microservice/api/v1/api-keys | Update API key
[**update_organization**](LeadScraperServiceApi.md#update_organization) | **PUT** /lead-scraper-microservice/api/v1/organization | Update organization details
[**update_tenant**](LeadScraperServiceApi.md#update_tenant) | **PUT** /lead-scraper-microservice/api/v1/organizations/tenants | Update tenant details
[**update_tenant_api_key**](LeadScraperServiceApi.md#update_tenant_api_key) | **PUT** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys | Update tenant API key
[**update_webhook**](LeadScraperServiceApi.md#update_webhook) | **PUT** /lead-scraper-microservice/api/v1/webhooks | Update webhook
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

# **create_api_key**
> CreateAPIKeyResponse create_api_key(create_api_key_request)

Create a new API key

Creates a new API key with specified permissions and settings

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.create_api_key_request import CreateAPIKeyRequest
from playbookmedia_backend_client_sdk.models.create_api_key_response import CreateAPIKeyResponse
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
    create_api_key_request = playbookmedia_backend_client_sdk.CreateAPIKeyRequest() # CreateAPIKeyRequest | 

    try:
        # Create a new API key
        api_response = await api_instance.create_api_key(create_api_key_request)
        print("The response of LeadScraperServiceApi->create_api_key:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->create_api_key: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **create_api_key_request** | [**CreateAPIKeyRequest**](CreateAPIKeyRequest.md)|  | 

### Return type

[**CreateAPIKeyResponse**](CreateAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | API key created successfully |  -  |
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

# **create_organization**
> CreateOrganizationResponse create_organization(create_organization_request)

Create a new organization

Creates a new organization within a tenant

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.create_organization_request import CreateOrganizationRequest
from playbookmedia_backend_client_sdk.models.create_organization_response import CreateOrganizationResponse
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
    create_organization_request = playbookmedia_backend_client_sdk.CreateOrganizationRequest() # CreateOrganizationRequest | 

    try:
        # Create a new organization
        api_response = await api_instance.create_organization(create_organization_request)
        print("The response of LeadScraperServiceApi->create_organization:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->create_organization: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **create_organization_request** | [**CreateOrganizationRequest**](CreateOrganizationRequest.md)|  | 

### Return type

[**CreateOrganizationResponse**](CreateOrganizationResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | A successful response. |  -  |
**201** | Organization created successfully |  -  |
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

# **create_tenant**
> CreateTenantResponse create_tenant(organization_id, create_tenant_body)

Create a new tenant

Creates a new tenant in the system

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.create_tenant_body import CreateTenantBody
from playbookmedia_backend_client_sdk.models.create_tenant_response import CreateTenantResponse
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
    organization_id = 'organization_id_example' # str | 
    create_tenant_body = playbookmedia_backend_client_sdk.CreateTenantBody() # CreateTenantBody | 

    try:
        # Create a new tenant
        api_response = await api_instance.create_tenant(organization_id, create_tenant_body)
        print("The response of LeadScraperServiceApi->create_tenant:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->create_tenant: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **organization_id** | **str**|  | 
 **create_tenant_body** | [**CreateTenantBody**](CreateTenantBody.md)|  | 

### Return type

[**CreateTenantResponse**](CreateTenantResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | A successful response. |  -  |
**201** | Tenant created successfully |  -  |
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

# **create_tenant_api_key**
> CreateTenantAPIKeyResponse create_tenant_api_key(create_tenant_api_key_request)

Create a new tenant API key

Creates a new API key for a tenant with specified permissions

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.create_tenant_api_key_request import CreateTenantAPIKeyRequest
from playbookmedia_backend_client_sdk.models.create_tenant_api_key_response import CreateTenantAPIKeyResponse
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
    create_tenant_api_key_request = playbookmedia_backend_client_sdk.CreateTenantAPIKeyRequest() # CreateTenantAPIKeyRequest | 

    try:
        # Create a new tenant API key
        api_response = await api_instance.create_tenant_api_key(create_tenant_api_key_request)
        print("The response of LeadScraperServiceApi->create_tenant_api_key:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->create_tenant_api_key: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **create_tenant_api_key_request** | [**CreateTenantAPIKeyRequest**](CreateTenantAPIKeyRequest.md)|  | 

### Return type

[**CreateTenantAPIKeyResponse**](CreateTenantAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Organizations retrieved successfully |  -  |
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

# **create_webhook**
> CreateWebhookResponse create_webhook(create_webhook_request)

Create webhook

Creates a new webhook configuration for event notifications

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.create_webhook_request import CreateWebhookRequest
from playbookmedia_backend_client_sdk.models.create_webhook_response import CreateWebhookResponse
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
    create_webhook_request = playbookmedia_backend_client_sdk.CreateWebhookRequest() # CreateWebhookRequest | 

    try:
        # Create webhook
        api_response = await api_instance.create_webhook(create_webhook_request)
        print("The response of LeadScraperServiceApi->create_webhook:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->create_webhook: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **create_webhook_request** | [**CreateWebhookRequest**](CreateWebhookRequest.md)|  | 

### Return type

[**CreateWebhookResponse**](CreateWebhookResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Webhook created successfully |  -  |
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

# **create_workflow**
> CreateWorkflowResponse create_workflow(workspace_id, create_workflow_body)

Create a new workflow

Creates a new workflow for a specific workspace

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
        api_response = await api_instance.create_workflow(workspace_id, create_workflow_body)
        print("The response of LeadScraperServiceApi->create_workflow:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->create_workflow: %s\n" % e)
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
> DeleteAccountResponse delete_account(id, organization_id=organization_id, tenant_id=tenant_id)

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
    organization_id = 'organization_id_example' # str |  (optional)
    tenant_id = 'tenant_id_example' # str |  (optional)

    try:
        # Delete account
        api_response = await api_instance.delete_account(id, organization_id=organization_id, tenant_id=tenant_id)
        print("The response of LeadScraperServiceApi->delete_account:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->delete_account: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**|  | 
 **organization_id** | **str**|  | [optional] 
 **tenant_id** | **str**|  | [optional] 

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

# **delete_api_key**
> DeleteAPIKeyResponse delete_api_key(key_id, organization_id=organization_id, workspace_id=workspace_id, tenant_id=tenant_id, account_id=account_id)

Delete API key

Permanently deletes an API key

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.delete_api_key_response import DeleteAPIKeyResponse
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
    key_id = 'key_id_example' # str | 
    organization_id = 'organization_id_example' # str |  (optional)
    workspace_id = 'workspace_id_example' # str |  (optional)
    tenant_id = 'tenant_id_example' # str |  (optional)
    account_id = 'account_id_example' # str |  (optional)

    try:
        # Delete API key
        api_response = await api_instance.delete_api_key(key_id, organization_id=organization_id, workspace_id=workspace_id, tenant_id=tenant_id, account_id=account_id)
        print("The response of LeadScraperServiceApi->delete_api_key:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->delete_api_key: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **key_id** | **str**|  | 
 **organization_id** | **str**|  | [optional] 
 **workspace_id** | **str**|  | [optional] 
 **tenant_id** | **str**|  | [optional] 
 **account_id** | **str**|  | [optional] 

### Return type

[**DeleteAPIKeyResponse**](DeleteAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | API key deleted successfully |  -  |
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

# **delete_organization**
> DeleteOrganizationResponse delete_organization(id)

Delete an organization

Permanently deletes an organization and all associated resources

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.delete_organization_response import DeleteOrganizationResponse
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
        # Delete an organization
        api_response = await api_instance.delete_organization(id)
        print("The response of LeadScraperServiceApi->delete_organization:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->delete_organization: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**|  | 

### Return type

[**DeleteOrganizationResponse**](DeleteOrganizationResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Organization deleted successfully |  -  |
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

# **delete_tenant**
> DeleteTenantResponse delete_tenant(organization_id, tenant_id)

Delete a tenant

Permanently deletes a tenant and all associated resources

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.delete_tenant_response import DeleteTenantResponse
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
    organization_id = 'organization_id_example' # str | 
    tenant_id = 'tenant_id_example' # str | 

    try:
        # Delete a tenant
        api_response = await api_instance.delete_tenant(organization_id, tenant_id)
        print("The response of LeadScraperServiceApi->delete_tenant:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->delete_tenant: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **organization_id** | **str**|  | 
 **tenant_id** | **str**|  | 

### Return type

[**DeleteTenantResponse**](DeleteTenantResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Tenant deleted successfully |  -  |
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

# **delete_tenant_api_key**
> DeleteTenantAPIKeyResponse delete_tenant_api_key(key_id, organization_id=organization_id, tenant_id=tenant_id)

Delete tenant API key

Permanently deletes a tenant API key

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.delete_tenant_api_key_response import DeleteTenantAPIKeyResponse
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
    key_id = 'key_id_example' # str | 
    organization_id = 'organization_id_example' # str |  (optional)
    tenant_id = 'tenant_id_example' # str |  (optional)

    try:
        # Delete tenant API key
        api_response = await api_instance.delete_tenant_api_key(key_id, organization_id=organization_id, tenant_id=tenant_id)
        print("The response of LeadScraperServiceApi->delete_tenant_api_key:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->delete_tenant_api_key: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **key_id** | **str**|  | 
 **organization_id** | **str**|  | [optional] 
 **tenant_id** | **str**|  | [optional] 

### Return type

[**DeleteTenantAPIKeyResponse**](DeleteTenantAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Tenant API key deleted successfully |  -  |
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

# **delete_webhook**
> DeleteWebhookResponse delete_webhook(webhook_id, organization_id=organization_id, workspace_id=workspace_id, tenant_id=tenant_id, account_id=account_id)

Delete webhook

Deletes a webhook configuration

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.delete_webhook_response import DeleteWebhookResponse
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
    webhook_id = 'webhook_id_example' # str | 
    organization_id = 'organization_id_example' # str |  (optional)
    workspace_id = 'workspace_id_example' # str |  (optional)
    tenant_id = 'tenant_id_example' # str |  (optional)
    account_id = 'account_id_example' # str |  (optional)

    try:
        # Delete webhook
        api_response = await api_instance.delete_webhook(webhook_id, organization_id=organization_id, workspace_id=workspace_id, tenant_id=tenant_id, account_id=account_id)
        print("The response of LeadScraperServiceApi->delete_webhook:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->delete_webhook: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **webhook_id** | **str**|  | 
 **organization_id** | **str**|  | [optional] 
 **workspace_id** | **str**|  | [optional] 
 **tenant_id** | **str**|  | [optional] 
 **account_id** | **str**|  | [optional] 

### Return type

[**DeleteWebhookResponse**](DeleteWebhookResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Webhook deleted successfully |  -  |
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

# **delete_workflow**
> DeleteWorkflowResponse delete_workflow(workspace_id, id, org_id, tenant_id, account_id=account_id)

Delete workflow

Deletes a specific workflow

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.delete_workflow_response import DeleteWorkflowResponse
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
    org_id = 'org_id_example' # str | 
    tenant_id = 'tenant_id_example' # str | 
    account_id = 'account_id_example' # str |  (optional)

    try:
        # Delete workflow
        api_response = await api_instance.delete_workflow(workspace_id, id, org_id, tenant_id, account_id=account_id)
        print("The response of LeadScraperServiceApi->delete_workflow:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->delete_workflow: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **workspace_id** | **str**|  | 
 **id** | **str**|  | 
 **org_id** | **str**|  | 
 **tenant_id** | **str**|  | 
 **account_id** | **str**|  | [optional] 

### Return type

[**DeleteWorkflowResponse**](DeleteWorkflowResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Workflow deleted successfully |  -  |
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
> GetAccountResponse get_account(id, organization_id=organization_id, tenant_id=tenant_id)

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
    organization_id = 'organization_id_example' # str |  (optional)
    tenant_id = 'tenant_id_example' # str |  (optional)

    try:
        # Get account details
        api_response = await api_instance.get_account(id, organization_id=organization_id, tenant_id=tenant_id)
        print("The response of LeadScraperServiceApi->get_account:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->get_account: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**|  | 
 **organization_id** | **str**|  | [optional] 
 **tenant_id** | **str**|  | [optional] 

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

# **get_api_key**
> GetAPIKeyResponse get_api_key(key_id, organization_id=organization_id, tenant_id=tenant_id, account_id=account_id, workspace_id=workspace_id)

Get API key details

Retrieves details of a specific API key

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.get_api_key_response import GetAPIKeyResponse
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
    key_id = 'key_id_example' # str | 
    organization_id = 'organization_id_example' # str |  (optional)
    tenant_id = 'tenant_id_example' # str |  (optional)
    account_id = 'account_id_example' # str |  (optional)
    workspace_id = 'workspace_id_example' # str |  (optional)

    try:
        # Get API key details
        api_response = await api_instance.get_api_key(key_id, organization_id=organization_id, tenant_id=tenant_id, account_id=account_id, workspace_id=workspace_id)
        print("The response of LeadScraperServiceApi->get_api_key:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->get_api_key: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **key_id** | **str**|  | 
 **organization_id** | **str**|  | [optional] 
 **tenant_id** | **str**|  | [optional] 
 **account_id** | **str**|  | [optional] 
 **workspace_id** | **str**|  | [optional] 

### Return type

[**GetAPIKeyResponse**](GetAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | API key retrieved successfully |  -  |
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

# **get_lead**
> GetLeadResponse get_lead(lead_id, organization_id=organization_id, workspace_id=workspace_id, tenant_id=tenant_id, account_id=account_id)

Get lead details

Retrieves detailed information about a specific lead

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.get_lead_response import GetLeadResponse
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
    lead_id = 'lead_id_example' # str | 
    organization_id = 'organization_id_example' # str |  (optional)
    workspace_id = 'workspace_id_example' # str |  (optional)
    tenant_id = 'tenant_id_example' # str |  (optional)
    account_id = 'account_id_example' # str |  (optional)

    try:
        # Get lead details
        api_response = await api_instance.get_lead(lead_id, organization_id=organization_id, workspace_id=workspace_id, tenant_id=tenant_id, account_id=account_id)
        print("The response of LeadScraperServiceApi->get_lead:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->get_lead: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **lead_id** | **str**|  | 
 **organization_id** | **str**|  | [optional] 
 **workspace_id** | **str**|  | [optional] 
 **tenant_id** | **str**|  | [optional] 
 **account_id** | **str**|  | [optional] 

### Return type

[**GetLeadResponse**](GetLeadResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Lead retrieved successfully |  -  |
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

# **get_organization**
> GetOrganizationResponse get_organization(id)

Get organization details

Retrieves details of a specific organization

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.get_organization_response import GetOrganizationResponse
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
        # Get organization details
        api_response = await api_instance.get_organization(id)
        print("The response of LeadScraperServiceApi->get_organization:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->get_organization: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**|  | 

### Return type

[**GetOrganizationResponse**](GetOrganizationResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Organization retrieved successfully |  -  |
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

# **get_tenant**
> GetTenantResponse get_tenant(organization_id, tenant_id)

Get tenant details

Retrieves details of a specific tenant

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.get_tenant_response import GetTenantResponse
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
    organization_id = 'organization_id_example' # str | 
    tenant_id = 'tenant_id_example' # str | 

    try:
        # Get tenant details
        api_response = await api_instance.get_tenant(organization_id, tenant_id)
        print("The response of LeadScraperServiceApi->get_tenant:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->get_tenant: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **organization_id** | **str**|  | 
 **tenant_id** | **str**|  | 

### Return type

[**GetTenantResponse**](GetTenantResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Tenant retrieved successfully |  -  |
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

# **get_tenant_api_key**
> GetTenantAPIKeyResponse get_tenant_api_key(key_id, organization_id=organization_id, tenant_id=tenant_id)

Get tenant API key details

Retrieves details of a specific tenant API key

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.get_tenant_api_key_response import GetTenantAPIKeyResponse
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
    key_id = 'key_id_example' # str | 
    organization_id = 'organization_id_example' # str |  (optional)
    tenant_id = 'tenant_id_example' # str |  (optional)

    try:
        # Get tenant API key details
        api_response = await api_instance.get_tenant_api_key(key_id, organization_id=organization_id, tenant_id=tenant_id)
        print("The response of LeadScraperServiceApi->get_tenant_api_key:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->get_tenant_api_key: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **key_id** | **str**|  | 
 **organization_id** | **str**|  | [optional] 
 **tenant_id** | **str**|  | [optional] 

### Return type

[**GetTenantAPIKeyResponse**](GetTenantAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Tenant API key retrieved successfully |  -  |
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

# **get_webhook**
> GetWebhookResponse get_webhook(webhook_id, organization_id=organization_id, workspace_id=workspace_id, tenant_id=tenant_id, account_id=account_id)

Get webhook

Retrieves details of a specific webhook configuration

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.get_webhook_response import GetWebhookResponse
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
    webhook_id = 'webhook_id_example' # str | 
    organization_id = 'organization_id_example' # str |  (optional)
    workspace_id = 'workspace_id_example' # str |  (optional)
    tenant_id = 'tenant_id_example' # str |  (optional)
    account_id = 'account_id_example' # str |  (optional)

    try:
        # Get webhook
        api_response = await api_instance.get_webhook(webhook_id, organization_id=organization_id, workspace_id=workspace_id, tenant_id=tenant_id, account_id=account_id)
        print("The response of LeadScraperServiceApi->get_webhook:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->get_webhook: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **webhook_id** | **str**|  | 
 **organization_id** | **str**|  | [optional] 
 **workspace_id** | **str**|  | [optional] 
 **tenant_id** | **str**|  | [optional] 
 **account_id** | **str**|  | [optional] 

### Return type

[**GetWebhookResponse**](GetWebhookResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Webhook retrieved successfully |  -  |
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
> GetWorkflowResponse get_workflow(workspace_id, id)

Get workflow details

Retrieves details of a specific workflow

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.get_workflow_response import GetWorkflowResponse
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

[**GetWorkflowResponse**](GetWorkflowResponse.md)

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
> GetWorkspaceResponse get_workspace(id, organization_id=organization_id, tenant_id=tenant_id, account_id=account_id)

Get workspace details

Retrieves details of a specific workspace

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.get_workspace_response import GetWorkspaceResponse
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
    organization_id = 'organization_id_example' # str |  (optional)
    tenant_id = 'tenant_id_example' # str |  (optional)
    account_id = 'account_id_example' # str |  (optional)

    try:
        # Get workspace details
        api_response = await api_instance.get_workspace(id, organization_id=organization_id, tenant_id=tenant_id, account_id=account_id)
        print("The response of LeadScraperServiceApi->get_workspace:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->get_workspace: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**|  | 
 **organization_id** | **str**|  | [optional] 
 **tenant_id** | **str**|  | [optional] 
 **account_id** | **str**|  | [optional] 

### Return type

[**GetWorkspaceResponse**](GetWorkspaceResponse.md)

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

# **list_accounts**
> ListAccountsResponse list_accounts(page_size=page_size, page_number=page_number, filter=filter, organization_id=organization_id, tenant_id=tenant_id)

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
    organization_id = 'organization_id_example' # str |  (optional)
    tenant_id = 'tenant_id_example' # str |  (optional)

    try:
        # List all accounts
        api_response = await api_instance.list_accounts(page_size=page_size, page_number=page_number, filter=filter, organization_id=organization_id, tenant_id=tenant_id)
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
 **organization_id** | **str**|  | [optional] 
 **tenant_id** | **str**|  | [optional] 

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

# **list_api_keys**
> ListAPIKeysResponse list_api_keys(organization_id=organization_id, tenant_id=tenant_id, account_id=account_id, workspace_id=workspace_id, page_size=page_size, page_number=page_number, status=status, search=search)

List API keys

Retrieves a list of API keys with optional filtering

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.list_api_keys_response import ListAPIKeysResponse
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
    organization_id = 'organization_id_example' # str |  (optional)
    tenant_id = 'tenant_id_example' # str |  (optional)
    account_id = 'account_id_example' # str |  (optional)
    workspace_id = 'workspace_id_example' # str |  (optional)
    page_size = 56 # int |  (optional)
    page_number = 56 # int |  (optional)
    status = 'status_example' # str |  (optional)
    search = 'search_example' # str |  (optional)

    try:
        # List API keys
        api_response = await api_instance.list_api_keys(organization_id=organization_id, tenant_id=tenant_id, account_id=account_id, workspace_id=workspace_id, page_size=page_size, page_number=page_number, status=status, search=search)
        print("The response of LeadScraperServiceApi->list_api_keys:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->list_api_keys: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **organization_id** | **str**|  | [optional] 
 **tenant_id** | **str**|  | [optional] 
 **account_id** | **str**|  | [optional] 
 **workspace_id** | **str**|  | [optional] 
 **page_size** | **int**|  | [optional] 
 **page_number** | **int**|  | [optional] 
 **status** | **str**|  | [optional] 
 **search** | **str**|  | [optional] 

### Return type

[**ListAPIKeysResponse**](ListAPIKeysResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | API keys retrieved successfully |  -  |
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

# **list_leads**
> ListLeadsResponse list_leads(organization_id=organization_id, workspace_id=workspace_id, tenant_id=tenant_id, account_id=account_id, page_size=page_size, page_number=page_number)

List leads

Retrieves a paginated list of leads with comprehensive filtering options

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.list_leads_response import ListLeadsResponse
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
    organization_id = 'organization_id_example' # str | Context identifiers (optional)
    workspace_id = 'workspace_id_example' # str |  (optional)
    tenant_id = 'tenant_id_example' # str |  (optional)
    account_id = 'account_id_example' # str |  (optional)
    page_size = 56 # int | Pagination (optional)
    page_number = 56 # int |  (optional)

    try:
        # List leads
        api_response = await api_instance.list_leads(organization_id=organization_id, workspace_id=workspace_id, tenant_id=tenant_id, account_id=account_id, page_size=page_size, page_number=page_number)
        print("The response of LeadScraperServiceApi->list_leads:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->list_leads: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **organization_id** | **str**| Context identifiers | [optional] 
 **workspace_id** | **str**|  | [optional] 
 **tenant_id** | **str**|  | [optional] 
 **account_id** | **str**|  | [optional] 
 **page_size** | **int**| Pagination | [optional] 
 **page_number** | **int**|  | [optional] 

### Return type

[**ListLeadsResponse**](ListLeadsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Leads retrieved successfully |  -  |
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

# **list_organizations**
> ListOrganizationsResponse list_organizations(page_size=page_size, page_number=page_number)

List all organizations

Retrieves a list of all organizations in a tenant

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.list_organizations_response import ListOrganizationsResponse
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

    try:
        # List all organizations
        api_response = await api_instance.list_organizations(page_size=page_size, page_number=page_number)
        print("The response of LeadScraperServiceApi->list_organizations:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->list_organizations: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **page_size** | **int**|  | [optional] 
 **page_number** | **int**|  | [optional] 

### Return type

[**ListOrganizationsResponse**](ListOrganizationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Organizations retrieved successfully |  -  |
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
> ListScrapingJobsResponse list_scraping_jobs(auth_platform_user_id, org_id, tenant_id)

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
    auth_platform_user_id = 'auth_platform_user_id_example' # str | 
    org_id = 'org_id_example' # str | 
    tenant_id = 'tenant_id_example' # str | 

    try:
        # Get all jobs
        api_response = await api_instance.list_scraping_jobs(auth_platform_user_id, org_id, tenant_id)
        print("The response of LeadScraperServiceApi->list_scraping_jobs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->list_scraping_jobs: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **auth_platform_user_id** | **str**|  | 
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

# **list_tenant_api_keys**
> ListTenantAPIKeysResponse list_tenant_api_keys(organization_id=organization_id, tenant_id=tenant_id, page_size=page_size, page_number=page_number, status=status)

List tenant API keys

Retrieves all API keys for a specific tenant

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.list_tenant_api_keys_response import ListTenantAPIKeysResponse
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
    organization_id = 'organization_id_example' # str |  (optional)
    tenant_id = 'tenant_id_example' # str |  (optional)
    page_size = 56 # int |  (optional)
    page_number = 56 # int |  (optional)
    status = 'status_example' # str |  (optional)

    try:
        # List tenant API keys
        api_response = await api_instance.list_tenant_api_keys(organization_id=organization_id, tenant_id=tenant_id, page_size=page_size, page_number=page_number, status=status)
        print("The response of LeadScraperServiceApi->list_tenant_api_keys:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->list_tenant_api_keys: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **organization_id** | **str**|  | [optional] 
 **tenant_id** | **str**|  | [optional] 
 **page_size** | **int**|  | [optional] 
 **page_number** | **int**|  | [optional] 
 **status** | **str**|  | [optional] 

### Return type

[**ListTenantAPIKeysResponse**](ListTenantAPIKeysResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Tenant API keys retrieved successfully |  -  |
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

# **list_tenants**
> ListTenantsResponse list_tenants(organization_id, page_size=page_size, page_number=page_number)

List all tenants

Retrieves a list of all tenants in the system

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.list_tenants_response import ListTenantsResponse
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
    organization_id = 'organization_id_example' # str | 
    page_size = 56 # int |  (optional)
    page_number = 56 # int |  (optional)

    try:
        # List all tenants
        api_response = await api_instance.list_tenants(organization_id, page_size=page_size, page_number=page_number)
        print("The response of LeadScraperServiceApi->list_tenants:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->list_tenants: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **organization_id** | **str**|  | 
 **page_size** | **int**|  | [optional] 
 **page_number** | **int**|  | [optional] 

### Return type

[**ListTenantsResponse**](ListTenantsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Tenants retrieved successfully |  -  |
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

# **list_webhooks**
> ListWebhooksResponse list_webhooks(organization_id=organization_id, workspace_id=workspace_id, tenant_id=tenant_id, account_id=account_id, page_size=page_size, page_number=page_number, status=status, search=search)

List webhooks

Lists all webhook configurations with pagination

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.list_webhooks_response import ListWebhooksResponse
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
    organization_id = 'organization_id_example' # str |  (optional)
    workspace_id = 'workspace_id_example' # str |  (optional)
    tenant_id = 'tenant_id_example' # str |  (optional)
    account_id = 'account_id_example' # str |  (optional)
    page_size = 56 # int |  (optional)
    page_number = 56 # int |  (optional)
    status = 'status_example' # str |  (optional)
    search = 'search_example' # str |  (optional)

    try:
        # List webhooks
        api_response = await api_instance.list_webhooks(organization_id=organization_id, workspace_id=workspace_id, tenant_id=tenant_id, account_id=account_id, page_size=page_size, page_number=page_number, status=status, search=search)
        print("The response of LeadScraperServiceApi->list_webhooks:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->list_webhooks: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **organization_id** | **str**|  | [optional] 
 **workspace_id** | **str**|  | [optional] 
 **tenant_id** | **str**|  | [optional] 
 **account_id** | **str**|  | [optional] 
 **page_size** | **int**|  | [optional] 
 **page_number** | **int**|  | [optional] 
 **status** | **str**|  | [optional] 
 **search** | **str**|  | [optional] 

### Return type

[**ListWebhooksResponse**](ListWebhooksResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Webhooks retrieved successfully |  -  |
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
> ListWorkflowsResponse list_workflows(workspace_id, page_size=page_size, page_number=page_number, filter=filter, organization_id=organization_id, tenant_id=tenant_id, account_id=account_id)

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
    organization_id = 'organization_id_example' # str |  (optional)
    tenant_id = 'tenant_id_example' # str |  (optional)
    account_id = 'account_id_example' # str |  (optional)

    try:
        # List workflows
        api_response = await api_instance.list_workflows(workspace_id, page_size=page_size, page_number=page_number, filter=filter, organization_id=organization_id, tenant_id=tenant_id, account_id=account_id)
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
 **organization_id** | **str**|  | [optional] 
 **tenant_id** | **str**|  | [optional] 
 **account_id** | **str**|  | [optional] 

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
> ListWorkspacesResponse list_workspaces(account_id=account_id, page_size=page_size, page_number=page_number, organization_id=organization_id, tenant_id=tenant_id)

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
    organization_id = 'organization_id_example' # str |  (optional)
    tenant_id = 'tenant_id_example' # str |  (optional)

    try:
        # List workspaces
        api_response = await api_instance.list_workspaces(account_id=account_id, page_size=page_size, page_number=page_number, organization_id=organization_id, tenant_id=tenant_id)
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
 **organization_id** | **str**|  | [optional] 
 **tenant_id** | **str**|  | [optional] 

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

# **rotate_api_key**
> RotateAPIKeyResponse rotate_api_key(rotate_api_key_request)

Rotate API key

Rotates an API key while maintaining its configuration

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.rotate_api_key_request import RotateAPIKeyRequest
from playbookmedia_backend_client_sdk.models.rotate_api_key_response import RotateAPIKeyResponse
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
    rotate_api_key_request = playbookmedia_backend_client_sdk.RotateAPIKeyRequest() # RotateAPIKeyRequest | 

    try:
        # Rotate API key
        api_response = await api_instance.rotate_api_key(rotate_api_key_request)
        print("The response of LeadScraperServiceApi->rotate_api_key:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->rotate_api_key: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **rotate_api_key_request** | [**RotateAPIKeyRequest**](RotateAPIKeyRequest.md)|  | 

### Return type

[**RotateAPIKeyResponse**](RotateAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | API key rotated successfully |  -  |
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

# **rotate_tenant_api_key**
> RotateTenantAPIKeyResponse rotate_tenant_api_key(rotate_tenant_api_key_request)

Rotate tenant API key

Rotates a tenant API key while maintaining its configuration

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.rotate_tenant_api_key_request import RotateTenantAPIKeyRequest
from playbookmedia_backend_client_sdk.models.rotate_tenant_api_key_response import RotateTenantAPIKeyResponse
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
    rotate_tenant_api_key_request = playbookmedia_backend_client_sdk.RotateTenantAPIKeyRequest() # RotateTenantAPIKeyRequest | 

    try:
        # Rotate tenant API key
        api_response = await api_instance.rotate_tenant_api_key(rotate_tenant_api_key_request)
        print("The response of LeadScraperServiceApi->rotate_tenant_api_key:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->rotate_tenant_api_key: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **rotate_tenant_api_key_request** | [**RotateTenantAPIKeyRequest**](RotateTenantAPIKeyRequest.md)|  | 

### Return type

[**RotateTenantAPIKeyResponse**](RotateTenantAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Tenant API key rotated successfully |  -  |
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

# **update_api_key**
> UpdateAPIKeyResponse update_api_key(update_api_key_request)

Update API key

Updates an existing API key's configuration

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.update_api_key_request import UpdateAPIKeyRequest
from playbookmedia_backend_client_sdk.models.update_api_key_response import UpdateAPIKeyResponse
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
    update_api_key_request = playbookmedia_backend_client_sdk.UpdateAPIKeyRequest() # UpdateAPIKeyRequest | 

    try:
        # Update API key
        api_response = await api_instance.update_api_key(update_api_key_request)
        print("The response of LeadScraperServiceApi->update_api_key:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->update_api_key: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **update_api_key_request** | [**UpdateAPIKeyRequest**](UpdateAPIKeyRequest.md)|  | 

### Return type

[**UpdateAPIKeyResponse**](UpdateAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | API key updated successfully |  -  |
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

# **update_organization**
> UpdateOrganizationResponse update_organization(update_organization_request)

Update organization details

Updates the configuration of a specific organization

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.update_organization_request import UpdateOrganizationRequest
from playbookmedia_backend_client_sdk.models.update_organization_response import UpdateOrganizationResponse
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
    update_organization_request = playbookmedia_backend_client_sdk.UpdateOrganizationRequest() # UpdateOrganizationRequest | 

    try:
        # Update organization details
        api_response = await api_instance.update_organization(update_organization_request)
        print("The response of LeadScraperServiceApi->update_organization:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->update_organization: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **update_organization_request** | [**UpdateOrganizationRequest**](UpdateOrganizationRequest.md)|  | 

### Return type

[**UpdateOrganizationResponse**](UpdateOrganizationResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Organization updated successfully |  -  |
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

# **update_tenant**
> UpdateTenantResponse update_tenant(update_tenant_request)

Update tenant details

Updates the configuration of a specific tenant

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.update_tenant_request import UpdateTenantRequest
from playbookmedia_backend_client_sdk.models.update_tenant_response import UpdateTenantResponse
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
    update_tenant_request = playbookmedia_backend_client_sdk.UpdateTenantRequest() # UpdateTenantRequest | 

    try:
        # Update tenant details
        api_response = await api_instance.update_tenant(update_tenant_request)
        print("The response of LeadScraperServiceApi->update_tenant:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->update_tenant: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **update_tenant_request** | [**UpdateTenantRequest**](UpdateTenantRequest.md)|  | 

### Return type

[**UpdateTenantResponse**](UpdateTenantResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Tenant updated successfully |  -  |
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

# **update_tenant_api_key**
> UpdateTenantAPIKeyResponse update_tenant_api_key(update_tenant_api_key_request)

Update tenant API key

Updates an existing tenant API key's configuration

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.update_tenant_api_key_request import UpdateTenantAPIKeyRequest
from playbookmedia_backend_client_sdk.models.update_tenant_api_key_response import UpdateTenantAPIKeyResponse
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
    update_tenant_api_key_request = playbookmedia_backend_client_sdk.UpdateTenantAPIKeyRequest() # UpdateTenantAPIKeyRequest | 

    try:
        # Update tenant API key
        api_response = await api_instance.update_tenant_api_key(update_tenant_api_key_request)
        print("The response of LeadScraperServiceApi->update_tenant_api_key:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->update_tenant_api_key: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **update_tenant_api_key_request** | [**UpdateTenantAPIKeyRequest**](UpdateTenantAPIKeyRequest.md)|  | 

### Return type

[**UpdateTenantAPIKeyResponse**](UpdateTenantAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Tenant API key updated successfully |  -  |
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

# **update_webhook**
> UpdateWebhookResponse update_webhook(update_webhook_request)

Update webhook

Updates an existing webhook configuration

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.update_webhook_request import UpdateWebhookRequest
from playbookmedia_backend_client_sdk.models.update_webhook_response import UpdateWebhookResponse
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
    update_webhook_request = playbookmedia_backend_client_sdk.UpdateWebhookRequest() # UpdateWebhookRequest | 

    try:
        # Update webhook
        api_response = await api_instance.update_webhook(update_webhook_request)
        print("The response of LeadScraperServiceApi->update_webhook:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling LeadScraperServiceApi->update_webhook: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **update_webhook_request** | [**UpdateWebhookRequest**](UpdateWebhookRequest.md)|  | 

### Return type

[**UpdateWebhookResponse**](UpdateWebhookResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: application/json, 

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Webhook updated successfully |  -  |
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
> UpdateWorkflowResponse update_workflow(update_workflow_request)

Update workflow details

Updates the details of a specific workflow

### Example


```python
import playbookmedia_backend_client_sdk
from playbookmedia_backend_client_sdk.models.update_workflow_request import UpdateWorkflowRequest
from playbookmedia_backend_client_sdk.models.update_workflow_response import UpdateWorkflowResponse
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

[**UpdateWorkflowResponse**](UpdateWorkflowResponse.md)

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

