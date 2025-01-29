# ResourceUtilization

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CpuUsage** | Pointer to **float64** |  | [optional] 
**MemoryUsage** | Pointer to **float64** |  | [optional] 
**ActiveConnections** | Pointer to **int32** |  | [optional] 
**Quotas** | Pointer to **map[string]float64** |  | [optional] 

## Methods

### NewResourceUtilization

`func NewResourceUtilization() *ResourceUtilization`

NewResourceUtilization instantiates a new ResourceUtilization object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewResourceUtilizationWithDefaults

`func NewResourceUtilizationWithDefaults() *ResourceUtilization`

NewResourceUtilizationWithDefaults instantiates a new ResourceUtilization object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCpuUsage

`func (o *ResourceUtilization) GetCpuUsage() float64`

GetCpuUsage returns the CpuUsage field if non-nil, zero value otherwise.

### GetCpuUsageOk

`func (o *ResourceUtilization) GetCpuUsageOk() (*float64, bool)`

GetCpuUsageOk returns a tuple with the CpuUsage field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCpuUsage

`func (o *ResourceUtilization) SetCpuUsage(v float64)`

SetCpuUsage sets CpuUsage field to given value.

### HasCpuUsage

`func (o *ResourceUtilization) HasCpuUsage() bool`

HasCpuUsage returns a boolean if a field has been set.

### GetMemoryUsage

`func (o *ResourceUtilization) GetMemoryUsage() float64`

GetMemoryUsage returns the MemoryUsage field if non-nil, zero value otherwise.

### GetMemoryUsageOk

`func (o *ResourceUtilization) GetMemoryUsageOk() (*float64, bool)`

GetMemoryUsageOk returns a tuple with the MemoryUsage field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMemoryUsage

`func (o *ResourceUtilization) SetMemoryUsage(v float64)`

SetMemoryUsage sets MemoryUsage field to given value.

### HasMemoryUsage

`func (o *ResourceUtilization) HasMemoryUsage() bool`

HasMemoryUsage returns a boolean if a field has been set.

### GetActiveConnections

`func (o *ResourceUtilization) GetActiveConnections() int32`

GetActiveConnections returns the ActiveConnections field if non-nil, zero value otherwise.

### GetActiveConnectionsOk

`func (o *ResourceUtilization) GetActiveConnectionsOk() (*int32, bool)`

GetActiveConnectionsOk returns a tuple with the ActiveConnections field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetActiveConnections

`func (o *ResourceUtilization) SetActiveConnections(v int32)`

SetActiveConnections sets ActiveConnections field to given value.

### HasActiveConnections

`func (o *ResourceUtilization) HasActiveConnections() bool`

HasActiveConnections returns a boolean if a field has been set.

### GetQuotas

`func (o *ResourceUtilization) GetQuotas() map[string]float64`

GetQuotas returns the Quotas field if non-nil, zero value otherwise.

### GetQuotasOk

`func (o *ResourceUtilization) GetQuotasOk() (*map[string]float64, bool)`

GetQuotasOk returns a tuple with the Quotas field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetQuotas

`func (o *ResourceUtilization) SetQuotas(v map[string]float64)`

SetQuotas sets Quotas field to given value.

### HasQuotas

`func (o *ResourceUtilization) HasQuotas() bool`

HasQuotas returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


