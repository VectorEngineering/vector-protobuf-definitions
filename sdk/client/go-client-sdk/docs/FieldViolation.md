# FieldViolation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Field** | Pointer to **string** |  | [optional] 
**Validation** | Pointer to **string** |  | [optional] 
**Message** | Pointer to **string** |  | [optional] 
**Expected** | Pointer to **string** |  | [optional] 
**Actual** | Pointer to **string** |  | [optional] 

## Methods

### NewFieldViolation

`func NewFieldViolation() *FieldViolation`

NewFieldViolation instantiates a new FieldViolation object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewFieldViolationWithDefaults

`func NewFieldViolationWithDefaults() *FieldViolation`

NewFieldViolationWithDefaults instantiates a new FieldViolation object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetField

`func (o *FieldViolation) GetField() string`

GetField returns the Field field if non-nil, zero value otherwise.

### GetFieldOk

`func (o *FieldViolation) GetFieldOk() (*string, bool)`

GetFieldOk returns a tuple with the Field field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetField

`func (o *FieldViolation) SetField(v string)`

SetField sets Field field to given value.

### HasField

`func (o *FieldViolation) HasField() bool`

HasField returns a boolean if a field has been set.

### GetValidation

`func (o *FieldViolation) GetValidation() string`

GetValidation returns the Validation field if non-nil, zero value otherwise.

### GetValidationOk

`func (o *FieldViolation) GetValidationOk() (*string, bool)`

GetValidationOk returns a tuple with the Validation field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidation

`func (o *FieldViolation) SetValidation(v string)`

SetValidation sets Validation field to given value.

### HasValidation

`func (o *FieldViolation) HasValidation() bool`

HasValidation returns a boolean if a field has been set.

### GetMessage

`func (o *FieldViolation) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *FieldViolation) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *FieldViolation) SetMessage(v string)`

SetMessage sets Message field to given value.

### HasMessage

`func (o *FieldViolation) HasMessage() bool`

HasMessage returns a boolean if a field has been set.

### GetExpected

`func (o *FieldViolation) GetExpected() string`

GetExpected returns the Expected field if non-nil, zero value otherwise.

### GetExpectedOk

`func (o *FieldViolation) GetExpectedOk() (*string, bool)`

GetExpectedOk returns a tuple with the Expected field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpected

`func (o *FieldViolation) SetExpected(v string)`

SetExpected sets Expected field to given value.

### HasExpected

`func (o *FieldViolation) HasExpected() bool`

HasExpected returns a boolean if a field has been set.

### GetActual

`func (o *FieldViolation) GetActual() string`

GetActual returns the Actual field if non-nil, zero value otherwise.

### GetActualOk

`func (o *FieldViolation) GetActualOk() (*string, bool)`

GetActualOk returns a tuple with the Actual field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetActual

`func (o *FieldViolation) SetActual(v string)`

SetActual sets Actual field to given value.

### HasActual

`func (o *FieldViolation) HasActual() bool`

HasActual returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


