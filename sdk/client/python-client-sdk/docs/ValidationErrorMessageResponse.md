# ValidationErrorMessageResponse

Represents data validation and business logic errors

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | [**ValidationErrorCode**](ValidationErrorCode.md) |  | [optional] [default to ValidationErrorCode.NO_ERROR]
**message** | **str** |  | [optional] 
**field_violations** | [**List[FieldViolation]**](FieldViolation.md) |  | [optional] 
**schema_validation** | [**SchemaValidation**](SchemaValidation.md) |  | [optional] 
**resource_validation** | [**ResourceValidation**](ResourceValidation.md) |  | [optional] 
**error_response** | [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Example

```python
from backend_client.models.validation_error_message_response import ValidationErrorMessageResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ValidationErrorMessageResponse from a JSON string
validation_error_message_response_instance = ValidationErrorMessageResponse.from_json(json)
# print the JSON string representation of the object
print(ValidationErrorMessageResponse.to_json())

# convert the object into a dict
validation_error_message_response_dict = validation_error_message_response_instance.to_dict()
# create an instance of ValidationErrorMessageResponse from a dict
validation_error_message_response_from_dict = ValidationErrorMessageResponse.from_dict(validation_error_message_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


