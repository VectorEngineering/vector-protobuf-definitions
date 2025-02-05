# RotateAPIKeyResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**new_api_key** | [**APIKey**](APIKey.md) |  | [optional] 
**new_key_value** | **str** |  | [optional] 
**old_key_expiry** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.rotate_api_key_response import RotateAPIKeyResponse

# TODO update the JSON string below
json = "{}"
# create an instance of RotateAPIKeyResponse from a JSON string
rotate_api_key_response_instance = RotateAPIKeyResponse.from_json(json)
# print the JSON string representation of the object
print(RotateAPIKeyResponse.to_json())

# convert the object into a dict
rotate_api_key_response_dict = rotate_api_key_response_instance.to_dict()
# create an instance of RotateAPIKeyResponse from a dict
rotate_api_key_response_from_dict = RotateAPIKeyResponse.from_dict(rotate_api_key_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


