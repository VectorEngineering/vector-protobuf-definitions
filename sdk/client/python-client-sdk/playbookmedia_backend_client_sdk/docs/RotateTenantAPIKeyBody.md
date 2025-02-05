# RotateTenantAPIKeyBody


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | **str** |  | [optional] 
**tenant_id** | **str** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.rotate_tenant_api_key_body import RotateTenantAPIKeyBody

# TODO update the JSON string below
json = "{}"
# create an instance of RotateTenantAPIKeyBody from a JSON string
rotate_tenant_api_key_body_instance = RotateTenantAPIKeyBody.from_json(json)
# print the JSON string representation of the object
print(RotateTenantAPIKeyBody.to_json())

# convert the object into a dict
rotate_tenant_api_key_body_dict = rotate_tenant_api_key_body_instance.to_dict()
# create an instance of RotateTenantAPIKeyBody from a dict
rotate_tenant_api_key_body_from_dict = RotateTenantAPIKeyBody.from_dict(rotate_tenant_api_key_body_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


