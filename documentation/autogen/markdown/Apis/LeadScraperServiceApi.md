# LeadScraperServiceApi

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**createAPIKey**](LeadScraperServiceApi.md#createAPIKey) | **POST** /lead-scraper-microservice/api/v1/api-keys | Create a new API key |
| [**createAccount**](LeadScraperServiceApi.md#createAccount) | **POST** /lead-scraper-microservice/api/v1/accounts | Create a new account |
| [**createOrganization**](LeadScraperServiceApi.md#createOrganization) | **POST** /lead-scraper-microservice/api/v1/organization | Create a new organization |
| [**createScrapingJob**](LeadScraperServiceApi.md#createScrapingJob) | **POST** /lead-scraper-microservice/api/v1/jobs | Create a new job scraping task |
| [**createTenant**](LeadScraperServiceApi.md#createTenant) | **POST** /lead-scraper-microservice/api/v1/organizations/{organizationId}/tenants | Create a new tenant |
| [**createTenantAPIKey**](LeadScraperServiceApi.md#createTenantAPIKey) | **POST** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys | Create a new tenant API key |
| [**createWebhook**](LeadScraperServiceApi.md#createWebhook) | **POST** /lead-scraper-microservice/api/v1/webhooks | Create webhook |
| [**createWorkspace**](LeadScraperServiceApi.md#createWorkspace) | **POST** /lead-scraper-microservice/api/v1/workspaces | Create a new workspace |
| [**deleteAPIKey**](LeadScraperServiceApi.md#deleteAPIKey) | **DELETE** /lead-scraper-microservice/api/v1/api-keys/{keyId} | Delete API key |
| [**deleteAccount**](LeadScraperServiceApi.md#deleteAccount) | **DELETE** /lead-scraper-microservice/api/v1/accounts/{id} | Delete account |
| [**deleteOrganization**](LeadScraperServiceApi.md#deleteOrganization) | **DELETE** /lead-scraper-microservice/api/v1/organization/{id} | Delete an organization |
| [**deleteScrapingJob**](LeadScraperServiceApi.md#deleteScrapingJob) | **DELETE** /lead-scraper-microservice/api/v1/jobs/{jobId} | Delete a specific job |
| [**deleteTenant**](LeadScraperServiceApi.md#deleteTenant) | **DELETE** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId}/{tenantId} | Delete a tenant |
| [**deleteTenantAPIKey**](LeadScraperServiceApi.md#deleteTenantAPIKey) | **DELETE** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/{keyId} | Delete tenant API key |
| [**deleteWebhook**](LeadScraperServiceApi.md#deleteWebhook) | **DELETE** /lead-scraper-microservice/api/v1/webhooks/{webhookId} | Delete webhook |
| [**deleteWorkflow**](LeadScraperServiceApi.md#deleteWorkflow) | **DELETE** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id} | Delete workflow |
| [**deleteWorkspace**](LeadScraperServiceApi.md#deleteWorkspace) | **DELETE** /lead-scraper-microservice/api/v1/workspace/{id} | Delete a workspace |
| [**downloadScrapingResults**](LeadScraperServiceApi.md#downloadScrapingResults) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId}/download | Download job results as CSV |
| [**getAPIKey**](LeadScraperServiceApi.md#getAPIKey) | **GET** /lead-scraper-microservice/api/v1/api-keys/{keyId} | Get API key details |
| [**getAccount**](LeadScraperServiceApi.md#getAccount) | **GET** /lead-scraper-microservice/api/v1/accounts/{id} | Get account details |
| [**getAccountUsage**](LeadScraperServiceApi.md#getAccountUsage) | **GET** /lead-scraper-microservice/api/v1/accounts/{id}/usage | Get account usage |
| [**getLead**](LeadScraperServiceApi.md#getLead) | **GET** /lead-scraper-microservice/api/v1/leads/{leadId} | Get lead details |
| [**getOrganization**](LeadScraperServiceApi.md#getOrganization) | **GET** /lead-scraper-microservice/api/v1/organization/{id} | Get organization details |
| [**getScrapingJob**](LeadScraperServiceApi.md#getScrapingJob) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId} | Get a specific job |
| [**getTenant**](LeadScraperServiceApi.md#getTenant) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/{organizationId}/{tenantId} | Get tenant details |
| [**getTenantAPIKey**](LeadScraperServiceApi.md#getTenantAPIKey) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/{keyId} | Get tenant API key details |
| [**getWebhook**](LeadScraperServiceApi.md#getWebhook) | **GET** /lead-scraper-microservice/api/v1/webhooks/{webhookId} | Get webhook |
| [**getWorkflow**](LeadScraperServiceApi.md#getWorkflow) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id} | Get workflow details |
| [**getWorkspace**](LeadScraperServiceApi.md#getWorkspace) | **GET** /lead-scraper-microservice/api/v1/workspace/{id} | Get workspace details |
| [**getWorkspaceAnalytics**](LeadScraperServiceApi.md#getWorkspaceAnalytics) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/analytics | Get workspace analytics |
| [**leadScraperServiceCreateWorkflow**](LeadScraperServiceApi.md#leadScraperServiceCreateWorkflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows | Create a new workflow |
| [**listAPIKeys**](LeadScraperServiceApi.md#listAPIKeys) | **GET** /lead-scraper-microservice/api/v1/api-keys/list | List API keys |
| [**listAccounts**](LeadScraperServiceApi.md#listAccounts) | **GET** /lead-scraper-microservice/api/v1/accounts | List all accounts |
| [**listLeads**](LeadScraperServiceApi.md#listLeads) | **GET** /lead-scraper-microservice/api/v1/leads | List leads |
| [**listOrganizations**](LeadScraperServiceApi.md#listOrganizations) | **GET** /lead-scraper-microservice/api/v1/organization | List all organizations |
| [**listScrapingJobs**](LeadScraperServiceApi.md#listScrapingJobs) | **GET** /lead-scraper-microservice/api/v1/jobs | Get all jobs |
| [**listTenantAPIKeys**](LeadScraperServiceApi.md#listTenantAPIKeys) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/list | List tenant API keys |
| [**listTenants**](LeadScraperServiceApi.md#listTenants) | **GET** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId} | List all tenants |
| [**listWebhooks**](LeadScraperServiceApi.md#listWebhooks) | **GET** /lead-scraper-microservice/api/v1/webhooks | List webhooks |
| [**listWorkflows**](LeadScraperServiceApi.md#listWorkflows) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows | List workflows |
| [**listWorkspaces**](LeadScraperServiceApi.md#listWorkspaces) | **GET** /lead-scraper-microservice/api/v1/workspaces | List workspaces |
| [**pauseWorkflow**](LeadScraperServiceApi.md#pauseWorkflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id}/pause | Pause workflow execution |
| [**rotateAPIKey**](LeadScraperServiceApi.md#rotateAPIKey) | **POST** /lead-scraper-microservice/api/v1/api-keys/rotate | Rotate API key |
| [**rotateTenantAPIKey**](LeadScraperServiceApi.md#rotateTenantAPIKey) | **POST** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/rotate | Rotate tenant API key |
| [**triggerWorkflow**](LeadScraperServiceApi.md#triggerWorkflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id}/trigger | Trigger workflow execution |
| [**updateAPIKey**](LeadScraperServiceApi.md#updateAPIKey) | **PUT** /lead-scraper-microservice/api/v1/api-keys | Update API key |
| [**updateAccount**](LeadScraperServiceApi.md#updateAccount) | **PUT** /lead-scraper-microservice/api/v1/accounts | Update account details |
| [**updateAccountSettings**](LeadScraperServiceApi.md#updateAccountSettings) | **PUT** /lead-scraper-microservice/api/v1/accounts/settings | Update account settings |
| [**updateOrganization**](LeadScraperServiceApi.md#updateOrganization) | **PUT** /lead-scraper-microservice/api/v1/organization | Update organization details |
| [**updateTenant**](LeadScraperServiceApi.md#updateTenant) | **PUT** /lead-scraper-microservice/api/v1/organizations/tenants | Update tenant details |
| [**updateTenantAPIKey**](LeadScraperServiceApi.md#updateTenantAPIKey) | **PUT** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys | Update tenant API key |
| [**updateWebhook**](LeadScraperServiceApi.md#updateWebhook) | **PUT** /lead-scraper-microservice/api/v1/webhooks | Update webhook |
| [**updateWorkflow**](LeadScraperServiceApi.md#updateWorkflow) | **PUT** /lead-scraper-microservice/api/v1/workspaces/workflow | Update workflow details |
| [**updateWorkspace**](LeadScraperServiceApi.md#updateWorkspace) | **PUT** /lead-scraper-microservice/api/v1/workspace | Update workspace details |


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

<a name="createAccount"></a>
# **createAccount**
> CreateAccountResponse createAccount(CreateAccountRequest)

Create a new account

    Creates a new user account in the workspace service

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **CreateAccountRequest** | [**CreateAccountRequest**](../Models/CreateAccountRequest.md)|  | |

### Return type

[**CreateAccountResponse**](../Models/CreateAccountResponse.md)

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

<a name="createScrapingJob"></a>
# **createScrapingJob**
> CreateScrapingJobResponse createScrapingJob(CreateScrapingJobRequest)

Create a new job scraping task

    This endpoint creates a new Google Maps scraping job

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **CreateScrapingJobRequest** | [**CreateScrapingJobRequest**](../Models/CreateScrapingJobRequest.md)|  | |

### Return type

[**CreateScrapingJobResponse**](../Models/CreateScrapingJobResponse.md)

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

<a name="createWorkspace"></a>
# **createWorkspace**
> CreateWorkspaceResponse createWorkspace(CreateWorkspaceRequest)

Create a new workspace

    Creates a new workspace for a given account

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **CreateWorkspaceRequest** | [**CreateWorkspaceRequest**](../Models/CreateWorkspaceRequest.md)|  | |

### Return type

[**CreateWorkspaceResponse**](../Models/CreateWorkspaceResponse.md)

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

<a name="deleteAccount"></a>
# **deleteAccount**
> DeleteAccountResponse deleteAccount(id, organizationId, tenantId)

Delete account

    Permanently deletes an account and associated resources

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | **String**|  | [default to null] |
| **organizationId** | **String**|  | [optional] [default to null] |
| **tenantId** | **String**|  | [optional] [default to null] |

### Return type

[**DeleteAccountResponse**](../Models/DeleteAccountResponse.md)

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

<a name="deleteScrapingJob"></a>
# **deleteScrapingJob**
> DeleteScrapingJobResponse deleteScrapingJob(jobId, userId, orgId, tenantId)

Delete a specific job

    This endpoint deletes a specific Google Maps scraping job

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **jobId** | **String**|  | [default to null] |
| **userId** | **String**|  | [default to null] |
| **orgId** | **String**|  | [default to null] |
| **tenantId** | **String**|  | [default to null] |

### Return type

[**DeleteScrapingJobResponse**](../Models/DeleteScrapingJobResponse.md)

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

<a name="deleteWorkflow"></a>
# **deleteWorkflow**
> DeleteWorkflowResponse deleteWorkflow(workspaceId, id, orgId, tenantId, accountId)

Delete workflow

    Deletes a specific workflow

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **workspaceId** | **String**|  | [default to null] |
| **id** | **String**|  | [default to null] |
| **orgId** | **String**|  | [default to null] |
| **tenantId** | **String**|  | [default to null] |
| **accountId** | **String**|  | [optional] [default to null] |

### Return type

[**DeleteWorkflowResponse**](../Models/DeleteWorkflowResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="deleteWorkspace"></a>
# **deleteWorkspace**
> DeleteWorkspaceResponse deleteWorkspace(id)

Delete a workspace

    Deletes a specific workspace

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | **String**|  | [default to null] |

### Return type

[**DeleteWorkspaceResponse**](../Models/DeleteWorkspaceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="downloadScrapingResults"></a>
# **downloadScrapingResults**
> DownloadScrapingResultsResponse downloadScrapingResults(jobId, userId, orgId, tenantId)

Download job results as CSV

    This endpoint downloads the results of a Google Maps scraping job as CSV

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **jobId** | **String**|  | [default to null] |
| **userId** | **String**|  | [default to null] |
| **orgId** | **String**|  | [default to null] |
| **tenantId** | **String**|  | [default to null] |

### Return type

[**DownloadScrapingResultsResponse**](../Models/DownloadScrapingResultsResponse.md)

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

<a name="getAccount"></a>
# **getAccount**
> GetAccountResponse getAccount(id, organizationId, tenantId)

Get account details

    Retrieves details of a specific account

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | **String**|  | [default to null] |
| **organizationId** | **String**|  | [optional] [default to null] |
| **tenantId** | **String**|  | [optional] [default to null] |

### Return type

[**GetAccountResponse**](../Models/GetAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="getAccountUsage"></a>
# **getAccountUsage**
> GetAccountUsageResponse getAccountUsage(id)

Get account usage

    Retrieves usage details for a given account

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | **String**|  | [default to null] |

### Return type

[**GetAccountUsageResponse**](../Models/GetAccountUsageResponse.md)

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

<a name="getScrapingJob"></a>
# **getScrapingJob**
> GetScrapingJobResponse getScrapingJob(jobId, userId, orgId, tenantId)

Get a specific job

    This endpoint retrieves a specific Google Maps scraping job

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **jobId** | **String**|  | [default to null] |
| **userId** | **String**|  | [default to null] |
| **orgId** | **String**|  | [default to null] |
| **tenantId** | **String**|  | [default to null] |

### Return type

[**GetScrapingJobResponse**](../Models/GetScrapingJobResponse.md)

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

<a name="getWorkflow"></a>
# **getWorkflow**
> GetWorkflowResponse getWorkflow(workspaceId, id)

Get workflow details

    Retrieves details of a specific workflow

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **workspaceId** | **String**|  | [default to null] |
| **id** | **String**|  | [default to null] |

### Return type

[**GetWorkflowResponse**](../Models/GetWorkflowResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="getWorkspace"></a>
# **getWorkspace**
> GetWorkspaceResponse getWorkspace(id, organizationId, tenantId, accountId)

Get workspace details

    Retrieves details of a specific workspace

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | **String**|  | [default to null] |
| **organizationId** | **String**|  | [optional] [default to null] |
| **tenantId** | **String**|  | [optional] [default to null] |
| **accountId** | **String**|  | [optional] [default to null] |

### Return type

[**GetWorkspaceResponse**](../Models/GetWorkspaceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="getWorkspaceAnalytics"></a>
# **getWorkspaceAnalytics**
> GetWorkspaceAnalyticsResponse getWorkspaceAnalytics(workspaceId, startTime, endTime)

Get workspace analytics

    Retrieves analytics data for a specific workspace

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **workspaceId** | **String**|  | [default to null] |
| **startTime** | **Date**|  | [optional] [default to null] |
| **endTime** | **Date**|  | [optional] [default to null] |

### Return type

[**GetWorkspaceAnalyticsResponse**](../Models/GetWorkspaceAnalyticsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="leadScraperServiceCreateWorkflow"></a>
# **leadScraperServiceCreateWorkflow**
> CreateWorkflowResponse leadScraperServiceCreateWorkflow(workspaceId, CreateWorkflowBody)

Create a new workflow

    This endpoint creates a new workflow configuration for a specific workspace  Required parameters: - workspace_id: Parent workspace ID - name: Human-readable workflow name - description: Optional workflow description - parameters: Job parameters for the workflow  Example: &#x60;&#x60;&#x60;json

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **workspaceId** | **String**|  | [default to null] |
| **CreateWorkflowBody** | [**CreateWorkflowBody**](../Models/CreateWorkflowBody.md)|  | |

### Return type

[**CreateWorkflowResponse**](../Models/CreateWorkflowResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
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

<a name="listAccounts"></a>
# **listAccounts**
> ListAccountsResponse listAccounts(pageSize, pageNumber, filter, organizationId, tenantId)

List all accounts

    Retrieves a list of accounts

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **pageSize** | **Integer**|  | [optional] [default to null] |
| **pageNumber** | **Integer**|  | [optional] [default to null] |
| **filter** | **String**|  | [optional] [default to null] |
| **organizationId** | **String**|  | [optional] [default to null] |
| **tenantId** | **String**|  | [optional] [default to null] |

### Return type

[**ListAccountsResponse**](../Models/ListAccountsResponse.md)

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

<a name="listScrapingJobs"></a>
# **listScrapingJobs**
> ListScrapingJobsResponse listScrapingJobs(authPlatformUserId, orgId, tenantId)

Get all jobs

    This endpoint retrieves all Google Maps scraping jobs

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **authPlatformUserId** | **String**|  | [default to null] |
| **orgId** | **String**|  | [default to null] |
| **tenantId** | **String**|  | [default to null] |

### Return type

[**ListScrapingJobsResponse**](../Models/ListScrapingJobsResponse.md)

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

<a name="listWorkflows"></a>
# **listWorkflows**
> ListWorkflowsResponse listWorkflows(workspaceId, pageSize, pageNumber, filter, organizationId, tenantId, accountId)

List workflows

    Retrieves a list of workflows for a given workspace

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **workspaceId** | **String**|  | [default to null] |
| **pageSize** | **Integer**|  | [optional] [default to null] |
| **pageNumber** | **Integer**|  | [optional] [default to null] |
| **filter** | **String**|  | [optional] [default to null] |
| **organizationId** | **String**|  | [optional] [default to null] |
| **tenantId** | **String**|  | [optional] [default to null] |
| **accountId** | **String**|  | [optional] [default to null] |

### Return type

[**ListWorkflowsResponse**](../Models/ListWorkflowsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="listWorkspaces"></a>
# **listWorkspaces**
> ListWorkspacesResponse listWorkspaces(accountId, pageSize, pageNumber, organizationId, tenantId)

List workspaces

    Retrieves a list of workspaces for a given account

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **accountId** | **String**|  | [optional] [default to null] |
| **pageSize** | **Integer**|  | [optional] [default to null] |
| **pageNumber** | **Integer**|  | [optional] [default to null] |
| **organizationId** | **String**|  | [optional] [default to null] |
| **tenantId** | **String**|  | [optional] [default to null] |

### Return type

[**ListWorkspacesResponse**](../Models/ListWorkspacesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="pauseWorkflow"></a>
# **pauseWorkflow**
> PauseWorkflowResponse pauseWorkflow(workspaceId, id, PauseWorkflowBody)

Pause workflow execution

    Pauses the execution of a specific workflow

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **workspaceId** | **String**|  | [default to null] |
| **id** | **String**|  | [default to null] |
| **PauseWorkflowBody** | [**PauseWorkflowBody**](../Models/PauseWorkflowBody.md)|  | |

### Return type

[**PauseWorkflowResponse**](../Models/PauseWorkflowResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
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

<a name="triggerWorkflow"></a>
# **triggerWorkflow**
> TriggerWorkflowResponse triggerWorkflow(workspaceId, id, TriggerWorkflowBody)

Trigger workflow execution

    Triggers the execution of a specific workflow

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **workspaceId** | **String**|  | [default to null] |
| **id** | **String**|  | [default to null] |
| **TriggerWorkflowBody** | [**TriggerWorkflowBody**](../Models/TriggerWorkflowBody.md)|  | |

### Return type

[**TriggerWorkflowResponse**](../Models/TriggerWorkflowResponse.md)

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

<a name="updateAccount"></a>
# **updateAccount**
> UpdateAccountResponse updateAccount(UpdateAccountRequest)

Update account details

    Updates specified fields of an existing account

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **UpdateAccountRequest** | [**UpdateAccountRequest**](../Models/UpdateAccountRequest.md)|  | |

### Return type

[**UpdateAccountResponse**](../Models/UpdateAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, 

<a name="updateAccountSettings"></a>
# **updateAccountSettings**
> UpdateAccountSettingsResponse updateAccountSettings(UpdateAccountSettingsRequest)

Update account settings

    Updates the settings for a given account

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **UpdateAccountSettingsRequest** | [**UpdateAccountSettingsRequest**](../Models/UpdateAccountSettingsRequest.md)|  | |

### Return type

[**UpdateAccountSettingsResponse**](../Models/UpdateAccountSettingsResponse.md)

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

<a name="updateWorkflow"></a>
# **updateWorkflow**
> UpdateWorkflowResponse updateWorkflow(UpdateWorkflowRequest)

Update workflow details

    Updates the details of a specific workflow

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **UpdateWorkflowRequest** | [**UpdateWorkflowRequest**](../Models/UpdateWorkflowRequest.md)|  | |

### Return type

[**UpdateWorkflowResponse**](../Models/UpdateWorkflowResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, 

<a name="updateWorkspace"></a>
# **updateWorkspace**
> UpdateWorkspaceResponse updateWorkspace(UpdateWorkspaceRequest)

Update workspace details

    Updates the details of a specific workspace

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **UpdateWorkspaceRequest** | [**UpdateWorkspaceRequest**](../Models/UpdateWorkspaceRequest.md)|  | |

### Return type

[**UpdateWorkspaceResponse**](../Models/UpdateWorkspaceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, 

