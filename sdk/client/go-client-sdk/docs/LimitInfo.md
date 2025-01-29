# LimitInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Current** | Pointer to **int32** |  | [optional] 
**Limit** | Pointer to **int32** |  | [optional] 
**ResetTime** | Pointer to **time.Time** |  | [optional] 
**WindowSize** | Pointer to **string** |  | [optional] 

## Methods

### NewLimitInfo

`func NewLimitInfo() *LimitInfo`

NewLimitInfo instantiates a new LimitInfo object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewLimitInfoWithDefaults

`func NewLimitInfoWithDefaults() *LimitInfo`

NewLimitInfoWithDefaults instantiates a new LimitInfo object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCurrent

`func (o *LimitInfo) GetCurrent() int32`

GetCurrent returns the Current field if non-nil, zero value otherwise.

### GetCurrentOk

`func (o *LimitInfo) GetCurrentOk() (*int32, bool)`

GetCurrentOk returns a tuple with the Current field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCurrent

`func (o *LimitInfo) SetCurrent(v int32)`

SetCurrent sets Current field to given value.

### HasCurrent

`func (o *LimitInfo) HasCurrent() bool`

HasCurrent returns a boolean if a field has been set.

### GetLimit

`func (o *LimitInfo) GetLimit() int32`

GetLimit returns the Limit field if non-nil, zero value otherwise.

### GetLimitOk

`func (o *LimitInfo) GetLimitOk() (*int32, bool)`

GetLimitOk returns a tuple with the Limit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLimit

`func (o *LimitInfo) SetLimit(v int32)`

SetLimit sets Limit field to given value.

### HasLimit

`func (o *LimitInfo) HasLimit() bool`

HasLimit returns a boolean if a field has been set.

### GetResetTime

`func (o *LimitInfo) GetResetTime() time.Time`

GetResetTime returns the ResetTime field if non-nil, zero value otherwise.

### GetResetTimeOk

`func (o *LimitInfo) GetResetTimeOk() (*time.Time, bool)`

GetResetTimeOk returns a tuple with the ResetTime field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResetTime

`func (o *LimitInfo) SetResetTime(v time.Time)`

SetResetTime sets ResetTime field to given value.

### HasResetTime

`func (o *LimitInfo) HasResetTime() bool`

HasResetTime returns a boolean if a field has been set.

### GetWindowSize

`func (o *LimitInfo) GetWindowSize() string`

GetWindowSize returns the WindowSize field if non-nil, zero value otherwise.

### GetWindowSizeOk

`func (o *LimitInfo) GetWindowSizeOk() (*string, bool)`

GetWindowSizeOk returns a tuple with the WindowSize field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetWindowSize

`func (o *LimitInfo) SetWindowSize(v string)`

SetWindowSize sets WindowSize field to given value.

### HasWindowSize

`func (o *LimitInfo) HasWindowSize() bool`

HasWindowSize returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


