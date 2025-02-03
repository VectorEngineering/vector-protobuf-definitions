# ActivityMetrics


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_files** | **int** |  | [optional] 
**total_folders** | **int** |  | [optional] 
**active_users** | **int** |  | [optional] 
**storage_used** | **str** |  | [optional] 
**storage_usage_percentage** | **float** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.activity_metrics import ActivityMetrics

# TODO update the JSON string below
json = "{}"
# create an instance of ActivityMetrics from a JSON string
activity_metrics_instance = ActivityMetrics.from_json(json)
# print the JSON string representation of the object
print(ActivityMetrics.to_json())

# convert the object into a dict
activity_metrics_dict = activity_metrics_instance.to_dict()
# create an instance of ActivityMetrics from a dict
activity_metrics_from_dict = ActivityMetrics.from_dict(activity_metrics_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


