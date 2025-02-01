# Workspace

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [optional] [default to None]
**name** | **String** |  | [optional] [default to None]
**industry** | **String** |  | [optional] [default to None]
**domain** | **String** |  | [optional] [default to None]
**gdpr_compliant** | **bool** |  | [optional] [default to None]
**hipaa_compliant** | **bool** |  | [optional] [default to None]
**soc2_compliant** | **bool** |  | [optional] [default to None]
**storage_quota** | **String** | in bytes | [optional] [default to None]
**used_storage** | **String** |  | [optional] [default to None]
**created_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**updated_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**deleted_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**workflows** | [**Vec<models::ScrapingWorkflow>**](ScrapingWorkflow.md) |  | [optional] [default to None]
**jobs_run_this_month** | **i32** |  | [optional] [default to None]
**workspace_job_limit** | **i32** |  | [optional] [default to None]
**daily_job_quota** | **i32** |  | [optional] [default to None]
**active_scrapers** | **i32** |  | [optional] [default to None]
**total_leads_collected** | **i32** |  | [optional] [default to None]
**last_job_run** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**scraping_jobs** | [**Vec<models::ScrapingJob>**](ScrapingJob.md) |  | [optional] [default to None]
**api_keys** | [**Vec<models::ApiKey>**](APIKey.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


