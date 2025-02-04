# LeadScraperServiceApi

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**createAccount**](LeadScraperServiceApi.md#createAccount) | **POST** /lead-scraper-microservice/api/v1/accounts | Create a new account |
| [**createOrganization**](LeadScraperServiceApi.md#createOrganization) | **POST** /lead-scraper-microservice/api/v1/organization | Create a new organization |
| [**createScrapingJob**](LeadScraperServiceApi.md#createScrapingJob) | **POST** /lead-scraper-microservice/api/v1/jobs | Create a new job scraping task |
| [**createTenant**](LeadScraperServiceApi.md#createTenant) | **POST** /lead-scraper-microservice/api/v1/organizations/{organizationId}/tenants | Create a new tenant |
| [**createWorkspace**](LeadScraperServiceApi.md#createWorkspace) | **POST** /lead-scraper-microservice/api/v1/workspaces | Create a new workspace |
| [**deleteAccount**](LeadScraperServiceApi.md#deleteAccount) | **DELETE** /lead-scraper-microservice/api/v1/accounts/{id} | Delete account |
| [**deleteOrganization**](LeadScraperServiceApi.md#deleteOrganization) | **DELETE** /lead-scraper-microservice/api/v1/organization/{id} | Delete an organization |
| [**deleteScrapingJob**](LeadScraperServiceApi.md#deleteScrapingJob) | **DELETE** /lead-scraper-microservice/api/v1/jobs/{jobId} | Delete a specific job |
| [**deleteTenant**](LeadScraperServiceApi.md#deleteTenant) | **DELETE** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId}/{tenantId} | Delete a tenant |
| [**deleteWorkspace**](LeadScraperServiceApi.md#deleteWorkspace) | **DELETE** /lead-scraper-microservice/api/v1/workspace/{id} | Delete a workspace |
| [**downloadScrapingResults**](LeadScraperServiceApi.md#downloadScrapingResults) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId}/download | Download job results as CSV |
| [**getAccount**](LeadScraperServiceApi.md#getAccount) | **GET** /lead-scraper-microservice/api/v1/accounts/{id} | Get account details |
| [**getAccountUsage**](LeadScraperServiceApi.md#getAccountUsage) | **GET** /lead-scraper-microservice/api/v1/accounts/{id}/usage | Get account usage |
| [**getOrganization**](LeadScraperServiceApi.md#getOrganization) | **GET** /lead-scraper-microservice/api/v1/organization/{id} | Get organization details |
| [**getScrapingJob**](LeadScraperServiceApi.md#getScrapingJob) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId} | Get a specific job |
| [**getTenant**](LeadScraperServiceApi.md#getTenant) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/{organizationId}/{tenantId} | Get tenant details |
| [**getWorkflow**](LeadScraperServiceApi.md#getWorkflow) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id} | Get workflow details |
| [**getWorkspace**](LeadScraperServiceApi.md#getWorkspace) | **GET** /lead-scraper-microservice/api/v1/workspace/{id} | Get workspace details |
| [**getWorkspaceAnalytics**](LeadScraperServiceApi.md#getWorkspaceAnalytics) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/analytics | Get workspace analytics |
| [**leadScraperServiceCreateWorkflow**](LeadScraperServiceApi.md#leadScraperServiceCreateWorkflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows | Create a new workflow |
| [**listAccounts**](LeadScraperServiceApi.md#listAccounts) | **GET** /lead-scraper-microservice/api/v1/accounts | List all accounts |
| [**listOrganizations**](LeadScraperServiceApi.md#listOrganizations) | **GET** /lead-scraper-microservice/api/v1/organization | List all organizations |
| [**listScrapingJobs**](LeadScraperServiceApi.md#listScrapingJobs) | **GET** /lead-scraper-microservice/api/v1/jobs | Get all jobs |
| [**listTenants**](LeadScraperServiceApi.md#listTenants) | **GET** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId} | List all tenants |
| [**listWorkflows**](LeadScraperServiceApi.md#listWorkflows) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows | List workflows |
| [**listWorkspaces**](LeadScraperServiceApi.md#listWorkspaces) | **GET** /lead-scraper-microservice/api/v1/workspaces | List workspaces |
| [**pauseWorkflow**](LeadScraperServiceApi.md#pauseWorkflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id}/pause | Pause workflow execution |
| [**triggerWorkflow**](LeadScraperServiceApi.md#triggerWorkflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id}/trigger | Trigger workflow execution |
| [**updateAccount**](LeadScraperServiceApi.md#updateAccount) | **PUT** /lead-scraper-microservice/api/v1/accounts | Update account details |
| [**updateAccountSettings**](LeadScraperServiceApi.md#updateAccountSettings) | **PUT** /lead-scraper-microservice/api/v1/accounts/settings | Update account settings |
| [**updateOrganization**](LeadScraperServiceApi.md#updateOrganization) | **PUT** /lead-scraper-microservice/api/v1/organization | Update organization details |
| [**updateTenant**](LeadScraperServiceApi.md#updateTenant) | **PUT** /lead-scraper-microservice/api/v1/organizations/tenants | Update tenant details |
| [**updateWorkflow**](LeadScraperServiceApi.md#updateWorkflow) | **PUT** /lead-scraper-microservice/api/v1/workspaces/workflow | Update workflow details |
| [**updateWorkspace**](LeadScraperServiceApi.md#updateWorkspace) | **PUT** /lead-scraper-microservice/api/v1/workspace | Update workspace details |


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

<a name="deleteAccount"></a>
# **deleteAccount**
> DeleteAccountResponse deleteAccount(id)

Delete account

    Permanently deletes an account and associated resources

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | **String**|  | [default to null] |

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

<a name="getAccount"></a>
# **getAccount**
> GetAccountResponse getAccount(id)

Get account details

    Retrieves details of a specific account

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | **String**|  | [default to null] |

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

<a name="getWorkflow"></a>
# **getWorkflow**
> ScrapingWorkflow getWorkflow(workspaceId, id)

Get workflow details

    Retrieves details of a specific workflow

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **workspaceId** | **String**|  | [default to null] |
| **id** | **String**|  | [default to null] |

### Return type

[**ScrapingWorkflow**](../Models/ScrapingWorkflow.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="getWorkspace"></a>
# **getWorkspace**
> Workspace getWorkspace(id)

Get workspace details

    Retrieves details of a specific workspace

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | **String**|  | [default to null] |

### Return type

[**Workspace**](../Models/Workspace.md)

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

<a name="listAccounts"></a>
# **listAccounts**
> ListAccountsResponse listAccounts(pageSize, pageNumber, filter)

List all accounts

    Retrieves a list of accounts

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **pageSize** | **Integer**|  | [optional] [default to null] |
| **pageNumber** | **Integer**|  | [optional] [default to null] |
| **filter** | **String**|  | [optional] [default to null] |

### Return type

[**ListAccountsResponse**](../Models/ListAccountsResponse.md)

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
> ListScrapingJobsResponse listScrapingJobs(userId, orgId, tenantId)

Get all jobs

    This endpoint retrieves all Google Maps scraping jobs

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **userId** | **String**|  | [default to null] |
| **orgId** | **String**|  | [default to null] |
| **tenantId** | **String**|  | [default to null] |

### Return type

[**ListScrapingJobsResponse**](../Models/ListScrapingJobsResponse.md)

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

<a name="listWorkflows"></a>
# **listWorkflows**
> ListWorkflowsResponse listWorkflows(workspaceId, pageSize, pageNumber, filter)

List workflows

    Retrieves a list of workflows for a given workspace

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **workspaceId** | **String**|  | [default to null] |
| **pageSize** | **Integer**|  | [optional] [default to null] |
| **pageNumber** | **Integer**|  | [optional] [default to null] |
| **filter** | **String**|  | [optional] [default to null] |

### Return type

[**ListWorkflowsResponse**](../Models/ListWorkflowsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

<a name="listWorkspaces"></a>
# **listWorkspaces**
> ListWorkspacesResponse listWorkspaces(accountId, pageSize, pageNumber)

List workspaces

    Retrieves a list of workspaces for a given account

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **accountId** | **String**|  | [optional] [default to null] |
| **pageSize** | **Integer**|  | [optional] [default to null] |
| **pageNumber** | **Integer**|  | [optional] [default to null] |

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

<a name="updateWorkflow"></a>
# **updateWorkflow**
> ScrapingWorkflow updateWorkflow(UpdateWorkflowRequest)

Update workflow details

    Updates the details of a specific workflow

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **UpdateWorkflowRequest** | [**UpdateWorkflowRequest**](../Models/UpdateWorkflowRequest.md)|  | |

### Return type

[**ScrapingWorkflow**](../Models/ScrapingWorkflow.md)

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

