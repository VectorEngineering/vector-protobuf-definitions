# WorkspaceActivity


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**activity_type** | **str** |  | [optional] 
**user_id** | **str** |  | [optional] 
**description** | **str** |  | [optional] 
**metadata** | **Dict[str, str]** |  | [optional] 
**created_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.workspace_activity import WorkspaceActivity

# TODO update the JSON string below
json = "{}"
# create an instance of WorkspaceActivity from a JSON string
workspace_activity_instance = WorkspaceActivity.from_json(json)
# print the JSON string representation of the object
print(WorkspaceActivity.to_json())

# convert the object into a dict
workspace_activity_dict = workspace_activity_instance.to_dict()
# create an instance of WorkspaceActivity from a dict
workspace_activity_from_dict = WorkspaceActivity.from_dict(workspace_activity_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


