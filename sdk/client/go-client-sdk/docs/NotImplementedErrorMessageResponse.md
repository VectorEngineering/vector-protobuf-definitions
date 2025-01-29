# NotImplementedErrorMessageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Code** | Pointer to **int32** |  | [optional] 
**Message** | Pointer to **string** |  | [optional] 
**Feature** | Pointer to **string** |  | [optional] 
**ErrorResponse** | Pointer to [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Methods

### NewNotImplementedErrorMessageResponse

`func NewNotImplementedErrorMessageResponse() *NotImplementedErrorMessageResponse`

NewNotImplementedErrorMessageResponse instantiates a new NotImplementedErrorMessageResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewNotImplementedErrorMessageResponseWithDefaults

`func NewNotImplementedErrorMessageResponseWithDefaults() *NotImplementedErrorMessageResponse`

NewNotImplementedErrorMessageResponseWithDefaults instantiates a new NotImplementedErrorMessageResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCode

`func (o *NotImplementedErrorMessageResponse) GetCode() int32`

GetCode returns the Code field if non-nil, zero value otherwise.

### GetCodeOk

`func (o *NotImplementedErrorMessageResponse) GetCodeOk() (*int32, bool)`

GetCodeOk returns a tuple with the Code field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCode

`func (o *NotImplementedErrorMessageResponse) SetCode(v int32)`

SetCode sets Code field to given value.

### HasCode

`func (o *NotImplementedErrorMessageResponse) HasCode() bool`

HasCode returns a boolean if a field has been set.

### GetMessage

`func (o *NotImplementedErrorMessageResponse) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *NotImplementedErrorMessageResponse) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *NotImplementedErrorMessageResponse) SetMessage(v string)`

SetMessage sets Message field to given value.

### HasMessage

`func (o *NotImplementedErrorMessageResponse) HasMessage() bool`

HasMessage returns a boolean if a field has been set.

### GetFeature

`func (o *NotImplementedErrorMessageResponse) GetFeature() string`

GetFeature returns the Feature field if non-nil, zero value otherwise.

### GetFeatureOk

`func (o *NotImplementedErrorMessageResponse) GetFeatureOk() (*string, bool)`

GetFeatureOk returns a tuple with the Feature field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFeature

`func (o *NotImplementedErrorMessageResponse) SetFeature(v string)`

SetFeature sets Feature field to given value.

### HasFeature

`func (o *NotImplementedErrorMessageResponse) HasFeature() bool`

HasFeature returns a boolean if a field has been set.

### GetErrorResponse

`func (o *NotImplementedErrorMessageResponse) GetErrorResponse() ErrorResponse`

GetErrorResponse returns the ErrorResponse field if non-nil, zero value otherwise.

### GetErrorResponseOk

`func (o *NotImplementedErrorMessageResponse) GetErrorResponseOk() (*ErrorResponse, bool)`

GetErrorResponseOk returns a tuple with the ErrorResponse field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetErrorResponse

`func (o *NotImplementedErrorMessageResponse) SetErrorResponse(v ErrorResponse)`

SetErrorResponse sets ErrorResponse field to given value.

### HasErrorResponse

`func (o *NotImplementedErrorMessageResponse) HasErrorResponse() bool`

HasErrorResponse returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


