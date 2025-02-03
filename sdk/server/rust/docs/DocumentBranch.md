# DocumentBranch

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [optional] [default to None]
**name** | **String** |  | [optional] [default to None]
**base_version_hash** | **String** |  | [optional] [default to None]
**current_head_hash** | **String** |  | [optional] [default to None]
**purpose** | **String** |  | [optional] [default to None]
**status** | **String** |  | [optional] [default to None]
**reviewers** | **Vec<String>** |  | [optional] [default to None]
**branch_metadata** | **std::collections::HashMap<String, String>** |  | [optional] [default to None]
**is_locked** | **bool** |  | [optional] [default to None]
**last_updated** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**merges** | [**Vec<models::BranchMerge>**](BranchMerge.md) |  | [optional] [default to None]
**policy** | [***models::BranchPolicy**](BranchPolicy.md) |  | [optional] [default to None]
**merge_requests** | [**Vec<models::MergeRequest>**](MergeRequest.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


