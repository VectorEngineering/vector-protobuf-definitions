# Workspace1


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**name** | **str** |  | [optional] 
**unique_identifier** | **str** |  | [optional] 
**s3_bucket_name** | **str** |  | [optional] 
**s3_folder_path** | **str** |  | [optional] 
**storage_quota** | **str** |  | [optional] 
**used_storage** | **str** |  | [optional] 
**allow_public_sharing** | **bool** |  | [optional] 
**require_approval** | **bool** |  | [optional] 
**gdpr_compliant** | **bool** |  | [optional] 
**hipaa_compliant** | **bool** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 
**deleted_at** | **datetime** |  | [optional] 
**folders** | [**List[FolderMetadata]**](FolderMetadata.md) |  | [optional] 
**templates** | [**List[DocumentTemplate]**](DocumentTemplate.md) |  | [optional] 
**sharing** | [**List[WorkspaceSharing]**](WorkspaceSharing.md) |  | [optional] 
**activities** | [**List[WorkspaceActivity]**](WorkspaceActivity.md) |  | [optional] 
**compliance** | [**List[WorkspaceCompliance]**](WorkspaceCompliance.md) |  | [optional] 
**installed_apps** | [**List[MarketplaceApp]**](MarketplaceApp.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.workspace1 import Workspace1

# TODO update the JSON string below
json = "{}"
# create an instance of Workspace1 from a JSON string
workspace1_instance = Workspace1.from_json(json)
# print the JSON string representation of the object
print(Workspace1.to_json())

# convert the object into a dict
workspace1_dict = workspace1_instance.to_dict()
# create an instance of Workspace1 from a dict
workspace1_from_dict = Workspace1.from_dict(workspace1_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


