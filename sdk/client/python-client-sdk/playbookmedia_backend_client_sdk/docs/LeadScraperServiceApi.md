# playbookmedia_backend_client_sdk.LeadScraperServiceApi

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_organization**](LeadScraperServiceApi.md#create_organization) | **POST** /lead-scraper-microservice/api/v1/organization | Create a new organization
[**create_tenant**](LeadScraperServiceApi.md#create_tenant) | **POST** /lead-scraper-microservice/api/v1/organizations/{organizationId}/tenants | Create a new tenant
[**delete_organization**](LeadScraperServiceApi.md#delete_organization) | **DELETE** /lead-scraper-microservice/api/v1/organization/{id} | Delete an organization
[**delete_tenant**](LeadScraperServiceApi.md#delete_tenant) | **DELETE** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId}/{tenantId} | Delete a tenant
[**get_organization**](LeadScraperServiceApi.md#get_organization) | **GET** /lead-scraper-microservice/api/v1/organization/{id} | Get organization details
[**get_tenant**](LeadScraperServiceApi.md#get_tenant) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/{organizationId}/{tenantId} | Get tenant details
[**list_organizations**](LeadScraperServiceApi.md#list_organizations) | **GET** /lead-scraper-microservice/api/v1/organization | List all organizations
[**list_tenants**](LeadScraperServiceApi.md#list_tenants) | **GET** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId} | List all tenants
[**update_organization**](LeadScraperServiceApi.md#update_organization) | **PUT** /lead-scraper-microservice/api/v1/organization | Update organization details
[**update_tenant**](LeadScraperServiceApi.md#update_tenant) | **PUT** /lead-scraper-microservice/api/v1/organizations/tenants | Update tenant details


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

