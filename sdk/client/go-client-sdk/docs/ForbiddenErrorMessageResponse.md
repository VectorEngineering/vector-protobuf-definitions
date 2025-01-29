# ForbiddenErrorMessageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Code** | Pointer to **int32** |  | [optional] 
**Message** | Pointer to **string** |  | [optional] 
**Reason** | Pointer to **string** |  | [optional] 
**RequiredPermissions** | Pointer to **[]string** |  | [optional] 
**ErrorResponse** | Pointer to [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Methods

### NewForbiddenErrorMessageResponse

`func NewForbiddenErrorMessageResponse() *ForbiddenErrorMessageResponse`

NewForbiddenErrorMessageResponse instantiates a new ForbiddenErrorMessageResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewForbiddenErrorMessageResponseWithDefaults

`func NewForbiddenErrorMessageResponseWithDefaults() *ForbiddenErrorMessageResponse`

NewForbiddenErrorMessageResponseWithDefaults instantiates a new ForbiddenErrorMessageResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCode

`func (o *ForbiddenErrorMessageResponse) GetCode() int32`

GetCode returns the Code field if non-nil, zero value otherwise.

### GetCodeOk

`func (o *ForbiddenErrorMessageResponse) GetCodeOk() (*int32, bool)`

GetCodeOk returns a tuple with the Code field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCode

`func (o *ForbiddenErrorMessageResponse) SetCode(v int32)`

SetCode sets Code field to given value.

### HasCode

`func (o *ForbiddenErrorMessageResponse) HasCode() bool`

HasCode returns a boolean if a field has been set.

### GetMessage

`func (o *ForbiddenErrorMessageResponse) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *ForbiddenErrorMessageResponse) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *ForbiddenErrorMessageResponse) SetMessage(v string)`

SetMessage sets Message field to given value.

### HasMessage

`func (o *ForbiddenErrorMessageResponse) HasMessage() bool`

HasMessage returns a boolean if a field has been set.

### GetReason

`func (o *ForbiddenErrorMessageResponse) GetReason() string`

GetReason returns the Reason field if non-nil, zero value otherwise.

### GetReasonOk

`func (o *ForbiddenErrorMessageResponse) GetReasonOk() (*string, bool)`

GetReasonOk returns a tuple with the Reason field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReason

`func (o *ForbiddenErrorMessageResponse) SetReason(v string)`

SetReason sets Reason field to given value.

### HasReason

`func (o *ForbiddenErrorMessageResponse) HasReason() bool`

HasReason returns a boolean if a field has been set.

### GetRequiredPermissions

`func (o *ForbiddenErrorMessageResponse) GetRequiredPermissions() []string`

GetRequiredPermissions returns the RequiredPermissions field if non-nil, zero value otherwise.

### GetRequiredPermissionsOk

`func (o *ForbiddenErrorMessageResponse) GetRequiredPermissionsOk() (*[]string, bool)`

GetRequiredPermissionsOk returns a tuple with the RequiredPermissions field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRequiredPermissions

`func (o *ForbiddenErrorMessageResponse) SetRequiredPermissions(v []string)`

SetRequiredPermissions sets RequiredPermissions field to given value.

### HasRequiredPermissions

`func (o *ForbiddenErrorMessageResponse) HasRequiredPermissions() bool`

HasRequiredPermissions returns a boolean if a field has been set.

### GetErrorResponse

`func (o *ForbiddenErrorMessageResponse) GetErrorResponse() ErrorResponse`

GetErrorResponse returns the ErrorResponse field if non-nil, zero value otherwise.

### GetErrorResponseOk

`func (o *ForbiddenErrorMessageResponse) GetErrorResponseOk() (*ErrorResponse, bool)`

GetErrorResponseOk returns a tuple with the ErrorResponse field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetErrorResponse

`func (o *ForbiddenErrorMessageResponse) SetErrorResponse(v ErrorResponse)`

SetErrorResponse sets ErrorResponse field to given value.

### HasErrorResponse

`func (o *ForbiddenErrorMessageResponse) HasErrorResponse() bool`

HasErrorResponse returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


