# AuthContext

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**RequiredScopes** | Pointer to **[]string** |  | [optional] 
**ProvidedScopes** | Pointer to **[]string** |  | [optional] 
**MissingPermissions** | Pointer to **[]string** |  | [optional] 
**TenantId** | Pointer to **string** |  | [optional] 

## Methods

### NewAuthContext

`func NewAuthContext() *AuthContext`

NewAuthContext instantiates a new AuthContext object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAuthContextWithDefaults

`func NewAuthContextWithDefaults() *AuthContext`

NewAuthContextWithDefaults instantiates a new AuthContext object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetRequiredScopes

`func (o *AuthContext) GetRequiredScopes() []string`

GetRequiredScopes returns the RequiredScopes field if non-nil, zero value otherwise.

### GetRequiredScopesOk

`func (o *AuthContext) GetRequiredScopesOk() (*[]string, bool)`

GetRequiredScopesOk returns a tuple with the RequiredScopes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRequiredScopes

`func (o *AuthContext) SetRequiredScopes(v []string)`

SetRequiredScopes sets RequiredScopes field to given value.

### HasRequiredScopes

`func (o *AuthContext) HasRequiredScopes() bool`

HasRequiredScopes returns a boolean if a field has been set.

### GetProvidedScopes

`func (o *AuthContext) GetProvidedScopes() []string`

GetProvidedScopes returns the ProvidedScopes field if non-nil, zero value otherwise.

### GetProvidedScopesOk

`func (o *AuthContext) GetProvidedScopesOk() (*[]string, bool)`

GetProvidedScopesOk returns a tuple with the ProvidedScopes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProvidedScopes

`func (o *AuthContext) SetProvidedScopes(v []string)`

SetProvidedScopes sets ProvidedScopes field to given value.

### HasProvidedScopes

`func (o *AuthContext) HasProvidedScopes() bool`

HasProvidedScopes returns a boolean if a field has been set.

### GetMissingPermissions

`func (o *AuthContext) GetMissingPermissions() []string`

GetMissingPermissions returns the MissingPermissions field if non-nil, zero value otherwise.

### GetMissingPermissionsOk

`func (o *AuthContext) GetMissingPermissionsOk() (*[]string, bool)`

GetMissingPermissionsOk returns a tuple with the MissingPermissions field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMissingPermissions

`func (o *AuthContext) SetMissingPermissions(v []string)`

SetMissingPermissions sets MissingPermissions field to given value.

### HasMissingPermissions

`func (o *AuthContext) HasMissingPermissions() bool`

HasMissingPermissions returns a boolean if a field has been set.

### GetTenantId

`func (o *AuthContext) GetTenantId() string`

GetTenantId returns the TenantId field if non-nil, zero value otherwise.

### GetTenantIdOk

`func (o *AuthContext) GetTenantIdOk() (*string, bool)`

GetTenantIdOk returns a tuple with the TenantId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTenantId

`func (o *AuthContext) SetTenantId(v string)`

SetTenantId sets TenantId field to given value.

### HasTenantId

`func (o *AuthContext) HasTenantId() bool`

HasTenantId returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


