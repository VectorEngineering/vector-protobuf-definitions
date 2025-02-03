# DocumentVersion

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [optional] [default to None]
**version_hash** | **String** |  | [optional] [default to None]
**parent_hash** | **String** |  | [optional] [default to None]
**commit_message** | **String** |  | [optional] [default to None]
**author_id** | **String** |  | [optional] [default to None]
**branch_name** | **String** |  | [optional] [default to None]
**metadata** | **std::collections::HashMap<String, String>** |  | [optional] [default to None]
**tags** | **Vec<String>** |  | [optional] [default to None]
**is_approved** | **bool** |  | [optional] [default to None]
**approval_chain** | **String** |  | [optional] [default to None]
**created_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**branches** | [**Vec<models::DocumentBranch>**](DocumentBranch.md) |  | [optional] [default to None]
**changes** | [**Vec<models::ChangeSet>**](ChangeSet.md) |  | [optional] [default to None]
**summaries** | [**Vec<models::ContextualSummary>**](ContextualSummary.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


