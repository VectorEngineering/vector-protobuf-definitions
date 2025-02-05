# ListLeadsResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**leads** | [**List[Lead]**](Lead.md) |  | [optional] 
**total_count** | **int** |  | [optional] 
**next_page_number** | **int** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.list_leads_response import ListLeadsResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ListLeadsResponse from a JSON string
list_leads_response_instance = ListLeadsResponse.from_json(json)
# print the JSON string representation of the object
print(ListLeadsResponse.to_json())

# convert the object into a dict
list_leads_response_dict = list_leads_response_instance.to_dict()
# create an instance of ListLeadsResponse from a dict
list_leads_response_from_dict = ListLeadsResponse.from_dict(list_leads_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


