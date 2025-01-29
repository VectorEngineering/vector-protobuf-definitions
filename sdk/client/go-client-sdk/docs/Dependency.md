# Dependency

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Name** | Pointer to **string** |  | [optional] 
**Status** | Pointer to **string** |  | [optional] 
**Error** | Pointer to **string** |  | [optional] 
**Latency** | Pointer to **int32** |  | [optional] 

## Methods

### NewDependency

`func NewDependency() *Dependency`

NewDependency instantiates a new Dependency object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewDependencyWithDefaults

`func NewDependencyWithDefaults() *Dependency`

NewDependencyWithDefaults instantiates a new Dependency object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetName

`func (o *Dependency) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *Dependency) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *Dependency) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *Dependency) HasName() bool`

HasName returns a boolean if a field has been set.

### GetStatus

`func (o *Dependency) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *Dependency) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *Dependency) SetStatus(v string)`

SetStatus sets Status field to given value.

### HasStatus

`func (o *Dependency) HasStatus() bool`

HasStatus returns a boolean if a field has been set.

### GetError

`func (o *Dependency) GetError() string`

GetError returns the Error field if non-nil, zero value otherwise.

### GetErrorOk

`func (o *Dependency) GetErrorOk() (*string, bool)`

GetErrorOk returns a tuple with the Error field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetError

`func (o *Dependency) SetError(v string)`

SetError sets Error field to given value.

### HasError

`func (o *Dependency) HasError() bool`

HasError returns a boolean if a field has been set.

### GetLatency

`func (o *Dependency) GetLatency() int32`

GetLatency returns the Latency field if non-nil, zero value otherwise.

### GetLatencyOk

`func (o *Dependency) GetLatencyOk() (*int32, bool)`

GetLatencyOk returns a tuple with the Latency field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLatency

`func (o *Dependency) SetLatency(v int32)`

SetLatency sets Latency field to given value.

### HasLatency

`func (o *Dependency) HasLatency() bool`

HasLatency returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


