# ScrapingWorkflow

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [optional] [default to None]
**name** | **String** |  | [optional] [default to None]
**cron_expression** | **String** |  | [optional] [default to None]
**next_run_time** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**last_run_time** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**status** | [***models::WorkflowStatus**](WorkflowStatus.md) |  | [optional] [default to None]
**retry_count** | **i32** |  | [optional] [default to None]
**max_retries** | **i32** |  | [optional] [default to None]
**alert_emails** | **String** |  | [optional] [default to None]
**created_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**updated_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**deleted_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**jobs** | [**Vec<models::ScrapingJob>**](ScrapingJob.md) |  | [optional] [default to None]
**geo_fencing_radius** | **f32** |  | [optional] [default to None]
**geo_fencing_lat** | **f64** |  | [optional] [default to None]
**geo_fencing_lon** | **f64** |  | [optional] [default to None]
**geo_fencing_zoom_min** | **i32** |  | [optional] [default to None]
**geo_fencing_zoom_max** | **i32** |  | [optional] [default to None]
**include_reviews** | **bool** |  | [optional] [default to None]
**include_photos** | **bool** |  | [optional] [default to None]
**include_business_hours** | **bool** |  | [optional] [default to None]
**max_reviews_per_business** | **i32** |  | [optional] [default to None]
**output_format** | [***models::OutputFormat**](OutputFormat.md) |  | [optional] [default to None]
**output_destination** | **String** |  | [optional] [default to None]
**data_retention** | **String** |  | [optional] [default to None]
**anonymize_pii** | **bool** |  | [optional] [default to None]
**notification_slack_channel** | **String** |  | [optional] [default to None]
**notification_email_group** | **String** |  | [optional] [default to None]
**notification_notify_on_start** | **bool** |  | [optional] [default to None]
**notification_notify_on_complete** | **bool** |  | [optional] [default to None]
**notification_notify_on_failure** | **bool** |  | [optional] [default to None]
**content_filter_allowed_countries** | **Vec<String>** |  | [optional] [default to None]
**content_filter_excluded_types** | **Vec<String>** |  | [optional] [default to None]
**content_filter_minimum_rating** | **f32** |  | [optional] [default to None]
**content_filter_minimum_reviews** | **i32** |  | [optional] [default to None]
**qos_max_concurrent_requests** | **i32** |  | [optional] [default to None]
**qos_max_retries** | **i32** |  | [optional] [default to None]
**qos_request_timeout** | **String** |  | [optional] [default to None]
**qos_enable_javascript** | **bool** |  | [optional] [default to None]
**respect_robots_txt** | **bool** |  | [optional] [default to None]
**accept_terms_of_service** | **bool** |  | [optional] [default to None]
**user_agent** | **String** |  | [optional] [default to None]
**search_terms** | **Vec<String>** |  | [optional] [default to None]
**scheduled_entry_id** | **String** |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


