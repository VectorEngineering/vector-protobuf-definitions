# ForbiddenErrorMessageResponse

Indicates that the server understood the request but refuses to authorize it

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **int** |  | [optional] 
**message** | **str** |  | [optional] 
**reason** | **str** |  | [optional] 
**required_permissions** | **List[str]** |  | [optional] 
**error_response** | [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.forbidden_error_message_response import ForbiddenErrorMessageResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ForbiddenErrorMessageResponse from a JSON string
forbidden_error_message_response_instance = ForbiddenErrorMessageResponse.from_json(json)
# print the JSON string representation of the object
print(ForbiddenErrorMessageResponse.to_json())

# convert the object into a dict
forbidden_error_message_response_dict = forbidden_error_message_response_instance.to_dict()
# create an instance of ForbiddenErrorMessageResponse from a dict
forbidden_error_message_response_from_dict = ForbiddenErrorMessageResponse.from_dict(forbidden_error_message_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


