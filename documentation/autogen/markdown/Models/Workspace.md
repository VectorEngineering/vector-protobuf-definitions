# Workspace
## Properties

| Name | Type | Description | Notes |
|------------ | ------------- | ------------- | -------------|
| **id** | **String** |  | [optional] [default to null] |
| **name** | **String** |  | [optional] [default to null] |
| **industry** | **String** |  | [optional] [default to null] |
| **domain** | **String** |  | [optional] [default to null] |
| **gdprCompliant** | **Boolean** |  | [optional] [default to null] |
| **hipaaCompliant** | **Boolean** |  | [optional] [default to null] |
| **soc2Compliant** | **Boolean** |  | [optional] [default to null] |
| **storageQuota** | **String** | in bytes | [optional] [default to null] |
| **usedStorage** | **String** |  | [optional] [default to null] |
| **createdAt** | **Date** |  | [optional] [default to null] |
| **updatedAt** | **Date** |  | [optional] [default to null] |
| **deletedAt** | **Date** |  | [optional] [default to null] |
| **workflows** | [**List**](ScrapingWorkflow.md) |  | [optional] [default to null] |
| **jobsRunThisMonth** | **Integer** |  | [optional] [default to null] |
| **workspaceJobLimit** | **Integer** |  | [optional] [default to null] |
| **dailyJobQuota** | **Integer** |  | [optional] [default to null] |
| **activeScrapers** | **Integer** |  | [optional] [default to null] |
| **totalLeadsCollected** | **Integer** |  | [optional] [default to null] |
| **lastJobRun** | **Date** |  | [optional] [default to null] |
| **scrapingJobs** | [**List**](ScrapingJob.md) |  | [optional] [default to null] |
| **apiKeys** | [**List**](APIKey.md) |  | [optional] [default to null] |
| **webhooks** | [**List**](WebhookConfig.md) |  | [optional] [default to null] |
| **workspaceType** | [**WorkspaceType**](WorkspaceType.md) |  | [optional] [default to null] |
| **description** | **String** |  | [optional] [default to null] |
| **metadata** | **Map** |  | [optional] [default to null] |
| **tags** | **List** |  | [optional] [default to null] |
| **maxTeamMembers** | **Integer** |  | [optional] [default to null] |
| **currentTeamMembers** | **Integer** |  | [optional] [default to null] |
| **allowGuestAccess** | **Boolean** |  | [optional] [default to null] |
| **allowedEmailDomains** | **List** |  | [optional] [default to null] |
| **proxyUrls** | **List** |  | [optional] [default to null] |
| **proxyType** | **String** |  | [optional] [default to null] |
| **rotateProxies** | **Boolean** |  | [optional] [default to null] |
| **proxyRotationInterval** | **Integer** |  | [optional] [default to null] |
| **proxyProvider** | **String** |  | [optional] [default to null] |
| **proxyAuth** | **Map** |  | [optional] [default to null] |
| **maxConcurrentScrapes** | **Integer** |  | [optional] [default to null] |
| **requestsPerSecond** | **Integer** |  | [optional] [default to null] |
| **maxRetries** | **Integer** |  | [optional] [default to null] |
| **retryInterval** | **String** |  | [optional] [default to null] |
| **exportFormats** | **List** | e.g., \&quot;csv\&quot;, \&quot;json\&quot;, \&quot;excel\&quot; | [optional] [default to null] |
| **exportSchedule** | **String** |  | [optional] [default to null] |
| **exportDestinations** | **List** |  | [optional] [default to null] |
| **exportDestinationConfig** | **Map** |  | [optional] [default to null] |
| **compressExports** | **Boolean** |  | [optional] [default to null] |
| **compressionFormat** | **String** |  | [optional] [default to null] |
| **salesforceEnabled** | **Boolean** |  | [optional] [default to null] |
| **salesforceConfig** | **String** |  | [optional] [default to null] |
| **hubspotEnabled** | **Boolean** |  | [optional] [default to null] |
| **hubspotConfig** | **String** |  | [optional] [default to null] |
| **zapierEnabled** | **Boolean** |  | [optional] [default to null] |
| **zapierWebhook** | **String** |  | [optional] [default to null] |
| **activeIntegrations** | **List** |  | [optional] [default to null] |
| **integrationSettings** | **Map** |  | [optional] [default to null] |
| **monthlyBudget** | **Float** |  | [optional] [default to null] |
| **costPerLead** | **Float** |  | [optional] [default to null] |
| **alertOnBudgetThreshold** | **Boolean** |  | [optional] [default to null] |
| **budgetAlertThreshold** | **Float** |  | [optional] [default to null] |
| **billingCurrency** | **String** |  | [optional] [default to null] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

