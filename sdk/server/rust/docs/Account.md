# Account

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [optional] [default to None]
**auth_platform_user_id** | **String** |  | [optional] [default to None]
**org_id** | **String** |  | [optional] [default to None]
**tenant_id** | **String** |  | [optional] [default to None]
**email** | **String** |  | [optional] [default to None]
**deleted_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**created_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**account_status** | [***models::AccountStatus**](AccountStatus.md) |  | [optional] [default to None]
**roles** | **Vec<String>** |  | [optional] [default to None]
**permissions** | **Vec<String>** |  | [optional] [default to None]
**mfa_enabled** | **bool** |  | [optional] [default to None]
**last_login_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**timezone** | **String** |  | [optional] [default to None]
**total_jobs_run** | **i32** |  | [optional] [default to None]
**monthly_job_limit** | **i32** |  | [optional] [default to None]
**concurrent_job_limit** | **i32** |  | [optional] [default to None]
**workspaces** | [**Vec<models::Workspace>**](Workspace.md) |  | [optional] [default to None]
**settings** | [***models::AccountSettings**](AccountSettings.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


