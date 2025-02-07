# CreateAccountBody


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account** | [**Account**](Account.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.create_account_body import CreateAccountBody

# TODO update the JSON string below
json = "{}"
# create an instance of CreateAccountBody from a JSON string
create_account_body_instance = CreateAccountBody.from_json(json)
# print the JSON string representation of the object
print(CreateAccountBody.to_json())

# convert the object into a dict
create_account_body_dict = create_account_body_instance.to_dict()
# create an instance of CreateAccountBody from a dict
create_account_body_from_dict = CreateAccountBody.from_dict(create_account_body_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


