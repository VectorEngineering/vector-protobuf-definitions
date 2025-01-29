# PreconditionFailedErrorMessageResponse

Represents errors when a precondition provided in the request fails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **int** |  | [optional] 
**message** | **str** |  | [optional] 
**failed_conditions** | **List[str]** |  | [optional] 
**error_response** | [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Example

```python
from backend_client.models.precondition_failed_error_message_response import PreconditionFailedErrorMessageResponse

# TODO update the JSON string below
json = "{}"
# create an instance of PreconditionFailedErrorMessageResponse from a JSON string
precondition_failed_error_message_response_instance = PreconditionFailedErrorMessageResponse.from_json(json)
# print the JSON string representation of the object
print(PreconditionFailedErrorMessageResponse.to_json())

# convert the object into a dict
precondition_failed_error_message_response_dict = precondition_failed_error_message_response_instance.to_dict()
# create an instance of PreconditionFailedErrorMessageResponse from a dict
precondition_failed_error_message_response_from_dict = PreconditionFailedErrorMessageResponse.from_dict(precondition_failed_error_message_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


