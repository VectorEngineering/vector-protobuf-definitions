# WebhookConfig
## Properties

| Name | Type | Description | Notes |
|------------ | ------------- | ------------- | -------------|
| **id** | **String** |  | [optional] [default to null] |
| **url** | **String** |  | [optional] [default to null] |
| **authType** | **String** | e.g., \&quot;basic\&quot;, \&quot;bearer\&quot;, \&quot;custom\&quot; | [optional] [default to null] |
| **authToken** | **String** |  | [optional] [default to null] |
| **customHeaders** | **Map** |  | [optional] [default to null] |
| **maxRetries** | **Integer** |  | [optional] [default to null] |
| **retryInterval** | **String** |  | [optional] [default to null] |
| **triggerEvents** | [**List**](TriggerEvent.md) |  | [optional] [default to null] |
| **includedFields** | [**List**](IncludedField.md) |  | [optional] [default to null] |
| **includeFullResults** | **Boolean** |  | [optional] [default to null] |
| **payloadFormat** | [**PayloadFormat**](PayloadFormat.md) |  | [optional] [default to null] |
| **verifySsl** | **Boolean** |  | [optional] [default to null] |
| **signingSecret** | **String** |  | [optional] [default to null] |
| **rateLimit** | **Integer** |  | [optional] [default to null] |
| **rateLimitInterval** | **String** |  | [optional] [default to null] |
| **createdAt** | **Date** |  | [optional] [default to null] |
| **updatedAt** | **Date** |  | [optional] [default to null] |
| **lastTriggeredAt** | **Date** |  | [optional] [default to null] |
| **successfulCalls** | **Integer** |  | [optional] [default to null] |
| **failedCalls** | **Integer** |  | [optional] [default to null] |
| **metadata** | [**Object**](.md) |  | [optional] [default to null] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

