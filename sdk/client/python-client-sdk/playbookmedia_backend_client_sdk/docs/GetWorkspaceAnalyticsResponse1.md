# GetWorkspaceAnalyticsResponse1


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activity** | [**ActivityMetrics**](ActivityMetrics.md) |  | [optional] 
**user_activities** | [**List[UserActivity]**](UserActivity.md) |  | [optional] 
**compliance** | [**ComplianceMetrics**](ComplianceMetrics.md) |  | [optional] 
**recent_activities** | [**List[WorkspaceActivity]**](WorkspaceActivity.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.get_workspace_analytics_response1 import GetWorkspaceAnalyticsResponse1

# TODO update the JSON string below
json = "{}"
# create an instance of GetWorkspaceAnalyticsResponse1 from a JSON string
get_workspace_analytics_response1_instance = GetWorkspaceAnalyticsResponse1.from_json(json)
# print the JSON string representation of the object
print(GetWorkspaceAnalyticsResponse1.to_json())

# convert the object into a dict
get_workspace_analytics_response1_dict = get_workspace_analytics_response1_instance.to_dict()
# create an instance of GetWorkspaceAnalyticsResponse1 from a dict
get_workspace_analytics_response1_from_dict = GetWorkspaceAnalyticsResponse1.from_dict(get_workspace_analytics_response1_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


