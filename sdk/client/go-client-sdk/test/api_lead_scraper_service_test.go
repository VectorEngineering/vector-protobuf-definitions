/*
Lead Scraping Service API

Testing LeadScraperServiceAPIService

*/

// Code generated by OpenAPI Generator (https://openapi-generator.tech);

package openapi

import (
	"context"
	"testing"

	openapiclient "github.com/VectorEngineering/vector-protobuf-definitions/sdk/client/go-client-sdk"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func Test_backend_client_LeadScraperServiceAPIService(t *testing.T) {

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)

	t.Run("Test LeadScraperServiceAPIService CreateAccount", func(t *testing.T) {

		t.Skip("skip test")  // remove to run test

		resp, httpRes, err := apiClient.LeadScraperServiceAPI.CreateAccount(context.Background()).Execute()

		require.Nil(t, err)
		require.NotNil(t, resp)
		assert.Equal(t, 200, httpRes.StatusCode)

	})

	t.Run("Test LeadScraperServiceAPIService CreateScrapingJob", func(t *testing.T) {

		t.Skip("skip test")  // remove to run test

		resp, httpRes, err := apiClient.LeadScraperServiceAPI.CreateScrapingJob(context.Background()).Execute()

		require.Nil(t, err)
		require.NotNil(t, resp)
		assert.Equal(t, 200, httpRes.StatusCode)

	})

	t.Run("Test LeadScraperServiceAPIService DeleteAccount", func(t *testing.T) {

		t.Skip("skip test")  // remove to run test

		var accountId string

		resp, httpRes, err := apiClient.LeadScraperServiceAPI.DeleteAccount(context.Background(), accountId).Execute()

		require.Nil(t, err)
		require.NotNil(t, resp)
		assert.Equal(t, 200, httpRes.StatusCode)

	})

	t.Run("Test LeadScraperServiceAPIService DeleteScrapingJob", func(t *testing.T) {

		t.Skip("skip test")  // remove to run test

		var jobId string

		resp, httpRes, err := apiClient.LeadScraperServiceAPI.DeleteScrapingJob(context.Background(), jobId).Execute()

		require.Nil(t, err)
		require.NotNil(t, resp)
		assert.Equal(t, 200, httpRes.StatusCode)

	})

	t.Run("Test LeadScraperServiceAPIService DownloadScrapingResults", func(t *testing.T) {

		t.Skip("skip test")  // remove to run test

		var jobId string

		resp, httpRes, err := apiClient.LeadScraperServiceAPI.DownloadScrapingResults(context.Background(), jobId).Execute()

		require.Nil(t, err)
		require.NotNil(t, resp)
		assert.Equal(t, 200, httpRes.StatusCode)

	})

	t.Run("Test LeadScraperServiceAPIService GetAccount", func(t *testing.T) {

		t.Skip("skip test")  // remove to run test

		var accountId string

		resp, httpRes, err := apiClient.LeadScraperServiceAPI.GetAccount(context.Background(), accountId).Execute()

		require.Nil(t, err)
		require.NotNil(t, resp)
		assert.Equal(t, 200, httpRes.StatusCode)

	})

	t.Run("Test LeadScraperServiceAPIService GetScrapingJob", func(t *testing.T) {

		t.Skip("skip test")  // remove to run test

		var jobId string

		resp, httpRes, err := apiClient.LeadScraperServiceAPI.GetScrapingJob(context.Background(), jobId).Execute()

		require.Nil(t, err)
		require.NotNil(t, resp)
		assert.Equal(t, 200, httpRes.StatusCode)

	})

	t.Run("Test LeadScraperServiceAPIService ListAccounts", func(t *testing.T) {

		t.Skip("skip test")  // remove to run test

		resp, httpRes, err := apiClient.LeadScraperServiceAPI.ListAccounts(context.Background()).Execute()

		require.Nil(t, err)
		require.NotNil(t, resp)
		assert.Equal(t, 200, httpRes.StatusCode)

	})

	t.Run("Test LeadScraperServiceAPIService ListScrapingJobs", func(t *testing.T) {

		t.Skip("skip test")  // remove to run test

		resp, httpRes, err := apiClient.LeadScraperServiceAPI.ListScrapingJobs(context.Background()).Execute()

		require.Nil(t, err)
		require.NotNil(t, resp)
		assert.Equal(t, 200, httpRes.StatusCode)

	})

	t.Run("Test LeadScraperServiceAPIService UpdateAccount", func(t *testing.T) {

		t.Skip("skip test")  // remove to run test

		resp, httpRes, err := apiClient.LeadScraperServiceAPI.UpdateAccount(context.Background()).Execute()

		require.Nil(t, err)
		require.NotNil(t, resp)
		assert.Equal(t, 200, httpRes.StatusCode)

	})

}
