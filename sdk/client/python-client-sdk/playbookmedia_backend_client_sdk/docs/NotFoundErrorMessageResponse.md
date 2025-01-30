# NotFoundErrorMessageResponse

Represents resource not found errors

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | [**NotFoundErrorCode**](NotFoundErrorCode.md) |  | [optional] [default to NotFoundErrorCode.NO_NOT_FOUND_ERROR]
**message** | **str** |  | [optional] 
**resource_info** | [**ResourceInfo**](ResourceInfo.md) |  | [optional] 
**suggestions** | [**Suggestions**](Suggestions.md) |  | [optional] 
**api_info** | [**APIInfo**](APIInfo.md) |  | [optional] 
**error_response** | [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.not_found_error_message_response import NotFoundErrorMessageResponse

# TODO update the JSON string below
json = "{}"
# create an instance of NotFoundErrorMessageResponse from a JSON string
not_found_error_message_response_instance = NotFoundErrorMessageResponse.from_json(json)
# print the JSON string representation of the object
print(NotFoundErrorMessageResponse.to_json())

# convert the object into a dict
not_found_error_message_response_dict = not_found_error_message_response_instance.to_dict()
# create an instance of NotFoundErrorMessageResponse from a dict
not_found_error_message_response_from_dict = NotFoundErrorMessageResponse.from_dict(not_found_error_message_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


