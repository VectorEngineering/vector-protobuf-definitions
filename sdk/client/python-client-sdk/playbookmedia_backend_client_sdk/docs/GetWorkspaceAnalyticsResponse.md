# GetWorkspaceAnalyticsResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_leads** | **int** |  | [optional] 
**active_workflows** | **int** |  | [optional] 
**jobs_last30_days** | **int** |  | [optional] 
**success_rates** | [**List[JobSuccessRate]**](JobSuccessRate.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.get_workspace_analytics_response import GetWorkspaceAnalyticsResponse

# TODO update the JSON string below
json = "{}"
# create an instance of GetWorkspaceAnalyticsResponse from a JSON string
get_workspace_analytics_response_instance = GetWorkspaceAnalyticsResponse.from_json(json)
# print the JSON string representation of the object
print(GetWorkspaceAnalyticsResponse.to_json())

# convert the object into a dict
get_workspace_analytics_response_dict = get_workspace_analytics_response_instance.to_dict()
# create an instance of GetWorkspaceAnalyticsResponse from a dict
get_workspace_analytics_response_from_dict = GetWorkspaceAnalyticsResponse.from_dict(get_workspace_analytics_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


