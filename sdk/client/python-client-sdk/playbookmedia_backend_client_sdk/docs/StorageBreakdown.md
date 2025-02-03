# StorageBreakdown


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file_type** | **str** |  | [optional] 
**size** | **str** |  | [optional] 
**file_count** | **int** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.storage_breakdown import StorageBreakdown

# TODO update the JSON string below
json = "{}"
# create an instance of StorageBreakdown from a JSON string
storage_breakdown_instance = StorageBreakdown.from_json(json)
# print the JSON string representation of the object
print(StorageBreakdown.to_json())

# convert the object into a dict
storage_breakdown_dict = storage_breakdown_instance.to_dict()
# create an instance of StorageBreakdown from a dict
storage_breakdown_from_dict = StorageBreakdown.from_dict(storage_breakdown_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


