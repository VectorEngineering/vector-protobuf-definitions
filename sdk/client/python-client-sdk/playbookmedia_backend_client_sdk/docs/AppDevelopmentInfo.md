# AppDevelopmentInfo


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**app_id** | **str** |  | [optional] 
**supported_languages** | **List[str]** |  | [optional] 
**sdk_features** | **List[str]** |  | [optional] 
**test_coverage** | **Dict[str, float]** |  | [optional] 
**security_scans** | **List[str]** |  | [optional] 
**performance_metrics** | **Dict[str, str]** |  | [optional] 
**development_status** | **str** |  | [optional] 
**known_issues** | **List[str]** |  | [optional] 
**roadmap_url** | **str** |  | [optional] 
**last_security_audit** | **datetime** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.app_development_info import AppDevelopmentInfo

# TODO update the JSON string below
json = "{}"
# create an instance of AppDevelopmentInfo from a JSON string
app_development_info_instance = AppDevelopmentInfo.from_json(json)
# print the JSON string representation of the object
print(AppDevelopmentInfo.to_json())

# convert the object into a dict
app_development_info_dict = app_development_info_instance.to_dict()
# create an instance of AppDevelopmentInfo from a dict
app_development_info_from_dict = AppDevelopmentInfo.from_dict(app_development_info_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


