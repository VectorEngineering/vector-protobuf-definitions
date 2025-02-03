# ListWorkspaceSharingsResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sharings** | [**List[WorkspaceSharing]**](WorkspaceSharing.md) |  | [optional] 
**next_page_token** | **str** |  | [optional] 
**total_size** | **int** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.list_workspace_sharings_response import ListWorkspaceSharingsResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ListWorkspaceSharingsResponse from a JSON string
list_workspace_sharings_response_instance = ListWorkspaceSharingsResponse.from_json(json)
# print the JSON string representation of the object
print(ListWorkspaceSharingsResponse.to_json())

# convert the object into a dict
list_workspace_sharings_response_dict = list_workspace_sharings_response_instance.to_dict()
# create an instance of ListWorkspaceSharingsResponse from a dict
list_workspace_sharings_response_from_dict = ListWorkspaceSharingsResponse.from_dict(list_workspace_sharings_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


