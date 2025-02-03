# UpdateWorkspaceSharingRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sharing** | [**WorkspaceSharing**](WorkspaceSharing.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.update_workspace_sharing_request import UpdateWorkspaceSharingRequest

# TODO update the JSON string below
json = "{}"
# create an instance of UpdateWorkspaceSharingRequest from a JSON string
update_workspace_sharing_request_instance = UpdateWorkspaceSharingRequest.from_json(json)
# print the JSON string representation of the object
print(UpdateWorkspaceSharingRequest.to_json())

# convert the object into a dict
update_workspace_sharing_request_dict = update_workspace_sharing_request_instance.to_dict()
# create an instance of UpdateWorkspaceSharingRequest from a dict
update_workspace_sharing_request_from_dict = UpdateWorkspaceSharingRequest.from_dict(update_workspace_sharing_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


