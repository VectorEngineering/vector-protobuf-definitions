# FieldViolation


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**var_field** | **str** |  | [optional] 
**validation** | **str** |  | [optional] 
**message** | **str** |  | [optional] 
**expected** | **str** |  | [optional] 
**actual** | **str** |  | [optional] 

## Example

```python
from backend_client.models.field_violation import FieldViolation

# TODO update the JSON string below
json = "{}"
# create an instance of FieldViolation from a JSON string
field_violation_instance = FieldViolation.from_json(json)
# print the JSON string representation of the object
print(FieldViolation.to_json())

# convert the object into a dict
field_violation_dict = field_violation_instance.to_dict()
# create an instance of FieldViolation from a dict
field_violation_from_dict = FieldViolation.from_dict(field_violation_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


