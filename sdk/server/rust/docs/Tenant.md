# Tenant

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [optional] [default to None]
**name** | **String** |  | [optional] [default to None]
**display_name** | **String** |  | [optional] [default to None]
**description** | **String** |  | [optional] [default to None]
**api_base_url** | **String** |  | [optional] [default to None]
**environment_variables** | **std::collections::HashMap<String, String>** |  | [optional] [default to None]
**allowed_origins** | **Vec<String>** |  | [optional] [default to None]
**storage_quota** | **String** |  | [optional] [default to None]
**monthly_request_limit** | **String** |  | [optional] [default to None]
**max_concurrent_jobs** | **i32** |  | [optional] [default to None]
**enable_caching** | **bool** |  | [optional] [default to None]
**enable_rate_limiting** | **bool** |  | [optional] [default to None]
**enable_request_logging** | **bool** |  | [optional] [default to None]
**accounts** | [**Vec<models::Account>**](Account.md) |  | [optional] [default to None]
**api_keys** | [**Vec<models::TenantApiKey>**](TenantAPIKey.md) |  | [optional] [default to None]
**total_requests** | **String** |  | [optional] [default to None]
**total_storage_used** | **String** |  | [optional] [default to None]
**average_response_time** | **f32** |  | [optional] [default to None]
**created_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**updated_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**deleted_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**status** | [***models::V1PeriodStatus**](v1.Status.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


