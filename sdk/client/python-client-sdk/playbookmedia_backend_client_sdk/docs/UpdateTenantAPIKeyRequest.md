# UpdateTenantAPIKeyRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_key** | [**TenantAPIKey**](TenantAPIKey.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.update_tenant_api_key_request import UpdateTenantAPIKeyRequest

# TODO update the JSON string below
json = "{}"
# create an instance of UpdateTenantAPIKeyRequest from a JSON string
update_tenant_api_key_request_instance = UpdateTenantAPIKeyRequest.from_json(json)
# print the JSON string representation of the object
print(UpdateTenantAPIKeyRequest.to_json())

# convert the object into a dict
update_tenant_api_key_request_dict = update_tenant_api_key_request_instance.to_dict()
# create an instance of UpdateTenantAPIKeyRequest from a dict
update_tenant_api_key_request_from_dict = UpdateTenantAPIKeyRequest.from_dict(update_tenant_api_key_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


