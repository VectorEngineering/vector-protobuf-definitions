# OperationDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**OperationId** | Pointer to **string** |  | [optional] 
**StartTime** | Pointer to **time.Time** |  | [optional] 
**EndTime** | Pointer to **time.Time** |  | [optional] 
**StackTrace** | Pointer to **[]string** |  | [optional] 
**Parameters** | Pointer to **map[string]string** |  | [optional] 

## Methods

### NewOperationDetails

`func NewOperationDetails() *OperationDetails`

NewOperationDetails instantiates a new OperationDetails object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewOperationDetailsWithDefaults

`func NewOperationDetailsWithDefaults() *OperationDetails`

NewOperationDetailsWithDefaults instantiates a new OperationDetails object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetOperationId

`func (o *OperationDetails) GetOperationId() string`

GetOperationId returns the OperationId field if non-nil, zero value otherwise.

### GetOperationIdOk

`func (o *OperationDetails) GetOperationIdOk() (*string, bool)`

GetOperationIdOk returns a tuple with the OperationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOperationId

`func (o *OperationDetails) SetOperationId(v string)`

SetOperationId sets OperationId field to given value.

### HasOperationId

`func (o *OperationDetails) HasOperationId() bool`

HasOperationId returns a boolean if a field has been set.

### GetStartTime

`func (o *OperationDetails) GetStartTime() time.Time`

GetStartTime returns the StartTime field if non-nil, zero value otherwise.

### GetStartTimeOk

`func (o *OperationDetails) GetStartTimeOk() (*time.Time, bool)`

GetStartTimeOk returns a tuple with the StartTime field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStartTime

`func (o *OperationDetails) SetStartTime(v time.Time)`

SetStartTime sets StartTime field to given value.

### HasStartTime

`func (o *OperationDetails) HasStartTime() bool`

HasStartTime returns a boolean if a field has been set.

### GetEndTime

`func (o *OperationDetails) GetEndTime() time.Time`

GetEndTime returns the EndTime field if non-nil, zero value otherwise.

### GetEndTimeOk

`func (o *OperationDetails) GetEndTimeOk() (*time.Time, bool)`

GetEndTimeOk returns a tuple with the EndTime field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEndTime

`func (o *OperationDetails) SetEndTime(v time.Time)`

SetEndTime sets EndTime field to given value.

### HasEndTime

`func (o *OperationDetails) HasEndTime() bool`

HasEndTime returns a boolean if a field has been set.

### GetStackTrace

`func (o *OperationDetails) GetStackTrace() []string`

GetStackTrace returns the StackTrace field if non-nil, zero value otherwise.

### GetStackTraceOk

`func (o *OperationDetails) GetStackTraceOk() (*[]string, bool)`

GetStackTraceOk returns a tuple with the StackTrace field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStackTrace

`func (o *OperationDetails) SetStackTrace(v []string)`

SetStackTrace sets StackTrace field to given value.

### HasStackTrace

`func (o *OperationDetails) HasStackTrace() bool`

HasStackTrace returns a boolean if a field has been set.

### GetParameters

`func (o *OperationDetails) GetParameters() map[string]string`

GetParameters returns the Parameters field if non-nil, zero value otherwise.

### GetParametersOk

`func (o *OperationDetails) GetParametersOk() (*map[string]string, bool)`

GetParametersOk returns a tuple with the Parameters field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetParameters

`func (o *OperationDetails) SetParameters(v map[string]string)`

SetParameters sets Parameters field to given value.

### HasParameters

`func (o *OperationDetails) HasParameters() bool`

HasParameters returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


