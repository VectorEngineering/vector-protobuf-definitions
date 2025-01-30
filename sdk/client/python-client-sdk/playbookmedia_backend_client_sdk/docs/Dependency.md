# Dependency


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **str** |  | [optional] 
**status** | **str** |  | [optional] 
**error** | **str** |  | [optional] 
**latency** | **int** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.dependency import Dependency

# TODO update the JSON string below
json = "{}"
# create an instance of Dependency from a JSON string
dependency_instance = Dependency.from_json(json)
# print the JSON string representation of the object
print(Dependency.to_json())

# convert the object into a dict
dependency_dict = dependency_instance.to_dict()
# create an instance of Dependency from a dict
dependency_from_dict = Dependency.from_dict(dependency_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


