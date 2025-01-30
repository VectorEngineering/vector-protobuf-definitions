# PauseWorkflowBody


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pause** | **bool** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.pause_workflow_body import PauseWorkflowBody

# TODO update the JSON string below
json = "{}"
# create an instance of PauseWorkflowBody from a JSON string
pause_workflow_body_instance = PauseWorkflowBody.from_json(json)
# print the JSON string representation of the object
print(PauseWorkflowBody.to_json())

# convert the object into a dict
pause_workflow_body_dict = pause_workflow_body_instance.to_dict()
# create an instance of PauseWorkflowBody from a dict
pause_workflow_body_from_dict = PauseWorkflowBody.from_dict(pause_workflow_body_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


