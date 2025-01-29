# ServiceUnavailableErrorMessageResponse

Represents service availability errors

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | [**InternalErrorCode**](InternalErrorCode.md) |  | [optional] [default to InternalErrorCode.NO_INTERNAL_ERROR]
**message** | **str** |  | [optional] 
**availability_info** | [**AvailabilityInfo**](AvailabilityInfo.md) |  | [optional] 
**error_response** | [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Example

```python
from backend_client.models.service_unavailable_error_message_response import ServiceUnavailableErrorMessageResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ServiceUnavailableErrorMessageResponse from a JSON string
service_unavailable_error_message_response_instance = ServiceUnavailableErrorMessageResponse.from_json(json)
# print the JSON string representation of the object
print(ServiceUnavailableErrorMessageResponse.to_json())

# convert the object into a dict
service_unavailable_error_message_response_dict = service_unavailable_error_message_response_instance.to_dict()
# create an instance of ServiceUnavailableErrorMessageResponse from a dict
service_unavailable_error_message_response_from_dict = ServiceUnavailableErrorMessageResponse.from_dict(service_unavailable_error_message_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


