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
	"fmt"
)

// NotFoundErrorCode - NO_NOT_FOUND_ERROR: Default value as per proto3 convention  - UNDEFINED_ENDPOINT: Endpoint errors  Requested endpoint doesn't exist  - UNIMPLEMENTED: Endpoint not implemented  - STORE_ID_NOT_FOUND: Resource errors  Requested store ID doesn't exist  - USER_NOT_FOUND: Requested user doesn't exist  - RESOURCE_NOT_FOUND: Generic resource not found  - TENANT_NOT_FOUND: Requested tenant doesn't exist
type NotFoundErrorCode string

// List of NotFoundErrorCode
const (
	NO_NOT_FOUND_ERROR NotFoundErrorCode = "NO_NOT_FOUND_ERROR"
	UNDEFINED_ENDPOINT NotFoundErrorCode = "UNDEFINED_ENDPOINT"
	UNIMPLEMENTED NotFoundErrorCode = "UNIMPLEMENTED"
	STORE_ID_NOT_FOUND NotFoundErrorCode = "STORE_ID_NOT_FOUND"
	USER_NOT_FOUND NotFoundErrorCode = "USER_NOT_FOUND"
	RESOURCE_NOT_FOUND NotFoundErrorCode = "RESOURCE_NOT_FOUND"
	TENANT_NOT_FOUND NotFoundErrorCode = "TENANT_NOT_FOUND"
)

// All allowed values of NotFoundErrorCode enum
var AllowedNotFoundErrorCodeEnumValues = []NotFoundErrorCode{
	"NO_NOT_FOUND_ERROR",
	"UNDEFINED_ENDPOINT",
	"UNIMPLEMENTED",
	"STORE_ID_NOT_FOUND",
	"USER_NOT_FOUND",
	"RESOURCE_NOT_FOUND",
	"TENANT_NOT_FOUND",
}

func (v *NotFoundErrorCode) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumTypeValue := NotFoundErrorCode(value)
	for _, existing := range AllowedNotFoundErrorCodeEnumValues {
		if existing == enumTypeValue {
			*v = enumTypeValue
			return nil
		}
	}

	return fmt.Errorf("%+v is not a valid NotFoundErrorCode", value)
}

// NewNotFoundErrorCodeFromValue returns a pointer to a valid NotFoundErrorCode
// for the value passed as argument, or an error if the value passed is not allowed by the enum
func NewNotFoundErrorCodeFromValue(v string) (*NotFoundErrorCode, error) {
	ev := NotFoundErrorCode(v)
	if ev.IsValid() {
		return &ev, nil
	} else {
		return nil, fmt.Errorf("invalid value '%v' for NotFoundErrorCode: valid values are %v", v, AllowedNotFoundErrorCodeEnumValues)
	}
}

// IsValid return true if the value is valid for the enum, false otherwise
func (v NotFoundErrorCode) IsValid() bool {
	for _, existing := range AllowedNotFoundErrorCodeEnumValues {
		if existing == v {
			return true
		}
	}
	return false
}

// Ptr returns reference to NotFoundErrorCode value
func (v NotFoundErrorCode) Ptr() *NotFoundErrorCode {
	return &v
}

type NullableNotFoundErrorCode struct {
	value *NotFoundErrorCode
	isSet bool
}

func (v NullableNotFoundErrorCode) Get() *NotFoundErrorCode {
	return v.value
}

func (v *NullableNotFoundErrorCode) Set(val *NotFoundErrorCode) {
	v.value = val
	v.isSet = true
}

func (v NullableNotFoundErrorCode) IsSet() bool {
	return v.isSet
}

func (v *NullableNotFoundErrorCode) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableNotFoundErrorCode(val *NotFoundErrorCode) *NullableNotFoundErrorCode {
	return &NullableNotFoundErrorCode{value: val, isSet: true}
}

func (v NullableNotFoundErrorCode) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableNotFoundErrorCode) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}

