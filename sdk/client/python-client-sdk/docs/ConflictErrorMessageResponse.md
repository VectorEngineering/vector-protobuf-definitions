# ConflictErrorMessageResponse

Represents resource conflict errors

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | [**ValidationErrorCode**](ValidationErrorCode.md) |  | [optional] [default to ValidationErrorCode.NO_ERROR]
**message** | **str** |  | [optional] 
**conflict_info** | [**ConflictInfo**](ConflictInfo.md) |  | [optional] 
**error_response** | [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Example

```python
from backend_client.models.conflict_error_message_response import ConflictErrorMessageResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ConflictErrorMessageResponse from a JSON string
conflict_error_message_response_instance = ConflictErrorMessageResponse.from_json(json)
# print the JSON string representation of the object
print(ConflictErrorMessageResponse.to_json())

# convert the object into a dict
conflict_error_message_response_dict = conflict_error_message_response_instance.to_dict()
# create an instance of ConflictErrorMessageResponse from a dict
conflict_error_message_response_from_dict = ConflictErrorMessageResponse.from_dict(conflict_error_message_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


