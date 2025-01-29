# Account

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Id** | Pointer to **string** |  | [optional] 
**AuthPlatformUserId** | Pointer to **string** |  | [optional] 
**OrgId** | Pointer to **string** |  | [optional] 
**TenantId** | Pointer to **string** |  | [optional] 
**Email** | Pointer to **string** |  | [optional] 
**LastModifiedAt** | Pointer to **time.Time** |  | [optional] 
**DeletedAt** | Pointer to **time.Time** |  | [optional] 
**CreatedAt** | Pointer to **time.Time** |  | [optional] 
**ScrapingJobs** | Pointer to [**[]ScrapingJob**](ScrapingJob.md) |  | [optional] 

## Methods

### NewAccount

`func NewAccount() *Account`

NewAccount instantiates a new Account object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAccountWithDefaults

`func NewAccountWithDefaults() *Account`

NewAccountWithDefaults instantiates a new Account object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetId

`func (o *Account) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *Account) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *Account) SetId(v string)`

SetId sets Id field to given value.

### HasId

`func (o *Account) HasId() bool`

HasId returns a boolean if a field has been set.

### GetAuthPlatformUserId

`func (o *Account) GetAuthPlatformUserId() string`

GetAuthPlatformUserId returns the AuthPlatformUserId field if non-nil, zero value otherwise.

### GetAuthPlatformUserIdOk

`func (o *Account) GetAuthPlatformUserIdOk() (*string, bool)`

GetAuthPlatformUserIdOk returns a tuple with the AuthPlatformUserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAuthPlatformUserId

`func (o *Account) SetAuthPlatformUserId(v string)`

SetAuthPlatformUserId sets AuthPlatformUserId field to given value.

### HasAuthPlatformUserId

`func (o *Account) HasAuthPlatformUserId() bool`

HasAuthPlatformUserId returns a boolean if a field has been set.

### GetOrgId

`func (o *Account) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *Account) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *Account) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.

### HasOrgId

`func (o *Account) HasOrgId() bool`

HasOrgId returns a boolean if a field has been set.

### GetTenantId

`func (o *Account) GetTenantId() string`

GetTenantId returns the TenantId field if non-nil, zero value otherwise.

### GetTenantIdOk

`func (o *Account) GetTenantIdOk() (*string, bool)`

GetTenantIdOk returns a tuple with the TenantId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTenantId

`func (o *Account) SetTenantId(v string)`

SetTenantId sets TenantId field to given value.

### HasTenantId

`func (o *Account) HasTenantId() bool`

HasTenantId returns a boolean if a field has been set.

### GetEmail

`func (o *Account) GetEmail() string`

GetEmail returns the Email field if non-nil, zero value otherwise.

### GetEmailOk

`func (o *Account) GetEmailOk() (*string, bool)`

GetEmailOk returns a tuple with the Email field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmail

`func (o *Account) SetEmail(v string)`

SetEmail sets Email field to given value.

### HasEmail

`func (o *Account) HasEmail() bool`

HasEmail returns a boolean if a field has been set.

### GetLastModifiedAt

`func (o *Account) GetLastModifiedAt() time.Time`

GetLastModifiedAt returns the LastModifiedAt field if non-nil, zero value otherwise.

### GetLastModifiedAtOk

`func (o *Account) GetLastModifiedAtOk() (*time.Time, bool)`

GetLastModifiedAtOk returns a tuple with the LastModifiedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastModifiedAt

`func (o *Account) SetLastModifiedAt(v time.Time)`

SetLastModifiedAt sets LastModifiedAt field to given value.

### HasLastModifiedAt

`func (o *Account) HasLastModifiedAt() bool`

HasLastModifiedAt returns a boolean if a field has been set.

### GetDeletedAt

`func (o *Account) GetDeletedAt() time.Time`

GetDeletedAt returns the DeletedAt field if non-nil, zero value otherwise.

### GetDeletedAtOk

`func (o *Account) GetDeletedAtOk() (*time.Time, bool)`

GetDeletedAtOk returns a tuple with the DeletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeletedAt

`func (o *Account) SetDeletedAt(v time.Time)`

SetDeletedAt sets DeletedAt field to given value.

### HasDeletedAt

`func (o *Account) HasDeletedAt() bool`

HasDeletedAt returns a boolean if a field has been set.

### GetCreatedAt

`func (o *Account) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *Account) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *Account) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *Account) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### GetScrapingJobs

`func (o *Account) GetScrapingJobs() []ScrapingJob`

GetScrapingJobs returns the ScrapingJobs field if non-nil, zero value otherwise.

### GetScrapingJobsOk

`func (o *Account) GetScrapingJobsOk() (*[]ScrapingJob, bool)`

GetScrapingJobsOk returns a tuple with the ScrapingJobs field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScrapingJobs

`func (o *Account) SetScrapingJobs(v []ScrapingJob)`

SetScrapingJobs sets ScrapingJobs field to given value.

### HasScrapingJobs

`func (o *Account) HasScrapingJobs() bool`

HasScrapingJobs returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


