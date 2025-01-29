# LeadScraperServiceApi

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**createAccount**](LeadScraperServiceApi.md#createAccount) | **POST** /lead-scraper-microservice/api/v1/accounts | Create a new account |
| [**createScrapingJob**](LeadScraperServiceApi.md#createScrapingJob) | **POST** /lead-scraper-microservice/api/v1/jobs | Create a new job scraping task |
| [**deleteAccount**](LeadScraperServiceApi.md#deleteAccount) | **DELETE** /lead-scraper-microservice/api/v1/accounts/{accountId} | Delete account |
| [**deleteScrapingJob**](LeadScraperServiceApi.md#deleteScrapingJob) | **DELETE** /lead-scraper-microservice/api/v1/jobs/{jobId} | Delete a specific job |
| [**downloadScrapingResults**](LeadScraperServiceApi.md#downloadScrapingResults) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId}/download | Download job results as CSV |
| [**getAccount**](LeadScraperServiceApi.md#getAccount) | **GET** /lead-scraper-microservice/api/v1/accounts/{accountId} | Get account details |
| [**getScrapingJob**](LeadScraperServiceApi.md#getScrapingJob) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId} | Get a specific job |
| [**listAccounts**](LeadScraperServiceApi.md#listAccounts) | **GET** /lead-scraper-microservice/api/v1/accounts | List accounts |
| [**listScrapingJobs**](LeadScraperServiceApi.md#listScrapingJobs) | **GET** /lead-scraper-microservice/api/v1/jobs | Get all jobs |
| [**updateAccount**](LeadScraperServiceApi.md#updateAccount) | **PATCH** /lead-scraper-microservice/api/v1/accounts | Update account details |


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

<a name="deleteAccount"></a>
# **deleteAccount**
> DeleteAccountResponse deleteAccount(accountId, orgId, tenantId)

Delete account

    Permanently deletes an account and associated resources

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **accountId** | **String**| Account identifier | [default to null] |
| **orgId** | **String**| Organization context | [default to null] |
| **tenantId** | **String**| Tenant context | [default to null] |

### Return type

[**DeleteAccountResponse**](../Models/DeleteAccountResponse.md)

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
> GetAccountResponse getAccount(accountId, orgId, tenantId)

Get account details

    Retrieves details of a specific account

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **accountId** | **String**| Account identifier | [default to null] |
| **orgId** | **String**| Organization context | [default to null] |
| **tenantId** | **String**| Tenant context | [default to null] |

### Return type

[**GetAccountResponse**](../Models/GetAccountResponse.md)

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

<a name="listAccounts"></a>
# **listAccounts**
> ListAccountsResponse listAccounts(orgId, tenantId, offset, limit)

List accounts

    Retrieves a list of accounts with optional filtering

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **orgId** | **String**| Organization context | [default to null] |
| **tenantId** | **String**| Tenant context | [default to null] |
| **offset** | **Integer**| Pagination offset | [optional] [default to null] |
| **limit** | **Integer**| Maximum number of results | [optional] [default to null] |

### Return type

[**ListAccountsResponse**](../Models/ListAccountsResponse.md)

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

