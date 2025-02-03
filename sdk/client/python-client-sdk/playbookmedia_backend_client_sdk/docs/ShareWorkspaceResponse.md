# ShareWorkspaceResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sharing** | [**WorkspaceSharing**](WorkspaceSharing.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.share_workspace_response import ShareWorkspaceResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ShareWorkspaceResponse from a JSON string
share_workspace_response_instance = ShareWorkspaceResponse.from_json(json)
# print the JSON string representation of the object
print(ShareWorkspaceResponse.to_json())

# convert the object into a dict
share_workspace_response_dict = share_workspace_response_instance.to_dict()
# create an instance of ShareWorkspaceResponse from a dict
share_workspace_response_from_dict = ShareWorkspaceResponse.from_dict(share_workspace_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


