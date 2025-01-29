# NotFoundErrorMessageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Code** | Pointer to [**NotFoundErrorCode**](NotFoundErrorCode.md) |  | [optional] [default to NO_NOT_FOUND_ERROR]
**Message** | Pointer to **string** |  | [optional] 
**ResourceInfo** | Pointer to [**ResourceInfo**](ResourceInfo.md) |  | [optional] 
**Suggestions** | Pointer to [**Suggestions**](Suggestions.md) |  | [optional] 
**ApiInfo** | Pointer to [**APIInfo**](APIInfo.md) |  | [optional] 
**ErrorResponse** | Pointer to [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Methods

### NewNotFoundErrorMessageResponse

`func NewNotFoundErrorMessageResponse() *NotFoundErrorMessageResponse`

NewNotFoundErrorMessageResponse instantiates a new NotFoundErrorMessageResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewNotFoundErrorMessageResponseWithDefaults

`func NewNotFoundErrorMessageResponseWithDefaults() *NotFoundErrorMessageResponse`

NewNotFoundErrorMessageResponseWithDefaults instantiates a new NotFoundErrorMessageResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCode

`func (o *NotFoundErrorMessageResponse) GetCode() NotFoundErrorCode`

GetCode returns the Code field if non-nil, zero value otherwise.

### GetCodeOk

`func (o *NotFoundErrorMessageResponse) GetCodeOk() (*NotFoundErrorCode, bool)`

GetCodeOk returns a tuple with the Code field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCode

`func (o *NotFoundErrorMessageResponse) SetCode(v NotFoundErrorCode)`

SetCode sets Code field to given value.

### HasCode

`func (o *NotFoundErrorMessageResponse) HasCode() bool`

HasCode returns a boolean if a field has been set.

### GetMessage

`func (o *NotFoundErrorMessageResponse) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *NotFoundErrorMessageResponse) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *NotFoundErrorMessageResponse) SetMessage(v string)`

SetMessage sets Message field to given value.

### HasMessage

`func (o *NotFoundErrorMessageResponse) HasMessage() bool`

HasMessage returns a boolean if a field has been set.

### GetResourceInfo

`func (o *NotFoundErrorMessageResponse) GetResourceInfo() ResourceInfo`

GetResourceInfo returns the ResourceInfo field if non-nil, zero value otherwise.

### GetResourceInfoOk

`func (o *NotFoundErrorMessageResponse) GetResourceInfoOk() (*ResourceInfo, bool)`

GetResourceInfoOk returns a tuple with the ResourceInfo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResourceInfo

`func (o *NotFoundErrorMessageResponse) SetResourceInfo(v ResourceInfo)`

SetResourceInfo sets ResourceInfo field to given value.

### HasResourceInfo

`func (o *NotFoundErrorMessageResponse) HasResourceInfo() bool`

HasResourceInfo returns a boolean if a field has been set.

### GetSuggestions

`func (o *NotFoundErrorMessageResponse) GetSuggestions() Suggestions`

GetSuggestions returns the Suggestions field if non-nil, zero value otherwise.

### GetSuggestionsOk

`func (o *NotFoundErrorMessageResponse) GetSuggestionsOk() (*Suggestions, bool)`

GetSuggestionsOk returns a tuple with the Suggestions field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSuggestions

`func (o *NotFoundErrorMessageResponse) SetSuggestions(v Suggestions)`

SetSuggestions sets Suggestions field to given value.

### HasSuggestions

`func (o *NotFoundErrorMessageResponse) HasSuggestions() bool`

HasSuggestions returns a boolean if a field has been set.

### GetApiInfo

`func (o *NotFoundErrorMessageResponse) GetApiInfo() APIInfo`

GetApiInfo returns the ApiInfo field if non-nil, zero value otherwise.

### GetApiInfoOk

`func (o *NotFoundErrorMessageResponse) GetApiInfoOk() (*APIInfo, bool)`

GetApiInfoOk returns a tuple with the ApiInfo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetApiInfo

`func (o *NotFoundErrorMessageResponse) SetApiInfo(v APIInfo)`

SetApiInfo sets ApiInfo field to given value.

### HasApiInfo

`func (o *NotFoundErrorMessageResponse) HasApiInfo() bool`

HasApiInfo returns a boolean if a field has been set.

### GetErrorResponse

`func (o *NotFoundErrorMessageResponse) GetErrorResponse() ErrorResponse`

GetErrorResponse returns the ErrorResponse field if non-nil, zero value otherwise.

### GetErrorResponseOk

`func (o *NotFoundErrorMessageResponse) GetErrorResponseOk() (*ErrorResponse, bool)`

GetErrorResponseOk returns a tuple with the ErrorResponse field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetErrorResponse

`func (o *NotFoundErrorMessageResponse) SetErrorResponse(v ErrorResponse)`

SetErrorResponse sets ErrorResponse field to given value.

### HasErrorResponse

`func (o *NotFoundErrorMessageResponse) HasErrorResponse() bool`

HasErrorResponse returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


