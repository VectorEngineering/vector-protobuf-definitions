# GatewayTimeoutErrorMessageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Code** | Pointer to **int32** |  | [optional] 
**Message** | Pointer to **string** |  | [optional] 
**UpstreamService** | Pointer to **string** |  | [optional] 
**ErrorResponse** | Pointer to [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Methods

### NewGatewayTimeoutErrorMessageResponse

`func NewGatewayTimeoutErrorMessageResponse() *GatewayTimeoutErrorMessageResponse`

NewGatewayTimeoutErrorMessageResponse instantiates a new GatewayTimeoutErrorMessageResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewGatewayTimeoutErrorMessageResponseWithDefaults

`func NewGatewayTimeoutErrorMessageResponseWithDefaults() *GatewayTimeoutErrorMessageResponse`

NewGatewayTimeoutErrorMessageResponseWithDefaults instantiates a new GatewayTimeoutErrorMessageResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCode

`func (o *GatewayTimeoutErrorMessageResponse) GetCode() int32`

GetCode returns the Code field if non-nil, zero value otherwise.

### GetCodeOk

`func (o *GatewayTimeoutErrorMessageResponse) GetCodeOk() (*int32, bool)`

GetCodeOk returns a tuple with the Code field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCode

`func (o *GatewayTimeoutErrorMessageResponse) SetCode(v int32)`

SetCode sets Code field to given value.

### HasCode

`func (o *GatewayTimeoutErrorMessageResponse) HasCode() bool`

HasCode returns a boolean if a field has been set.

### GetMessage

`func (o *GatewayTimeoutErrorMessageResponse) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *GatewayTimeoutErrorMessageResponse) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *GatewayTimeoutErrorMessageResponse) SetMessage(v string)`

SetMessage sets Message field to given value.

### HasMessage

`func (o *GatewayTimeoutErrorMessageResponse) HasMessage() bool`

HasMessage returns a boolean if a field has been set.

### GetUpstreamService

`func (o *GatewayTimeoutErrorMessageResponse) GetUpstreamService() string`

GetUpstreamService returns the UpstreamService field if non-nil, zero value otherwise.

### GetUpstreamServiceOk

`func (o *GatewayTimeoutErrorMessageResponse) GetUpstreamServiceOk() (*string, bool)`

GetUpstreamServiceOk returns a tuple with the UpstreamService field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpstreamService

`func (o *GatewayTimeoutErrorMessageResponse) SetUpstreamService(v string)`

SetUpstreamService sets UpstreamService field to given value.

### HasUpstreamService

`func (o *GatewayTimeoutErrorMessageResponse) HasUpstreamService() bool`

HasUpstreamService returns a boolean if a field has been set.

### GetErrorResponse

`func (o *GatewayTimeoutErrorMessageResponse) GetErrorResponse() ErrorResponse`

GetErrorResponse returns the ErrorResponse field if non-nil, zero value otherwise.

### GetErrorResponseOk

`func (o *GatewayTimeoutErrorMessageResponse) GetErrorResponseOk() (*ErrorResponse, bool)`

GetErrorResponseOk returns a tuple with the ErrorResponse field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetErrorResponse

`func (o *GatewayTimeoutErrorMessageResponse) SetErrorResponse(v ErrorResponse)`

SetErrorResponse sets ErrorResponse field to given value.

### HasErrorResponse

`func (o *GatewayTimeoutErrorMessageResponse) HasErrorResponse() bool`

HasErrorResponse returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


