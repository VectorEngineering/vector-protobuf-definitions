# lead_scraper_service_api

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CreateAPIKey**](lead_scraper_service_api.md#CreateAPIKey) | **POST** /lead-scraper-microservice/api/v1/api-keys | Create a new API key
[**CreateOrganization**](lead_scraper_service_api.md#CreateOrganization) | **POST** /lead-scraper-microservice/api/v1/organization | Create a new organization
[**CreateTenant**](lead_scraper_service_api.md#CreateTenant) | **POST** /lead-scraper-microservice/api/v1/organizations/{organizationId}/tenants | Create a new tenant
[**CreateTenantAPIKey**](lead_scraper_service_api.md#CreateTenantAPIKey) | **POST** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys | Create a new tenant API key
[**CreateWebhook**](lead_scraper_service_api.md#CreateWebhook) | **POST** /lead-scraper-microservice/api/v1/webhooks | Create webhook
[**DeleteAPIKey**](lead_scraper_service_api.md#DeleteAPIKey) | **DELETE** /lead-scraper-microservice/api/v1/api-keys/{keyId} | Delete API key
[**DeleteOrganization**](lead_scraper_service_api.md#DeleteOrganization) | **DELETE** /lead-scraper-microservice/api/v1/organization/{id} | Delete an organization
[**DeleteTenant**](lead_scraper_service_api.md#DeleteTenant) | **DELETE** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId}/{tenantId} | Delete a tenant
[**DeleteTenantAPIKey**](lead_scraper_service_api.md#DeleteTenantAPIKey) | **DELETE** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/{keyId} | Delete tenant API key
[**DeleteWebhook**](lead_scraper_service_api.md#DeleteWebhook) | **DELETE** /lead-scraper-microservice/api/v1/webhooks/{webhookId} | Delete webhook
[**DeleteWorkspace**](lead_scraper_service_api.md#DeleteWorkspace) | **DELETE** /lead-scraper-microservice/api/v1/workspace/{id} | Delete a workspace
[**GetAPIKey**](lead_scraper_service_api.md#GetAPIKey) | **GET** /lead-scraper-microservice/api/v1/api-keys/{keyId} | Get API key details
[**GetLead**](lead_scraper_service_api.md#GetLead) | **GET** /lead-scraper-microservice/api/v1/leads/{leadId} | Get lead details
[**GetOrganization**](lead_scraper_service_api.md#GetOrganization) | **GET** /lead-scraper-microservice/api/v1/organization/{id} | Get organization details
[**GetTenant**](lead_scraper_service_api.md#GetTenant) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/{organizationId}/{tenantId} | Get tenant details
[**GetTenantAPIKey**](lead_scraper_service_api.md#GetTenantAPIKey) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/{keyId} | Get tenant API key details
[**GetWebhook**](lead_scraper_service_api.md#GetWebhook) | **GET** /lead-scraper-microservice/api/v1/webhooks/{webhookId} | Get webhook
[**GetWorkspace**](lead_scraper_service_api.md#GetWorkspace) | **GET** /lead-scraper-microservice/api/v1/workspace/{id} | Get workspace details
[**GetWorkspaceAnalytics**](lead_scraper_service_api.md#GetWorkspaceAnalytics) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/analytics | Get workspace analytics
[**ListAPIKeys**](lead_scraper_service_api.md#ListAPIKeys) | **GET** /lead-scraper-microservice/api/v1/api-keys/list | List API keys
[**ListLeads**](lead_scraper_service_api.md#ListLeads) | **GET** /lead-scraper-microservice/api/v1/leads | List leads
[**ListOrganizations**](lead_scraper_service_api.md#ListOrganizations) | **GET** /lead-scraper-microservice/api/v1/organization | List all organizations
[**ListTenantAPIKeys**](lead_scraper_service_api.md#ListTenantAPIKeys) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/list | List tenant API keys
[**ListTenants**](lead_scraper_service_api.md#ListTenants) | **GET** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId} | List all tenants
[**ListWebhooks**](lead_scraper_service_api.md#ListWebhooks) | **GET** /lead-scraper-microservice/api/v1/webhooks | List webhooks
[**RotateAPIKey**](lead_scraper_service_api.md#RotateAPIKey) | **POST** /lead-scraper-microservice/api/v1/api-keys/rotate | Rotate API key
[**RotateTenantAPIKey**](lead_scraper_service_api.md#RotateTenantAPIKey) | **POST** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/rotate | Rotate tenant API key
[**UpdateAPIKey**](lead_scraper_service_api.md#UpdateAPIKey) | **PUT** /lead-scraper-microservice/api/v1/api-keys | Update API key
[**UpdateOrganization**](lead_scraper_service_api.md#UpdateOrganization) | **PUT** /lead-scraper-microservice/api/v1/organization | Update organization details
[**UpdateTenant**](lead_scraper_service_api.md#UpdateTenant) | **PUT** /lead-scraper-microservice/api/v1/organizations/tenants | Update tenant details
[**UpdateTenantAPIKey**](lead_scraper_service_api.md#UpdateTenantAPIKey) | **PUT** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys | Update tenant API key
[**UpdateWebhook**](lead_scraper_service_api.md#UpdateWebhook) | **PUT** /lead-scraper-microservice/api/v1/webhooks | Update webhook
[**UpdateWorkspace**](lead_scraper_service_api.md#UpdateWorkspace) | **PUT** /lead-scraper-microservice/api/v1/workspace | Update workspace details


<a name="CreateAPIKey"></a>
# **CreateAPIKey**
> CreateApiKeyResponse CreateAPIKey(createApiKeyRequest)

Create a new API key

Creates a new API key with specified permissions and settings
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
<a name="CreateTenantAPIKey"></a>
# **CreateTenantAPIKey**
> CreateTenantApiKeyResponse CreateTenantAPIKey(createTenantApiKeyRequest)

Create a new tenant API key

Creates a new API key for a tenant with specified permissions
<a name="CreateWebhook"></a>
# **CreateWebhook**
> CreateWebhookResponse CreateWebhook(createWebhookRequest)

Create webhook

Creates a new webhook configuration for event notifications
<a name="DeleteAPIKey"></a>
# **DeleteAPIKey**
> DeleteApiKeyResponse DeleteAPIKey(keyId, organizationId, workspaceId, tenantId, accountId)

Delete API key

Permanently deletes an API key
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
<a name="DeleteTenantAPIKey"></a>
# **DeleteTenantAPIKey**
> DeleteTenantApiKeyResponse DeleteTenantAPIKey(keyId, organizationId, tenantId)

Delete tenant API key

Permanently deletes a tenant API key
<a name="DeleteWebhook"></a>
# **DeleteWebhook**
> DeleteWebhookResponse DeleteWebhook(webhookId, organizationId, workspaceId, tenantId, accountId)

Delete webhook

Deletes a webhook configuration
<a name="DeleteWorkspace"></a>
# **DeleteWorkspace**
> DeleteWorkspaceResponse DeleteWorkspace(Id_)

Delete a workspace

Deletes a specific workspace
<a name="GetAPIKey"></a>
# **GetAPIKey**
> GetApiKeyResponse GetAPIKey(keyId, organizationId, tenantId, accountId, workspaceId)

Get API key details

Retrieves details of a specific API key
<a name="GetLead"></a>
# **GetLead**
> GetLeadResponse GetLead(leadId, organizationId, workspaceId, tenantId, accountId)

Get lead details

Retrieves detailed information about a specific lead
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
<a name="GetTenantAPIKey"></a>
# **GetTenantAPIKey**
> GetTenantApiKeyResponse GetTenantAPIKey(keyId, organizationId, tenantId)

Get tenant API key details

Retrieves details of a specific tenant API key
<a name="GetWebhook"></a>
# **GetWebhook**
> GetWebhookResponse GetWebhook(webhookId, organizationId, workspaceId, tenantId, accountId)

Get webhook

Retrieves details of a specific webhook configuration
<a name="GetWorkspace"></a>
# **GetWorkspace**
> GetWorkspaceResponse GetWorkspace(Id_, organizationId, tenantId, accountId)

Get workspace details

Retrieves details of a specific workspace
<a name="GetWorkspaceAnalytics"></a>
# **GetWorkspaceAnalytics**
> GetWorkspaceAnalyticsResponse GetWorkspaceAnalytics(workspaceId, startTime, endTime)

Get workspace analytics

Retrieves analytics data for a specific workspace
<a name="ListAPIKeys"></a>
# **ListAPIKeys**
> ListApiKeysResponse ListAPIKeys(organizationId, tenantId, accountId, workspaceId, pageSize, pageNumber, status, search)

List API keys

Retrieves a list of API keys with optional filtering
<a name="ListLeads"></a>
# **ListLeads**
> ListLeadsResponse ListLeads(organizationId, workspaceId, tenantId, accountId, pageSize, pageNumber)

List leads

Retrieves a paginated list of leads with comprehensive filtering options
<a name="ListOrganizations"></a>
# **ListOrganizations**
> ListOrganizationsResponse ListOrganizations(pageSize, pageNumber)

List all organizations

Retrieves a list of all organizations in a tenant
<a name="ListTenantAPIKeys"></a>
# **ListTenantAPIKeys**
> ListTenantApiKeysResponse ListTenantAPIKeys(organizationId, tenantId, pageSize, pageNumber, status)

List tenant API keys

Retrieves all API keys for a specific tenant
<a name="ListTenants"></a>
# **ListTenants**
> ListTenantsResponse ListTenants(organizationId, pageSize, pageNumber)

List all tenants

Retrieves a list of all tenants in the system
<a name="ListWebhooks"></a>
# **ListWebhooks**
> ListWebhooksResponse ListWebhooks(organizationId, workspaceId, tenantId, accountId, pageSize, pageNumber, status, search)

List webhooks

Lists all webhook configurations with pagination
<a name="RotateAPIKey"></a>
# **RotateAPIKey**
> RotateApiKeyResponse RotateAPIKey(rotateApiKeyRequest)

Rotate API key

Rotates an API key while maintaining its configuration
<a name="RotateTenantAPIKey"></a>
# **RotateTenantAPIKey**
> RotateTenantApiKeyResponse RotateTenantAPIKey(rotateTenantApiKeyRequest)

Rotate tenant API key

Rotates a tenant API key while maintaining its configuration
<a name="UpdateAPIKey"></a>
# **UpdateAPIKey**
> UpdateApiKeyResponse UpdateAPIKey(updateApiKeyRequest)

Update API key

Updates an existing API key&#39;s configuration
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
<a name="UpdateTenantAPIKey"></a>
# **UpdateTenantAPIKey**
> UpdateTenantApiKeyResponse UpdateTenantAPIKey(updateTenantApiKeyRequest)

Update tenant API key

Updates an existing tenant API key&#39;s configuration
<a name="UpdateWebhook"></a>
# **UpdateWebhook**
> UpdateWebhookResponse UpdateWebhook(updateWebhookRequest)

Update webhook

Updates an existing webhook configuration
<a name="UpdateWorkspace"></a>
# **UpdateWorkspace**
> UpdateWorkspaceResponse UpdateWorkspace(updateWorkspaceRequest)

Update workspace details

Updates the details of a specific workspace
