# ChangeSet


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**version_id** | **str** |  | [optional] 
**change_type** | **str** |  | [optional] 
**content_before** | **str** |  | [optional] 
**content_after** | **str** |  | [optional] 
**metadata** | **Dict[str, str]** |  | [optional] 
**created_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.change_set import ChangeSet

# TODO update the JSON string below
json = "{}"
# create an instance of ChangeSet from a JSON string
change_set_instance = ChangeSet.from_json(json)
# print the JSON string representation of the object
print(ChangeSet.to_json())

# convert the object into a dict
change_set_dict = change_set_instance.to_dict()
# create an instance of ChangeSet from a dict
change_set_from_dict = ChangeSet.from_dict(change_set_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


