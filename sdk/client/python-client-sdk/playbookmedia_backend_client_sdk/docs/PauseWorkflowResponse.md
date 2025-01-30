# PauseWorkflowResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**success** | **bool** |  | [optional] 
**workflow** | [**ScrapingWorkflow**](ScrapingWorkflow.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.pause_workflow_response import PauseWorkflowResponse

# TODO update the JSON string below
json = "{}"
# create an instance of PauseWorkflowResponse from a JSON string
pause_workflow_response_instance = PauseWorkflowResponse.from_json(json)
# print the JSON string representation of the object
print(PauseWorkflowResponse.to_json())

# convert the object into a dict
pause_workflow_response_dict = pause_workflow_response_instance.to_dict()
# create an instance of PauseWorkflowResponse from a dict
pause_workflow_response_from_dict = PauseWorkflowResponse.from_dict(pause_workflow_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


