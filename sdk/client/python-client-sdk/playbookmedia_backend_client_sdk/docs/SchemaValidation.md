# SchemaValidation


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**schema_version** | **str** |  | [optional] 
**invalid_patterns** | **List[str]** |  | [optional] 
**type_mismatches** | **List[str]** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.schema_validation import SchemaValidation

# TODO update the JSON string below
json = "{}"
# create an instance of SchemaValidation from a JSON string
schema_validation_instance = SchemaValidation.from_json(json)
# print the JSON string representation of the object
print(SchemaValidation.to_json())

# convert the object into a dict
schema_validation_dict = schema_validation_instance.to_dict()
# create an instance of SchemaValidation from a dict
schema_validation_from_dict = SchemaValidation.from_dict(schema_validation_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


