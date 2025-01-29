# \LeadScraperServiceAPI

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CreateAccount**](LeadScraperServiceAPI.md#CreateAccount) | **Post** /lead-scraper-microservice/api/v1/accounts | Create a new account
[**CreateScrapingJob**](LeadScraperServiceAPI.md#CreateScrapingJob) | **Post** /lead-scraper-microservice/api/v1/jobs | Create a new job scraping task
[**DeleteAccount**](LeadScraperServiceAPI.md#DeleteAccount) | **Delete** /lead-scraper-microservice/api/v1/accounts/{accountId} | Delete account
[**DeleteScrapingJob**](LeadScraperServiceAPI.md#DeleteScrapingJob) | **Delete** /lead-scraper-microservice/api/v1/jobs/{jobId} | Delete a specific job
[**DownloadScrapingResults**](LeadScraperServiceAPI.md#DownloadScrapingResults) | **Get** /lead-scraper-microservice/api/v1/jobs/{jobId}/download | Download job results as CSV
[**GetAccount**](LeadScraperServiceAPI.md#GetAccount) | **Get** /lead-scraper-microservice/api/v1/accounts/{accountId} | Get account details
[**GetScrapingJob**](LeadScraperServiceAPI.md#GetScrapingJob) | **Get** /lead-scraper-microservice/api/v1/jobs/{jobId} | Get a specific job
[**ListAccounts**](LeadScraperServiceAPI.md#ListAccounts) | **Get** /lead-scraper-microservice/api/v1/accounts | List accounts
[**ListScrapingJobs**](LeadScraperServiceAPI.md#ListScrapingJobs) | **Get** /lead-scraper-microservice/api/v1/jobs | Get all jobs
[**UpdateAccount**](LeadScraperServiceAPI.md#UpdateAccount) | **Patch** /lead-scraper-microservice/api/v1/accounts | Update account details



## CreateAccount

> CreateAccountResponse CreateAccount(ctx).CreateAccountRequest(createAccountRequest).Execute()

Create a new account



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	createAccountRequest := *openapiclient.NewCreateAccountRequest("AuthPlatformUserId_example", "OrgId_example", "TenantId_example", "Email_example") // CreateAccountRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.LeadScraperServiceAPI.CreateAccount(context.Background()).CreateAccountRequest(createAccountRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LeadScraperServiceAPI.CreateAccount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateAccount`: CreateAccountResponse
	fmt.Fprintf(os.Stdout, "Response from `LeadScraperServiceAPI.CreateAccount`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiCreateAccountRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **createAccountRequest** | [**CreateAccountRequest**](CreateAccountRequest.md) |  | 

### Return type

[**CreateAccountResponse**](CreateAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## CreateScrapingJob

> CreateScrapingJobResponse CreateScrapingJob(ctx).CreateScrapingJobRequest(createScrapingJobRequest).Execute()

Create a new job scraping task



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	createScrapingJobRequest := *openapiclient.NewCreateScrapingJobRequest("UserId_example", "OrgId_example", "TenantId_example") // CreateScrapingJobRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.LeadScraperServiceAPI.CreateScrapingJob(context.Background()).CreateScrapingJobRequest(createScrapingJobRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LeadScraperServiceAPI.CreateScrapingJob``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateScrapingJob`: CreateScrapingJobResponse
	fmt.Fprintf(os.Stdout, "Response from `LeadScraperServiceAPI.CreateScrapingJob`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiCreateScrapingJobRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **createScrapingJobRequest** | [**CreateScrapingJobRequest**](CreateScrapingJobRequest.md) |  | 

### Return type

[**CreateScrapingJobResponse**](CreateScrapingJobResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## DeleteAccount

> DeleteAccountResponse DeleteAccount(ctx, accountId).OrgId(orgId).TenantId(tenantId).Execute()

Delete account



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	accountId := "accountId_example" // string | Account identifier
	orgId := "orgId_example" // string | Organization context
	tenantId := "tenantId_example" // string | Tenant context

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.LeadScraperServiceAPI.DeleteAccount(context.Background(), accountId).OrgId(orgId).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LeadScraperServiceAPI.DeleteAccount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DeleteAccount`: DeleteAccountResponse
	fmt.Fprintf(os.Stdout, "Response from `LeadScraperServiceAPI.DeleteAccount`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**accountId** | **string** | Account identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiDeleteAccountRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **orgId** | **string** | Organization context | 
 **tenantId** | **string** | Tenant context | 

### Return type

[**DeleteAccountResponse**](DeleteAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## DeleteScrapingJob

> DeleteScrapingJobResponse DeleteScrapingJob(ctx, jobId).UserId(userId).OrgId(orgId).TenantId(tenantId).Execute()

Delete a specific job



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	jobId := "jobId_example" // string | 
	userId := "userId_example" // string | 
	orgId := "orgId_example" // string | 
	tenantId := "tenantId_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.LeadScraperServiceAPI.DeleteScrapingJob(context.Background(), jobId).UserId(userId).OrgId(orgId).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LeadScraperServiceAPI.DeleteScrapingJob``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DeleteScrapingJob`: DeleteScrapingJobResponse
	fmt.Fprintf(os.Stdout, "Response from `LeadScraperServiceAPI.DeleteScrapingJob`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**jobId** | **string** |  | 

### Other Parameters

Other parameters are passed through a pointer to a apiDeleteScrapingJobRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **userId** | **string** |  | 
 **orgId** | **string** |  | 
 **tenantId** | **string** |  | 

### Return type

[**DeleteScrapingJobResponse**](DeleteScrapingJobResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## DownloadScrapingResults

> DownloadScrapingResultsResponse DownloadScrapingResults(ctx, jobId).UserId(userId).OrgId(orgId).TenantId(tenantId).Execute()

Download job results as CSV



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	jobId := "jobId_example" // string | 
	userId := "userId_example" // string | 
	orgId := "orgId_example" // string | 
	tenantId := "tenantId_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.LeadScraperServiceAPI.DownloadScrapingResults(context.Background(), jobId).UserId(userId).OrgId(orgId).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LeadScraperServiceAPI.DownloadScrapingResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DownloadScrapingResults`: DownloadScrapingResultsResponse
	fmt.Fprintf(os.Stdout, "Response from `LeadScraperServiceAPI.DownloadScrapingResults`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**jobId** | **string** |  | 

### Other Parameters

Other parameters are passed through a pointer to a apiDownloadScrapingResultsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **userId** | **string** |  | 
 **orgId** | **string** |  | 
 **tenantId** | **string** |  | 

### Return type

[**DownloadScrapingResultsResponse**](DownloadScrapingResultsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetAccount

> GetAccountResponse GetAccount(ctx, accountId).OrgId(orgId).TenantId(tenantId).Execute()

Get account details



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	accountId := "accountId_example" // string | Account identifier
	orgId := "orgId_example" // string | Organization context
	tenantId := "tenantId_example" // string | Tenant context

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.LeadScraperServiceAPI.GetAccount(context.Background(), accountId).OrgId(orgId).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LeadScraperServiceAPI.GetAccount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetAccount`: GetAccountResponse
	fmt.Fprintf(os.Stdout, "Response from `LeadScraperServiceAPI.GetAccount`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**accountId** | **string** | Account identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiGetAccountRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **orgId** | **string** | Organization context | 
 **tenantId** | **string** | Tenant context | 

### Return type

[**GetAccountResponse**](GetAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetScrapingJob

> GetScrapingJobResponse GetScrapingJob(ctx, jobId).UserId(userId).OrgId(orgId).TenantId(tenantId).Execute()

Get a specific job



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	jobId := "jobId_example" // string | 
	userId := "userId_example" // string | 
	orgId := "orgId_example" // string | 
	tenantId := "tenantId_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.LeadScraperServiceAPI.GetScrapingJob(context.Background(), jobId).UserId(userId).OrgId(orgId).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LeadScraperServiceAPI.GetScrapingJob``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetScrapingJob`: GetScrapingJobResponse
	fmt.Fprintf(os.Stdout, "Response from `LeadScraperServiceAPI.GetScrapingJob`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**jobId** | **string** |  | 

### Other Parameters

Other parameters are passed through a pointer to a apiGetScrapingJobRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **userId** | **string** |  | 
 **orgId** | **string** |  | 
 **tenantId** | **string** |  | 

### Return type

[**GetScrapingJobResponse**](GetScrapingJobResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListAccounts

> ListAccountsResponse ListAccounts(ctx).OrgId(orgId).TenantId(tenantId).Offset(offset).Limit(limit).Execute()

List accounts



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	orgId := "orgId_example" // string | Organization context
	tenantId := "tenantId_example" // string | Tenant context
	offset := int32(56) // int32 | Pagination offset (optional)
	limit := int32(56) // int32 | Maximum number of results (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.LeadScraperServiceAPI.ListAccounts(context.Background()).OrgId(orgId).TenantId(tenantId).Offset(offset).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LeadScraperServiceAPI.ListAccounts``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListAccounts`: ListAccountsResponse
	fmt.Fprintf(os.Stdout, "Response from `LeadScraperServiceAPI.ListAccounts`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiListAccountsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **orgId** | **string** | Organization context | 
 **tenantId** | **string** | Tenant context | 
 **offset** | **int32** | Pagination offset | 
 **limit** | **int32** | Maximum number of results | 

### Return type

[**ListAccountsResponse**](ListAccountsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListScrapingJobs

> ListScrapingJobsResponse ListScrapingJobs(ctx).UserId(userId).OrgId(orgId).TenantId(tenantId).Execute()

Get all jobs



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	userId := "userId_example" // string | 
	orgId := "orgId_example" // string | 
	tenantId := "tenantId_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.LeadScraperServiceAPI.ListScrapingJobs(context.Background()).UserId(userId).OrgId(orgId).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LeadScraperServiceAPI.ListScrapingJobs``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListScrapingJobs`: ListScrapingJobsResponse
	fmt.Fprintf(os.Stdout, "Response from `LeadScraperServiceAPI.ListScrapingJobs`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiListScrapingJobsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **userId** | **string** |  | 
 **orgId** | **string** |  | 
 **tenantId** | **string** |  | 

### Return type

[**ListScrapingJobsResponse**](ListScrapingJobsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## UpdateAccount

> UpdateAccountResponse UpdateAccount(ctx).UpdateAccountRequest(updateAccountRequest).Execute()

Update account details



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	updateAccountRequest := *openapiclient.NewUpdateAccountRequest() // UpdateAccountRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.LeadScraperServiceAPI.UpdateAccount(context.Background()).UpdateAccountRequest(updateAccountRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LeadScraperServiceAPI.UpdateAccount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdateAccount`: UpdateAccountResponse
	fmt.Fprintf(os.Stdout, "Response from `LeadScraperServiceAPI.UpdateAccount`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiUpdateAccountRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **updateAccountRequest** | [**UpdateAccountRequest**](UpdateAccountRequest.md) |  | 

### Return type

[**UpdateAccountResponse**](UpdateAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

