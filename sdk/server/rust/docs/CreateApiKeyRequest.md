# CreateApiKeyRequest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | **String** |  | [optional] [default to None]
**workspace_id** | **String** |  | [optional] [default to None]
**tenant_id** | **String** |  | [optional] [default to None]
**account_id** | **String** |  | [optional] [default to None]
**name** | **String** |  | [optional] [default to None]
**description** | **String** |  | [optional] [default to None]
**scopes** | **Vec<String>** |  | [optional] [default to None]
**expires_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**max_uses** | **i32** |  | [optional] [default to None]
**allowed_ips** | **Vec<String>** |  | [optional] [default to None]
**rate_limit** | **i32** |  | [optional] [default to None]
**enforce_signing** | **bool** |  | [optional] [default to None]
**allowed_signature_algorithms** | **Vec<String>** |  | [optional] [default to None]
**enforce_mutual_tls** | **bool** |  | [optional] [default to None]
**alert_emails** | **Vec<String>** |  | [optional] [default to None]
**alert_on_quota_threshold** | **bool** |  | [optional] [default to None]
**quota_alert_threshold** | **f32** |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


