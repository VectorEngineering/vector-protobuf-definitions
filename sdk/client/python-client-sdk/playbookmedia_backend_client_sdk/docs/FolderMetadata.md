# FolderMetadata


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**name** | **str** |  | [optional] 
**s3_bucket_name** | **str** |  | [optional] 
**s3_folder_path** | **str** |  | [optional] 
**is_deleted** | **bool** |  | [optional] 
**parent_folder_id** | **str** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 
**deleted_at** | **datetime** |  | [optional] 
**child_folders** | [**List[FolderMetadata]**](FolderMetadata.md) |  | [optional] 
**files** | [**List[FileMetadata]**](FileMetadata.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.folder_metadata import FolderMetadata

# TODO update the JSON string below
json = "{}"
# create an instance of FolderMetadata from a JSON string
folder_metadata_instance = FolderMetadata.from_json(json)
# print the JSON string representation of the object
print(FolderMetadata.to_json())

# convert the object into a dict
folder_metadata_dict = folder_metadata_instance.to_dict()
# create an instance of FolderMetadata from a dict
folder_metadata_from_dict = FolderMetadata.from_dict(folder_metadata_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


