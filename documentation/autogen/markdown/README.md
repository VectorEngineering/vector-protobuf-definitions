# Documentation for Lead Scraping Service API

<a name="documentation-for-api-endpoints"></a>
## Documentation for API Endpoints

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

| Class | Method | HTTP request | Description |
|------------ | ------------- | ------------- | -------------|
| *LeadScraperServiceApi* | [**createAccount**](Apis/LeadScraperServiceApi.md#createaccount) | **POST** /lead-scraper-microservice/api/v1/accounts | Create a new account |
*LeadScraperServiceApi* | [**createScrapingJob**](Apis/LeadScraperServiceApi.md#createscrapingjob) | **POST** /lead-scraper-microservice/api/v1/jobs | Create a new job scraping task |
*LeadScraperServiceApi* | [**deleteAccount**](Apis/LeadScraperServiceApi.md#deleteaccount) | **DELETE** /lead-scraper-microservice/api/v1/accounts/{id} | Delete account |
*LeadScraperServiceApi* | [**deleteScrapingJob**](Apis/LeadScraperServiceApi.md#deletescrapingjob) | **DELETE** /lead-scraper-microservice/api/v1/jobs/{jobId} | Delete a specific job |
*LeadScraperServiceApi* | [**downloadScrapingResults**](Apis/LeadScraperServiceApi.md#downloadscrapingresults) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId}/download | Download job results as CSV |
*LeadScraperServiceApi* | [**getAccount**](Apis/LeadScraperServiceApi.md#getaccount) | **GET** /lead-scraper-microservice/api/v1/accounts/{id} | Get account details |
*LeadScraperServiceApi* | [**getScrapingJob**](Apis/LeadScraperServiceApi.md#getscrapingjob) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId} | Get a specific job |
*LeadScraperServiceApi* | [**listScrapingJobs**](Apis/LeadScraperServiceApi.md#listscrapingjobs) | **GET** /lead-scraper-microservice/api/v1/jobs | Get all jobs |
*LeadScraperServiceApi* | [**updateAccount**](Apis/LeadScraperServiceApi.md#updateaccount) | **PUT** /lead-scraper-microservice/api/v1/accounts | Update account details |


<a name="documentation-for-models"></a>
## Documentation for Models

 - [APIInfo](./Models/APIInfo.md)
 - [APIKey](./Models/APIKey.md)
 - [APIKey.Status](./Models/APIKey.Status.md)
 - [Account](./Models/Account.md)
 - [AccountSettings](./Models/AccountSettings.md)
 - [AccountStatus](./Models/AccountStatus.md)
 - [Any](./Models/Any.md)
 - [AuthContext](./Models/AuthContext.md)
 - [AuthErrorCode](./Models/AuthErrorCode.md)
 - [AuthenticationErrorMessageResponse](./Models/AuthenticationErrorMessageResponse.md)
 - [AvailabilityInfo](./Models/AvailabilityInfo.md)
 - [BackgroundJobStatus](./Models/BackgroundJobStatus.md)
 - [BadGatewayErrorMessageResponse](./Models/BadGatewayErrorMessageResponse.md)
 - [BusinessHours](./Models/BusinessHours.md)
 - [ConflictErrorMessageResponse](./Models/ConflictErrorMessageResponse.md)
 - [ConflictInfo](./Models/ConflictInfo.md)
 - [CreateAccountRequest](./Models/CreateAccountRequest.md)
 - [CreateAccountResponse](./Models/CreateAccountResponse.md)
 - [CreateScrapingJobRequest](./Models/CreateScrapingJobRequest.md)
 - [CreateScrapingJobResponse](./Models/CreateScrapingJobResponse.md)
 - [CreateWorkflowBody](./Models/CreateWorkflowBody.md)
 - [CreateWorkflowResponse](./Models/CreateWorkflowResponse.md)
 - [CreateWorkspaceRequest](./Models/CreateWorkspaceRequest.md)
 - [CreateWorkspaceResponse](./Models/CreateWorkspaceResponse.md)
 - [DayOfWeek](./Models/DayOfWeek.md)
 - [DeleteAccountResponse](./Models/DeleteAccountResponse.md)
 - [DeleteScrapingJobResponse](./Models/DeleteScrapingJobResponse.md)
 - [DeleteWorkspaceResponse](./Models/DeleteWorkspaceResponse.md)
 - [Dependency](./Models/Dependency.md)
 - [DownloadScrapingResultsResponse](./Models/DownloadScrapingResultsResponse.md)
 - [EmployeeBenefit](./Models/EmployeeBenefit.md)
 - [ErrorResponse](./Models/ErrorResponse.md)
 - [FieldError](./Models/FieldError.md)
 - [FieldViolation](./Models/FieldViolation.md)
 - [ForbiddenErrorMessageResponse](./Models/ForbiddenErrorMessageResponse.md)
 - [GatewayTimeoutErrorMessageResponse](./Models/GatewayTimeoutErrorMessageResponse.md)
 - [GetAccountResponse](./Models/GetAccountResponse.md)
 - [GetAccountUsageResponse](./Models/GetAccountUsageResponse.md)
 - [GetScrapingJobResponse](./Models/GetScrapingJobResponse.md)
 - [GetWorkflowResponse](./Models/GetWorkflowResponse.md)
 - [GetWorkspaceAnalyticsResponse](./Models/GetWorkspaceAnalyticsResponse.md)
 - [GetWorkspaceResponse](./Models/GetWorkspaceResponse.md)
 - [GoneErrorMessageResponse](./Models/GoneErrorMessageResponse.md)
 - [IncludedField](./Models/IncludedField.md)
 - [InternalErrorCode](./Models/InternalErrorCode.md)
 - [InternalErrorMessageResponse](./Models/InternalErrorMessageResponse.md)
 - [JobSuccessRate](./Models/JobSuccessRate.md)
 - [Lead](./Models/Lead.md)
 - [LimitInfo](./Models/LimitInfo.md)
 - [ListAccountsResponse](./Models/ListAccountsResponse.md)
 - [ListScrapingJobsResponse](./Models/ListScrapingJobsResponse.md)
 - [ListWorkflowsResponse](./Models/ListWorkflowsResponse.md)
 - [ListWorkspacesResponse](./Models/ListWorkspacesResponse.md)
 - [MFAInfo](./Models/MFAInfo.md)
 - [MethodNotAllowedErrorMessageResponse](./Models/MethodNotAllowedErrorMessageResponse.md)
 - [NotFoundErrorCode](./Models/NotFoundErrorCode.md)
 - [NotFoundErrorMessageResponse](./Models/NotFoundErrorMessageResponse.md)
 - [NotImplementedErrorMessageResponse](./Models/NotImplementedErrorMessageResponse.md)
 - [NullValue](./Models/NullValue.md)
 - [OperationDetails](./Models/OperationDetails.md)
 - [OutputFormat](./Models/OutputFormat.md)
 - [PauseWorkflowBody](./Models/PauseWorkflowBody.md)
 - [PauseWorkflowResponse](./Models/PauseWorkflowResponse.md)
 - [PayloadFormat](./Models/PayloadFormat.md)
 - [PaymentInfo](./Models/PaymentInfo.md)
 - [PaymentRequiredErrorMessageResponse](./Models/PaymentRequiredErrorMessageResponse.md)
 - [PreconditionFailedErrorMessageResponse](./Models/PreconditionFailedErrorMessageResponse.md)
 - [QuotaInfo](./Models/QuotaInfo.md)
 - [RateLimitContext](./Models/RateLimitContext.md)
 - [RateLimitErrorMessageResponse](./Models/RateLimitErrorMessageResponse.md)
 - [ResourceInfo](./Models/ResourceInfo.md)
 - [ResourceUtilization](./Models/ResourceUtilization.md)
 - [ResourceValidation](./Models/ResourceValidation.md)
 - [RevenueRange](./Models/RevenueRange.md)
 - [Review](./Models/Review.md)
 - [SchemaValidation](./Models/SchemaValidation.md)
 - [ScrapingJob](./Models/ScrapingJob.md)
 - [ScrapingWorkflow](./Models/ScrapingWorkflow.md)
 - [ServiceStatus](./Models/ServiceStatus.md)
 - [ServiceUnavailableErrorMessageResponse](./Models/ServiceUnavailableErrorMessageResponse.md)
 - [SessionInfo](./Models/SessionInfo.md)
 - [Suggestions](./Models/Suggestions.md)
 - [TokenInfo](./Models/TokenInfo.md)
 - [TooEarlyErrorMessageResponse](./Models/TooEarlyErrorMessageResponse.md)
 - [TriggerEvent](./Models/TriggerEvent.md)
 - [TriggerWorkflowBody](./Models/TriggerWorkflowBody.md)
 - [TriggerWorkflowResponse](./Models/TriggerWorkflowResponse.md)
 - [UnprocessableEntityErrorMessageResponse](./Models/UnprocessableEntityErrorMessageResponse.md)
 - [UpdateAccountRequest](./Models/UpdateAccountRequest.md)
 - [UpdateAccountResponse](./Models/UpdateAccountResponse.md)
 - [UpdateAccountSettingsRequest](./Models/UpdateAccountSettingsRequest.md)
 - [UpdateAccountSettingsResponse](./Models/UpdateAccountSettingsResponse.md)
 - [UpdateWorkflowRequest](./Models/UpdateWorkflowRequest.md)
 - [UpdateWorkflowResponse](./Models/UpdateWorkflowResponse.md)
 - [UpdateWorkspaceRequest](./Models/UpdateWorkspaceRequest.md)
 - [UpdateWorkspaceResponse](./Models/UpdateWorkspaceResponse.md)
 - [ValidationErrorCode](./Models/ValidationErrorCode.md)
 - [ValidationErrorMessageResponse](./Models/ValidationErrorMessageResponse.md)
 - [WebhookConfig](./Models/WebhookConfig.md)
 - [WorkflowStatus](./Models/WorkflowStatus.md)
 - [Workspace](./Models/Workspace.md)
 - [rpc.Status](./Models/rpc.Status.md)


<a name="documentation-for-authorization"></a>
## Documentation for Authorization

<a name="ApiKeyAuth"></a>
### ApiKeyAuth

- **Type**: API key
- **API key parameter name**: X-API-Key
- **Location**: HTTP header

