# CreateTenantBody


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tenant** | [**Tenant**](Tenant.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.create_tenant_body import CreateTenantBody

# TODO update the JSON string below
json = "{}"
# create an instance of CreateTenantBody from a JSON string
create_tenant_body_instance = CreateTenantBody.from_json(json)
# print the JSON string representation of the object
print(CreateTenantBody.to_json())

# convert the object into a dict
create_tenant_body_dict = create_tenant_body_instance.to_dict()
# create an instance of CreateTenantBody from a dict
create_tenant_body_from_dict = CreateTenantBody.from_dict(create_tenant_body_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


