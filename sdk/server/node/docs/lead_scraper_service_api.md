# lead_scraper_service_api

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CreateOrganization**](lead_scraper_service_api.md#CreateOrganization) | **POST** /lead-scraper-microservice/api/v1/organization | Create a new organization
[**CreateTenant**](lead_scraper_service_api.md#CreateTenant) | **POST** /lead-scraper-microservice/api/v1/organizations/{organizationId}/tenants | Create a new tenant
[**DeleteOrganization**](lead_scraper_service_api.md#DeleteOrganization) | **DELETE** /lead-scraper-microservice/api/v1/organization/{id} | Delete an organization
[**DeleteTenant**](lead_scraper_service_api.md#DeleteTenant) | **DELETE** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId}/{tenantId} | Delete a tenant
[**GetOrganization**](lead_scraper_service_api.md#GetOrganization) | **GET** /lead-scraper-microservice/api/v1/organization/{id} | Get organization details
[**GetTenant**](lead_scraper_service_api.md#GetTenant) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/{organizationId}/{tenantId} | Get tenant details
[**ListOrganizations**](lead_scraper_service_api.md#ListOrganizations) | **GET** /lead-scraper-microservice/api/v1/organization | List all organizations
[**ListTenants**](lead_scraper_service_api.md#ListTenants) | **GET** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId} | List all tenants
[**UpdateOrganization**](lead_scraper_service_api.md#UpdateOrganization) | **PUT** /lead-scraper-microservice/api/v1/organization | Update organization details
[**UpdateTenant**](lead_scraper_service_api.md#UpdateTenant) | **PUT** /lead-scraper-microservice/api/v1/organizations/tenants | Update tenant details


<a name="CreateOrganization"></a>
# **CreateOrganization**
> CreateOrganizationResponse CreateOrganization(createOrganizationRequest)

Create a new organization

Creates a new organization within a tenant
<a name="CreateTenant"></a>
# **CreateTenant**
> CreateTenantResponse CreateTenant(organizationId, createTenantBody)

Create a new tenant

Creates a new tenant in the system
<a name="DeleteOrganization"></a>
# **DeleteOrganization**
> DeleteOrganizationResponse DeleteOrganization(Id_)

Delete an organization

Permanently deletes an organization and all associated resources
<a name="DeleteTenant"></a>
# **DeleteTenant**
> DeleteTenantResponse DeleteTenant(organizationId, tenantId)

Delete a tenant

Permanently deletes a tenant and all associated resources
<a name="GetOrganization"></a>
# **GetOrganization**
> GetOrganizationResponse GetOrganization(Id_)

Get organization details

Retrieves details of a specific organization
<a name="GetTenant"></a>
# **GetTenant**
> GetTenantResponse GetTenant(organizationId, tenantId)

Get tenant details

Retrieves details of a specific tenant
<a name="ListOrganizations"></a>
# **ListOrganizations**
> ListOrganizationsResponse ListOrganizations(pageSize, pageNumber)

List all organizations

Retrieves a list of all organizations in a tenant
<a name="ListTenants"></a>
# **ListTenants**
> ListTenantsResponse ListTenants(organizationId, pageSize, pageNumber)

List all tenants

Retrieves a list of all tenants in the system
<a name="UpdateOrganization"></a>
# **UpdateOrganization**
> UpdateOrganizationResponse UpdateOrganization(updateOrganizationRequest)

Update organization details

Updates the configuration of a specific organization
<a name="UpdateTenant"></a>
# **UpdateTenant**
> UpdateTenantResponse UpdateTenant(updateTenantRequest)

Update tenant details

Updates the configuration of a specific tenant
