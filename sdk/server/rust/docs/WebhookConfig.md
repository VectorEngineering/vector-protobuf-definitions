# WebhookConfig

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [optional] [default to None]
**url** | **String** |  | [optional] [default to None]
**auth_type** | **String** | e.g., \"basic\", \"bearer\", \"custom\" | [optional] [default to None]
**auth_token** | **String** |  | [optional] [default to None]
**custom_headers** | **std::collections::HashMap<String, String>** |  | [optional] [default to None]
**max_retries** | **i32** |  | [optional] [default to None]
**retry_interval** | **String** |  | [optional] [default to None]
**trigger_events** | [**Vec<models::TriggerEvent>**](TriggerEvent.md) |  | [optional] [default to None]
**included_fields** | [**Vec<models::IncludedField>**](IncludedField.md) |  | [optional] [default to None]
**include_full_results** | **bool** |  | [optional] [default to None]
**payload_format** | [***models::PayloadFormat**](PayloadFormat.md) |  | [optional] [default to None]
**verify_ssl** | **bool** |  | [optional] [default to None]
**signing_secret** | **String** |  | [optional] [default to None]
**rate_limit** | **i32** |  | [optional] [default to None]
**rate_limit_interval** | **String** |  | [optional] [default to None]
**created_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**updated_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**last_triggered_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**successful_calls** | **i32** |  | [optional] [default to None]
**failed_calls** | **i32** |  | [optional] [default to None]
**metadata** | [***serde_json::Value**](.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


