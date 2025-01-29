# GoneErrorMessageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Code** | Pointer to **int32** |  | [optional] 
**Message** | Pointer to **string** |  | [optional] 
**Resource** | Pointer to **string** |  | [optional] 
**ErrorResponse** | Pointer to [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Methods

### NewGoneErrorMessageResponse

`func NewGoneErrorMessageResponse() *GoneErrorMessageResponse`

NewGoneErrorMessageResponse instantiates a new GoneErrorMessageResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewGoneErrorMessageResponseWithDefaults

`func NewGoneErrorMessageResponseWithDefaults() *GoneErrorMessageResponse`

NewGoneErrorMessageResponseWithDefaults instantiates a new GoneErrorMessageResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCode

`func (o *GoneErrorMessageResponse) GetCode() int32`

GetCode returns the Code field if non-nil, zero value otherwise.

### GetCodeOk

`func (o *GoneErrorMessageResponse) GetCodeOk() (*int32, bool)`

GetCodeOk returns a tuple with the Code field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCode

`func (o *GoneErrorMessageResponse) SetCode(v int32)`

SetCode sets Code field to given value.

### HasCode

`func (o *GoneErrorMessageResponse) HasCode() bool`

HasCode returns a boolean if a field has been set.

### GetMessage

`func (o *GoneErrorMessageResponse) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *GoneErrorMessageResponse) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *GoneErrorMessageResponse) SetMessage(v string)`

SetMessage sets Message field to given value.

### HasMessage

`func (o *GoneErrorMessageResponse) HasMessage() bool`

HasMessage returns a boolean if a field has been set.

### GetResource

`func (o *GoneErrorMessageResponse) GetResource() string`

GetResource returns the Resource field if non-nil, zero value otherwise.

### GetResourceOk

`func (o *GoneErrorMessageResponse) GetResourceOk() (*string, bool)`

GetResourceOk returns a tuple with the Resource field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResource

`func (o *GoneErrorMessageResponse) SetResource(v string)`

SetResource sets Resource field to given value.

### HasResource

`func (o *GoneErrorMessageResponse) HasResource() bool`

HasResource returns a boolean if a field has been set.

### GetErrorResponse

`func (o *GoneErrorMessageResponse) GetErrorResponse() ErrorResponse`

GetErrorResponse returns the ErrorResponse field if non-nil, zero value otherwise.

### GetErrorResponseOk

`func (o *GoneErrorMessageResponse) GetErrorResponseOk() (*ErrorResponse, bool)`

GetErrorResponseOk returns a tuple with the ErrorResponse field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetErrorResponse

`func (o *GoneErrorMessageResponse) SetErrorResponse(v ErrorResponse)`

SetErrorResponse sets ErrorResponse field to given value.

### HasErrorResponse

`func (o *GoneErrorMessageResponse) HasErrorResponse() bool`

HasErrorResponse returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


