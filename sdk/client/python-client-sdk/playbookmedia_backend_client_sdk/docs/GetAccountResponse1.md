# GetAccountResponse1


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account** | [**Account1**](Account1.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.get_account_response1 import GetAccountResponse1

# TODO update the JSON string below
json = "{}"
# create an instance of GetAccountResponse1 from a JSON string
get_account_response1_instance = GetAccountResponse1.from_json(json)
# print the JSON string representation of the object
print(GetAccountResponse1.to_json())

# convert the object into a dict
get_account_response1_dict = get_account_response1_instance.to_dict()
# create an instance of GetAccountResponse1 from a dict
get_account_response1_from_dict = GetAccountResponse1.from_dict(get_account_response1_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


