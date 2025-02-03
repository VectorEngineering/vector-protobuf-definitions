# ContextualSummary


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**version_id** | **str** |  | [optional] 
**summary_type** | **str** |  | [optional] 
**content** | **str** |  | [optional] 
**target_audience** | **str** |  | [optional] 
**key_points** | **List[str]** |  | [optional] 
**metadata** | **Dict[str, str]** |  | [optional] 
**created_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.contextual_summary import ContextualSummary

# TODO update the JSON string below
json = "{}"
# create an instance of ContextualSummary from a JSON string
contextual_summary_instance = ContextualSummary.from_json(json)
# print the JSON string representation of the object
print(ContextualSummary.to_json())

# convert the object into a dict
contextual_summary_dict = contextual_summary_instance.to_dict()
# create an instance of ContextualSummary from a dict
contextual_summary_from_dict = ContextualSummary.from_dict(contextual_summary_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


