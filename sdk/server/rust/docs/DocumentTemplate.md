# DocumentTemplate

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [optional] [default to None]
**name** | **String** |  | [optional] [default to None]
**description** | **String** |  | [optional] [default to None]
**industry_type** | **String** |  | [optional] [default to None]
**document_type** | **String** |  | [optional] [default to None]
**base_content** | **String** |  | [optional] [default to None]
**is_adaptive** | **bool** |  | [optional] [default to None]
**metadata** | **std::collections::HashMap<String, String>** |  | [optional] [default to None]
**supported_languages** | **Vec<String>** |  | [optional] [default to None]
**required_data_fields** | **Vec<String>** |  | [optional] [default to None]
**version** | **String** |  | [optional] [default to None]
**created_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**updated_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**template_type** | [***models::TemplateType**](TemplateType.md) |  | [optional] [default to None]
**content** | **String** |  | [optional] [default to None]
**detected_variables** | **Vec<String>** |  | [optional] [default to None]
**variable_mappings** | **std::collections::HashMap<String, String>** |  | [optional] [default to None]
**is_context_aware** | **bool** |  | [optional] [default to None]
**supported_contexts** | **Vec<String>** |  | [optional] [default to None]
**formatting_rules** | **std::collections::HashMap<String, String>** |  | [optional] [default to None]
**conditional_sections** | **Vec<String>** |  | [optional] [default to None]
**versions** | [**Vec<models::TemplateVersion>**](TemplateVersion.md) |  | [optional] [default to None]
**variables** | [**Vec<models::TemplateVariable>**](TemplateVariable.md) |  | [optional] [default to None]
**instances** | [**Vec<models::DocumentInstance>**](DocumentInstance.md) |  | [optional] [default to None]
**explanations** | [**Vec<models::ExplanationBlock>**](ExplanationBlock.md) |  | [optional] [default to None]
**intelligence** | [**Vec<models::ContractIntelligence>**](ContractIntelligence.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


