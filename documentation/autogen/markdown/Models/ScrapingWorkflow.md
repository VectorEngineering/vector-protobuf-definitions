# ScrapingWorkflow
## Properties

| Name | Type | Description | Notes |
|------------ | ------------- | ------------- | -------------|
| **id** | **String** |  | [optional] [default to null] |
| **cronExpression** | **String** |  | [optional] [default to null] |
| **nextRunTime** | **Date** |  | [optional] [default to null] |
| **lastRunTime** | **Date** |  | [optional] [default to null] |
| **status** | [**BackgroundJobStatus**](BackgroundJobStatus.md) |  | [optional] [default to null] |
| **retryCount** | **Integer** |  | [optional] [default to null] |
| **maxRetries** | **Integer** |  | [optional] [default to null] |
| **alertEmails** | **String** |  | [optional] [default to null] |
| **orgId** | **String** |  | [optional] [default to null] |
| **tenantId** | **String** |  | [optional] [default to null] |
| **createdAt** | **Date** |  | [optional] [default to null] |
| **updatedAt** | **Date** |  | [optional] [default to null] |
| **deletedAt** | **Date** |  | [optional] [default to null] |
| **jobs** | [**List**](ScrapingJob.md) |  | [optional] [default to null] |
| **workspace** | [**Workspace**](Workspace.md) |  | [optional] [default to null] |
| **geoFencingRadius** | **Float** |  | [optional] [default to null] |
| **geoFencingLat** | **Double** |  | [optional] [default to null] |
| **geoFencingLon** | **Double** |  | [optional] [default to null] |
| **geoFencingZoomMin** | **Integer** |  | [optional] [default to null] |
| **geoFencingZoomMax** | **Integer** |  | [optional] [default to null] |
| **includeReviews** | **Boolean** |  | [optional] [default to null] |
| **includePhotos** | **Boolean** |  | [optional] [default to null] |
| **includeBusinessHours** | **Boolean** |  | [optional] [default to null] |
| **maxReviewsPerBusiness** | **Integer** |  | [optional] [default to null] |
| **outputFormat** | [**OutputFormat**](OutputFormat.md) |  | [optional] [default to null] |
| **outputDestination** | **String** |  | [optional] [default to null] |
| **dataRetention** | **String** |  | [optional] [default to null] |
| **anonymizePii** | **Boolean** |  | [optional] [default to null] |
| **notificationWebhookUrl** | **String** |  | [optional] [default to null] |
| **notificationSlackChannel** | **String** |  | [optional] [default to null] |
| **notificationEmailGroup** | **String** |  | [optional] [default to null] |
| **notificationNotifyOnStart** | **Boolean** |  | [optional] [default to null] |
| **notificationNotifyOnComplete** | **Boolean** |  | [optional] [default to null] |
| **notificationNotifyOnFailure** | **Boolean** |  | [optional] [default to null] |
| **contentFilterAllowedCountries** | **List** |  | [optional] [default to null] |
| **contentFilterExcludedTypes** | **List** |  | [optional] [default to null] |
| **contentFilterMinimumRating** | **Float** |  | [optional] [default to null] |
| **contentFilterMinimumReviews** | **Integer** |  | [optional] [default to null] |
| **qosMaxConcurrentRequests** | **Integer** |  | [optional] [default to null] |
| **qosMaxRetries** | **Integer** |  | [optional] [default to null] |
| **qosRequestTimeout** | **String** |  | [optional] [default to null] |
| **qosEnableJavascript** | **Boolean** |  | [optional] [default to null] |
| **respectRobotsTxt** | **Boolean** |  | [optional] [default to null] |
| **acceptTermsOfService** | **Boolean** |  | [optional] [default to null] |
| **userAgent** | **String** |  | [optional] [default to null] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

