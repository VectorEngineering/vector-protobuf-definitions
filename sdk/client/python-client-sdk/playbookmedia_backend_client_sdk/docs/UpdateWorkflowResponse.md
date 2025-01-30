# UpdateWorkflowResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**workflow** | [**ScrapingWorkflow**](ScrapingWorkflow.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.update_workflow_response import UpdateWorkflowResponse

# TODO update the JSON string below
json = "{}"
# create an instance of UpdateWorkflowResponse from a JSON string
update_workflow_response_instance = UpdateWorkflowResponse.from_json(json)
# print the JSON string representation of the object
print(UpdateWorkflowResponse.to_json())

# convert the object into a dict
update_workflow_response_dict = update_workflow_response_instance.to_dict()
# create an instance of UpdateWorkflowResponse from a dict
update_workflow_response_from_dict = UpdateWorkflowResponse.from_dict(update_workflow_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


