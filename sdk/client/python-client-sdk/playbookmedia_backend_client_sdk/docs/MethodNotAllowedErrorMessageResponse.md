# MethodNotAllowedErrorMessageResponse

Represents errors when HTTP method is not allowed on the endpoint

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **int** |  | [optional] 
**message** | **str** |  | [optional] 
**allowed_methods** | **List[str]** |  | [optional] 
**error_response** | [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.method_not_allowed_error_message_response import MethodNotAllowedErrorMessageResponse

# TODO update the JSON string below
json = "{}"
# create an instance of MethodNotAllowedErrorMessageResponse from a JSON string
method_not_allowed_error_message_response_instance = MethodNotAllowedErrorMessageResponse.from_json(json)
# print the JSON string representation of the object
print(MethodNotAllowedErrorMessageResponse.to_json())

# convert the object into a dict
method_not_allowed_error_message_response_dict = method_not_allowed_error_message_response_instance.to_dict()
# create an instance of MethodNotAllowedErrorMessageResponse from a dict
method_not_allowed_error_message_response_from_dict = MethodNotAllowedErrorMessageResponse.from_dict(method_not_allowed_error_message_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


