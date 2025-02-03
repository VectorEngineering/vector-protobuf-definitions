# DocumentTemplate


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**name** | **str** |  | [optional] 
**description** | **str** |  | [optional] 
**industry_type** | **str** |  | [optional] 
**document_type** | **str** |  | [optional] 
**base_content** | **str** |  | [optional] 
**is_adaptive** | **bool** |  | [optional] 
**metadata** | **Dict[str, str]** |  | [optional] 
**supported_languages** | **List[str]** |  | [optional] 
**required_data_fields** | **List[str]** |  | [optional] 
**version** | **str** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 
**template_type** | [**TemplateType**](TemplateType.md) |  | [optional] [default to TemplateType.UNSPECIFIED]
**content** | **str** |  | [optional] 
**detected_variables** | **List[str]** |  | [optional] 
**variable_mappings** | **Dict[str, str]** |  | [optional] 
**is_context_aware** | **bool** |  | [optional] 
**supported_contexts** | **List[str]** |  | [optional] 
**formatting_rules** | **Dict[str, str]** |  | [optional] 
**conditional_sections** | **List[str]** |  | [optional] 
**versions** | [**List[TemplateVersion]**](TemplateVersion.md) |  | [optional] 
**variables** | [**List[TemplateVariable]**](TemplateVariable.md) |  | [optional] 
**instances** | [**List[DocumentInstance]**](DocumentInstance.md) |  | [optional] 
**explanations** | [**List[ExplanationBlock]**](ExplanationBlock.md) |  | [optional] 
**intelligence** | [**List[ContractIntelligence]**](ContractIntelligence.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.document_template import DocumentTemplate

# TODO update the JSON string below
json = "{}"
# create an instance of DocumentTemplate from a JSON string
document_template_instance = DocumentTemplate.from_json(json)
# print the JSON string representation of the object
print(DocumentTemplate.to_json())

# convert the object into a dict
document_template_dict = document_template_instance.to_dict()
# create an instance of DocumentTemplate from a dict
document_template_from_dict = DocumentTemplate.from_dict(document_template_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


