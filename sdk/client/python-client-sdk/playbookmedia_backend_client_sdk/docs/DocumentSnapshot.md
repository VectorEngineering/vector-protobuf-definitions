# DocumentSnapshot


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**file_id** | **str** |  | [optional] 
**snapshot_hash** | **str** |  | [optional] 
**content** | **bytearray** |  | [optional] 
**author_id** | **str** |  | [optional] 
**reason** | **str** |  | [optional] 
**created_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.document_snapshot import DocumentSnapshot

# TODO update the JSON string below
json = "{}"
# create an instance of DocumentSnapshot from a JSON string
document_snapshot_instance = DocumentSnapshot.from_json(json)
# print the JSON string representation of the object
print(DocumentSnapshot.to_json())

# convert the object into a dict
document_snapshot_dict = document_snapshot_instance.to_dict()
# create an instance of DocumentSnapshot from a dict
document_snapshot_from_dict = DocumentSnapshot.from_dict(document_snapshot_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


