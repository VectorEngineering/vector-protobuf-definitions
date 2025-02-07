# CreateWorkspaceRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | **str** |  | [optional] 
**name** | **str** |  | 
**storage_quota** | **str** |  | [optional] 
**allow_public_sharing** | **bool** |  | [optional] 
**require_approval** | **bool** |  | [optional] 
**gdpr_compliant** | **bool** |  | [optional] 
**hipaa_compliant** | **bool** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.create_workspace_request import CreateWorkspaceRequest

# TODO update the JSON string below
json = "{}"
# create an instance of CreateWorkspaceRequest from a JSON string
create_workspace_request_instance = CreateWorkspaceRequest.from_json(json)
# print the JSON string representation of the object
print(CreateWorkspaceRequest.to_json())

# convert the object into a dict
create_workspace_request_dict = create_workspace_request_instance.to_dict()
# create an instance of CreateWorkspaceRequest from a dict
create_workspace_request_from_dict = CreateWorkspaceRequest.from_dict(create_workspace_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


