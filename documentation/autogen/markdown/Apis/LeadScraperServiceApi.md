# LeadScraperServiceApi

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**createOrganization**](LeadScraperServiceApi.md#createOrganization) | **POST** /lead-scraper-microservice/api/v1/organization | Create a new organization |
| [**createTenant**](LeadScraperServiceApi.md#createTenant) | **POST** /lead-scraper-microservice/api/v1/organizations/{organizationId}/tenants | Create a new tenant |
| [**deleteOrganization**](LeadScraperServiceApi.md#deleteOrganization) | **DELETE** /lead-scraper-microservice/api/v1/organization/{id} | Delete an organization |
| [**deleteTenant**](LeadScraperServiceApi.md#deleteTenant) | **DELETE** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId}/{tenantId} | Delete a tenant |
| [**getOrganization**](LeadScraperServiceApi.md#getOrganization) | **GET** /lead-scraper-microservice/api/v1/organization/{id} | Get organization details |
| [**getTenant**](LeadScraperServiceApi.md#getTenant) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/{organizationId}/{tenantId} | Get tenant details |
| [**listOrganizations**](LeadScraperServiceApi.md#listOrganizations) | **GET** /lead-scraper-microservice/api/v1/organization | List all organizations |
| [**listTenants**](LeadScraperServiceApi.md#listTenants) | **GET** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId} | List all tenants |
| [**updateOrganization**](LeadScraperServiceApi.md#updateOrganization) | **PUT** /lead-scraper-microservice/api/v1/organization | Update organization details |
| [**updateTenant**](LeadScraperServiceApi.md#updateTenant) | **PUT** /lead-scraper-microservice/api/v1/organizations/tenants | Update tenant details |


<a name="createOrganization"></a>
# **createOrganization**
> CreateOrganizationResponse createOrganization(CreateOrganizationRequest)

Create a new organization

    Creates a new organization within a tenant

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **CreateOrganizationRequest** | [**CreateOrganizationRequest**](../Models/CreateOrganizationRequest.md)|  | |

### Return type

[**CreateOrganizationResponse**](../Models/CreateOrganizationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, 

<a name="createTenant"></a>
# **createTenant**
> CreateTenantResponse createTenant(organizationId, CreateTenantBody)

Create a new tenant

    Creates a new tenant in the system

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **organizationId** | **String**|  | [default to null] |
| **CreateTenantBody** | [**CreateTenantBody**](../Models/CreateTenantBody.md)|  | |

### Return type

[**CreateTenantResponse**](../Models/CreateTenantResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, 

<a name="deleteOrganization"></a>
# **deleteOrganization**
> DeleteOrganizationResponse deleteOrganization(id)

Delete an organization

    Permanently deletes an organization and all associated resources

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | **String**|  | [default to null] |

### Return type

[**DeleteOrganizationResponse**](../Models/DeleteOrganizationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="deleteTenant"></a>
# **deleteTenant**
> DeleteTenantResponse deleteTenant(organizationId, tenantId)

Delete a tenant

    Permanently deletes a tenant and all associated resources

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **organizationId** | **String**|  | [default to null] |
| **tenantId** | **String**|  | [default to null] |

### Return type

[**DeleteTenantResponse**](../Models/DeleteTenantResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="getOrganization"></a>
# **getOrganization**
> GetOrganizationResponse getOrganization(id)

Get organization details

    Retrieves details of a specific organization

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | **String**|  | [default to null] |

### Return type

[**GetOrganizationResponse**](../Models/GetOrganizationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="getTenant"></a>
# **getTenant**
> GetTenantResponse getTenant(organizationId, tenantId)

Get tenant details

    Retrieves details of a specific tenant

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **organizationId** | **String**|  | [default to null] |
| **tenantId** | **String**|  | [default to null] |

### Return type

[**GetTenantResponse**](../Models/GetTenantResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="listOrganizations"></a>
# **listOrganizations**
> ListOrganizationsResponse listOrganizations(pageSize, pageNumber)

List all organizations

    Retrieves a list of all organizations in a tenant

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **pageSize** | **Integer**|  | [optional] [default to null] |
| **pageNumber** | **Integer**|  | [optional] [default to null] |

### Return type

[**ListOrganizationsResponse**](../Models/ListOrganizationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="listTenants"></a>
# **listTenants**
> ListTenantsResponse listTenants(organizationId, pageSize, pageNumber)

List all tenants

    Retrieves a list of all tenants in the system

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **organizationId** | **String**|  | [default to null] |
| **pageSize** | **Integer**|  | [optional] [default to null] |
| **pageNumber** | **Integer**|  | [optional] [default to null] |

### Return type

[**ListTenantsResponse**](../Models/ListTenantsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="updateOrganization"></a>
# **updateOrganization**
> UpdateOrganizationResponse updateOrganization(UpdateOrganizationRequest)

Update organization details

    Updates the configuration of a specific organization

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **UpdateOrganizationRequest** | [**UpdateOrganizationRequest**](../Models/UpdateOrganizationRequest.md)|  | |

### Return type

[**UpdateOrganizationResponse**](../Models/UpdateOrganizationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, 

<a name="updateTenant"></a>
# **updateTenant**
> UpdateTenantResponse updateTenant(UpdateTenantRequest)

Update tenant details

    Updates the configuration of a specific tenant

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **UpdateTenantRequest** | [**UpdateTenantRequest**](../Models/UpdateTenantRequest.md)|  | |

### Return type

[**UpdateTenantResponse**](../Models/UpdateTenantResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, 

