# CreateTenantAPIKeyRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | **str** |  | [optional] 
**tenant_id** | **str** |  | [optional] 
**api_key** | [**TenantAPIKey**](TenantAPIKey.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.create_tenant_api_key_request import CreateTenantAPIKeyRequest

# TODO update the JSON string below
json = "{}"
# create an instance of CreateTenantAPIKeyRequest from a JSON string
create_tenant_api_key_request_instance = CreateTenantAPIKeyRequest.from_json(json)
# print the JSON string representation of the object
print(CreateTenantAPIKeyRequest.to_json())

# convert the object into a dict
create_tenant_api_key_request_dict = create_tenant_api_key_request_instance.to_dict()
# create an instance of CreateTenantAPIKeyRequest from a dict
create_tenant_api_key_request_from_dict = CreateTenantAPIKeyRequest.from_dict(create_tenant_api_key_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


