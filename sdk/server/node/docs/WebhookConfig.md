# WebhookConfig

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Id_** | **String!** |  | [optional] [default to null]
**url** | **String!** |  | [optional] [default to null]
**authType** | **String!** | e.g., \&quot;basic\&quot;, \&quot;bearer\&quot;, \&quot;custom\&quot; | [optional] [default to null]
**authToken** | **String!** |  | [optional] [default to null]
**customHeaders** | **String!** |  | [optional] [default to null]
**maxRetries** | **Int!** |  | [optional] [default to null]
**retryInterval** | **String!** |  | [optional] [default to null]
**triggerEvents** | [**TriggerEvent**](TriggerEvent.md) |  | [optional] [default to null]
**includedFields** | [**IncludedField**](IncludedField.md) |  | [optional] [default to null]
**includeFullResults** | **Boolean!** |  | [optional] [default to null]
**payloadFormat** | [***PayloadFormat**](PayloadFormat.md) |  | [optional] [default to null]
**verifySsl** | **Boolean!** |  | [optional] [default to null]
**signingSecret** | **String!** |  | [optional] [default to null]
**rateLimit** | **Int!** |  | [optional] [default to null]
**rateLimitInterval** | **String!** |  | [optional] [default to null]
**createdAt** | **String!** |  | [optional] [default to null]
**updatedAt** | **String!** |  | [optional] [default to null]
**lastTriggeredAt** | **String!** |  | [optional] [default to null]
**successfulCalls** | **Int!** |  | [optional] [default to null]
**failedCalls** | **Int!** |  | [optional] [default to null]
**metadata** | [***TodoObjectMapping**](.md) |  | [optional] [default to null]
**webhookName** | **String!** |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


