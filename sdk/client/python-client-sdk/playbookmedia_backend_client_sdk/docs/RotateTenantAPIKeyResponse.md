# RotateTenantAPIKeyResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**new_api_key** | [**TenantAPIKey**](TenantAPIKey.md) |  | [optional] 
**new_key_value** | **str** |  | [optional] 
**old_key_expiry** | **datetime** |  | [optional] 
**tenant_id** | **str** |  | [optional] 
**organization_id** | **str** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.rotate_tenant_api_key_response import RotateTenantAPIKeyResponse

# TODO update the JSON string below
json = "{}"
# create an instance of RotateTenantAPIKeyResponse from a JSON string
rotate_tenant_api_key_response_instance = RotateTenantAPIKeyResponse.from_json(json)
# print the JSON string representation of the object
print(RotateTenantAPIKeyResponse.to_json())

# convert the object into a dict
rotate_tenant_api_key_response_dict = rotate_tenant_api_key_response_instance.to_dict()
# create an instance of RotateTenantAPIKeyResponse from a dict
rotate_tenant_api_key_response_from_dict = RotateTenantAPIKeyResponse.from_dict(rotate_tenant_api_key_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


