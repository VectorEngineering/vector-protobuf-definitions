# DataProfile


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**name** | **str** |  | [optional] 
**profile_type** | **str** |  | [optional] 
**data_fields** | **Dict[str, str]** |  | [optional] 
**is_default** | **bool** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.data_profile import DataProfile

# TODO update the JSON string below
json = "{}"
# create an instance of DataProfile from a JSON string
data_profile_instance = DataProfile.from_json(json)
# print the JSON string representation of the object
print(DataProfile.to_json())

# convert the object into a dict
data_profile_dict = data_profile_instance.to_dict()
# create an instance of DataProfile from a dict
data_profile_from_dict = DataProfile.from_dict(data_profile_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


