# FileSharing


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
from playbookmedia_backend_client_sdk.models.file_sharing import FileSharing

# TODO update the JSON string below
json = "{}"
# create an instance of FileSharing from a JSON string
file_sharing_instance = FileSharing.from_json(json)
# print the JSON string representation of the object
print(FileSharing.to_json())

# convert the object into a dict
file_sharing_dict = file_sharing_instance.to_dict()
# create an instance of FileSharing from a dict
file_sharing_from_dict = FileSharing.from_dict(file_sharing_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


