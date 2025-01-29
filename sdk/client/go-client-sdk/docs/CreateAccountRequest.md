# CreateAccountRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AuthPlatformUserId** | **string** |  | 
**OrgId** | **string** |  | 
**TenantId** | **string** |  | 
**Email** | **string** |  | 

## Methods

### NewCreateAccountRequest

`func NewCreateAccountRequest(authPlatformUserId string, orgId string, tenantId string, email string, ) *CreateAccountRequest`

NewCreateAccountRequest instantiates a new CreateAccountRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateAccountRequestWithDefaults

`func NewCreateAccountRequestWithDefaults() *CreateAccountRequest`

NewCreateAccountRequestWithDefaults instantiates a new CreateAccountRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAuthPlatformUserId

`func (o *CreateAccountRequest) GetAuthPlatformUserId() string`

GetAuthPlatformUserId returns the AuthPlatformUserId field if non-nil, zero value otherwise.

### GetAuthPlatformUserIdOk

`func (o *CreateAccountRequest) GetAuthPlatformUserIdOk() (*string, bool)`

GetAuthPlatformUserIdOk returns a tuple with the AuthPlatformUserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAuthPlatformUserId

`func (o *CreateAccountRequest) SetAuthPlatformUserId(v string)`

SetAuthPlatformUserId sets AuthPlatformUserId field to given value.


### GetOrgId

`func (o *CreateAccountRequest) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *CreateAccountRequest) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *CreateAccountRequest) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetTenantId

`func (o *CreateAccountRequest) GetTenantId() string`

GetTenantId returns the TenantId field if non-nil, zero value otherwise.

### GetTenantIdOk

`func (o *CreateAccountRequest) GetTenantIdOk() (*string, bool)`

GetTenantIdOk returns a tuple with the TenantId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTenantId

`func (o *CreateAccountRequest) SetTenantId(v string)`

SetTenantId sets TenantId field to given value.


### GetEmail

`func (o *CreateAccountRequest) GetEmail() string`

GetEmail returns the Email field if non-nil, zero value otherwise.

### GetEmailOk

`func (o *CreateAccountRequest) GetEmailOk() (*string, bool)`

GetEmailOk returns a tuple with the Email field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmail

`func (o *CreateAccountRequest) SetEmail(v string)`

SetEmail sets Email field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


