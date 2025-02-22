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

// checks if the ServiceStatus type satisfies the MappedNullable interface at compile time
var _ MappedNullable = &ServiceStatus{}

// ServiceStatus struct for ServiceStatus
type ServiceStatus struct {
	Name *string `json:"name,omitempty"`
	Status *string `json:"status,omitempty"`
	Metrics *map[string]string `json:"metrics,omitempty"`
	Dependencies []Dependency `json:"dependencies,omitempty"`
}

// NewServiceStatus instantiates a new ServiceStatus object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewServiceStatus() *ServiceStatus {
	this := ServiceStatus{}
	return &this
}

// NewServiceStatusWithDefaults instantiates a new ServiceStatus object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewServiceStatusWithDefaults() *ServiceStatus {
	this := ServiceStatus{}
	return &this
}

// GetName returns the Name field value if set, zero value otherwise.
func (o *ServiceStatus) GetName() string {
	if o == nil || IsNil(o.Name) {
		var ret string
		return ret
	}
	return *o.Name
}

// GetNameOk returns a tuple with the Name field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *ServiceStatus) GetNameOk() (*string, bool) {
	if o == nil || IsNil(o.Name) {
		return nil, false
	}
	return o.Name, true
}

// HasName returns a boolean if a field has been set.
func (o *ServiceStatus) HasName() bool {
	if o != nil && !IsNil(o.Name) {
		return true
	}

	return false
}

// SetName gets a reference to the given string and assigns it to the Name field.
func (o *ServiceStatus) SetName(v string) {
	o.Name = &v
}

// GetStatus returns the Status field value if set, zero value otherwise.
func (o *ServiceStatus) GetStatus() string {
	if o == nil || IsNil(o.Status) {
		var ret string
		return ret
	}
	return *o.Status
}

// GetStatusOk returns a tuple with the Status field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *ServiceStatus) GetStatusOk() (*string, bool) {
	if o == nil || IsNil(o.Status) {
		return nil, false
	}
	return o.Status, true
}

// HasStatus returns a boolean if a field has been set.
func (o *ServiceStatus) HasStatus() bool {
	if o != nil && !IsNil(o.Status) {
		return true
	}

	return false
}

// SetStatus gets a reference to the given string and assigns it to the Status field.
func (o *ServiceStatus) SetStatus(v string) {
	o.Status = &v
}

// GetMetrics returns the Metrics field value if set, zero value otherwise.
func (o *ServiceStatus) GetMetrics() map[string]string {
	if o == nil || IsNil(o.Metrics) {
		var ret map[string]string
		return ret
	}
	return *o.Metrics
}

// GetMetricsOk returns a tuple with the Metrics field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *ServiceStatus) GetMetricsOk() (*map[string]string, bool) {
	if o == nil || IsNil(o.Metrics) {
		return nil, false
	}
	return o.Metrics, true
}

// HasMetrics returns a boolean if a field has been set.
func (o *ServiceStatus) HasMetrics() bool {
	if o != nil && !IsNil(o.Metrics) {
		return true
	}

	return false
}

// SetMetrics gets a reference to the given map[string]string and assigns it to the Metrics field.
func (o *ServiceStatus) SetMetrics(v map[string]string) {
	o.Metrics = &v
}

// GetDependencies returns the Dependencies field value if set, zero value otherwise.
func (o *ServiceStatus) GetDependencies() []Dependency {
	if o == nil || IsNil(o.Dependencies) {
		var ret []Dependency
		return ret
	}
	return o.Dependencies
}

// GetDependenciesOk returns a tuple with the Dependencies field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *ServiceStatus) GetDependenciesOk() ([]Dependency, bool) {
	if o == nil || IsNil(o.Dependencies) {
		return nil, false
	}
	return o.Dependencies, true
}

// HasDependencies returns a boolean if a field has been set.
func (o *ServiceStatus) HasDependencies() bool {
	if o != nil && !IsNil(o.Dependencies) {
		return true
	}

	return false
}

// SetDependencies gets a reference to the given []Dependency and assigns it to the Dependencies field.
func (o *ServiceStatus) SetDependencies(v []Dependency) {
	o.Dependencies = v
}

func (o ServiceStatus) MarshalJSON() ([]byte, error) {
	toSerialize,err := o.ToMap()
	if err != nil {
		return []byte{}, err
	}
	return json.Marshal(toSerialize)
}

func (o ServiceStatus) ToMap() (map[string]interface{}, error) {
	toSerialize := map[string]interface{}{}
	if !IsNil(o.Name) {
		toSerialize["name"] = o.Name
	}
	if !IsNil(o.Status) {
		toSerialize["status"] = o.Status
	}
	if !IsNil(o.Metrics) {
		toSerialize["metrics"] = o.Metrics
	}
	if !IsNil(o.Dependencies) {
		toSerialize["dependencies"] = o.Dependencies
	}
	return toSerialize, nil
}

type NullableServiceStatus struct {
	value *ServiceStatus
	isSet bool
}

func (v NullableServiceStatus) Get() *ServiceStatus {
	return v.value
}

func (v *NullableServiceStatus) Set(val *ServiceStatus) {
	v.value = val
	v.isSet = true
}

func (v NullableServiceStatus) IsSet() bool {
	return v.isSet
}

func (v *NullableServiceStatus) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableServiceStatus(val *ServiceStatus) *NullableServiceStatus {
	return &NullableServiceStatus{value: val, isSet: true}
}

func (v NullableServiceStatus) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableServiceStatus) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


