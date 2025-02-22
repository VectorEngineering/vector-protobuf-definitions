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




// ForbiddenErrorMessageResponse - Indicates that the server understood the request but refuses to authorize it
type ForbiddenErrorMessageResponse struct {

	Code int32 `json:"code,omitempty"`

	Message string `json:"message,omitempty"`

	Reason string `json:"reason,omitempty"`

	RequiredPermissions []string `json:"requiredPermissions,omitempty"`

	ErrorResponse ErrorResponse `json:"errorResponse,omitempty"`
}

// AssertForbiddenErrorMessageResponseRequired checks if the required fields are not zero-ed
func AssertForbiddenErrorMessageResponseRequired(obj ForbiddenErrorMessageResponse) error {
	if err := AssertErrorResponseRequired(obj.ErrorResponse); err != nil {
		return err
	}
	return nil
}

// AssertForbiddenErrorMessageResponseConstraints checks if the values respects the defined constraints
func AssertForbiddenErrorMessageResponseConstraints(obj ForbiddenErrorMessageResponse) error {
	if err := AssertErrorResponseConstraints(obj.ErrorResponse); err != nil {
		return err
	}
	return nil
}
