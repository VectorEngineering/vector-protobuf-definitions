# GetWorkspaceStorageStatsResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_storage_used** | **str** |  | [optional] 
**storage_quota** | **str** |  | [optional] 
**usage_percentage** | **float** |  | [optional] 
**storage_by_type** | [**List[StorageBreakdown]**](StorageBreakdown.md) |  | [optional] 
**total_files** | **int** |  | [optional] 
**total_folders** | **int** |  | [optional] 
**last_updated** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.get_workspace_storage_stats_response import GetWorkspaceStorageStatsResponse

# TODO update the JSON string below
json = "{}"
# create an instance of GetWorkspaceStorageStatsResponse from a JSON string
get_workspace_storage_stats_response_instance = GetWorkspaceStorageStatsResponse.from_json(json)
# print the JSON string representation of the object
print(GetWorkspaceStorageStatsResponse.to_json())

# convert the object into a dict
get_workspace_storage_stats_response_dict = get_workspace_storage_stats_response_instance.to_dict()
# create an instance of GetWorkspaceStorageStatsResponse from a dict
get_workspace_storage_stats_response_from_dict = GetWorkspaceStorageStatsResponse.from_dict(get_workspace_storage_stats_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


