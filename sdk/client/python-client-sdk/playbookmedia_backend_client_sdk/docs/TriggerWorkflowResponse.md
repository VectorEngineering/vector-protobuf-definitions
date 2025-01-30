# TriggerWorkflowResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**job_id** | **str** |  | [optional] 
**status** | [**BackgroundJobStatus**](BackgroundJobStatus.md) |  | [optional] [default to BackgroundJobStatus.UNSPECIFIED]

## Example

```python
from playbookmedia_backend_client_sdk.models.trigger_workflow_response import TriggerWorkflowResponse

# TODO update the JSON string below
json = "{}"
# create an instance of TriggerWorkflowResponse from a JSON string
trigger_workflow_response_instance = TriggerWorkflowResponse.from_json(json)
# print the JSON string representation of the object
print(TriggerWorkflowResponse.to_json())

# convert the object into a dict
trigger_workflow_response_dict = trigger_workflow_response_instance.to_dict()
# create an instance of TriggerWorkflowResponse from a dict
trigger_workflow_response_from_dict = TriggerWorkflowResponse.from_dict(trigger_workflow_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


