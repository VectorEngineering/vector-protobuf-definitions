# UpdateAccountSettingsResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**settings** | [**AccountSettings**](AccountSettings.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.update_account_settings_response import UpdateAccountSettingsResponse

# TODO update the JSON string below
json = "{}"
# create an instance of UpdateAccountSettingsResponse from a JSON string
update_account_settings_response_instance = UpdateAccountSettingsResponse.from_json(json)
# print the JSON string representation of the object
print(UpdateAccountSettingsResponse.to_json())

# convert the object into a dict
update_account_settings_response_dict = update_account_settings_response_instance.to_dict()
# create an instance of UpdateAccountSettingsResponse from a dict
update_account_settings_response_from_dict = UpdateAccountSettingsResponse.from_dict(update_account_settings_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


