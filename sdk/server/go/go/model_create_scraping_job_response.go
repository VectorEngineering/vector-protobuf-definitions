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




type CreateScrapingJobResponse struct {

	JobId string `json:"jobId,omitempty"`

	Status BackgroundJobStatus `json:"status,omitempty"`
}

// AssertCreateScrapingJobResponseRequired checks if the required fields are not zero-ed
func AssertCreateScrapingJobResponseRequired(obj CreateScrapingJobResponse) error {
	return nil
}

// AssertCreateScrapingJobResponseConstraints checks if the values respects the defined constraints
func AssertCreateScrapingJobResponseConstraints(obj CreateScrapingJobResponse) error {
	return nil
}
