# ListTenantsResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tenants** | [**List[Tenant]**](Tenant.md) |  | [optional] 
**next_page_number** | **int** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.list_tenants_response import ListTenantsResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ListTenantsResponse from a JSON string
list_tenants_response_instance = ListTenantsResponse.from_json(json)
# print the JSON string representation of the object
print(ListTenantsResponse.to_json())

# convert the object into a dict
list_tenants_response_dict = list_tenants_response_instance.to_dict()
# create an instance of ListTenantsResponse from a dict
list_tenants_response_from_dict = ListTenantsResponse.from_dict(list_tenants_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


