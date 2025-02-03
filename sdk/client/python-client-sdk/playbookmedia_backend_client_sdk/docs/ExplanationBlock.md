# ExplanationBlock


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**template_id** | **str** |  | [optional] 
**section_identifier** | **str** |  | [optional] 
**explanation_text** | **str** |  | [optional] 
**complexity_level** | **str** |  | [optional] 
**related_terms** | **List[str]** |  | [optional] 
**legal_references** | **List[str]** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.explanation_block import ExplanationBlock

# TODO update the JSON string below
json = "{}"
# create an instance of ExplanationBlock from a JSON string
explanation_block_instance = ExplanationBlock.from_json(json)
# print the JSON string representation of the object
print(ExplanationBlock.to_json())

# convert the object into a dict
explanation_block_dict = explanation_block_instance.to_dict()
# create an instance of ExplanationBlock from a dict
explanation_block_from_dict = ExplanationBlock.from_dict(explanation_block_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


