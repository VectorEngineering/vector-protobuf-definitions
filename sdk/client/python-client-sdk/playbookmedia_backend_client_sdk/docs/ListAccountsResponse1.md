# ListAccountsResponse1


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accounts** | [**List[Account1]**](Account1.md) |  | [optional] 
**next_page_token** | **str** |  | [optional] 
**total_size** | **int** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.list_accounts_response1 import ListAccountsResponse1

# TODO update the JSON string below
json = "{}"
# create an instance of ListAccountsResponse1 from a JSON string
list_accounts_response1_instance = ListAccountsResponse1.from_json(json)
# print the JSON string representation of the object
print(ListAccountsResponse1.to_json())

# convert the object into a dict
list_accounts_response1_dict = list_accounts_response1_instance.to_dict()
# create an instance of ListAccountsResponse1 from a dict
list_accounts_response1_from_dict = ListAccountsResponse1.from_dict(list_accounts_response1_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


