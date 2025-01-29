# RateLimitContext

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**UserId** | Pointer to **string** |  | [optional] 
**IpAddress** | Pointer to **string** |  | [optional] 
**AffectedEndpoints** | Pointer to **[]string** |  | [optional] 
**Headers** | Pointer to **map[string]string** |  | [optional] 

## Methods

### NewRateLimitContext

`func NewRateLimitContext() *RateLimitContext`

NewRateLimitContext instantiates a new RateLimitContext object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewRateLimitContextWithDefaults

`func NewRateLimitContextWithDefaults() *RateLimitContext`

NewRateLimitContextWithDefaults instantiates a new RateLimitContext object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetUserId

`func (o *RateLimitContext) GetUserId() string`

GetUserId returns the UserId field if non-nil, zero value otherwise.

### GetUserIdOk

`func (o *RateLimitContext) GetUserIdOk() (*string, bool)`

GetUserIdOk returns a tuple with the UserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserId

`func (o *RateLimitContext) SetUserId(v string)`

SetUserId sets UserId field to given value.

### HasUserId

`func (o *RateLimitContext) HasUserId() bool`

HasUserId returns a boolean if a field has been set.

### GetIpAddress

`func (o *RateLimitContext) GetIpAddress() string`

GetIpAddress returns the IpAddress field if non-nil, zero value otherwise.

### GetIpAddressOk

`func (o *RateLimitContext) GetIpAddressOk() (*string, bool)`

GetIpAddressOk returns a tuple with the IpAddress field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIpAddress

`func (o *RateLimitContext) SetIpAddress(v string)`

SetIpAddress sets IpAddress field to given value.

### HasIpAddress

`func (o *RateLimitContext) HasIpAddress() bool`

HasIpAddress returns a boolean if a field has been set.

### GetAffectedEndpoints

`func (o *RateLimitContext) GetAffectedEndpoints() []string`

GetAffectedEndpoints returns the AffectedEndpoints field if non-nil, zero value otherwise.

### GetAffectedEndpointsOk

`func (o *RateLimitContext) GetAffectedEndpointsOk() (*[]string, bool)`

GetAffectedEndpointsOk returns a tuple with the AffectedEndpoints field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAffectedEndpoints

`func (o *RateLimitContext) SetAffectedEndpoints(v []string)`

SetAffectedEndpoints sets AffectedEndpoints field to given value.

### HasAffectedEndpoints

`func (o *RateLimitContext) HasAffectedEndpoints() bool`

HasAffectedEndpoints returns a boolean if a field has been set.

### GetHeaders

`func (o *RateLimitContext) GetHeaders() map[string]string`

GetHeaders returns the Headers field if non-nil, zero value otherwise.

### GetHeadersOk

`func (o *RateLimitContext) GetHeadersOk() (*map[string]string, bool)`

GetHeadersOk returns a tuple with the Headers field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHeaders

`func (o *RateLimitContext) SetHeaders(v map[string]string)`

SetHeaders sets Headers field to given value.

### HasHeaders

`func (o *RateLimitContext) HasHeaders() bool`

HasHeaders returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


