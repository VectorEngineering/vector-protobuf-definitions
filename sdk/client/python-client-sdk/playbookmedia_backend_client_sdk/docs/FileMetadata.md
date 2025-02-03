# FileMetadata


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**name** | **str** |  | [optional] 
**size** | **str** |  | [optional] 
**s3_key** | **str** |  | [optional] 
**s3_bucket_name** | **str** |  | [optional] 
**is_deleted** | **bool** |  | [optional] 
**version** | **int** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 
**deleted_at** | **datetime** |  | [optional] 
**embeddings** | [**FileEmbeddings**](FileEmbeddings.md) |  | [optional] 
**versions** | [**List[FileVersion]**](FileVersion.md) |  | [optional] 
**comments** | [**List[CommentThread]**](CommentThread.md) |  | [optional] 
**sharing** | [**List[FileSharing]**](FileSharing.md) |  | [optional] 
**snapshots** | [**List[DocumentSnapshot]**](DocumentSnapshot.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.file_metadata import FileMetadata

# TODO update the JSON string below
json = "{}"
# create an instance of FileMetadata from a JSON string
file_metadata_instance = FileMetadata.from_json(json)
# print the JSON string representation of the object
print(FileMetadata.to_json())

# convert the object into a dict
file_metadata_dict = file_metadata_instance.to_dict()
# create an instance of FileMetadata from a dict
file_metadata_from_dict = FileMetadata.from_dict(file_metadata_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


