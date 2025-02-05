# UpdateAccountRequest1


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account** | [**Account1**](Account1.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.update_account_request1 import UpdateAccountRequest1

# TODO update the JSON string below
json = "{}"
# create an instance of UpdateAccountRequest1 from a JSON string
update_account_request1_instance = UpdateAccountRequest1.from_json(json)
# print the JSON string representation of the object
print(UpdateAccountRequest1.to_json())

# convert the object into a dict
update_account_request1_dict = update_account_request1_instance.to_dict()
# create an instance of UpdateAccountRequest1 from a dict
update_account_request1_from_dict = UpdateAccountRequest1.from_dict(update_account_request1_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


