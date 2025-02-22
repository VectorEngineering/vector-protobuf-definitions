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


import (
	"time"
)



// Account - Account represents a user account within the workspace service. It serves as the top-level container for all user-specific workspaces and settings.  Key features: - Unique identification via Auth0 - Organization and tenant context - Audit timestamps - Associated scraping jobs  Database considerations: - Uses GORM for ORM mapping - Includes indexes for efficient querying - Supports soft deletes via deleted_at  Usage example: ```go account := &Account{     AuthPlatformUserId: \"auth0|123\",     OrgId: \"org_123\",     TenantId: \"tenant_456\", } ```
type Account struct {

	Id string `json:"id,omitempty"`

	AuthPlatformUserId string `json:"authPlatformUserId,omitempty"`

	OrgId string `json:"orgId,omitempty"`

	TenantId string `json:"tenantId,omitempty"`

	Email string `json:"email,omitempty"`

	LastModifiedAt time.Time `json:"lastModifiedAt,omitempty"`

	DeletedAt time.Time `json:"deletedAt,omitempty"`

	CreatedAt time.Time `json:"createdAt,omitempty"`

	ScrapingJobs []ScrapingJob `json:"scrapingJobs,omitempty"`
}

// AssertAccountRequired checks if the required fields are not zero-ed
func AssertAccountRequired(obj Account) error {
	for _, el := range obj.ScrapingJobs {
		if err := AssertScrapingJobRequired(el); err != nil {
			return err
		}
	}
	return nil
}

// AssertAccountConstraints checks if the values respects the defined constraints
func AssertAccountConstraints(obj Account) error {
	for _, el := range obj.ScrapingJobs {
		if err := AssertScrapingJobConstraints(el); err != nil {
			return err
		}
	}
	return nil
}
