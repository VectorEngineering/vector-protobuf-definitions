# RotateTenantAPIKeyRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | **str** |  | [optional] 
**tenant_id** | **str** |  | [optional] 
**key_id** | **str** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.rotate_tenant_api_key_request import RotateTenantAPIKeyRequest

# TODO update the JSON string below
json = "{}"
# create an instance of RotateTenantAPIKeyRequest from a JSON string
rotate_tenant_api_key_request_instance = RotateTenantAPIKeyRequest.from_json(json)
# print the JSON string representation of the object
print(RotateTenantAPIKeyRequest.to_json())

# convert the object into a dict
rotate_tenant_api_key_request_dict = rotate_tenant_api_key_request_instance.to_dict()
# create an instance of RotateTenantAPIKeyRequest from a dict
rotate_tenant_api_key_request_from_dict = RotateTenantAPIKeyRequest.from_dict(rotate_tenant_api_key_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


