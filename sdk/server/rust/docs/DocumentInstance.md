# DocumentInstance

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [optional] [default to None]
**template_id** | **String** |  | [optional] [default to None]
**status** | [***models::DocumentStatus**](DocumentStatus.md) |  | [optional] [default to None]
**field_values** | **std::collections::HashMap<String, String>** |  | [optional] [default to None]
**signers** | **Vec<String>** |  | [optional] [default to None]
**language** | **String** |  | [optional] [default to None]
**is_completed** | **bool** |  | [optional] [default to None]
**due_date** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**generated_content** | **String** |  | [optional] [default to None]
**attachments** | **Vec<String>** |  | [optional] [default to None]
**created_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**updated_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**signature_requests** | [**Vec<models::SignatureRequest>**](SignatureRequest.md) |  | [optional] [default to None]
**ai_assistance** | [**Vec<models::AiAssistanceLog>**](AIAssistanceLog.md) |  | [optional] [default to None]
**negotiation** | [***models::NegotiationHistory**](NegotiationHistory.md) |  | [optional] [default to None]
**versions** | [**Vec<models::DocumentVersion>**](DocumentVersion.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


