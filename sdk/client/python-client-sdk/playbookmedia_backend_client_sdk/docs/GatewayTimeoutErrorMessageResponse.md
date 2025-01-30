# GatewayTimeoutErrorMessageResponse

Represents errors when the server did not receive a timely response from an upstream server

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **int** |  | [optional] 
**message** | **str** |  | [optional] 
**upstream_service** | **str** |  | [optional] 
**error_response** | [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.gateway_timeout_error_message_response import GatewayTimeoutErrorMessageResponse

# TODO update the JSON string below
json = "{}"
# create an instance of GatewayTimeoutErrorMessageResponse from a JSON string
gateway_timeout_error_message_response_instance = GatewayTimeoutErrorMessageResponse.from_json(json)
# print the JSON string representation of the object
print(GatewayTimeoutErrorMessageResponse.to_json())

# convert the object into a dict
gateway_timeout_error_message_response_dict = gateway_timeout_error_message_response_instance.to_dict()
# create an instance of GatewayTimeoutErrorMessageResponse from a dict
gateway_timeout_error_message_response_from_dict = GatewayTimeoutErrorMessageResponse.from_dict(gateway_timeout_error_message_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


