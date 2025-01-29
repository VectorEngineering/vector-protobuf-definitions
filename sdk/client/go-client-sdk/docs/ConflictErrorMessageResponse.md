# ConflictErrorMessageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Code** | Pointer to [**ValidationErrorCode**](ValidationErrorCode.md) |  | [optional] [default to NO_ERROR]
**Message** | Pointer to **string** |  | [optional] 
**ConflictInfo** | Pointer to [**ConflictInfo**](ConflictInfo.md) |  | [optional] 
**ErrorResponse** | Pointer to [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Methods

### NewConflictErrorMessageResponse

`func NewConflictErrorMessageResponse() *ConflictErrorMessageResponse`

NewConflictErrorMessageResponse instantiates a new ConflictErrorMessageResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewConflictErrorMessageResponseWithDefaults

`func NewConflictErrorMessageResponseWithDefaults() *ConflictErrorMessageResponse`

NewConflictErrorMessageResponseWithDefaults instantiates a new ConflictErrorMessageResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCode

`func (o *ConflictErrorMessageResponse) GetCode() ValidationErrorCode`

GetCode returns the Code field if non-nil, zero value otherwise.

### GetCodeOk

`func (o *ConflictErrorMessageResponse) GetCodeOk() (*ValidationErrorCode, bool)`

GetCodeOk returns a tuple with the Code field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCode

`func (o *ConflictErrorMessageResponse) SetCode(v ValidationErrorCode)`

SetCode sets Code field to given value.

### HasCode

`func (o *ConflictErrorMessageResponse) HasCode() bool`

HasCode returns a boolean if a field has been set.

### GetMessage

`func (o *ConflictErrorMessageResponse) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *ConflictErrorMessageResponse) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *ConflictErrorMessageResponse) SetMessage(v string)`

SetMessage sets Message field to given value.

### HasMessage

`func (o *ConflictErrorMessageResponse) HasMessage() bool`

HasMessage returns a boolean if a field has been set.

### GetConflictInfo

`func (o *ConflictErrorMessageResponse) GetConflictInfo() ConflictInfo`

GetConflictInfo returns the ConflictInfo field if non-nil, zero value otherwise.

### GetConflictInfoOk

`func (o *ConflictErrorMessageResponse) GetConflictInfoOk() (*ConflictInfo, bool)`

GetConflictInfoOk returns a tuple with the ConflictInfo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConflictInfo

`func (o *ConflictErrorMessageResponse) SetConflictInfo(v ConflictInfo)`

SetConflictInfo sets ConflictInfo field to given value.

### HasConflictInfo

`func (o *ConflictErrorMessageResponse) HasConflictInfo() bool`

HasConflictInfo returns a boolean if a field has been set.

### GetErrorResponse

`func (o *ConflictErrorMessageResponse) GetErrorResponse() ErrorResponse`

GetErrorResponse returns the ErrorResponse field if non-nil, zero value otherwise.

### GetErrorResponseOk

`func (o *ConflictErrorMessageResponse) GetErrorResponseOk() (*ErrorResponse, bool)`

GetErrorResponseOk returns a tuple with the ErrorResponse field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetErrorResponse

`func (o *ConflictErrorMessageResponse) SetErrorResponse(v ErrorResponse)`

SetErrorResponse sets ErrorResponse field to given value.

### HasErrorResponse

`func (o *ConflictErrorMessageResponse) HasErrorResponse() bool`

HasErrorResponse returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


