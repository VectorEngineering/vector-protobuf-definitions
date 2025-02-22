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

// BackgroundJobStatus BackgroundJobStatus represents the possible states of a background job. This enum is used to track the lifecycle of asynchronous tasks like scraping jobs.  State transitions: 1. QUEUED -> IN_PROGRESS 2. IN_PROGRESS -> COMPLETED/FAILED/CANCELLED/TIMED_OUT  Usage example: ```go job := &ScrapingJob{     Status: BackgroundJobStatus_BACKGROUND_JOB_STATUS_IN_PROGRESS, } ```   - BACKGROUND_JOB_STATUS_UNSPECIFIED: Default state, should not be used explicitly  - BACKGROUND_JOB_STATUS_QUEUED: Job is queued and waiting to be processed  - BACKGROUND_JOB_STATUS_IN_PROGRESS: Job is currently being processed  - BACKGROUND_JOB_STATUS_COMPLETED: Job has completed successfully  - BACKGROUND_JOB_STATUS_FAILED: Job encountered an error and failed  - BACKGROUND_JOB_STATUS_CANCELLED: Job was manually cancelled by user  - BACKGROUND_JOB_STATUS_TIMED_OUT: Job exceeded its maximum execution time
type BackgroundJobStatus string

// List of BackgroundJobStatus
const (
	UNSPECIFIED BackgroundJobStatus = "BACKGROUND_JOB_STATUS_UNSPECIFIED"
	QUEUED BackgroundJobStatus = "BACKGROUND_JOB_STATUS_QUEUED"
	IN_PROGRESS BackgroundJobStatus = "BACKGROUND_JOB_STATUS_IN_PROGRESS"
	COMPLETED BackgroundJobStatus = "BACKGROUND_JOB_STATUS_COMPLETED"
	FAILED BackgroundJobStatus = "BACKGROUND_JOB_STATUS_FAILED"
	TIMED_OUT BackgroundJobStatus = "BACKGROUND_JOB_STATUS_TIMED_OUT"
)

// All allowed values of BackgroundJobStatus enum
var AllowedBackgroundJobStatusEnumValues = []BackgroundJobStatus{
	"BACKGROUND_JOB_STATUS_UNSPECIFIED",
	"BACKGROUND_JOB_STATUS_QUEUED",
	"BACKGROUND_JOB_STATUS_IN_PROGRESS",
	"BACKGROUND_JOB_STATUS_COMPLETED",
	"BACKGROUND_JOB_STATUS_FAILED",
	"BACKGROUND_JOB_STATUS_CANCELLED",
	"BACKGROUND_JOB_STATUS_TIMED_OUT",
}

func (v *BackgroundJobStatus) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumTypeValue := BackgroundJobStatus(value)
	for _, existing := range AllowedBackgroundJobStatusEnumValues {
		if existing == enumTypeValue {
			*v = enumTypeValue
			return nil
		}
	}

	return fmt.Errorf("%+v is not a valid BackgroundJobStatus", value)
}

// NewBackgroundJobStatusFromValue returns a pointer to a valid BackgroundJobStatus
// for the value passed as argument, or an error if the value passed is not allowed by the enum
func NewBackgroundJobStatusFromValue(v string) (*BackgroundJobStatus, error) {
	ev := BackgroundJobStatus(v)
	if ev.IsValid() {
		return &ev, nil
	} else {
		return nil, fmt.Errorf("invalid value '%v' for BackgroundJobStatus: valid values are %v", v, AllowedBackgroundJobStatusEnumValues)
	}
}

// IsValid return true if the value is valid for the enum, false otherwise
func (v BackgroundJobStatus) IsValid() bool {
	for _, existing := range AllowedBackgroundJobStatusEnumValues {
		if existing == v {
			return true
		}
	}
	return false
}

// Ptr returns reference to BackgroundJobStatus value
func (v BackgroundJobStatus) Ptr() *BackgroundJobStatus {
	return &v
}

type NullableBackgroundJobStatus struct {
	value *BackgroundJobStatus
	isSet bool
}

func (v NullableBackgroundJobStatus) Get() *BackgroundJobStatus {
	return v.value
}

func (v *NullableBackgroundJobStatus) Set(val *BackgroundJobStatus) {
	v.value = val
	v.isSet = true
}

func (v NullableBackgroundJobStatus) IsSet() bool {
	return v.isSet
}

func (v *NullableBackgroundJobStatus) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableBackgroundJobStatus(val *BackgroundJobStatus) *NullableBackgroundJobStatus {
	return &NullableBackgroundJobStatus{value: val, isSet: true}
}

func (v NullableBackgroundJobStatus) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableBackgroundJobStatus) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}

