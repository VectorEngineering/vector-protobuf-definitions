# Documentation for Lead Scraping Service API

<a name="documentation-for-api-endpoints"></a>
## Documentation for API Endpoints

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

| Class | Method | HTTP request | Description |
|------------ | ------------- | ------------- | -------------|
| *LeadScraperServiceApi* | [**createAccount**](Apis/LeadScraperServiceApi.md#createaccount) | **POST** /lead-scraper-microservice/api/v1/accounts | Create a new account |
*LeadScraperServiceApi* | [**createScrapingJob**](Apis/LeadScraperServiceApi.md#createscrapingjob) | **POST** /lead-scraper-microservice/api/v1/jobs | Create a new job scraping task |
*LeadScraperServiceApi* | [**deleteAccount**](Apis/LeadScraperServiceApi.md#deleteaccount) | **DELETE** /lead-scraper-microservice/api/v1/accounts/{accountId} | Delete account |
*LeadScraperServiceApi* | [**deleteScrapingJob**](Apis/LeadScraperServiceApi.md#deletescrapingjob) | **DELETE** /lead-scraper-microservice/api/v1/jobs/{jobId} | Delete a specific job |
*LeadScraperServiceApi* | [**downloadScrapingResults**](Apis/LeadScraperServiceApi.md#downloadscrapingresults) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId}/download | Download job results as CSV |
*LeadScraperServiceApi* | [**getAccount**](Apis/LeadScraperServiceApi.md#getaccount) | **GET** /lead-scraper-microservice/api/v1/accounts/{accountId} | Get account details |
*LeadScraperServiceApi* | [**getScrapingJob**](Apis/LeadScraperServiceApi.md#getscrapingjob) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId} | Get a specific job |
*LeadScraperServiceApi* | [**listAccounts**](Apis/LeadScraperServiceApi.md#listaccounts) | **GET** /lead-scraper-microservice/api/v1/accounts | List accounts |
*LeadScraperServiceApi* | [**listScrapingJobs**](Apis/LeadScraperServiceApi.md#listscrapingjobs) | **GET** /lead-scraper-microservice/api/v1/jobs | Get all jobs |
*LeadScraperServiceApi* | [**updateAccount**](Apis/LeadScraperServiceApi.md#updateaccount) | **PATCH** /lead-scraper-microservice/api/v1/accounts | Update account details |


<a name="documentation-for-models"></a>
## Documentation for Models

 - [APIInfo](./Models/APIInfo.md)
 - [Account](./Models/Account.md)
 - [Any](./Models/Any.md)
 - [AuthContext](./Models/AuthContext.md)
 - [AuthErrorCode](./Models/AuthErrorCode.md)
 - [AuthenticationErrorMessageResponse](./Models/AuthenticationErrorMessageResponse.md)
 - [AvailabilityInfo](./Models/AvailabilityInfo.md)
 - [BackgroundJobStatus](./Models/BackgroundJobStatus.md)
 - [BadGatewayErrorMessageResponse](./Models/BadGatewayErrorMessageResponse.md)
 - [ConflictErrorMessageResponse](./Models/ConflictErrorMessageResponse.md)
 - [ConflictInfo](./Models/ConflictInfo.md)
 - [CreateAccountRequest](./Models/CreateAccountRequest.md)
 - [CreateAccountResponse](./Models/CreateAccountResponse.md)
 - [CreateScrapingJobRequest](./Models/CreateScrapingJobRequest.md)
 - [CreateScrapingJobResponse](./Models/CreateScrapingJobResponse.md)
 - [DeleteAccountResponse](./Models/DeleteAccountResponse.md)
 - [DeleteScrapingJobResponse](./Models/DeleteScrapingJobResponse.md)
 - [Dependency](./Models/Dependency.md)
 - [DownloadScrapingResultsResponse](./Models/DownloadScrapingResultsResponse.md)
 - [ErrorResponse](./Models/ErrorResponse.md)
 - [FieldError](./Models/FieldError.md)
 - [FieldViolation](./Models/FieldViolation.md)
 - [ForbiddenErrorMessageResponse](./Models/ForbiddenErrorMessageResponse.md)
 - [GatewayTimeoutErrorMessageResponse](./Models/GatewayTimeoutErrorMessageResponse.md)
 - [GetAccountResponse](./Models/GetAccountResponse.md)
 - [GetScrapingJobResponse](./Models/GetScrapingJobResponse.md)
 - [GoneErrorMessageResponse](./Models/GoneErrorMessageResponse.md)
 - [InternalErrorCode](./Models/InternalErrorCode.md)
 - [InternalErrorMessageResponse](./Models/InternalErrorMessageResponse.md)
 - [LimitInfo](./Models/LimitInfo.md)
 - [ListAccountsResponse](./Models/ListAccountsResponse.md)
 - [ListScrapingJobsResponse](./Models/ListScrapingJobsResponse.md)
 - [MFAInfo](./Models/MFAInfo.md)
 - [MethodNotAllowedErrorMessageResponse](./Models/MethodNotAllowedErrorMessageResponse.md)
 - [NotFoundErrorCode](./Models/NotFoundErrorCode.md)
 - [NotFoundErrorMessageResponse](./Models/NotFoundErrorMessageResponse.md)
 - [NotImplementedErrorMessageResponse](./Models/NotImplementedErrorMessageResponse.md)
 - [OperationDetails](./Models/OperationDetails.md)
 - [PaymentInfo](./Models/PaymentInfo.md)
 - [PaymentRequiredErrorMessageResponse](./Models/PaymentRequiredErrorMessageResponse.md)
 - [PreconditionFailedErrorMessageResponse](./Models/PreconditionFailedErrorMessageResponse.md)
 - [QuotaInfo](./Models/QuotaInfo.md)
 - [RateLimitContext](./Models/RateLimitContext.md)
 - [RateLimitErrorMessageResponse](./Models/RateLimitErrorMessageResponse.md)
 - [ResourceInfo](./Models/ResourceInfo.md)
 - [ResourceUtilization](./Models/ResourceUtilization.md)
 - [ResourceValidation](./Models/ResourceValidation.md)
 - [SchemaValidation](./Models/SchemaValidation.md)
 - [ScrapingJob](./Models/ScrapingJob.md)
 - [ServiceStatus](./Models/ServiceStatus.md)
 - [ServiceUnavailableErrorMessageResponse](./Models/ServiceUnavailableErrorMessageResponse.md)
 - [SessionInfo](./Models/SessionInfo.md)
 - [Status](./Models/Status.md)
 - [Suggestions](./Models/Suggestions.md)
 - [TokenInfo](./Models/TokenInfo.md)
 - [TooEarlyErrorMessageResponse](./Models/TooEarlyErrorMessageResponse.md)
 - [UnprocessableEntityErrorMessageResponse](./Models/UnprocessableEntityErrorMessageResponse.md)
 - [UpdateAccountRequest](./Models/UpdateAccountRequest.md)
 - [UpdateAccountResponse](./Models/UpdateAccountResponse.md)
 - [ValidationErrorCode](./Models/ValidationErrorCode.md)
 - [ValidationErrorMessageResponse](./Models/ValidationErrorMessageResponse.md)


<a name="documentation-for-authorization"></a>
## Documentation for Authorization

<a name="ApiKeyAuth"></a>
### ApiKeyAuth

- **Type**: API key
- **API key parameter name**: X-API-Key
- **Location**: HTTP header

