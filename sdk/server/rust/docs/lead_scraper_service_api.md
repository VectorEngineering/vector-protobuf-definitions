# lead_scraper_service_api

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

Method | HTTP request | Description
------------- | ------------- | -------------
**CreateOrganization**](lead_scraper_service_api.md#CreateOrganization) | **POST** /lead-scraper-microservice/api/v1/organization | Create a new organization
**CreateTenant**](lead_scraper_service_api.md#CreateTenant) | **POST** /lead-scraper-microservice/api/v1/organizations/{organizationId}/tenants | Create a new tenant
**DeleteOrganization**](lead_scraper_service_api.md#DeleteOrganization) | **DELETE** /lead-scraper-microservice/api/v1/organization/{id} | Delete an organization
**DeleteTenant**](lead_scraper_service_api.md#DeleteTenant) | **DELETE** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId}/{tenantId} | Delete a tenant
**GetOrganization**](lead_scraper_service_api.md#GetOrganization) | **GET** /lead-scraper-microservice/api/v1/organization/{id} | Get organization details
**GetTenant**](lead_scraper_service_api.md#GetTenant) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/{organizationId}/{tenantId} | Get tenant details
**ListOrganizations**](lead_scraper_service_api.md#ListOrganizations) | **GET** /lead-scraper-microservice/api/v1/organization | List all organizations
**ListTenants**](lead_scraper_service_api.md#ListTenants) | **GET** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId} | List all tenants
**UpdateOrganization**](lead_scraper_service_api.md#UpdateOrganization) | **PUT** /lead-scraper-microservice/api/v1/organization | Update organization details
**UpdateTenant**](lead_scraper_service_api.md#UpdateTenant) | **PUT** /lead-scraper-microservice/api/v1/organizations/tenants | Update tenant details


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

