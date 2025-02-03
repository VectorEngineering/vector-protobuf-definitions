# UpdateWorkspaceSharingResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sharing** | [**WorkspaceSharing**](WorkspaceSharing.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.update_workspace_sharing_response import UpdateWorkspaceSharingResponse

# TODO update the JSON string below
json = "{}"
# create an instance of UpdateWorkspaceSharingResponse from a JSON string
update_workspace_sharing_response_instance = UpdateWorkspaceSharingResponse.from_json(json)
# print the JSON string representation of the object
print(UpdateWorkspaceSharingResponse.to_json())

# convert the object into a dict
update_workspace_sharing_response_dict = update_workspace_sharing_response_instance.to_dict()
# create an instance of UpdateWorkspaceSharingResponse from a dict
update_workspace_sharing_response_from_dict = UpdateWorkspaceSharingResponse.from_dict(update_workspace_sharing_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


