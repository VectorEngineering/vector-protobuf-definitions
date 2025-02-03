# GetWorkspaceStorageStatsResponse

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_storage_used** | **String** |  | [optional] [default to None]
**storage_quota** | **String** |  | [optional] [default to None]
**usage_percentage** | **f32** |  | [optional] [default to None]
**storage_by_type** | [**Vec<models::StorageBreakdown>**](StorageBreakdown.md) |  | [optional] [default to None]
**total_files** | **i32** |  | [optional] [default to None]
**total_folders** | **i32** |  | [optional] [default to None]
**last_updated** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


