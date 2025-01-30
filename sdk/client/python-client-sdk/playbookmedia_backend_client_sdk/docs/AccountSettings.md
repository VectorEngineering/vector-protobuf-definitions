# AccountSettings


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**email_notifications** | **bool** |  | [optional] 
**slack_notifications** | **bool** |  | [optional] 
**default_data_retention** | **str** |  | [optional] 
**auto_purge_enabled** | **bool** |  | [optional] 
**require2fa** | **bool** |  | [optional] 
**session_timeout** | **str** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.account_settings import AccountSettings

# TODO update the JSON string below
json = "{}"
# create an instance of AccountSettings from a JSON string
account_settings_instance = AccountSettings.from_json(json)
# print the JSON string representation of the object
print(AccountSettings.to_json())

# convert the object into a dict
account_settings_dict = account_settings_instance.to_dict()
# create an instance of AccountSettings from a dict
account_settings_from_dict = AccountSettings.from_dict(account_settings_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


