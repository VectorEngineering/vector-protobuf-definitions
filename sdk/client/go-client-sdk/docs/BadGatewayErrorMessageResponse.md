# BadGatewayErrorMessageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Code** | Pointer to **int32** |  | [optional] 
**Message** | Pointer to **string** |  | [optional] 
**UpstreamService** | Pointer to **string** |  | [optional] 
**ErrorResponse** | Pointer to [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Methods

### NewBadGatewayErrorMessageResponse

`func NewBadGatewayErrorMessageResponse() *BadGatewayErrorMessageResponse`

NewBadGatewayErrorMessageResponse instantiates a new BadGatewayErrorMessageResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewBadGatewayErrorMessageResponseWithDefaults

`func NewBadGatewayErrorMessageResponseWithDefaults() *BadGatewayErrorMessageResponse`

NewBadGatewayErrorMessageResponseWithDefaults instantiates a new BadGatewayErrorMessageResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCode

`func (o *BadGatewayErrorMessageResponse) GetCode() int32`

GetCode returns the Code field if non-nil, zero value otherwise.

### GetCodeOk

`func (o *BadGatewayErrorMessageResponse) GetCodeOk() (*int32, bool)`

GetCodeOk returns a tuple with the Code field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCode

`func (o *BadGatewayErrorMessageResponse) SetCode(v int32)`

SetCode sets Code field to given value.

### HasCode

`func (o *BadGatewayErrorMessageResponse) HasCode() bool`

HasCode returns a boolean if a field has been set.

### GetMessage

`func (o *BadGatewayErrorMessageResponse) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *BadGatewayErrorMessageResponse) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *BadGatewayErrorMessageResponse) SetMessage(v string)`

SetMessage sets Message field to given value.

### HasMessage

`func (o *BadGatewayErrorMessageResponse) HasMessage() bool`

HasMessage returns a boolean if a field has been set.

### GetUpstreamService

`func (o *BadGatewayErrorMessageResponse) GetUpstreamService() string`

GetUpstreamService returns the UpstreamService field if non-nil, zero value otherwise.

### GetUpstreamServiceOk

`func (o *BadGatewayErrorMessageResponse) GetUpstreamServiceOk() (*string, bool)`

GetUpstreamServiceOk returns a tuple with the UpstreamService field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpstreamService

`func (o *BadGatewayErrorMessageResponse) SetUpstreamService(v string)`

SetUpstreamService sets UpstreamService field to given value.

### HasUpstreamService

`func (o *BadGatewayErrorMessageResponse) HasUpstreamService() bool`

HasUpstreamService returns a boolean if a field has been set.

### GetErrorResponse

`func (o *BadGatewayErrorMessageResponse) GetErrorResponse() ErrorResponse`

GetErrorResponse returns the ErrorResponse field if non-nil, zero value otherwise.

### GetErrorResponseOk

`func (o *BadGatewayErrorMessageResponse) GetErrorResponseOk() (*ErrorResponse, bool)`

GetErrorResponseOk returns a tuple with the ErrorResponse field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetErrorResponse

`func (o *BadGatewayErrorMessageResponse) SetErrorResponse(v ErrorResponse)`

SetErrorResponse sets ErrorResponse field to given value.

### HasErrorResponse

`func (o *BadGatewayErrorMessageResponse) HasErrorResponse() bool`

HasErrorResponse returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


