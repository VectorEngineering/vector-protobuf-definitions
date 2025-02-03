# AppVersion


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**app_id** | **str** |  | [optional] 
**version_number** | **str** |  | [optional] 
**changelog** | **str** |  | [optional] 
**requirements** | **Dict[str, str]** |  | [optional] 
**is_public** | **bool** |  | [optional] 
**release_date** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.app_version import AppVersion

# TODO update the JSON string below
json = "{}"
# create an instance of AppVersion from a JSON string
app_version_instance = AppVersion.from_json(json)
# print the JSON string representation of the object
print(AppVersion.to_json())

# convert the object into a dict
app_version_dict = app_version_instance.to_dict()
# create an instance of AppVersion from a dict
app_version_from_dict = AppVersion.from_dict(app_version_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


