# AppAnalytics


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**app_id** | **str** |  | [optional] 
**metric_name** | **str** |  | [optional] 
**metric_value** | **float** |  | [optional] 
**dimensions** | **Dict[str, str]** |  | [optional] 
**recorded_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.app_analytics import AppAnalytics

# TODO update the JSON string below
json = "{}"
# create an instance of AppAnalytics from a JSON string
app_analytics_instance = AppAnalytics.from_json(json)
# print the JSON string representation of the object
print(AppAnalytics.to_json())

# convert the object into a dict
app_analytics_dict = app_analytics_instance.to_dict()
# create an instance of AppAnalytics from a dict
app_analytics_from_dict = AppAnalytics.from_dict(app_analytics_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


