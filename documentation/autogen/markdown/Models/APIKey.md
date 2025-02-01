# APIKey
## Properties

| Name | Type | Description | Notes |
|------------ | ------------- | ------------- | -------------|
| **id** | **String** |  | [optional] [default to null] |
| **name** | **String** |  | [optional] [default to null] |
| **keyHash** | **String** |  | [optional] [default to null] |
| **keyPrefix** | **String** |  | [optional] [default to null] |
| **orgId** | **String** |  | [optional] [default to null] |
| **tenantId** | **String** |  | [optional] [default to null] |
| **scopes** | **List** |  | [optional] [default to null] |
| **allowedIps** | **List** |  | [optional] [default to null] |
| **allowedDomains** | **List** |  | [optional] [default to null] |
| **allowedEnvironments** | **List** |  | [optional] [default to null] |
| **isTestKey** | **Boolean** |  | [optional] [default to null] |
| **requestsPerSecond** | **Integer** |  | [optional] [default to null] |
| **requestsPerDay** | **Integer** |  | [optional] [default to null] |
| **concurrentRequests** | **Integer** |  | [optional] [default to null] |
| **monthlyRequestQuota** | **String** |  | [optional] [default to null] |
| **costPerRequest** | **Float** |  | [optional] [default to null] |
| **billingTier** | **String** |  | [optional] [default to null] |
| **totalRequests** | **String** |  | [optional] [default to null] |
| **totalErrors** | **String** |  | [optional] [default to null] |
| **lastUsedAt** | **Date** |  | [optional] [default to null] |
| **averageResponseTime** | **Float** |  | [optional] [default to null] |
| **endpointUsageJson** | **byte[]** | Tracks usage per endpoint | [optional] [default to null] |
| **errorRatesJson** | **byte[]** |  | [optional] [default to null] |
| **recentErrors** | **byte[]** |  | [optional] [default to null] |
| **successfulRequestsCount** | **Integer** |  | [optional] [default to null] |
| **successRate** | **Float** |  | [optional] [default to null] |
| **status** | [**APIKey.Status**](APIKey.Status.md) |  | [optional] [default to null] |
| **createdAt** | **Date** |  | [optional] [default to null] |
| **updatedAt** | **Date** |  | [optional] [default to null] |
| **expiresAt** | **Date** |  | [optional] [default to null] |
| **deletedAt** | **Date** |  | [optional] [default to null] |
| **lastRotatedAt** | **Date** |  | [optional] [default to null] |
| **lastSecurityReviewAt** | **Date** |  | [optional] [default to null] |
| **requiresClientSecret** | **Boolean** |  | [optional] [default to null] |
| **clientSecretHash** | **String** |  | [optional] [default to null] |
| **enforceHttps** | **Boolean** |  | [optional] [default to null] |
| **enforceSigning** | **Boolean** |  | [optional] [default to null] |
| **allowedSignatureAlgorithms** | **List** |  | [optional] [default to null] |
| **enforceMutualTls** | **Boolean** |  | [optional] [default to null] |
| **clientCertificateHash** | **String** |  | [optional] [default to null] |
| **requireRequestSigning** | **Boolean** |  | [optional] [default to null] |
| **description** | **String** | Detailed description of key usage | [optional] [default to null] |
| **metadataJson** | **byte[]** |  | [optional] [default to null] |
| **tags** | **List** |  | [optional] [default to null] |
| **apiVersion** | **String** |  | [optional] [default to null] |
| **supportedFeatures** | **List** |  | [optional] [default to null] |
| **documentationUrl** | **String** |  | [optional] [default to null] |
| **supportContact** | **String** |  | [optional] [default to null] |
| **account** | [**Account**](Account.md) |  | [optional] [default to null] |
| **workspace** | [**Workspace**](Workspace.md) |  | [optional] [default to null] |
| **logAllRequests** | **Boolean** |  | [optional] [default to null] |
| **lastRotationReason** | **String** |  | [optional] [default to null] |
| **lastRotationDate** | **Date** |  | [optional] [default to null] |
| **rotationFrequencyDays** | **Integer** |  | [optional] [default to null] |
| **complianceStandards** | **List** |  | [optional] [default to null] |
| **requiresAuditLogging** | **Boolean** |  | [optional] [default to null] |
| **dataResidency** | **String** |  | [optional] [default to null] |
| **approvedIntegrations** | **List** |  | [optional] [default to null] |
| **alertEmails** | **List** |  | [optional] [default to null] |
| **webhookUrl** | **String** |  | [optional] [default to null] |
| **alertOnQuotaThreshold** | **Boolean** |  | [optional] [default to null] |
| **quotaAlertThreshold** | **Float** |  | [optional] [default to null] |
| **alertOnErrorSpike** | **Boolean** |  | [optional] [default to null] |
| **errorAlertThreshold** | **Float** |  | [optional] [default to null] |
| **monitoringIntegrations** | **List** |  | [optional] [default to null] |
| **encrypted** | **Boolean** |  | [optional] [default to null] |
| **dataClassification** | **String** |  | [optional] [default to null] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

