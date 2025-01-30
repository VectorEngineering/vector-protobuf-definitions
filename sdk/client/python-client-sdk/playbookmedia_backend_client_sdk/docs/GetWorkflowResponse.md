# GetWorkflowResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**workflow** | [**ScrapingWorkflow**](ScrapingWorkflow.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.get_workflow_response import GetWorkflowResponse

# TODO update the JSON string below
json = "{}"
# create an instance of GetWorkflowResponse from a JSON string
get_workflow_response_instance = GetWorkflowResponse.from_json(json)
# print the JSON string representation of the object
print(GetWorkflowResponse.to_json())

# convert the object into a dict
get_workflow_response_dict = get_workflow_response_instance.to_dict()
# create an instance of GetWorkflowResponse from a dict
get_workflow_response_from_dict = GetWorkflowResponse.from_dict(get_workflow_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


