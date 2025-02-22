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

// checks if the ResourceUtilization type satisfies the MappedNullable interface at compile time
var _ MappedNullable = &ResourceUtilization{}

// ResourceUtilization struct for ResourceUtilization
type ResourceUtilization struct {
	CpuUsage *float64 `json:"cpuUsage,omitempty"`
	MemoryUsage *float64 `json:"memoryUsage,omitempty"`
	ActiveConnections *int32 `json:"activeConnections,omitempty"`
	Quotas *map[string]float64 `json:"quotas,omitempty"`
}

// NewResourceUtilization instantiates a new ResourceUtilization object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewResourceUtilization() *ResourceUtilization {
	this := ResourceUtilization{}
	return &this
}

// NewResourceUtilizationWithDefaults instantiates a new ResourceUtilization object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewResourceUtilizationWithDefaults() *ResourceUtilization {
	this := ResourceUtilization{}
	return &this
}

// GetCpuUsage returns the CpuUsage field value if set, zero value otherwise.
func (o *ResourceUtilization) GetCpuUsage() float64 {
	if o == nil || IsNil(o.CpuUsage) {
		var ret float64
		return ret
	}
	return *o.CpuUsage
}

// GetCpuUsageOk returns a tuple with the CpuUsage field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *ResourceUtilization) GetCpuUsageOk() (*float64, bool) {
	if o == nil || IsNil(o.CpuUsage) {
		return nil, false
	}
	return o.CpuUsage, true
}

// HasCpuUsage returns a boolean if a field has been set.
func (o *ResourceUtilization) HasCpuUsage() bool {
	if o != nil && !IsNil(o.CpuUsage) {
		return true
	}

	return false
}

// SetCpuUsage gets a reference to the given float64 and assigns it to the CpuUsage field.
func (o *ResourceUtilization) SetCpuUsage(v float64) {
	o.CpuUsage = &v
}

// GetMemoryUsage returns the MemoryUsage field value if set, zero value otherwise.
func (o *ResourceUtilization) GetMemoryUsage() float64 {
	if o == nil || IsNil(o.MemoryUsage) {
		var ret float64
		return ret
	}
	return *o.MemoryUsage
}

// GetMemoryUsageOk returns a tuple with the MemoryUsage field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *ResourceUtilization) GetMemoryUsageOk() (*float64, bool) {
	if o == nil || IsNil(o.MemoryUsage) {
		return nil, false
	}
	return o.MemoryUsage, true
}

// HasMemoryUsage returns a boolean if a field has been set.
func (o *ResourceUtilization) HasMemoryUsage() bool {
	if o != nil && !IsNil(o.MemoryUsage) {
		return true
	}

	return false
}

// SetMemoryUsage gets a reference to the given float64 and assigns it to the MemoryUsage field.
func (o *ResourceUtilization) SetMemoryUsage(v float64) {
	o.MemoryUsage = &v
}

// GetActiveConnections returns the ActiveConnections field value if set, zero value otherwise.
func (o *ResourceUtilization) GetActiveConnections() int32 {
	if o == nil || IsNil(o.ActiveConnections) {
		var ret int32
		return ret
	}
	return *o.ActiveConnections
}

// GetActiveConnectionsOk returns a tuple with the ActiveConnections field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *ResourceUtilization) GetActiveConnectionsOk() (*int32, bool) {
	if o == nil || IsNil(o.ActiveConnections) {
		return nil, false
	}
	return o.ActiveConnections, true
}

// HasActiveConnections returns a boolean if a field has been set.
func (o *ResourceUtilization) HasActiveConnections() bool {
	if o != nil && !IsNil(o.ActiveConnections) {
		return true
	}

	return false
}

// SetActiveConnections gets a reference to the given int32 and assigns it to the ActiveConnections field.
func (o *ResourceUtilization) SetActiveConnections(v int32) {
	o.ActiveConnections = &v
}

// GetQuotas returns the Quotas field value if set, zero value otherwise.
func (o *ResourceUtilization) GetQuotas() map[string]float64 {
	if o == nil || IsNil(o.Quotas) {
		var ret map[string]float64
		return ret
	}
	return *o.Quotas
}

// GetQuotasOk returns a tuple with the Quotas field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *ResourceUtilization) GetQuotasOk() (*map[string]float64, bool) {
	if o == nil || IsNil(o.Quotas) {
		return nil, false
	}
	return o.Quotas, true
}

// HasQuotas returns a boolean if a field has been set.
func (o *ResourceUtilization) HasQuotas() bool {
	if o != nil && !IsNil(o.Quotas) {
		return true
	}

	return false
}

// SetQuotas gets a reference to the given map[string]float64 and assigns it to the Quotas field.
func (o *ResourceUtilization) SetQuotas(v map[string]float64) {
	o.Quotas = &v
}

func (o ResourceUtilization) MarshalJSON() ([]byte, error) {
	toSerialize,err := o.ToMap()
	if err != nil {
		return []byte{}, err
	}
	return json.Marshal(toSerialize)
}

func (o ResourceUtilization) ToMap() (map[string]interface{}, error) {
	toSerialize := map[string]interface{}{}
	if !IsNil(o.CpuUsage) {
		toSerialize["cpuUsage"] = o.CpuUsage
	}
	if !IsNil(o.MemoryUsage) {
		toSerialize["memoryUsage"] = o.MemoryUsage
	}
	if !IsNil(o.ActiveConnections) {
		toSerialize["activeConnections"] = o.ActiveConnections
	}
	if !IsNil(o.Quotas) {
		toSerialize["quotas"] = o.Quotas
	}
	return toSerialize, nil
}

type NullableResourceUtilization struct {
	value *ResourceUtilization
	isSet bool
}

func (v NullableResourceUtilization) Get() *ResourceUtilization {
	return v.value
}

func (v *NullableResourceUtilization) Set(val *ResourceUtilization) {
	v.value = val
	v.isSet = true
}

func (v NullableResourceUtilization) IsSet() bool {
	return v.isSet
}

func (v *NullableResourceUtilization) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableResourceUtilization(val *ResourceUtilization) *NullableResourceUtilization {
	return &NullableResourceUtilization{value: val, isSet: true}
}

func (v NullableResourceUtilization) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableResourceUtilization) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


