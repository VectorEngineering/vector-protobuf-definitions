# ListAPIKeysResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_keys** | [**List[APIKey]**](APIKey.md) |  | [optional] 
**next_page_number** | **int** |  | [optional] 
**total_count** | **int** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.list_api_keys_response import ListAPIKeysResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ListAPIKeysResponse from a JSON string
list_api_keys_response_instance = ListAPIKeysResponse.from_json(json)
# print the JSON string representation of the object
print(ListAPIKeysResponse.to_json())

# convert the object into a dict
list_api_keys_response_dict = list_api_keys_response_instance.to_dict()
# create an instance of ListAPIKeysResponse from a dict
list_api_keys_response_from_dict = ListAPIKeysResponse.from_dict(list_api_keys_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


