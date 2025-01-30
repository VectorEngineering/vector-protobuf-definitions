# InternalErrorMessageResponse

Represents internal server and system-level errors

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | [**InternalErrorCode**](InternalErrorCode.md) |  | [optional] [default to InternalErrorCode.NO_INTERNAL_ERROR]
**message** | **str** |  | [optional] 
**incident_id** | **str** |  | [optional] 
**service_status** | [**ServiceStatus**](ServiceStatus.md) |  | [optional] 
**resource_utilization** | [**ResourceUtilization**](ResourceUtilization.md) |  | [optional] 
**operation_details** | [**OperationDetails**](OperationDetails.md) |  | [optional] 
**error_response** | [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.internal_error_message_response import InternalErrorMessageResponse

# TODO update the JSON string below
json = "{}"
# create an instance of InternalErrorMessageResponse from a JSON string
internal_error_message_response_instance = InternalErrorMessageResponse.from_json(json)
# print the JSON string representation of the object
print(InternalErrorMessageResponse.to_json())

# convert the object into a dict
internal_error_message_response_dict = internal_error_message_response_instance.to_dict()
# create an instance of InternalErrorMessageResponse from a dict
internal_error_message_response_from_dict = InternalErrorMessageResponse.from_dict(internal_error_message_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


