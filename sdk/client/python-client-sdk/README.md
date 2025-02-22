# backend_client
Vector Lead Scraping Service API - Manages Lead Scraping Jobs

This Python package is automatically generated by the [OpenAPI Generator](https://openapi-generator.tech) project:

- API version: 1.0
- Package version: 1.0.0
- Generator version: 7.7.0
- Build package: org.openapitools.codegen.languages.PythonClientCodegen
For more information, please visit [https://vector.ai](https://vector.ai)

## Requirements.

Python 3.7+

## Installation & Usage
### pip install

If the python package is hosted on a repository, you can install directly using:

```sh
pip install git+https://github.com/GIT_USER_ID/GIT_REPO_ID.git
```
(you may need to run `pip` with root permission: `sudo pip install git+https://github.com/GIT_USER_ID/GIT_REPO_ID.git`)

Then import the package:
```python
import backend_client
```

### Setuptools

Install via [Setuptools](http://pypi.python.org/pypi/setuptools).

```sh
python setup.py install --user
```
(or `sudo python setup.py install` to install the package for all users)

Then import the package:
```python
import backend_client
```

### Tests

Execute `pytest` to run the tests.

## Getting Started

Please follow the [installation procedure](#installation--usage) and then run the following:

```python

import backend_client
from backend_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://lead-scraping-microservice.vector.svc.cluster.local:9896
# See configuration.py for a list of all supported configuration parameters.
configuration = backend_client.Configuration(
    host = "http://lead-scraping-microservice.vector.svc.cluster.local:9896"
)



# Enter a context with an instance of the API client
with backend_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = backend_client.LeadScraperServiceApi(api_client)
    create_account_request = backend_client.CreateAccountRequest() # CreateAccountRequest | 

    try:
        # Create a new account
        api_response = api_instance.create_account(create_account_request)
        print("The response of LeadScraperServiceApi->create_account:\n")
        pprint(api_response)
    except ApiException as e:
        print("Exception when calling LeadScraperServiceApi->create_account: %s\n" % e)

```

## Documentation for API Endpoints

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*LeadScraperServiceApi* | [**create_account**](docs/LeadScraperServiceApi.md#create_account) | **POST** /lead-scraper-microservice/api/v1/accounts | Create a new account
*LeadScraperServiceApi* | [**create_scraping_job**](docs/LeadScraperServiceApi.md#create_scraping_job) | **POST** /lead-scraper-microservice/api/v1/jobs | Create a new job scraping task
*LeadScraperServiceApi* | [**delete_account**](docs/LeadScraperServiceApi.md#delete_account) | **DELETE** /lead-scraper-microservice/api/v1/accounts/{accountId} | Delete account
*LeadScraperServiceApi* | [**delete_scraping_job**](docs/LeadScraperServiceApi.md#delete_scraping_job) | **DELETE** /lead-scraper-microservice/api/v1/jobs/{jobId} | Delete a specific job
*LeadScraperServiceApi* | [**download_scraping_results**](docs/LeadScraperServiceApi.md#download_scraping_results) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId}/download | Download job results as CSV
*LeadScraperServiceApi* | [**get_account**](docs/LeadScraperServiceApi.md#get_account) | **GET** /lead-scraper-microservice/api/v1/accounts/{accountId} | Get account details
*LeadScraperServiceApi* | [**get_scraping_job**](docs/LeadScraperServiceApi.md#get_scraping_job) | **GET** /lead-scraper-microservice/api/v1/jobs/{jobId} | Get a specific job
*LeadScraperServiceApi* | [**list_accounts**](docs/LeadScraperServiceApi.md#list_accounts) | **GET** /lead-scraper-microservice/api/v1/accounts | List accounts
*LeadScraperServiceApi* | [**list_scraping_jobs**](docs/LeadScraperServiceApi.md#list_scraping_jobs) | **GET** /lead-scraper-microservice/api/v1/jobs | Get all jobs
*LeadScraperServiceApi* | [**update_account**](docs/LeadScraperServiceApi.md#update_account) | **PATCH** /lead-scraper-microservice/api/v1/accounts | Update account details


## Documentation For Models

 - [APIInfo](docs/APIInfo.md)
 - [Account](docs/Account.md)
 - [Any](docs/Any.md)
 - [AuthContext](docs/AuthContext.md)
 - [AuthErrorCode](docs/AuthErrorCode.md)
 - [AuthenticationErrorMessageResponse](docs/AuthenticationErrorMessageResponse.md)
 - [AvailabilityInfo](docs/AvailabilityInfo.md)
 - [BackgroundJobStatus](docs/BackgroundJobStatus.md)
 - [BadGatewayErrorMessageResponse](docs/BadGatewayErrorMessageResponse.md)
 - [ConflictErrorMessageResponse](docs/ConflictErrorMessageResponse.md)
 - [ConflictInfo](docs/ConflictInfo.md)
 - [CreateAccountRequest](docs/CreateAccountRequest.md)
 - [CreateAccountResponse](docs/CreateAccountResponse.md)
 - [CreateScrapingJobRequest](docs/CreateScrapingJobRequest.md)
 - [CreateScrapingJobResponse](docs/CreateScrapingJobResponse.md)
 - [DeleteAccountResponse](docs/DeleteAccountResponse.md)
 - [DeleteScrapingJobResponse](docs/DeleteScrapingJobResponse.md)
 - [Dependency](docs/Dependency.md)
 - [DownloadScrapingResultsResponse](docs/DownloadScrapingResultsResponse.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [FieldError](docs/FieldError.md)
 - [FieldViolation](docs/FieldViolation.md)
 - [ForbiddenErrorMessageResponse](docs/ForbiddenErrorMessageResponse.md)
 - [GatewayTimeoutErrorMessageResponse](docs/GatewayTimeoutErrorMessageResponse.md)
 - [GetAccountResponse](docs/GetAccountResponse.md)
 - [GetScrapingJobResponse](docs/GetScrapingJobResponse.md)
 - [GoneErrorMessageResponse](docs/GoneErrorMessageResponse.md)
 - [InternalErrorCode](docs/InternalErrorCode.md)
 - [InternalErrorMessageResponse](docs/InternalErrorMessageResponse.md)
 - [LimitInfo](docs/LimitInfo.md)
 - [ListAccountsResponse](docs/ListAccountsResponse.md)
 - [ListScrapingJobsResponse](docs/ListScrapingJobsResponse.md)
 - [MFAInfo](docs/MFAInfo.md)
 - [MethodNotAllowedErrorMessageResponse](docs/MethodNotAllowedErrorMessageResponse.md)
 - [NotFoundErrorCode](docs/NotFoundErrorCode.md)
 - [NotFoundErrorMessageResponse](docs/NotFoundErrorMessageResponse.md)
 - [NotImplementedErrorMessageResponse](docs/NotImplementedErrorMessageResponse.md)
 - [OperationDetails](docs/OperationDetails.md)
 - [PaymentInfo](docs/PaymentInfo.md)
 - [PaymentRequiredErrorMessageResponse](docs/PaymentRequiredErrorMessageResponse.md)
 - [PreconditionFailedErrorMessageResponse](docs/PreconditionFailedErrorMessageResponse.md)
 - [QuotaInfo](docs/QuotaInfo.md)
 - [RateLimitContext](docs/RateLimitContext.md)
 - [RateLimitErrorMessageResponse](docs/RateLimitErrorMessageResponse.md)
 - [ResourceInfo](docs/ResourceInfo.md)
 - [ResourceUtilization](docs/ResourceUtilization.md)
 - [ResourceValidation](docs/ResourceValidation.md)
 - [SchemaValidation](docs/SchemaValidation.md)
 - [ScrapingJob](docs/ScrapingJob.md)
 - [ServiceStatus](docs/ServiceStatus.md)
 - [ServiceUnavailableErrorMessageResponse](docs/ServiceUnavailableErrorMessageResponse.md)
 - [SessionInfo](docs/SessionInfo.md)
 - [Status](docs/Status.md)
 - [Suggestions](docs/Suggestions.md)
 - [TokenInfo](docs/TokenInfo.md)
 - [TooEarlyErrorMessageResponse](docs/TooEarlyErrorMessageResponse.md)
 - [UnprocessableEntityErrorMessageResponse](docs/UnprocessableEntityErrorMessageResponse.md)
 - [UpdateAccountRequest](docs/UpdateAccountRequest.md)
 - [UpdateAccountResponse](docs/UpdateAccountResponse.md)
 - [ValidationErrorCode](docs/ValidationErrorCode.md)
 - [ValidationErrorMessageResponse](docs/ValidationErrorMessageResponse.md)


<a id="documentation-for-authorization"></a>
## Documentation For Authorization


Authentication schemes defined for the API:
<a id="ApiKeyAuth"></a>
### ApiKeyAuth

- **Type**: API key
- **API key parameter name**: X-API-Key
- **Location**: HTTP header


## Author

yoanyomba@vector.ai


