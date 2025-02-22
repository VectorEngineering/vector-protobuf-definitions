/*
Lead Scraping Service API

Vector Lead Scraping Service API - Manages Lead Scraping Jobs

API version: 1.0
Contact: yoanyomba@vector.ai
*/

// Code generated by OpenAPI Generator (https://openapi-generator.tech); DO NOT EDIT.

package backend_client

import (
	"encoding/json"
)

// checks if the RateLimitErrorMessageResponse type satisfies the MappedNullable interface at compile time
var _ MappedNullable = &RateLimitErrorMessageResponse{}

// RateLimitErrorMessageResponse Represents rate limiting and quota exceeded errors
type RateLimitErrorMessageResponse struct {
	Code *InternalErrorCode `json:"code,omitempty"`
	Message *string `json:"message,omitempty"`
	LimitInfo *LimitInfo `json:"limitInfo,omitempty"`
	QuotaInfo *QuotaInfo `json:"quotaInfo,omitempty"`
	Context *RateLimitContext `json:"context,omitempty"`
	ErrorResponse *ErrorResponse `json:"errorResponse,omitempty"`
}

// NewRateLimitErrorMessageResponse instantiates a new RateLimitErrorMessageResponse object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewRateLimitErrorMessageResponse() *RateLimitErrorMessageResponse {
	this := RateLimitErrorMessageResponse{}
	var code InternalErrorCode = NO_INTERNAL_ERROR
	this.Code = &code
	return &this
}

// NewRateLimitErrorMessageResponseWithDefaults instantiates a new RateLimitErrorMessageResponse object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewRateLimitErrorMessageResponseWithDefaults() *RateLimitErrorMessageResponse {
	this := RateLimitErrorMessageResponse{}
	var code InternalErrorCode = NO_INTERNAL_ERROR
	this.Code = &code
	return &this
}

// GetCode returns the Code field value if set, zero value otherwise.
func (o *RateLimitErrorMessageResponse) GetCode() InternalErrorCode {
	if o == nil || IsNil(o.Code) {
		var ret InternalErrorCode
		return ret
	}
	return *o.Code
}

// GetCodeOk returns a tuple with the Code field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *RateLimitErrorMessageResponse) GetCodeOk() (*InternalErrorCode, bool) {
	if o == nil || IsNil(o.Code) {
		return nil, false
	}
	return o.Code, true
}

// HasCode returns a boolean if a field has been set.
func (o *RateLimitErrorMessageResponse) HasCode() bool {
	if o != nil && !IsNil(o.Code) {
		return true
	}

	return false
}

// SetCode gets a reference to the given InternalErrorCode and assigns it to the Code field.
func (o *RateLimitErrorMessageResponse) SetCode(v InternalErrorCode) {
	o.Code = &v
}

// GetMessage returns the Message field value if set, zero value otherwise.
func (o *RateLimitErrorMessageResponse) GetMessage() string {
	if o == nil || IsNil(o.Message) {
		var ret string
		return ret
	}
	return *o.Message
}

// GetMessageOk returns a tuple with the Message field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *RateLimitErrorMessageResponse) GetMessageOk() (*string, bool) {
	if o == nil || IsNil(o.Message) {
		return nil, false
	}
	return o.Message, true
}

// HasMessage returns a boolean if a field has been set.
func (o *RateLimitErrorMessageResponse) HasMessage() bool {
	if o != nil && !IsNil(o.Message) {
		return true
	}

	return false
}

// SetMessage gets a reference to the given string and assigns it to the Message field.
func (o *RateLimitErrorMessageResponse) SetMessage(v string) {
	o.Message = &v
}

// GetLimitInfo returns the LimitInfo field value if set, zero value otherwise.
func (o *RateLimitErrorMessageResponse) GetLimitInfo() LimitInfo {
	if o == nil || IsNil(o.LimitInfo) {
		var ret LimitInfo
		return ret
	}
	return *o.LimitInfo
}

// GetLimitInfoOk returns a tuple with the LimitInfo field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *RateLimitErrorMessageResponse) GetLimitInfoOk() (*LimitInfo, bool) {
	if o == nil || IsNil(o.LimitInfo) {
		return nil, false
	}
	return o.LimitInfo, true
}

// HasLimitInfo returns a boolean if a field has been set.
func (o *RateLimitErrorMessageResponse) HasLimitInfo() bool {
	if o != nil && !IsNil(o.LimitInfo) {
		return true
	}

	return false
}

// SetLimitInfo gets a reference to the given LimitInfo and assigns it to the LimitInfo field.
func (o *RateLimitErrorMessageResponse) SetLimitInfo(v LimitInfo) {
	o.LimitInfo = &v
}

// GetQuotaInfo returns the QuotaInfo field value if set, zero value otherwise.
func (o *RateLimitErrorMessageResponse) GetQuotaInfo() QuotaInfo {
	if o == nil || IsNil(o.QuotaInfo) {
		var ret QuotaInfo
		return ret
	}
	return *o.QuotaInfo
}

// GetQuotaInfoOk returns a tuple with the QuotaInfo field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *RateLimitErrorMessageResponse) GetQuotaInfoOk() (*QuotaInfo, bool) {
	if o == nil || IsNil(o.QuotaInfo) {
		return nil, false
	}
	return o.QuotaInfo, true
}

// HasQuotaInfo returns a boolean if a field has been set.
func (o *RateLimitErrorMessageResponse) HasQuotaInfo() bool {
	if o != nil && !IsNil(o.QuotaInfo) {
		return true
	}

	return false
}

// SetQuotaInfo gets a reference to the given QuotaInfo and assigns it to the QuotaInfo field.
func (o *RateLimitErrorMessageResponse) SetQuotaInfo(v QuotaInfo) {
	o.QuotaInfo = &v
}

// GetContext returns the Context field value if set, zero value otherwise.
func (o *RateLimitErrorMessageResponse) GetContext() RateLimitContext {
	if o == nil || IsNil(o.Context) {
		var ret RateLimitContext
		return ret
	}
	return *o.Context
}

// GetContextOk returns a tuple with the Context field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *RateLimitErrorMessageResponse) GetContextOk() (*RateLimitContext, bool) {
	if o == nil || IsNil(o.Context) {
		return nil, false
	}
	return o.Context, true
}

// HasContext returns a boolean if a field has been set.
func (o *RateLimitErrorMessageResponse) HasContext() bool {
	if o != nil && !IsNil(o.Context) {
		return true
	}

	return false
}

// SetContext gets a reference to the given RateLimitContext and assigns it to the Context field.
func (o *RateLimitErrorMessageResponse) SetContext(v RateLimitContext) {
	o.Context = &v
}

// GetErrorResponse returns the ErrorResponse field value if set, zero value otherwise.
func (o *RateLimitErrorMessageResponse) GetErrorResponse() ErrorResponse {
	if o == nil || IsNil(o.ErrorResponse) {
		var ret ErrorResponse
		return ret
	}
	return *o.ErrorResponse
}

// GetErrorResponseOk returns a tuple with the ErrorResponse field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *RateLimitErrorMessageResponse) GetErrorResponseOk() (*ErrorResponse, bool) {
	if o == nil || IsNil(o.ErrorResponse) {
		return nil, false
	}
	return o.ErrorResponse, true
}

// HasErrorResponse returns a boolean if a field has been set.
func (o *RateLimitErrorMessageResponse) HasErrorResponse() bool {
	if o != nil && !IsNil(o.ErrorResponse) {
		return true
	}

	return false
}

// SetErrorResponse gets a reference to the given ErrorResponse and assigns it to the ErrorResponse field.
func (o *RateLimitErrorMessageResponse) SetErrorResponse(v ErrorResponse) {
	o.ErrorResponse = &v
}

func (o RateLimitErrorMessageResponse) MarshalJSON() ([]byte, error) {
	toSerialize,err := o.ToMap()
	if err != nil {
		return []byte{}, err
	}
	return json.Marshal(toSerialize)
}

func (o RateLimitErrorMessageResponse) ToMap() (map[string]interface{}, error) {
	toSerialize := map[string]interface{}{}
	if !IsNil(o.Code) {
		toSerialize["code"] = o.Code
	}
	if !IsNil(o.Message) {
		toSerialize["message"] = o.Message
	}
	if !IsNil(o.LimitInfo) {
		toSerialize["limitInfo"] = o.LimitInfo
	}
	if !IsNil(o.QuotaInfo) {
		toSerialize["quotaInfo"] = o.QuotaInfo
	}
	if !IsNil(o.Context) {
		toSerialize["context"] = o.Context
	}
	if !IsNil(o.ErrorResponse) {
		toSerialize["errorResponse"] = o.ErrorResponse
	}
	return toSerialize, nil
}

type NullableRateLimitErrorMessageResponse struct {
	value *RateLimitErrorMessageResponse
	isSet bool
}

func (v NullableRateLimitErrorMessageResponse) Get() *RateLimitErrorMessageResponse {
	return v.value
}

func (v *NullableRateLimitErrorMessageResponse) Set(val *RateLimitErrorMessageResponse) {
	v.value = val
	v.isSet = true
}

func (v NullableRateLimitErrorMessageResponse) IsSet() bool {
	return v.isSet
}

func (v *NullableRateLimitErrorMessageResponse) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableRateLimitErrorMessageResponse(val *RateLimitErrorMessageResponse) *NullableRateLimitErrorMessageResponse {
	return &NullableRateLimitErrorMessageResponse{value: val, isSet: true}
}

func (v NullableRateLimitErrorMessageResponse) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableRateLimitErrorMessageResponse) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


