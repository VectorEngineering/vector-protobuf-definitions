# ConflictInfo


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**resource_type** | **str** |  | [optional] 
**identifier** | **str** |  | [optional] 
**conflict_reason** | **str** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**created_by** | **str** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.conflict_info import ConflictInfo

# TODO update the JSON string below
json = "{}"
# create an instance of ConflictInfo from a JSON string
conflict_info_instance = ConflictInfo.from_json(json)
# print the JSON string representation of the object
print(ConflictInfo.to_json())

# convert the object into a dict
conflict_info_dict = conflict_info_instance.to_dict()
# create an instance of ConflictInfo from a dict
conflict_info_from_dict = ConflictInfo.from_dict(conflict_info_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


