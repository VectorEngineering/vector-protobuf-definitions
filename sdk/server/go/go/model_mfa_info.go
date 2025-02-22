// Code generated by OpenAPI Generator (https://openapi-generator.tech); DO NOT EDIT.

/*
 * Lead Scraping Service API
 *
 * Vector Lead Scraping Service API - Manages Lead Scraping Jobs
 *
 * API version: 1.0
 * Contact: yoanyomba@vector.ai
 */

package backend_server




type MfaInfo struct {

	MfaRequired bool `json:"mfaRequired,omitempty"`

	AllowedMethods []string `json:"allowedMethods,omitempty"`

	FailedReason string `json:"failedReason,omitempty"`
}

// AssertMfaInfoRequired checks if the required fields are not zero-ed
func AssertMfaInfoRequired(obj MfaInfo) error {
	return nil
}

// AssertMfaInfoConstraints checks if the values respects the defined constraints
func AssertMfaInfoConstraints(obj MfaInfo) error {
	return nil
}
