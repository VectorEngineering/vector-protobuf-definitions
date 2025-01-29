# UnprocessableEntityErrorMessageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Code** | Pointer to **int32** |  | [optional] 
**Message** | Pointer to **string** |  | [optional] 
**Errors** | Pointer to [**[]FieldError**](FieldError.md) |  | [optional] 
**ErrorResponse** | Pointer to [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Methods

### NewUnprocessableEntityErrorMessageResponse

`func NewUnprocessableEntityErrorMessageResponse() *UnprocessableEntityErrorMessageResponse`

NewUnprocessableEntityErrorMessageResponse instantiates a new UnprocessableEntityErrorMessageResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUnprocessableEntityErrorMessageResponseWithDefaults

`func NewUnprocessableEntityErrorMessageResponseWithDefaults() *UnprocessableEntityErrorMessageResponse`

NewUnprocessableEntityErrorMessageResponseWithDefaults instantiates a new UnprocessableEntityErrorMessageResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCode

`func (o *UnprocessableEntityErrorMessageResponse) GetCode() int32`

GetCode returns the Code field if non-nil, zero value otherwise.

### GetCodeOk

`func (o *UnprocessableEntityErrorMessageResponse) GetCodeOk() (*int32, bool)`

GetCodeOk returns a tuple with the Code field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCode

`func (o *UnprocessableEntityErrorMessageResponse) SetCode(v int32)`

SetCode sets Code field to given value.

### HasCode

`func (o *UnprocessableEntityErrorMessageResponse) HasCode() bool`

HasCode returns a boolean if a field has been set.

### GetMessage

`func (o *UnprocessableEntityErrorMessageResponse) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *UnprocessableEntityErrorMessageResponse) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *UnprocessableEntityErrorMessageResponse) SetMessage(v string)`

SetMessage sets Message field to given value.

### HasMessage

`func (o *UnprocessableEntityErrorMessageResponse) HasMessage() bool`

HasMessage returns a boolean if a field has been set.

### GetErrors

`func (o *UnprocessableEntityErrorMessageResponse) GetErrors() []FieldError`

GetErrors returns the Errors field if non-nil, zero value otherwise.

### GetErrorsOk

`func (o *UnprocessableEntityErrorMessageResponse) GetErrorsOk() (*[]FieldError, bool)`

GetErrorsOk returns a tuple with the Errors field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetErrors

`func (o *UnprocessableEntityErrorMessageResponse) SetErrors(v []FieldError)`

SetErrors sets Errors field to given value.

### HasErrors

`func (o *UnprocessableEntityErrorMessageResponse) HasErrors() bool`

HasErrors returns a boolean if a field has been set.

### GetErrorResponse

`func (o *UnprocessableEntityErrorMessageResponse) GetErrorResponse() ErrorResponse`

GetErrorResponse returns the ErrorResponse field if non-nil, zero value otherwise.

### GetErrorResponseOk

`func (o *UnprocessableEntityErrorMessageResponse) GetErrorResponseOk() (*ErrorResponse, bool)`

GetErrorResponseOk returns a tuple with the ErrorResponse field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetErrorResponse

`func (o *UnprocessableEntityErrorMessageResponse) SetErrorResponse(v ErrorResponse)`

SetErrorResponse sets ErrorResponse field to given value.

### HasErrorResponse

`func (o *UnprocessableEntityErrorMessageResponse) HasErrorResponse() bool`

HasErrorResponse returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


