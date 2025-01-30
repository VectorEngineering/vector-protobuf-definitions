# CreateWorkflowBody


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**workflow** | [**ScrapingWorkflow**](ScrapingWorkflow.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.create_workflow_body import CreateWorkflowBody

# TODO update the JSON string below
json = "{}"
# create an instance of CreateWorkflowBody from a JSON string
create_workflow_body_instance = CreateWorkflowBody.from_json(json)
# print the JSON string representation of the object
print(CreateWorkflowBody.to_json())

# convert the object into a dict
create_workflow_body_dict = create_workflow_body_instance.to_dict()
# create an instance of CreateWorkflowBody from a dict
create_workflow_body_from_dict = CreateWorkflowBody.from_dict(create_workflow_body_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


