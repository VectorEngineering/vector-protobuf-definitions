# UnprocessableEntityErrorMessageResponse

Represents errors when the server understands the content type but cannot process the instructions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **int** |  | [optional] 
**message** | **str** |  | [optional] 
**errors** | [**List[FieldError]**](FieldError.md) |  | [optional] 
**error_response** | [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.unprocessable_entity_error_message_response import UnprocessableEntityErrorMessageResponse

# TODO update the JSON string below
json = "{}"
# create an instance of UnprocessableEntityErrorMessageResponse from a JSON string
unprocessable_entity_error_message_response_instance = UnprocessableEntityErrorMessageResponse.from_json(json)
# print the JSON string representation of the object
print(UnprocessableEntityErrorMessageResponse.to_json())

# convert the object into a dict
unprocessable_entity_error_message_response_dict = unprocessable_entity_error_message_response_instance.to_dict()
# create an instance of UnprocessableEntityErrorMessageResponse from a dict
unprocessable_entity_error_message_response_from_dict = UnprocessableEntityErrorMessageResponse.from_dict(unprocessable_entity_error_message_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


