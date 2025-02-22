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




type AuthContext struct {

	RequiredScopes []string `json:"requiredScopes,omitempty"`

	ProvidedScopes []string `json:"providedScopes,omitempty"`

	MissingPermissions []string `json:"missingPermissions,omitempty"`

	TenantId string `json:"tenantId,omitempty"`
}

// AssertAuthContextRequired checks if the required fields are not zero-ed
func AssertAuthContextRequired(obj AuthContext) error {
	return nil
}

// AssertAuthContextConstraints checks if the values respects the defined constraints
func AssertAuthContextConstraints(obj AuthContext) error {
	return nil
}
