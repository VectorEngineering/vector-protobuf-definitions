# ResourceValidation


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**missing_resources** | **List[str]** |  | [optional] 
**invalid_references** | **List[str]** |  | [optional] 
**id_violations** | **Dict[str, str]** |  | [optional] 

## Example

```python
from backend_client.models.resource_validation import ResourceValidation

# TODO update the JSON string below
json = "{}"
# create an instance of ResourceValidation from a JSON string
resource_validation_instance = ResourceValidation.from_json(json)
# print the JSON string representation of the object
print(ResourceValidation.to_json())

# convert the object into a dict
resource_validation_dict = resource_validation_instance.to_dict()
# create an instance of ResourceValidation from a dict
resource_validation_from_dict = ResourceValidation.from_dict(resource_validation_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


