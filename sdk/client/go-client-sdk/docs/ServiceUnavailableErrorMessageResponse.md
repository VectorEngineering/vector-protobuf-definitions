# ServiceUnavailableErrorMessageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Code** | Pointer to [**InternalErrorCode**](InternalErrorCode.md) |  | [optional] [default to NO_INTERNAL_ERROR]
**Message** | Pointer to **string** |  | [optional] 
**AvailabilityInfo** | Pointer to [**AvailabilityInfo**](AvailabilityInfo.md) |  | [optional] 
**ErrorResponse** | Pointer to [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Methods

### NewServiceUnavailableErrorMessageResponse

`func NewServiceUnavailableErrorMessageResponse() *ServiceUnavailableErrorMessageResponse`

NewServiceUnavailableErrorMessageResponse instantiates a new ServiceUnavailableErrorMessageResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewServiceUnavailableErrorMessageResponseWithDefaults

`func NewServiceUnavailableErrorMessageResponseWithDefaults() *ServiceUnavailableErrorMessageResponse`

NewServiceUnavailableErrorMessageResponseWithDefaults instantiates a new ServiceUnavailableErrorMessageResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCode

`func (o *ServiceUnavailableErrorMessageResponse) GetCode() InternalErrorCode`

GetCode returns the Code field if non-nil, zero value otherwise.

### GetCodeOk

`func (o *ServiceUnavailableErrorMessageResponse) GetCodeOk() (*InternalErrorCode, bool)`

GetCodeOk returns a tuple with the Code field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCode

`func (o *ServiceUnavailableErrorMessageResponse) SetCode(v InternalErrorCode)`

SetCode sets Code field to given value.

### HasCode

`func (o *ServiceUnavailableErrorMessageResponse) HasCode() bool`

HasCode returns a boolean if a field has been set.

### GetMessage

`func (o *ServiceUnavailableErrorMessageResponse) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *ServiceUnavailableErrorMessageResponse) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *ServiceUnavailableErrorMessageResponse) SetMessage(v string)`

SetMessage sets Message field to given value.

### HasMessage

`func (o *ServiceUnavailableErrorMessageResponse) HasMessage() bool`

HasMessage returns a boolean if a field has been set.

### GetAvailabilityInfo

`func (o *ServiceUnavailableErrorMessageResponse) GetAvailabilityInfo() AvailabilityInfo`

GetAvailabilityInfo returns the AvailabilityInfo field if non-nil, zero value otherwise.

### GetAvailabilityInfoOk

`func (o *ServiceUnavailableErrorMessageResponse) GetAvailabilityInfoOk() (*AvailabilityInfo, bool)`

GetAvailabilityInfoOk returns a tuple with the AvailabilityInfo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAvailabilityInfo

`func (o *ServiceUnavailableErrorMessageResponse) SetAvailabilityInfo(v AvailabilityInfo)`

SetAvailabilityInfo sets AvailabilityInfo field to given value.

### HasAvailabilityInfo

`func (o *ServiceUnavailableErrorMessageResponse) HasAvailabilityInfo() bool`

HasAvailabilityInfo returns a boolean if a field has been set.

### GetErrorResponse

`func (o *ServiceUnavailableErrorMessageResponse) GetErrorResponse() ErrorResponse`

GetErrorResponse returns the ErrorResponse field if non-nil, zero value otherwise.

### GetErrorResponseOk

`func (o *ServiceUnavailableErrorMessageResponse) GetErrorResponseOk() (*ErrorResponse, bool)`

GetErrorResponseOk returns a tuple with the ErrorResponse field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetErrorResponse

`func (o *ServiceUnavailableErrorMessageResponse) SetErrorResponse(v ErrorResponse)`

SetErrorResponse sets ErrorResponse field to given value.

### HasErrorResponse

`func (o *ServiceUnavailableErrorMessageResponse) HasErrorResponse() bool`

HasErrorResponse returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


