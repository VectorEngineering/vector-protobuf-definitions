# GetTenantAPIKeyResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_key** | [**TenantAPIKey**](TenantAPIKey.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.get_tenant_api_key_response import GetTenantAPIKeyResponse

# TODO update the JSON string below
json = "{}"
# create an instance of GetTenantAPIKeyResponse from a JSON string
get_tenant_api_key_response_instance = GetTenantAPIKeyResponse.from_json(json)
# print the JSON string representation of the object
print(GetTenantAPIKeyResponse.to_json())

# convert the object into a dict
get_tenant_api_key_response_dict = get_tenant_api_key_response_instance.to_dict()
# create an instance of GetTenantAPIKeyResponse from a dict
get_tenant_api_key_response_from_dict = GetTenantAPIKeyResponse.from_dict(get_tenant_api_key_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


