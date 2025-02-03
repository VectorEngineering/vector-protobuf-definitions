# SignatureRequest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [optional] [default to None]
**status** | [***models::SignatureStatus**](SignatureStatus.md) |  | [optional] [default to None]
**signer_email** | **String** |  | [optional] [default to None]
**signer_name** | **String** |  | [optional] [default to None]
**role** | **String** |  | [optional] [default to None]
**authentication_method** | **String** |  | [optional] [default to None]
**expires_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**requires_mfa** | **bool** |  | [optional] [default to None]
**signature_type** | **String** |  | [optional] [default to None]
**reminder_schedule** | **Vec<String>** |  | [optional] [default to None]
**document_id** | **String** |  | [optional] [default to None]
**signature_blocks** | [**Vec<models::SignatureBlock>**](SignatureBlock.md) |  | [optional] [default to None]
**workflow** | [***models::SignatureWorkflow**](SignatureWorkflow.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


