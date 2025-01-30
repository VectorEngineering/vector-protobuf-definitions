# UpdateWorkflowRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**workflow** | [**ScrapingWorkflow**](ScrapingWorkflow.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.update_workflow_request import UpdateWorkflowRequest

# TODO update the JSON string below
json = "{}"
# create an instance of UpdateWorkflowRequest from a JSON string
update_workflow_request_instance = UpdateWorkflowRequest.from_json(json)
# print the JSON string representation of the object
print(UpdateWorkflowRequest.to_json())

# convert the object into a dict
update_workflow_request_dict = update_workflow_request_instance.to_dict()
# create an instance of UpdateWorkflowRequest from a dict
update_workflow_request_from_dict = UpdateWorkflowRequest.from_dict(update_workflow_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


