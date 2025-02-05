# Tenant

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Id_** | **String!** |  | [optional] [default to null]
**name** | **String!** |  | [optional] [default to null]
**displayName** | **String!** |  | [optional] [default to null]
**description** | **String!** |  | [optional] [default to null]
**organization** | [***Organization**](Organization.md) |  | [optional] [default to null]
**apiBaseUrl** | **String!** |  | [optional] [default to null]
**environmentVariables** | **String!** |  | [optional] [default to null]
**allowedOrigins** | **String!** |  | [optional] [default to null]
**storageQuota** | **String!** |  | [optional] [default to null]
**monthlyRequestLimit** | **String!** |  | [optional] [default to null]
**maxConcurrentJobs** | **Int!** |  | [optional] [default to null]
**enableCaching** | **Boolean!** |  | [optional] [default to null]
**enableRateLimiting** | **Boolean!** |  | [optional] [default to null]
**enableRequestLogging** | **Boolean!** |  | [optional] [default to null]
**accounts** | [**Account**](Account.md) |  | [optional] [default to null]
**apiKeys** | [**TenantApiKey**](TenantAPIKey.md) |  | [optional] [default to null]
**totalRequests** | **String!** |  | [optional] [default to null]
**totalStorageUsed** | **String!** |  | [optional] [default to null]
**averageResponseTime** | **Float!** |  | [optional] [default to null]
**createdAt** | **String!** |  | [optional] [default to null]
**updatedAt** | **String!** |  | [optional] [default to null]
**deletedAt** | **String!** |  | [optional] [default to null]
**status** | [***V1Status**](v1.Status.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


