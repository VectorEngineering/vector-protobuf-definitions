# QuotaInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Resource** | Pointer to **string** |  | [optional] 
**Used** | Pointer to **string** |  | [optional] 
**Limit** | Pointer to **string** |  | [optional] 
**ResetTime** | Pointer to **time.Time** |  | [optional] 
**ResourceUsage** | Pointer to **map[string]string** |  | [optional] 

## Methods

### NewQuotaInfo

`func NewQuotaInfo() *QuotaInfo`

NewQuotaInfo instantiates a new QuotaInfo object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewQuotaInfoWithDefaults

`func NewQuotaInfoWithDefaults() *QuotaInfo`

NewQuotaInfoWithDefaults instantiates a new QuotaInfo object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetResource

`func (o *QuotaInfo) GetResource() string`

GetResource returns the Resource field if non-nil, zero value otherwise.

### GetResourceOk

`func (o *QuotaInfo) GetResourceOk() (*string, bool)`

GetResourceOk returns a tuple with the Resource field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResource

`func (o *QuotaInfo) SetResource(v string)`

SetResource sets Resource field to given value.

### HasResource

`func (o *QuotaInfo) HasResource() bool`

HasResource returns a boolean if a field has been set.

### GetUsed

`func (o *QuotaInfo) GetUsed() string`

GetUsed returns the Used field if non-nil, zero value otherwise.

### GetUsedOk

`func (o *QuotaInfo) GetUsedOk() (*string, bool)`

GetUsedOk returns a tuple with the Used field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUsed

`func (o *QuotaInfo) SetUsed(v string)`

SetUsed sets Used field to given value.

### HasUsed

`func (o *QuotaInfo) HasUsed() bool`

HasUsed returns a boolean if a field has been set.

### GetLimit

`func (o *QuotaInfo) GetLimit() string`

GetLimit returns the Limit field if non-nil, zero value otherwise.

### GetLimitOk

`func (o *QuotaInfo) GetLimitOk() (*string, bool)`

GetLimitOk returns a tuple with the Limit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLimit

`func (o *QuotaInfo) SetLimit(v string)`

SetLimit sets Limit field to given value.

### HasLimit

`func (o *QuotaInfo) HasLimit() bool`

HasLimit returns a boolean if a field has been set.

### GetResetTime

`func (o *QuotaInfo) GetResetTime() time.Time`

GetResetTime returns the ResetTime field if non-nil, zero value otherwise.

### GetResetTimeOk

`func (o *QuotaInfo) GetResetTimeOk() (*time.Time, bool)`

GetResetTimeOk returns a tuple with the ResetTime field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResetTime

`func (o *QuotaInfo) SetResetTime(v time.Time)`

SetResetTime sets ResetTime field to given value.

### HasResetTime

`func (o *QuotaInfo) HasResetTime() bool`

HasResetTime returns a boolean if a field has been set.

### GetResourceUsage

`func (o *QuotaInfo) GetResourceUsage() map[string]string`

GetResourceUsage returns the ResourceUsage field if non-nil, zero value otherwise.

### GetResourceUsageOk

`func (o *QuotaInfo) GetResourceUsageOk() (*map[string]string, bool)`

GetResourceUsageOk returns a tuple with the ResourceUsage field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResourceUsage

`func (o *QuotaInfo) SetResourceUsage(v map[string]string)`

SetResourceUsage sets ResourceUsage field to given value.

### HasResourceUsage

`func (o *QuotaInfo) HasResourceUsage() bool`

HasResourceUsage returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


