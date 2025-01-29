# lead_scraper_service_api

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CreateAccount**](lead_scraper_service_api.md#CreateAccount) | **POST** /lead-scraper-microservice/api/v1/accounts | Create a new account
[**CreateScrapingJob**](lead_scraper_service_api.md#CreateScrapingJob) | **POST** /lead-scraper-microservice/api/v1/jobs | Create a new job scraping task
[**DeleteAccount**](lead_scraper_service_api.md#DeleteAccount) | **DELETE** /lead-scraper-microservice/api/v1/accounts/{id} | Delete account
[**DeleteScrapingJob**](lead_scraper_service_api.md#DeleteScrapingJob) | **DELETE** /lead-scraper-microservice/api/v1/jobs/{jobId} | Delete a specific job
[**DownloadScrapingResults**](lead_scraper_service_api.md#DownloadScrapingResults) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId}/download | Download job results as CSV
[**GetAccount**](lead_scraper_service_api.md#GetAccount) | **GET** /lead-scraper-microservice/api/v1/accounts/{id} | Get account details
[**GetScrapingJob**](lead_scraper_service_api.md#GetScrapingJob) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId} | Get a specific job
[**ListScrapingJobs**](lead_scraper_service_api.md#ListScrapingJobs) | **GET** /lead-scraper-microservice/api/v1/jobs | Get all jobs
[**UpdateAccount**](lead_scraper_service_api.md#UpdateAccount) | **PUT** /lead-scraper-microservice/api/v1/accounts | Update account details


<a name="CreateAccount"></a>
# **CreateAccount**
> CreateAccountResponse CreateAccount(createAccountRequest)

Create a new account

Creates a new user account in the workspace service
<a name="CreateScrapingJob"></a>
# **CreateScrapingJob**
> CreateScrapingJobResponse CreateScrapingJob(createScrapingJobRequest)

Create a new job scraping task

This endpoint creates a new Google Maps scraping job
<a name="DeleteAccount"></a>
# **DeleteAccount**
> DeleteAccountResponse DeleteAccount(Id_)

Delete account

Permanently deletes an account and associated resources
<a name="DeleteScrapingJob"></a>
# **DeleteScrapingJob**
> DeleteScrapingJobResponse DeleteScrapingJob(jobId, userId, orgId, tenantId)

Delete a specific job

This endpoint deletes a specific Google Maps scraping job
<a name="DownloadScrapingResults"></a>
# **DownloadScrapingResults**
> DownloadScrapingResultsResponse DownloadScrapingResults(jobId, userId, orgId, tenantId)

Download job results as CSV

This endpoint downloads the results of a Google Maps scraping job as CSV
<a name="GetAccount"></a>
# **GetAccount**
> GetAccountResponse GetAccount(Id_)

Get account details

Retrieves details of a specific account
<a name="GetScrapingJob"></a>
# **GetScrapingJob**
> GetScrapingJobResponse GetScrapingJob(jobId, userId, orgId, tenantId)

Get a specific job

This endpoint retrieves a specific Google Maps scraping job
<a name="ListScrapingJobs"></a>
# **ListScrapingJobs**
> ListScrapingJobsResponse ListScrapingJobs(userId, orgId, tenantId)

Get all jobs

This endpoint retrieves all Google Maps scraping jobs
<a name="UpdateAccount"></a>
# **UpdateAccount**
> UpdateAccountResponse UpdateAccount(updateAccountRequest)

Update account details

Updates specified fields of an existing account
