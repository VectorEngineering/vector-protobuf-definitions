# MethodNotAllowedErrorMessageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Code** | Pointer to **int32** |  | [optional] 
**Message** | Pointer to **string** |  | [optional] 
**AllowedMethods** | Pointer to **[]string** |  | [optional] 
**ErrorResponse** | Pointer to [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Methods

### NewMethodNotAllowedErrorMessageResponse

`func NewMethodNotAllowedErrorMessageResponse() *MethodNotAllowedErrorMessageResponse`

NewMethodNotAllowedErrorMessageResponse instantiates a new MethodNotAllowedErrorMessageResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMethodNotAllowedErrorMessageResponseWithDefaults

`func NewMethodNotAllowedErrorMessageResponseWithDefaults() *MethodNotAllowedErrorMessageResponse`

NewMethodNotAllowedErrorMessageResponseWithDefaults instantiates a new MethodNotAllowedErrorMessageResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCode

`func (o *MethodNotAllowedErrorMessageResponse) GetCode() int32`

GetCode returns the Code field if non-nil, zero value otherwise.

### GetCodeOk

`func (o *MethodNotAllowedErrorMessageResponse) GetCodeOk() (*int32, bool)`

GetCodeOk returns a tuple with the Code field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCode

`func (o *MethodNotAllowedErrorMessageResponse) SetCode(v int32)`

SetCode sets Code field to given value.

### HasCode

`func (o *MethodNotAllowedErrorMessageResponse) HasCode() bool`

HasCode returns a boolean if a field has been set.

### GetMessage

`func (o *MethodNotAllowedErrorMessageResponse) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *MethodNotAllowedErrorMessageResponse) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *MethodNotAllowedErrorMessageResponse) SetMessage(v string)`

SetMessage sets Message field to given value.

### HasMessage

`func (o *MethodNotAllowedErrorMessageResponse) HasMessage() bool`

HasMessage returns a boolean if a field has been set.

### GetAllowedMethods

`func (o *MethodNotAllowedErrorMessageResponse) GetAllowedMethods() []string`

GetAllowedMethods returns the AllowedMethods field if non-nil, zero value otherwise.

### GetAllowedMethodsOk

`func (o *MethodNotAllowedErrorMessageResponse) GetAllowedMethodsOk() (*[]string, bool)`

GetAllowedMethodsOk returns a tuple with the AllowedMethods field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedMethods

`func (o *MethodNotAllowedErrorMessageResponse) SetAllowedMethods(v []string)`

SetAllowedMethods sets AllowedMethods field to given value.

### HasAllowedMethods

`func (o *MethodNotAllowedErrorMessageResponse) HasAllowedMethods() bool`

HasAllowedMethods returns a boolean if a field has been set.

### GetErrorResponse

`func (o *MethodNotAllowedErrorMessageResponse) GetErrorResponse() ErrorResponse`

GetErrorResponse returns the ErrorResponse field if non-nil, zero value otherwise.

### GetErrorResponseOk

`func (o *MethodNotAllowedErrorMessageResponse) GetErrorResponseOk() (*ErrorResponse, bool)`

GetErrorResponseOk returns a tuple with the ErrorResponse field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetErrorResponse

`func (o *MethodNotAllowedErrorMessageResponse) SetErrorResponse(v ErrorResponse)`

SetErrorResponse sets ErrorResponse field to given value.

### HasErrorResponse

`func (o *MethodNotAllowedErrorMessageResponse) HasErrorResponse() bool`

HasErrorResponse returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


