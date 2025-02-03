# WorkspaceSharing


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**shared_with_email** | **str** |  | [optional] 
**permission_level** | **str** |  | [optional] 
**expires_at** | **datetime** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.workspace_sharing import WorkspaceSharing

# TODO update the JSON string below
json = "{}"
# create an instance of WorkspaceSharing from a JSON string
workspace_sharing_instance = WorkspaceSharing.from_json(json)
# print the JSON string representation of the object
print(WorkspaceSharing.to_json())

# convert the object into a dict
workspace_sharing_dict = workspace_sharing_instance.to_dict()
# create an instance of WorkspaceSharing from a dict
workspace_sharing_from_dict = WorkspaceSharing.from_dict(workspace_sharing_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


