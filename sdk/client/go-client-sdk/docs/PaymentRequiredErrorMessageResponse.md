# PaymentRequiredErrorMessageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Code** | Pointer to **int32** |  | [optional] 
**Message** | Pointer to **string** |  | [optional] 
**PaymentInfo** | Pointer to [**PaymentInfo**](PaymentInfo.md) |  | [optional] 
**ErrorResponse** | Pointer to [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Methods

### NewPaymentRequiredErrorMessageResponse

`func NewPaymentRequiredErrorMessageResponse() *PaymentRequiredErrorMessageResponse`

NewPaymentRequiredErrorMessageResponse instantiates a new PaymentRequiredErrorMessageResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPaymentRequiredErrorMessageResponseWithDefaults

`func NewPaymentRequiredErrorMessageResponseWithDefaults() *PaymentRequiredErrorMessageResponse`

NewPaymentRequiredErrorMessageResponseWithDefaults instantiates a new PaymentRequiredErrorMessageResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCode

`func (o *PaymentRequiredErrorMessageResponse) GetCode() int32`

GetCode returns the Code field if non-nil, zero value otherwise.

### GetCodeOk

`func (o *PaymentRequiredErrorMessageResponse) GetCodeOk() (*int32, bool)`

GetCodeOk returns a tuple with the Code field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCode

`func (o *PaymentRequiredErrorMessageResponse) SetCode(v int32)`

SetCode sets Code field to given value.

### HasCode

`func (o *PaymentRequiredErrorMessageResponse) HasCode() bool`

HasCode returns a boolean if a field has been set.

### GetMessage

`func (o *PaymentRequiredErrorMessageResponse) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *PaymentRequiredErrorMessageResponse) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *PaymentRequiredErrorMessageResponse) SetMessage(v string)`

SetMessage sets Message field to given value.

### HasMessage

`func (o *PaymentRequiredErrorMessageResponse) HasMessage() bool`

HasMessage returns a boolean if a field has been set.

### GetPaymentInfo

`func (o *PaymentRequiredErrorMessageResponse) GetPaymentInfo() PaymentInfo`

GetPaymentInfo returns the PaymentInfo field if non-nil, zero value otherwise.

### GetPaymentInfoOk

`func (o *PaymentRequiredErrorMessageResponse) GetPaymentInfoOk() (*PaymentInfo, bool)`

GetPaymentInfoOk returns a tuple with the PaymentInfo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPaymentInfo

`func (o *PaymentRequiredErrorMessageResponse) SetPaymentInfo(v PaymentInfo)`

SetPaymentInfo sets PaymentInfo field to given value.

### HasPaymentInfo

`func (o *PaymentRequiredErrorMessageResponse) HasPaymentInfo() bool`

HasPaymentInfo returns a boolean if a field has been set.

### GetErrorResponse

`func (o *PaymentRequiredErrorMessageResponse) GetErrorResponse() ErrorResponse`

GetErrorResponse returns the ErrorResponse field if non-nil, zero value otherwise.

### GetErrorResponseOk

`func (o *PaymentRequiredErrorMessageResponse) GetErrorResponseOk() (*ErrorResponse, bool)`

GetErrorResponseOk returns a tuple with the ErrorResponse field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetErrorResponse

`func (o *PaymentRequiredErrorMessageResponse) SetErrorResponse(v ErrorResponse)`

SetErrorResponse sets ErrorResponse field to given value.

### HasErrorResponse

`func (o *PaymentRequiredErrorMessageResponse) HasErrorResponse() bool`

HasErrorResponse returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


