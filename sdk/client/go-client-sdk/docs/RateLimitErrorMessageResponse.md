# RateLimitErrorMessageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Code** | Pointer to [**InternalErrorCode**](InternalErrorCode.md) |  | [optional] [default to NO_INTERNAL_ERROR]
**Message** | Pointer to **string** |  | [optional] 
**LimitInfo** | Pointer to [**LimitInfo**](LimitInfo.md) |  | [optional] 
**QuotaInfo** | Pointer to [**QuotaInfo**](QuotaInfo.md) |  | [optional] 
**Context** | Pointer to [**RateLimitContext**](RateLimitContext.md) |  | [optional] 
**ErrorResponse** | Pointer to [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Methods

### NewRateLimitErrorMessageResponse

`func NewRateLimitErrorMessageResponse() *RateLimitErrorMessageResponse`

NewRateLimitErrorMessageResponse instantiates a new RateLimitErrorMessageResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewRateLimitErrorMessageResponseWithDefaults

`func NewRateLimitErrorMessageResponseWithDefaults() *RateLimitErrorMessageResponse`

NewRateLimitErrorMessageResponseWithDefaults instantiates a new RateLimitErrorMessageResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCode

`func (o *RateLimitErrorMessageResponse) GetCode() InternalErrorCode`

GetCode returns the Code field if non-nil, zero value otherwise.

### GetCodeOk

`func (o *RateLimitErrorMessageResponse) GetCodeOk() (*InternalErrorCode, bool)`

GetCodeOk returns a tuple with the Code field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCode

`func (o *RateLimitErrorMessageResponse) SetCode(v InternalErrorCode)`

SetCode sets Code field to given value.

### HasCode

`func (o *RateLimitErrorMessageResponse) HasCode() bool`

HasCode returns a boolean if a field has been set.

### GetMessage

`func (o *RateLimitErrorMessageResponse) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *RateLimitErrorMessageResponse) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *RateLimitErrorMessageResponse) SetMessage(v string)`

SetMessage sets Message field to given value.

### HasMessage

`func (o *RateLimitErrorMessageResponse) HasMessage() bool`

HasMessage returns a boolean if a field has been set.

### GetLimitInfo

`func (o *RateLimitErrorMessageResponse) GetLimitInfo() LimitInfo`

GetLimitInfo returns the LimitInfo field if non-nil, zero value otherwise.

### GetLimitInfoOk

`func (o *RateLimitErrorMessageResponse) GetLimitInfoOk() (*LimitInfo, bool)`

GetLimitInfoOk returns a tuple with the LimitInfo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLimitInfo

`func (o *RateLimitErrorMessageResponse) SetLimitInfo(v LimitInfo)`

SetLimitInfo sets LimitInfo field to given value.

### HasLimitInfo

`func (o *RateLimitErrorMessageResponse) HasLimitInfo() bool`

HasLimitInfo returns a boolean if a field has been set.

### GetQuotaInfo

`func (o *RateLimitErrorMessageResponse) GetQuotaInfo() QuotaInfo`

GetQuotaInfo returns the QuotaInfo field if non-nil, zero value otherwise.

### GetQuotaInfoOk

`func (o *RateLimitErrorMessageResponse) GetQuotaInfoOk() (*QuotaInfo, bool)`

GetQuotaInfoOk returns a tuple with the QuotaInfo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetQuotaInfo

`func (o *RateLimitErrorMessageResponse) SetQuotaInfo(v QuotaInfo)`

SetQuotaInfo sets QuotaInfo field to given value.

### HasQuotaInfo

`func (o *RateLimitErrorMessageResponse) HasQuotaInfo() bool`

HasQuotaInfo returns a boolean if a field has been set.

### GetContext

`func (o *RateLimitErrorMessageResponse) GetContext() RateLimitContext`

GetContext returns the Context field if non-nil, zero value otherwise.

### GetContextOk

`func (o *RateLimitErrorMessageResponse) GetContextOk() (*RateLimitContext, bool)`

GetContextOk returns a tuple with the Context field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetContext

`func (o *RateLimitErrorMessageResponse) SetContext(v RateLimitContext)`

SetContext sets Context field to given value.

### HasContext

`func (o *RateLimitErrorMessageResponse) HasContext() bool`

HasContext returns a boolean if a field has been set.

### GetErrorResponse

`func (o *RateLimitErrorMessageResponse) GetErrorResponse() ErrorResponse`

GetErrorResponse returns the ErrorResponse field if non-nil, zero value otherwise.

### GetErrorResponseOk

`func (o *RateLimitErrorMessageResponse) GetErrorResponseOk() (*ErrorResponse, bool)`

GetErrorResponseOk returns a tuple with the ErrorResponse field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetErrorResponse

`func (o *RateLimitErrorMessageResponse) SetErrorResponse(v ErrorResponse)`

SetErrorResponse sets ErrorResponse field to given value.

### HasErrorResponse

`func (o *RateLimitErrorMessageResponse) HasErrorResponse() bool`

HasErrorResponse returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


