# GetOrganizationResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization** | [**Organization**](Organization.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.get_organization_response import GetOrganizationResponse

# TODO update the JSON string below
json = "{}"
# create an instance of GetOrganizationResponse from a JSON string
get_organization_response_instance = GetOrganizationResponse.from_json(json)
# print the JSON string representation of the object
print(GetOrganizationResponse.to_json())

# convert the object into a dict
get_organization_response_dict = get_organization_response_instance.to_dict()
# create an instance of GetOrganizationResponse from a dict
get_organization_response_from_dict = GetOrganizationResponse.from_dict(get_organization_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


