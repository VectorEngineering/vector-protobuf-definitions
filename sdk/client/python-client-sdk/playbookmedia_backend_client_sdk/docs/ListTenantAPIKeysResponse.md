# ListTenantAPIKeysResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_keys** | [**List[TenantAPIKey]**](TenantAPIKey.md) |  | [optional] 
**next_page_number** | **int** |  | [optional] 
**total_count** | **int** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.list_tenant_api_keys_response import ListTenantAPIKeysResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ListTenantAPIKeysResponse from a JSON string
list_tenant_api_keys_response_instance = ListTenantAPIKeysResponse.from_json(json)
# print the JSON string representation of the object
print(ListTenantAPIKeysResponse.to_json())

# convert the object into a dict
list_tenant_api_keys_response_dict = list_tenant_api_keys_response_instance.to_dict()
# create an instance of ListTenantAPIKeysResponse from a dict
list_tenant_api_keys_response_from_dict = ListTenantAPIKeysResponse.from_dict(list_tenant_api_keys_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


