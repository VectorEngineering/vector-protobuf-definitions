# CreateTenantAPIKeyResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key_id** | **str** |  | [optional] 
**key_value** | **str** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.create_tenant_api_key_response import CreateTenantAPIKeyResponse

# TODO update the JSON string below
json = "{}"
# create an instance of CreateTenantAPIKeyResponse from a JSON string
create_tenant_api_key_response_instance = CreateTenantAPIKeyResponse.from_json(json)
# print the JSON string representation of the object
print(CreateTenantAPIKeyResponse.to_json())

# convert the object into a dict
create_tenant_api_key_response_dict = create_tenant_api_key_response_instance.to_dict()
# create an instance of CreateTenantAPIKeyResponse from a dict
create_tenant_api_key_response_from_dict = CreateTenantAPIKeyResponse.from_dict(create_tenant_api_key_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


