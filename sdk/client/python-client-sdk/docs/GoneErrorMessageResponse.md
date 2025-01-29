# GoneErrorMessageResponse

Indicates that the resource requested is no longer available and will not be available again

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **int** |  | [optional] 
**message** | **str** |  | [optional] 
**resource** | **str** |  | [optional] 
**error_response** | [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Example

```python
from backend_client.models.gone_error_message_response import GoneErrorMessageResponse

# TODO update the JSON string below
json = "{}"
# create an instance of GoneErrorMessageResponse from a JSON string
gone_error_message_response_instance = GoneErrorMessageResponse.from_json(json)
# print the JSON string representation of the object
print(GoneErrorMessageResponse.to_json())

# convert the object into a dict
gone_error_message_response_dict = gone_error_message_response_instance.to_dict()
# create an instance of GoneErrorMessageResponse from a dict
gone_error_message_response_from_dict = GoneErrorMessageResponse.from_dict(gone_error_message_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


