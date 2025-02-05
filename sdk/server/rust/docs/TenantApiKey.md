# TenantApiKey

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [optional] [default to None]
**key_hash** | **String** |  | [optional] [default to None]
**key_prefix** | **String** |  | [optional] [default to None]
**name** | **String** |  | [optional] [default to None]
**description** | **String** |  | [optional] [default to None]
**status** | [***models::V1PeriodStatus**](v1.Status.md) |  | [optional] [default to None]
**scopes** | [**Vec<models::TenantApiKeyScope>**](TenantAPIKeyScope.md) |  | [optional] [default to None]
**max_uses** | **i32** |  | [optional] [default to None]
**allowed_ips** | **Vec<String>** |  | [optional] [default to None]
**use_count** | **i32** |  | [optional] [default to None]
**expires_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**created_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**updated_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**deleted_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


