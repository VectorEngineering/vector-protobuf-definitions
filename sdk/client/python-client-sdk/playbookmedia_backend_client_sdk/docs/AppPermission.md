# AppPermission


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**app_id** | **str** |  | [optional] 
**scope** | **str** |  | [optional] 
**description** | **str** |  | [optional] 
**is_required** | **bool** |  | [optional] 
**access_levels** | **List[str]** |  | [optional] 
**constraints** | **Dict[str, str]** |  | [optional] 
**requires_approval** | **bool** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.app_permission import AppPermission

# TODO update the JSON string below
json = "{}"
# create an instance of AppPermission from a JSON string
app_permission_instance = AppPermission.from_json(json)
# print the JSON string representation of the object
print(AppPermission.to_json())

# convert the object into a dict
app_permission_dict = app_permission_instance.to_dict()
# create an instance of AppPermission from a dict
app_permission_from_dict = AppPermission.from_dict(app_permission_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


