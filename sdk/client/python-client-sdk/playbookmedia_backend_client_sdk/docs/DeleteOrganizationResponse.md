# DeleteOrganizationResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**success** | **bool** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.delete_organization_response import DeleteOrganizationResponse

# TODO update the JSON string below
json = "{}"
# create an instance of DeleteOrganizationResponse from a JSON string
delete_organization_response_instance = DeleteOrganizationResponse.from_json(json)
# print the JSON string representation of the object
print(DeleteOrganizationResponse.to_json())

# convert the object into a dict
delete_organization_response_dict = delete_organization_response_instance.to_dict()
# create an instance of DeleteOrganizationResponse from a dict
delete_organization_response_from_dict = DeleteOrganizationResponse.from_dict(delete_organization_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


