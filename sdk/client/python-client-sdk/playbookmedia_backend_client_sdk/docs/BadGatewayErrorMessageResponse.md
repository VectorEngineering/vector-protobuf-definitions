# BadGatewayErrorMessageResponse

Represents errors when the server received an invalid response from an upstream server

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **int** |  | [optional] 
**message** | **str** |  | [optional] 
**upstream_service** | **str** |  | [optional] 
**error_response** | [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.bad_gateway_error_message_response import BadGatewayErrorMessageResponse

# TODO update the JSON string below
json = "{}"
# create an instance of BadGatewayErrorMessageResponse from a JSON string
bad_gateway_error_message_response_instance = BadGatewayErrorMessageResponse.from_json(json)
# print the JSON string representation of the object
print(BadGatewayErrorMessageResponse.to_json())

# convert the object into a dict
bad_gateway_error_message_response_dict = bad_gateway_error_message_response_instance.to_dict()
# create an instance of BadGatewayErrorMessageResponse from a dict
bad_gateway_error_message_response_from_dict = BadGatewayErrorMessageResponse.from_dict(bad_gateway_error_message_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


