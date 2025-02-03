# ShareWorkspaceBody


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shared_with_email** | **str** |  | [optional] 
**permission_level** | **str** |  | [optional] 
**expires_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.share_workspace_body import ShareWorkspaceBody

# TODO update the JSON string below
json = "{}"
# create an instance of ShareWorkspaceBody from a JSON string
share_workspace_body_instance = ShareWorkspaceBody.from_json(json)
# print the JSON string representation of the object
print(ShareWorkspaceBody.to_json())

# convert the object into a dict
share_workspace_body_dict = share_workspace_body_instance.to_dict()
# create an instance of ShareWorkspaceBody from a dict
share_workspace_body_from_dict = ShareWorkspaceBody.from_dict(share_workspace_body_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


