# TriggerWorkflowBody


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**parameters** | **Dict[str, str]** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.trigger_workflow_body import TriggerWorkflowBody

# TODO update the JSON string below
json = "{}"
# create an instance of TriggerWorkflowBody from a JSON string
trigger_workflow_body_instance = TriggerWorkflowBody.from_json(json)
# print the JSON string representation of the object
print(TriggerWorkflowBody.to_json())

# convert the object into a dict
trigger_workflow_body_dict = trigger_workflow_body_instance.to_dict()
# create an instance of TriggerWorkflowBody from a dict
trigger_workflow_body_from_dict = TriggerWorkflowBody.from_dict(trigger_workflow_body_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


