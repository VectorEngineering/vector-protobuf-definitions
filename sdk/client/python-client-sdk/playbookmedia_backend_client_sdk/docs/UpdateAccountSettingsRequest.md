# UpdateAccountSettingsRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**settings** | [**AccountSettings**](AccountSettings.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.update_account_settings_request import UpdateAccountSettingsRequest

# TODO update the JSON string below
json = "{}"
# create an instance of UpdateAccountSettingsRequest from a JSON string
update_account_settings_request_instance = UpdateAccountSettingsRequest.from_json(json)
# print the JSON string representation of the object
print(UpdateAccountSettingsRequest.to_json())

# convert the object into a dict
update_account_settings_request_dict = update_account_settings_request_instance.to_dict()
# create an instance of UpdateAccountSettingsRequest from a dict
update_account_settings_request_from_dict = UpdateAccountSettingsRequest.from_dict(update_account_settings_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


