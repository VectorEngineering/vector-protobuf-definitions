# ListOrganizationsResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organizations** | [**List[Organization]**](Organization.md) |  | [optional] 
**total_count** | **int** |  | [optional] 
**next_page_number** | **int** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.list_organizations_response import ListOrganizationsResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ListOrganizationsResponse from a JSON string
list_organizations_response_instance = ListOrganizationsResponse.from_json(json)
# print the JSON string representation of the object
print(ListOrganizationsResponse.to_json())

# convert the object into a dict
list_organizations_response_dict = list_organizations_response_instance.to_dict()
# create an instance of ListOrganizationsResponse from a dict
list_organizations_response_from_dict = ListOrganizationsResponse.from_dict(list_organizations_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


