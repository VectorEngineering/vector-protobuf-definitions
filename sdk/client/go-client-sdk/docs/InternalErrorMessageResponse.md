# InternalErrorMessageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Code** | Pointer to [**InternalErrorCode**](InternalErrorCode.md) |  | [optional] [default to NO_INTERNAL_ERROR]
**Message** | Pointer to **string** |  | [optional] 
**IncidentId** | Pointer to **string** |  | [optional] 
**ServiceStatus** | Pointer to [**ServiceStatus**](ServiceStatus.md) |  | [optional] 
**ResourceUtilization** | Pointer to [**ResourceUtilization**](ResourceUtilization.md) |  | [optional] 
**OperationDetails** | Pointer to [**OperationDetails**](OperationDetails.md) |  | [optional] 
**ErrorResponse** | Pointer to [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Methods

### NewInternalErrorMessageResponse

`func NewInternalErrorMessageResponse() *InternalErrorMessageResponse`

NewInternalErrorMessageResponse instantiates a new InternalErrorMessageResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewInternalErrorMessageResponseWithDefaults

`func NewInternalErrorMessageResponseWithDefaults() *InternalErrorMessageResponse`

NewInternalErrorMessageResponseWithDefaults instantiates a new InternalErrorMessageResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCode

`func (o *InternalErrorMessageResponse) GetCode() InternalErrorCode`

GetCode returns the Code field if non-nil, zero value otherwise.

### GetCodeOk

`func (o *InternalErrorMessageResponse) GetCodeOk() (*InternalErrorCode, bool)`

GetCodeOk returns a tuple with the Code field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCode

`func (o *InternalErrorMessageResponse) SetCode(v InternalErrorCode)`

SetCode sets Code field to given value.

### HasCode

`func (o *InternalErrorMessageResponse) HasCode() bool`

HasCode returns a boolean if a field has been set.

### GetMessage

`func (o *InternalErrorMessageResponse) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *InternalErrorMessageResponse) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *InternalErrorMessageResponse) SetMessage(v string)`

SetMessage sets Message field to given value.

### HasMessage

`func (o *InternalErrorMessageResponse) HasMessage() bool`

HasMessage returns a boolean if a field has been set.

### GetIncidentId

`func (o *InternalErrorMessageResponse) GetIncidentId() string`

GetIncidentId returns the IncidentId field if non-nil, zero value otherwise.

### GetIncidentIdOk

`func (o *InternalErrorMessageResponse) GetIncidentIdOk() (*string, bool)`

GetIncidentIdOk returns a tuple with the IncidentId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIncidentId

`func (o *InternalErrorMessageResponse) SetIncidentId(v string)`

SetIncidentId sets IncidentId field to given value.

### HasIncidentId

`func (o *InternalErrorMessageResponse) HasIncidentId() bool`

HasIncidentId returns a boolean if a field has been set.

### GetServiceStatus

`func (o *InternalErrorMessageResponse) GetServiceStatus() ServiceStatus`

GetServiceStatus returns the ServiceStatus field if non-nil, zero value otherwise.

### GetServiceStatusOk

`func (o *InternalErrorMessageResponse) GetServiceStatusOk() (*ServiceStatus, bool)`

GetServiceStatusOk returns a tuple with the ServiceStatus field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetServiceStatus

`func (o *InternalErrorMessageResponse) SetServiceStatus(v ServiceStatus)`

SetServiceStatus sets ServiceStatus field to given value.

### HasServiceStatus

`func (o *InternalErrorMessageResponse) HasServiceStatus() bool`

HasServiceStatus returns a boolean if a field has been set.

### GetResourceUtilization

`func (o *InternalErrorMessageResponse) GetResourceUtilization() ResourceUtilization`

GetResourceUtilization returns the ResourceUtilization field if non-nil, zero value otherwise.

### GetResourceUtilizationOk

`func (o *InternalErrorMessageResponse) GetResourceUtilizationOk() (*ResourceUtilization, bool)`

GetResourceUtilizationOk returns a tuple with the ResourceUtilization field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResourceUtilization

`func (o *InternalErrorMessageResponse) SetResourceUtilization(v ResourceUtilization)`

SetResourceUtilization sets ResourceUtilization field to given value.

### HasResourceUtilization

`func (o *InternalErrorMessageResponse) HasResourceUtilization() bool`

HasResourceUtilization returns a boolean if a field has been set.

### GetOperationDetails

`func (o *InternalErrorMessageResponse) GetOperationDetails() OperationDetails`

GetOperationDetails returns the OperationDetails field if non-nil, zero value otherwise.

### GetOperationDetailsOk

`func (o *InternalErrorMessageResponse) GetOperationDetailsOk() (*OperationDetails, bool)`

GetOperationDetailsOk returns a tuple with the OperationDetails field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOperationDetails

`func (o *InternalErrorMessageResponse) SetOperationDetails(v OperationDetails)`

SetOperationDetails sets OperationDetails field to given value.

### HasOperationDetails

`func (o *InternalErrorMessageResponse) HasOperationDetails() bool`

HasOperationDetails returns a boolean if a field has been set.

### GetErrorResponse

`func (o *InternalErrorMessageResponse) GetErrorResponse() ErrorResponse`

GetErrorResponse returns the ErrorResponse field if non-nil, zero value otherwise.

### GetErrorResponseOk

`func (o *InternalErrorMessageResponse) GetErrorResponseOk() (*ErrorResponse, bool)`

GetErrorResponseOk returns a tuple with the ErrorResponse field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetErrorResponse

`func (o *InternalErrorMessageResponse) SetErrorResponse(v ErrorResponse)`

SetErrorResponse sets ErrorResponse field to given value.

### HasErrorResponse

`func (o *InternalErrorMessageResponse) HasErrorResponse() bool`

HasErrorResponse returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


