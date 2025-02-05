# UpdateAPIKeyResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_key** | [**APIKey**](APIKey.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.update_api_key_response import UpdateAPIKeyResponse

# TODO update the JSON string below
json = "{}"
# create an instance of UpdateAPIKeyResponse from a JSON string
update_api_key_response_instance = UpdateAPIKeyResponse.from_json(json)
# print the JSON string representation of the object
print(UpdateAPIKeyResponse.to_json())

# convert the object into a dict
update_api_key_response_dict = update_api_key_response_instance.to_dict()
# create an instance of UpdateAPIKeyResponse from a dict
update_api_key_response_from_dict = UpdateAPIKeyResponse.from_dict(update_api_key_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


