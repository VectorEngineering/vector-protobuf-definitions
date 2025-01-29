# lead_scraper_service_api

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

Method | HTTP request | Description
------------- | ------------- | -------------
**CreateAccount**](lead_scraper_service_api.md#CreateAccount) | **POST** /lead-scraper-microservice/api/v1/accounts | Create a new account
**CreateScrapingJob**](lead_scraper_service_api.md#CreateScrapingJob) | **POST** /lead-scraper-microservice/api/v1/jobs | Create a new job scraping task
**DeleteAccount**](lead_scraper_service_api.md#DeleteAccount) | **DELETE** /lead-scraper-microservice/api/v1/accounts/{accountId} | Delete account
**DeleteScrapingJob**](lead_scraper_service_api.md#DeleteScrapingJob) | **DELETE** /lead-scraper-microservice/api/v1/jobs/{jobId} | Delete a specific job
**DownloadScrapingResults**](lead_scraper_service_api.md#DownloadScrapingResults) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId}/download | Download job results as CSV
**GetAccount**](lead_scraper_service_api.md#GetAccount) | **GET** /lead-scraper-microservice/api/v1/accounts/{accountId} | Get account details
**GetScrapingJob**](lead_scraper_service_api.md#GetScrapingJob) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId} | Get a specific job
**ListAccounts**](lead_scraper_service_api.md#ListAccounts) | **GET** /lead-scraper-microservice/api/v1/accounts | List accounts
**ListScrapingJobs**](lead_scraper_service_api.md#ListScrapingJobs) | **GET** /lead-scraper-microservice/api/v1/jobs | Get all jobs
**UpdateAccount**](lead_scraper_service_api.md#UpdateAccount) | **PATCH** /lead-scraper-microservice/api/v1/accounts | Update account details


# **CreateAccount**
> models::CreateAccountResponse CreateAccount(create_account_request)
Create a new account

Creates a new user account in the workspace service

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **create_account_request** | [**CreateAccountRequest**](CreateAccountRequest.md)|  | 

### Return type

[**models::CreateAccountResponse**](CreateAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **CreateScrapingJob**
> models::CreateScrapingJobResponse CreateScrapingJob(create_scraping_job_request)
Create a new job scraping task

This endpoint creates a new Google Maps scraping job

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **create_scraping_job_request** | [**CreateScrapingJobRequest**](CreateScrapingJobRequest.md)|  | 

### Return type

[**models::CreateScrapingJobResponse**](CreateScrapingJobResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DeleteAccount**
> models::DeleteAccountResponse DeleteAccount(account_id, org_id, tenant_id)
Delete account

Permanently deletes an account and associated resources

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| Account identifier | 
  **org_id** | **String**| Organization context | 
  **tenant_id** | **String**| Tenant context | 

### Return type

[**models::DeleteAccountResponse**](DeleteAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DeleteScrapingJob**
> models::DeleteScrapingJobResponse DeleteScrapingJob(job_id, user_id, org_id, tenant_id)
Delete a specific job

This endpoint deletes a specific Google Maps scraping job

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **job_id** | **String**|  | 
  **user_id** | **String**|  | 
  **org_id** | **String**|  | 
  **tenant_id** | **String**|  | 

### Return type

[**models::DeleteScrapingJobResponse**](DeleteScrapingJobResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **DownloadScrapingResults**
> models::DownloadScrapingResultsResponse DownloadScrapingResults(job_id, user_id, org_id, tenant_id)
Download job results as CSV

This endpoint downloads the results of a Google Maps scraping job as CSV

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **job_id** | **String**|  | 
  **user_id** | **String**|  | 
  **org_id** | **String**|  | 
  **tenant_id** | **String**|  | 

### Return type

[**models::DownloadScrapingResultsResponse**](DownloadScrapingResultsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetAccount**
> models::GetAccountResponse GetAccount(account_id, org_id, tenant_id)
Get account details

Retrieves details of a specific account

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| Account identifier | 
  **org_id** | **String**| Organization context | 
  **tenant_id** | **String**| Tenant context | 

### Return type

[**models::GetAccountResponse**](GetAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **GetScrapingJob**
> models::GetScrapingJobResponse GetScrapingJob(job_id, user_id, org_id, tenant_id)
Get a specific job

This endpoint retrieves a specific Google Maps scraping job

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **job_id** | **String**|  | 
  **user_id** | **String**|  | 
  **org_id** | **String**|  | 
  **tenant_id** | **String**|  | 

### Return type

[**models::GetScrapingJobResponse**](GetScrapingJobResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ListAccounts**
> models::ListAccountsResponse ListAccounts(org_id, tenant_id, optional)
List accounts

Retrieves a list of accounts with optional filtering

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **org_id** | **String**| Organization context | 
  **tenant_id** | **String**| Tenant context | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **org_id** | **String**| Organization context | 
 **tenant_id** | **String**| Tenant context | 
 **offset** | **i32**| Pagination offset | 
 **limit** | **i32**| Maximum number of results | 

### Return type

[**models::ListAccountsResponse**](ListAccountsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ListScrapingJobs**
> models::ListScrapingJobsResponse ListScrapingJobs(user_id, org_id, tenant_id)
Get all jobs

This endpoint retrieves all Google Maps scraping jobs

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **user_id** | **String**|  | 
  **org_id** | **String**|  | 
  **tenant_id** | **String**|  | 

### Return type

[**models::ListScrapingJobsResponse**](ListScrapingJobsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **UpdateAccount**
> models::UpdateAccountResponse UpdateAccount(update_account_request)
Update account details

Updates specified fields of an existing account

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **update_account_request** | [**UpdateAccountRequest**](UpdateAccountRequest.md)|  | 

### Return type

[**models::UpdateAccountResponse**](UpdateAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/x-www-form-urlencoded
 - **Accept**: , application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

