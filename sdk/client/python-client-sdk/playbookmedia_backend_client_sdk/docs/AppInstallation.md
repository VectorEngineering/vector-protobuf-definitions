# AppInstallation


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**app_id** | **str** |  | [optional] 
**workspace_id** | **str** |  | [optional] 
**version_installed** | **str** |  | [optional] 
**configuration** | **Dict[str, str]** |  | [optional] 
**installation_status** | **str** |  | [optional] 
**installed_at** | **datetime** |  | [optional] 
**last_used** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.app_installation import AppInstallation

# TODO update the JSON string below
json = "{}"
# create an instance of AppInstallation from a JSON string
app_installation_instance = AppInstallation.from_json(json)
# print the JSON string representation of the object
print(AppInstallation.to_json())

# convert the object into a dict
app_installation_dict = app_installation_instance.to_dict()
# create an instance of AppInstallation from a dict
app_installation_from_dict = AppInstallation.from_dict(app_installation_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


