# GetAPIKeyResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_key** | [**APIKey**](APIKey.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.get_api_key_response import GetAPIKeyResponse

# TODO update the JSON string below
json = "{}"
# create an instance of GetAPIKeyResponse from a JSON string
get_api_key_response_instance = GetAPIKeyResponse.from_json(json)
# print the JSON string representation of the object
print(GetAPIKeyResponse.to_json())

# convert the object into a dict
get_api_key_response_dict = get_api_key_response_instance.to_dict()
# create an instance of GetAPIKeyResponse from a dict
get_api_key_response_from_dict = GetAPIKeyResponse.from_dict(get_api_key_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


