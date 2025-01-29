# ServiceStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Name** | Pointer to **string** |  | [optional] 
**Status** | Pointer to **string** |  | [optional] 
**Metrics** | Pointer to **map[string]string** |  | [optional] 
**Dependencies** | Pointer to [**[]Dependency**](Dependency.md) |  | [optional] 

## Methods

### NewServiceStatus

`func NewServiceStatus() *ServiceStatus`

NewServiceStatus instantiates a new ServiceStatus object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewServiceStatusWithDefaults

`func NewServiceStatusWithDefaults() *ServiceStatus`

NewServiceStatusWithDefaults instantiates a new ServiceStatus object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetName

`func (o *ServiceStatus) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ServiceStatus) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ServiceStatus) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *ServiceStatus) HasName() bool`

HasName returns a boolean if a field has been set.

### GetStatus

`func (o *ServiceStatus) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *ServiceStatus) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *ServiceStatus) SetStatus(v string)`

SetStatus sets Status field to given value.

### HasStatus

`func (o *ServiceStatus) HasStatus() bool`

HasStatus returns a boolean if a field has been set.

### GetMetrics

`func (o *ServiceStatus) GetMetrics() map[string]string`

GetMetrics returns the Metrics field if non-nil, zero value otherwise.

### GetMetricsOk

`func (o *ServiceStatus) GetMetricsOk() (*map[string]string, bool)`

GetMetricsOk returns a tuple with the Metrics field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMetrics

`func (o *ServiceStatus) SetMetrics(v map[string]string)`

SetMetrics sets Metrics field to given value.

### HasMetrics

`func (o *ServiceStatus) HasMetrics() bool`

HasMetrics returns a boolean if a field has been set.

### GetDependencies

`func (o *ServiceStatus) GetDependencies() []Dependency`

GetDependencies returns the Dependencies field if non-nil, zero value otherwise.

### GetDependenciesOk

`func (o *ServiceStatus) GetDependenciesOk() (*[]Dependency, bool)`

GetDependenciesOk returns a tuple with the Dependencies field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDependencies

`func (o *ServiceStatus) SetDependencies(v []Dependency)`

SetDependencies sets Dependencies field to given value.

### HasDependencies

`func (o *ServiceStatus) HasDependencies() bool`

HasDependencies returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


