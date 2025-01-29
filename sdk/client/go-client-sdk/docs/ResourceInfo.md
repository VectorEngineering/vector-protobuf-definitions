# ResourceInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Type** | Pointer to **string** |  | [optional] 
**Id** | Pointer to **string** |  | [optional] 
**Path** | Pointer to **string** |  | [optional] 
**TenantId** | Pointer to **string** |  | [optional] 
**Scopes** | Pointer to **[]string** |  | [optional] 

## Methods

### NewResourceInfo

`func NewResourceInfo() *ResourceInfo`

NewResourceInfo instantiates a new ResourceInfo object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewResourceInfoWithDefaults

`func NewResourceInfoWithDefaults() *ResourceInfo`

NewResourceInfoWithDefaults instantiates a new ResourceInfo object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetType

`func (o *ResourceInfo) GetType() string`

GetType returns the Type field if non-nil, zero value otherwise.

### GetTypeOk

`func (o *ResourceInfo) GetTypeOk() (*string, bool)`

GetTypeOk returns a tuple with the Type field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetType

`func (o *ResourceInfo) SetType(v string)`

SetType sets Type field to given value.

### HasType

`func (o *ResourceInfo) HasType() bool`

HasType returns a boolean if a field has been set.

### GetId

`func (o *ResourceInfo) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *ResourceInfo) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *ResourceInfo) SetId(v string)`

SetId sets Id field to given value.

### HasId

`func (o *ResourceInfo) HasId() bool`

HasId returns a boolean if a field has been set.

### GetPath

`func (o *ResourceInfo) GetPath() string`

GetPath returns the Path field if non-nil, zero value otherwise.

### GetPathOk

`func (o *ResourceInfo) GetPathOk() (*string, bool)`

GetPathOk returns a tuple with the Path field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPath

`func (o *ResourceInfo) SetPath(v string)`

SetPath sets Path field to given value.

### HasPath

`func (o *ResourceInfo) HasPath() bool`

HasPath returns a boolean if a field has been set.

### GetTenantId

`func (o *ResourceInfo) GetTenantId() string`

GetTenantId returns the TenantId field if non-nil, zero value otherwise.

### GetTenantIdOk

`func (o *ResourceInfo) GetTenantIdOk() (*string, bool)`

GetTenantIdOk returns a tuple with the TenantId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTenantId

`func (o *ResourceInfo) SetTenantId(v string)`

SetTenantId sets TenantId field to given value.

### HasTenantId

`func (o *ResourceInfo) HasTenantId() bool`

HasTenantId returns a boolean if a field has been set.

### GetScopes

`func (o *ResourceInfo) GetScopes() []string`

GetScopes returns the Scopes field if non-nil, zero value otherwise.

### GetScopesOk

`func (o *ResourceInfo) GetScopesOk() (*[]string, bool)`

GetScopesOk returns a tuple with the Scopes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScopes

`func (o *ResourceInfo) SetScopes(v []string)`

SetScopes sets Scopes field to given value.

### HasScopes

`func (o *ResourceInfo) HasScopes() bool`

HasScopes returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


