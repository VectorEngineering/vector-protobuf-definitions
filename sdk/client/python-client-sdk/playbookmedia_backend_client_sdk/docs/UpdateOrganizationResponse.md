# UpdateOrganizationResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization** | [**Organization**](Organization.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.update_organization_response import UpdateOrganizationResponse

# TODO update the JSON string below
json = "{}"
# create an instance of UpdateOrganizationResponse from a JSON string
update_organization_response_instance = UpdateOrganizationResponse.from_json(json)
# print the JSON string representation of the object
print(UpdateOrganizationResponse.to_json())

# convert the object into a dict
update_organization_response_dict = update_organization_response_instance.to_dict()
# create an instance of UpdateOrganizationResponse from a dict
update_organization_response_from_dict = UpdateOrganizationResponse.from_dict(update_organization_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


