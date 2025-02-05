# DeleteTenantAPIKeyResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**success** | **bool** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.delete_tenant_api_key_response import DeleteTenantAPIKeyResponse

# TODO update the JSON string below
json = "{}"
# create an instance of DeleteTenantAPIKeyResponse from a JSON string
delete_tenant_api_key_response_instance = DeleteTenantAPIKeyResponse.from_json(json)
# print the JSON string representation of the object
print(DeleteTenantAPIKeyResponse.to_json())

# convert the object into a dict
delete_tenant_api_key_response_dict = delete_tenant_api_key_response_instance.to_dict()
# create an instance of DeleteTenantAPIKeyResponse from a dict
delete_tenant_api_key_response_from_dict = DeleteTenantAPIKeyResponse.from_dict(delete_tenant_api_key_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


