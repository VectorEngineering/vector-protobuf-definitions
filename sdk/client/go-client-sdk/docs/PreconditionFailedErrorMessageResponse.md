# PreconditionFailedErrorMessageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Code** | Pointer to **int32** |  | [optional] 
**Message** | Pointer to **string** |  | [optional] 
**FailedConditions** | Pointer to **[]string** |  | [optional] 
**ErrorResponse** | Pointer to [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Methods

### NewPreconditionFailedErrorMessageResponse

`func NewPreconditionFailedErrorMessageResponse() *PreconditionFailedErrorMessageResponse`

NewPreconditionFailedErrorMessageResponse instantiates a new PreconditionFailedErrorMessageResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPreconditionFailedErrorMessageResponseWithDefaults

`func NewPreconditionFailedErrorMessageResponseWithDefaults() *PreconditionFailedErrorMessageResponse`

NewPreconditionFailedErrorMessageResponseWithDefaults instantiates a new PreconditionFailedErrorMessageResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCode

`func (o *PreconditionFailedErrorMessageResponse) GetCode() int32`

GetCode returns the Code field if non-nil, zero value otherwise.

### GetCodeOk

`func (o *PreconditionFailedErrorMessageResponse) GetCodeOk() (*int32, bool)`

GetCodeOk returns a tuple with the Code field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCode

`func (o *PreconditionFailedErrorMessageResponse) SetCode(v int32)`

SetCode sets Code field to given value.

### HasCode

`func (o *PreconditionFailedErrorMessageResponse) HasCode() bool`

HasCode returns a boolean if a field has been set.

### GetMessage

`func (o *PreconditionFailedErrorMessageResponse) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *PreconditionFailedErrorMessageResponse) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *PreconditionFailedErrorMessageResponse) SetMessage(v string)`

SetMessage sets Message field to given value.

### HasMessage

`func (o *PreconditionFailedErrorMessageResponse) HasMessage() bool`

HasMessage returns a boolean if a field has been set.

### GetFailedConditions

`func (o *PreconditionFailedErrorMessageResponse) GetFailedConditions() []string`

GetFailedConditions returns the FailedConditions field if non-nil, zero value otherwise.

### GetFailedConditionsOk

`func (o *PreconditionFailedErrorMessageResponse) GetFailedConditionsOk() (*[]string, bool)`

GetFailedConditionsOk returns a tuple with the FailedConditions field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFailedConditions

`func (o *PreconditionFailedErrorMessageResponse) SetFailedConditions(v []string)`

SetFailedConditions sets FailedConditions field to given value.

### HasFailedConditions

`func (o *PreconditionFailedErrorMessageResponse) HasFailedConditions() bool`

HasFailedConditions returns a boolean if a field has been set.

### GetErrorResponse

`func (o *PreconditionFailedErrorMessageResponse) GetErrorResponse() ErrorResponse`

GetErrorResponse returns the ErrorResponse field if non-nil, zero value otherwise.

### GetErrorResponseOk

`func (o *PreconditionFailedErrorMessageResponse) GetErrorResponseOk() (*ErrorResponse, bool)`

GetErrorResponseOk returns a tuple with the ErrorResponse field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetErrorResponse

`func (o *PreconditionFailedErrorMessageResponse) SetErrorResponse(v ErrorResponse)`

SetErrorResponse sets ErrorResponse field to given value.

### HasErrorResponse

`func (o *PreconditionFailedErrorMessageResponse) HasErrorResponse() bool`

HasErrorResponse returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


