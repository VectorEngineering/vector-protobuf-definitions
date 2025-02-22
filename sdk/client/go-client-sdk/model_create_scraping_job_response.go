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

// checks if the CreateScrapingJobResponse type satisfies the MappedNullable interface at compile time
var _ MappedNullable = &CreateScrapingJobResponse{}

// CreateScrapingJobResponse struct for CreateScrapingJobResponse
type CreateScrapingJobResponse struct {
	JobId *string `json:"jobId,omitempty"`
	Status *BackgroundJobStatus `json:"status,omitempty"`
}

// NewCreateScrapingJobResponse instantiates a new CreateScrapingJobResponse object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewCreateScrapingJobResponse() *CreateScrapingJobResponse {
	this := CreateScrapingJobResponse{}
	var status BackgroundJobStatus = UNSPECIFIED
	this.Status = &status
	return &this
}

// NewCreateScrapingJobResponseWithDefaults instantiates a new CreateScrapingJobResponse object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewCreateScrapingJobResponseWithDefaults() *CreateScrapingJobResponse {
	this := CreateScrapingJobResponse{}
	var status BackgroundJobStatus = UNSPECIFIED
	this.Status = &status
	return &this
}

// GetJobId returns the JobId field value if set, zero value otherwise.
func (o *CreateScrapingJobResponse) GetJobId() string {
	if o == nil || IsNil(o.JobId) {
		var ret string
		return ret
	}
	return *o.JobId
}

// GetJobIdOk returns a tuple with the JobId field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *CreateScrapingJobResponse) GetJobIdOk() (*string, bool) {
	if o == nil || IsNil(o.JobId) {
		return nil, false
	}
	return o.JobId, true
}

// HasJobId returns a boolean if a field has been set.
func (o *CreateScrapingJobResponse) HasJobId() bool {
	if o != nil && !IsNil(o.JobId) {
		return true
	}

	return false
}

// SetJobId gets a reference to the given string and assigns it to the JobId field.
func (o *CreateScrapingJobResponse) SetJobId(v string) {
	o.JobId = &v
}

// GetStatus returns the Status field value if set, zero value otherwise.
func (o *CreateScrapingJobResponse) GetStatus() BackgroundJobStatus {
	if o == nil || IsNil(o.Status) {
		var ret BackgroundJobStatus
		return ret
	}
	return *o.Status
}

// GetStatusOk returns a tuple with the Status field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *CreateScrapingJobResponse) GetStatusOk() (*BackgroundJobStatus, bool) {
	if o == nil || IsNil(o.Status) {
		return nil, false
	}
	return o.Status, true
}

// HasStatus returns a boolean if a field has been set.
func (o *CreateScrapingJobResponse) HasStatus() bool {
	if o != nil && !IsNil(o.Status) {
		return true
	}

	return false
}

// SetStatus gets a reference to the given BackgroundJobStatus and assigns it to the Status field.
func (o *CreateScrapingJobResponse) SetStatus(v BackgroundJobStatus) {
	o.Status = &v
}

func (o CreateScrapingJobResponse) MarshalJSON() ([]byte, error) {
	toSerialize,err := o.ToMap()
	if err != nil {
		return []byte{}, err
	}
	return json.Marshal(toSerialize)
}

func (o CreateScrapingJobResponse) ToMap() (map[string]interface{}, error) {
	toSerialize := map[string]interface{}{}
	if !IsNil(o.JobId) {
		toSerialize["jobId"] = o.JobId
	}
	if !IsNil(o.Status) {
		toSerialize["status"] = o.Status
	}
	return toSerialize, nil
}

type NullableCreateScrapingJobResponse struct {
	value *CreateScrapingJobResponse
	isSet bool
}

func (v NullableCreateScrapingJobResponse) Get() *CreateScrapingJobResponse {
	return v.value
}

func (v *NullableCreateScrapingJobResponse) Set(val *CreateScrapingJobResponse) {
	v.value = val
	v.isSet = true
}

func (v NullableCreateScrapingJobResponse) IsSet() bool {
	return v.isSet
}

func (v *NullableCreateScrapingJobResponse) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableCreateScrapingJobResponse(val *CreateScrapingJobResponse) *NullableCreateScrapingJobResponse {
	return &NullableCreateScrapingJobResponse{value: val, isSet: true}
}

func (v NullableCreateScrapingJobResponse) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableCreateScrapingJobResponse) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


