# TemplateVariable


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**name** | **str** |  | [optional] 
**description** | **str** |  | [optional] 
**variable_type** | **str** |  | [optional] 
**default_value** | **str** |  | [optional] 
**is_required** | **bool** |  | [optional] 
**validation_rules** | **str** |  | [optional] 
**data_source** | **str** |  | [optional] 
**ai_extraction_rules** | **str** |  | [optional] 
**alternatives** | **List[str]** |  | [optional] 
**template_id** | **str** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.template_variable import TemplateVariable

# TODO update the JSON string below
json = "{}"
# create an instance of TemplateVariable from a JSON string
template_variable_instance = TemplateVariable.from_json(json)
# print the JSON string representation of the object
print(TemplateVariable.to_json())

# convert the object into a dict
template_variable_dict = template_variable_instance.to_dict()
# create an instance of TemplateVariable from a dict
template_variable_from_dict = TemplateVariable.from_dict(template_variable_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


