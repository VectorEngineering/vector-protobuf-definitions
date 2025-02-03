# FileEmbeddings


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**embeddings** | **List[float]** |  | [optional] 
**model_version** | **str** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.file_embeddings import FileEmbeddings

# TODO update the JSON string below
json = "{}"
# create an instance of FileEmbeddings from a JSON string
file_embeddings_instance = FileEmbeddings.from_json(json)
# print the JSON string representation of the object
print(FileEmbeddings.to_json())

# convert the object into a dict
file_embeddings_dict = file_embeddings_instance.to_dict()
# create an instance of FileEmbeddings from a dict
file_embeddings_from_dict = FileEmbeddings.from_dict(file_embeddings_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


