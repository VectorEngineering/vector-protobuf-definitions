# CreateTenantResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tenant_id** | **str** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.create_tenant_response import CreateTenantResponse

# TODO update the JSON string below
json = "{}"
# create an instance of CreateTenantResponse from a JSON string
create_tenant_response_instance = CreateTenantResponse.from_json(json)
# print the JSON string representation of the object
print(CreateTenantResponse.to_json())

# convert the object into a dict
create_tenant_response_dict = create_tenant_response_instance.to_dict()
# create an instance of CreateTenantResponse from a dict
create_tenant_response_from_dict = CreateTenantResponse.from_dict(create_tenant_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


