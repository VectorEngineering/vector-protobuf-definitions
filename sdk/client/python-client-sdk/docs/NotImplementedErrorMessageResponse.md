# NotImplementedErrorMessageResponse

Represents errors when the requested functionality is not implemented

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **int** |  | [optional] 
**message** | **str** |  | [optional] 
**feature** | **str** |  | [optional] 
**error_response** | [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Example

```python
from backend_client.models.not_implemented_error_message_response import NotImplementedErrorMessageResponse

# TODO update the JSON string below
json = "{}"
# create an instance of NotImplementedErrorMessageResponse from a JSON string
not_implemented_error_message_response_instance = NotImplementedErrorMessageResponse.from_json(json)
# print the JSON string representation of the object
print(NotImplementedErrorMessageResponse.to_json())

# convert the object into a dict
not_implemented_error_message_response_dict = not_implemented_error_message_response_instance.to_dict()
# create an instance of NotImplementedErrorMessageResponse from a dict
not_implemented_error_message_response_from_dict = NotImplementedErrorMessageResponse.from_dict(not_implemented_error_message_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


