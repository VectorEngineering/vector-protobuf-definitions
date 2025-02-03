# BranchPolicy

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [optional] [default to None]
**branch_id** | **String** |  | [optional] [default to None]
**required_approvers** | **Vec<String>** |  | [optional] [default to None]
**minimum_approvals** | **i32** |  | [optional] [default to None]
**enforce_linear_history** | **bool** |  | [optional] [default to None]
**allow_force_push** | **bool** |  | [optional] [default to None]
**protected_paths** | **Vec<String>** |  | [optional] [default to None]
**merge_rules** | **std::collections::HashMap<String, String>** |  | [optional] [default to None]
**automated_checks** | **Vec<String>** |  | [optional] [default to None]
**created_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**updated_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


