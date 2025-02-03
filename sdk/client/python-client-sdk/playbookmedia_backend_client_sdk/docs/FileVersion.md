# FileVersion


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**file_id** | **str** |  | [optional] 
**version_number** | **int** |  | [optional] 
**s3_key** | **str** |  | [optional] 
**size** | **str** |  | [optional] 
**commit_message** | **str** |  | [optional] 
**author_id** | **str** |  | [optional] 
**created_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.file_version import FileVersion

# TODO update the JSON string below
json = "{}"
# create an instance of FileVersion from a JSON string
file_version_instance = FileVersion.from_json(json)
# print the JSON string representation of the object
print(FileVersion.to_json())

# convert the object into a dict
file_version_dict = file_version_instance.to_dict()
# create an instance of FileVersion from a dict
file_version_from_dict = FileVersion.from_dict(file_version_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


