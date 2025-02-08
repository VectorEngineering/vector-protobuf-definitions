# LeadScraperServiceApi

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**createAPIKey**](LeadScraperServiceApi.md#createAPIKey) | **POST** /lead-scraper-microservice/api/v1/api-keys | Create a new API key |
| [**createOrganization**](LeadScraperServiceApi.md#createOrganization) | **POST** /lead-scraper-microservice/api/v1/organization | Create a new organization |
| [**createTenant**](LeadScraperServiceApi.md#createTenant) | **POST** /lead-scraper-microservice/api/v1/organizations/{organizationId}/tenants | Create a new tenant |
| [**createTenantAPIKey**](LeadScraperServiceApi.md#createTenantAPIKey) | **POST** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys | Create a new tenant API key |
| [**createWebhook**](LeadScraperServiceApi.md#createWebhook) | **POST** /lead-scraper-microservice/api/v1/webhooks | Create webhook |
| [**deleteAPIKey**](LeadScraperServiceApi.md#deleteAPIKey) | **DELETE** /lead-scraper-microservice/api/v1/api-keys/{keyId} | Delete API key |
| [**deleteOrganization**](LeadScraperServiceApi.md#deleteOrganization) | **DELETE** /lead-scraper-microservice/api/v1/organization/{id} | Delete an organization |
| [**deleteTenant**](LeadScraperServiceApi.md#deleteTenant) | **DELETE** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId}/{tenantId} | Delete a tenant |
| [**deleteTenantAPIKey**](LeadScraperServiceApi.md#deleteTenantAPIKey) | **DELETE** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/{keyId} | Delete tenant API key |
| [**deleteWebhook**](LeadScraperServiceApi.md#deleteWebhook) | **DELETE** /lead-scraper-microservice/api/v1/webhooks/{webhookId} | Delete webhook |
| [**getAPIKey**](LeadScraperServiceApi.md#getAPIKey) | **GET** /lead-scraper-microservice/api/v1/api-keys/{keyId} | Get API key details |
| [**getLead**](LeadScraperServiceApi.md#getLead) | **GET** /lead-scraper-microservice/api/v1/leads/{leadId} | Get lead details |
| [**getOrganization**](LeadScraperServiceApi.md#getOrganization) | **GET** /lead-scraper-microservice/api/v1/organization/{id} | Get organization details |
| [**getTenant**](LeadScraperServiceApi.md#getTenant) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/{organizationId}/{tenantId} | Get tenant details |
| [**getTenantAPIKey**](LeadScraperServiceApi.md#getTenantAPIKey) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/{keyId} | Get tenant API key details |
| [**getWebhook**](LeadScraperServiceApi.md#getWebhook) | **GET** /lead-scraper-microservice/api/v1/webhooks/{webhookId} | Get webhook |
| [**listAPIKeys**](LeadScraperServiceApi.md#listAPIKeys) | **GET** /lead-scraper-microservice/api/v1/api-keys/list | List API keys |
| [**listLeads**](LeadScraperServiceApi.md#listLeads) | **GET** /lead-scraper-microservice/api/v1/leads | List leads |
| [**listOrganizations**](LeadScraperServiceApi.md#listOrganizations) | **GET** /lead-scraper-microservice/api/v1/organization | List all organizations |
| [**listTenantAPIKeys**](LeadScraperServiceApi.md#listTenantAPIKeys) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/list | List tenant API keys |
| [**listTenants**](LeadScraperServiceApi.md#listTenants) | **GET** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId} | List all tenants |
| [**listWebhooks**](LeadScraperServiceApi.md#listWebhooks) | **GET** /lead-scraper-microservice/api/v1/webhooks | List webhooks |
| [**rotateAPIKey**](LeadScraperServiceApi.md#rotateAPIKey) | **POST** /lead-scraper-microservice/api/v1/api-keys/rotate | Rotate API key |
| [**rotateTenantAPIKey**](LeadScraperServiceApi.md#rotateTenantAPIKey) | **POST** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/rotate | Rotate tenant API key |
| [**updateAPIKey**](LeadScraperServiceApi.md#updateAPIKey) | **PUT** /lead-scraper-microservice/api/v1/api-keys | Update API key |
| [**updateOrganization**](LeadScraperServiceApi.md#updateOrganization) | **PUT** /lead-scraper-microservice/api/v1/organization | Update organization details |
| [**updateTenant**](LeadScraperServiceApi.md#updateTenant) | **PUT** /lead-scraper-microservice/api/v1/organizations/tenants | Update tenant details |
| [**updateTenantAPIKey**](LeadScraperServiceApi.md#updateTenantAPIKey) | **PUT** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys | Update tenant API key |
| [**updateWebhook**](LeadScraperServiceApi.md#updateWebhook) | **PUT** /lead-scraper-microservice/api/v1/webhooks | Update webhook |


<a name="createAPIKey"></a>
# **createAPIKey**
> CreateAPIKeyResponse createAPIKey(CreateAPIKeyRequest)

Create a new API key

    Creates a new API key with specified permissions and settings

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **CreateAPIKeyRequest** | [**CreateAPIKeyRequest**](../Models/CreateAPIKeyRequest.md)|  | |

### Return type

[**CreateAPIKeyResponse**](../Models/CreateAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, 

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

<a name="createTenantAPIKey"></a>
# **createTenantAPIKey**
> CreateTenantAPIKeyResponse createTenantAPIKey(CreateTenantAPIKeyRequest)

Create a new tenant API key

    Creates a new API key for a tenant with specified permissions

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **CreateTenantAPIKeyRequest** | [**CreateTenantAPIKeyRequest**](../Models/CreateTenantAPIKeyRequest.md)|  | |

### Return type

[**CreateTenantAPIKeyResponse**](../Models/CreateTenantAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, 

<a name="createWebhook"></a>
# **createWebhook**
> CreateWebhookResponse createWebhook(CreateWebhookRequest)

Create webhook

    Creates a new webhook configuration for event notifications

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **CreateWebhookRequest** | [**CreateWebhookRequest**](../Models/CreateWebhookRequest.md)|  | |

### Return type

[**CreateWebhookResponse**](../Models/CreateWebhookResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, 

<a name="deleteAPIKey"></a>
# **deleteAPIKey**
> DeleteAPIKeyResponse deleteAPIKey(keyId, organizationId, workspaceId, tenantId, accountId)

Delete API key

    Permanently deletes an API key

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **keyId** | **String**|  | [default to null] |
| **organizationId** | **String**|  | [optional] [default to null] |
| **workspaceId** | **String**|  | [optional] [default to null] |
| **tenantId** | **String**|  | [optional] [default to null] |
| **accountId** | **String**|  | [optional] [default to null] |

### Return type

[**DeleteAPIKeyResponse**](../Models/DeleteAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
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

<a name="deleteTenantAPIKey"></a>
# **deleteTenantAPIKey**
> DeleteTenantAPIKeyResponse deleteTenantAPIKey(keyId, organizationId, tenantId)

Delete tenant API key

    Permanently deletes a tenant API key

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **keyId** | **String**|  | [default to null] |
| **organizationId** | **String**|  | [optional] [default to null] |
| **tenantId** | **String**|  | [optional] [default to null] |

### Return type

[**DeleteTenantAPIKeyResponse**](../Models/DeleteTenantAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="deleteWebhook"></a>
# **deleteWebhook**
> DeleteWebhookResponse deleteWebhook(webhookId, organizationId, workspaceId, tenantId, accountId)

Delete webhook

    Deletes a webhook configuration

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **webhookId** | **String**|  | [default to null] |
| **organizationId** | **String**|  | [optional] [default to null] |
| **workspaceId** | **String**|  | [optional] [default to null] |
| **tenantId** | **String**|  | [optional] [default to null] |
| **accountId** | **String**|  | [optional] [default to null] |

### Return type

[**DeleteWebhookResponse**](../Models/DeleteWebhookResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="getAPIKey"></a>
# **getAPIKey**
> GetAPIKeyResponse getAPIKey(keyId, organizationId, tenantId, accountId, workspaceId)

Get API key details

    Retrieves details of a specific API key

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **keyId** | **String**|  | [default to null] |
| **organizationId** | **String**|  | [optional] [default to null] |
| **tenantId** | **String**|  | [optional] [default to null] |
| **accountId** | **String**|  | [optional] [default to null] |
| **workspaceId** | **String**|  | [optional] [default to null] |

### Return type

[**GetAPIKeyResponse**](../Models/GetAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="getLead"></a>
# **getLead**
> GetLeadResponse getLead(leadId, organizationId, workspaceId, tenantId, accountId)

Get lead details

    Retrieves detailed information about a specific lead

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **leadId** | **String**|  | [default to null] |
| **organizationId** | **String**|  | [optional] [default to null] |
| **workspaceId** | **String**|  | [optional] [default to null] |
| **tenantId** | **String**|  | [optional] [default to null] |
| **accountId** | **String**|  | [optional] [default to null] |

### Return type

[**GetLeadResponse**](../Models/GetLeadResponse.md)

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

<a name="getTenantAPIKey"></a>
# **getTenantAPIKey**
> GetTenantAPIKeyResponse getTenantAPIKey(keyId, organizationId, tenantId)

Get tenant API key details

    Retrieves details of a specific tenant API key

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **keyId** | **String**|  | [default to null] |
| **organizationId** | **String**|  | [optional] [default to null] |
| **tenantId** | **String**|  | [optional] [default to null] |

### Return type

[**GetTenantAPIKeyResponse**](../Models/GetTenantAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="getWebhook"></a>
# **getWebhook**
> GetWebhookResponse getWebhook(webhookId, organizationId, workspaceId, tenantId, accountId)

Get webhook

    Retrieves details of a specific webhook configuration

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **webhookId** | **String**|  | [default to null] |
| **organizationId** | **String**|  | [optional] [default to null] |
| **workspaceId** | **String**|  | [optional] [default to null] |
| **tenantId** | **String**|  | [optional] [default to null] |
| **accountId** | **String**|  | [optional] [default to null] |

### Return type

[**GetWebhookResponse**](../Models/GetWebhookResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="listAPIKeys"></a>
# **listAPIKeys**
> ListAPIKeysResponse listAPIKeys(organizationId, tenantId, accountId, workspaceId, pageSize, pageNumber, status, search)

List API keys

    Retrieves a list of API keys with optional filtering

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **organizationId** | **String**|  | [optional] [default to null] |
| **tenantId** | **String**|  | [optional] [default to null] |
| **accountId** | **String**|  | [optional] [default to null] |
| **workspaceId** | **String**|  | [optional] [default to null] |
| **pageSize** | **Integer**|  | [optional] [default to null] |
| **pageNumber** | **Integer**|  | [optional] [default to null] |
| **status** | **String**|  | [optional] [default to null] |
| **search** | **String**|  | [optional] [default to null] |

### Return type

[**ListAPIKeysResponse**](../Models/ListAPIKeysResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="listLeads"></a>
# **listLeads**
> ListLeadsResponse listLeads(organizationId, workspaceId, tenantId, accountId, pageSize, pageNumber)

List leads

    Retrieves a paginated list of leads with comprehensive filtering options

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **organizationId** | **String**| Context identifiers | [optional] [default to null] |
| **workspaceId** | **String**|  | [optional] [default to null] |
| **tenantId** | **String**|  | [optional] [default to null] |
| **accountId** | **String**|  | [optional] [default to null] |
| **pageSize** | **Integer**| Pagination | [optional] [default to null] |
| **pageNumber** | **Integer**|  | [optional] [default to null] |

### Return type

[**ListLeadsResponse**](../Models/ListLeadsResponse.md)

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

<a name="listTenantAPIKeys"></a>
# **listTenantAPIKeys**
> ListTenantAPIKeysResponse listTenantAPIKeys(organizationId, tenantId, pageSize, pageNumber, status)

List tenant API keys

    Retrieves all API keys for a specific tenant

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **organizationId** | **String**|  | [optional] [default to null] |
| **tenantId** | **String**|  | [optional] [default to null] |
| **pageSize** | **Integer**|  | [optional] [default to null] |
| **pageNumber** | **Integer**|  | [optional] [default to null] |
| **status** | **String**|  | [optional] [default to null] |

### Return type

[**ListTenantAPIKeysResponse**](../Models/ListTenantAPIKeysResponse.md)

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

<a name="listWebhooks"></a>
# **listWebhooks**
> ListWebhooksResponse listWebhooks(organizationId, workspaceId, tenantId, accountId, pageSize, pageNumber, status, search)

List webhooks

    Lists all webhook configurations with pagination

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **organizationId** | **String**|  | [optional] [default to null] |
| **workspaceId** | **String**|  | [optional] [default to null] |
| **tenantId** | **String**|  | [optional] [default to null] |
| **accountId** | **String**|  | [optional] [default to null] |
| **pageSize** | **Integer**|  | [optional] [default to null] |
| **pageNumber** | **Integer**|  | [optional] [default to null] |
| **status** | **String**|  | [optional] [default to null] |
| **search** | **String**|  | [optional] [default to null] |

### Return type

[**ListWebhooksResponse**](../Models/ListWebhooksResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="rotateAPIKey"></a>
# **rotateAPIKey**
> RotateAPIKeyResponse rotateAPIKey(RotateAPIKeyRequest)

Rotate API key

    Rotates an API key while maintaining its configuration

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **RotateAPIKeyRequest** | [**RotateAPIKeyRequest**](../Models/RotateAPIKeyRequest.md)|  | |

### Return type

[**RotateAPIKeyResponse**](../Models/RotateAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, 

<a name="rotateTenantAPIKey"></a>
# **rotateTenantAPIKey**
> RotateTenantAPIKeyResponse rotateTenantAPIKey(RotateTenantAPIKeyRequest)

Rotate tenant API key

    Rotates a tenant API key while maintaining its configuration

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **RotateTenantAPIKeyRequest** | [**RotateTenantAPIKeyRequest**](../Models/RotateTenantAPIKeyRequest.md)|  | |

### Return type

[**RotateTenantAPIKeyResponse**](../Models/RotateTenantAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, 

<a name="updateAPIKey"></a>
# **updateAPIKey**
> UpdateAPIKeyResponse updateAPIKey(UpdateAPIKeyRequest)

Update API key

    Updates an existing API key&#39;s configuration

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **UpdateAPIKeyRequest** | [**UpdateAPIKeyRequest**](../Models/UpdateAPIKeyRequest.md)|  | |

### Return type

[**UpdateAPIKeyResponse**](../Models/UpdateAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
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

<a name="updateTenantAPIKey"></a>
# **updateTenantAPIKey**
> UpdateTenantAPIKeyResponse updateTenantAPIKey(UpdateTenantAPIKeyRequest)

Update tenant API key

    Updates an existing tenant API key&#39;s configuration

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **UpdateTenantAPIKeyRequest** | [**UpdateTenantAPIKeyRequest**](../Models/UpdateTenantAPIKeyRequest.md)|  | |

### Return type

[**UpdateTenantAPIKeyResponse**](../Models/UpdateTenantAPIKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, 

<a name="updateWebhook"></a>
# **updateWebhook**
> UpdateWebhookResponse updateWebhook(UpdateWebhookRequest)

Update webhook

    Updates an existing webhook configuration

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **UpdateWebhookRequest** | [**UpdateWebhookRequest**](../Models/UpdateWebhookRequest.md)|  | |

### Return type

[**UpdateWebhookResponse**](../Models/UpdateWebhookResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, 

