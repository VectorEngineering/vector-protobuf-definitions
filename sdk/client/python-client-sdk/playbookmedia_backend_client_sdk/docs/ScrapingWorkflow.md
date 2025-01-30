# ScrapingWorkflow


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**cron_expression** | **str** |  | [optional] 
**next_run_time** | **datetime** |  | [optional] 
**last_run_time** | **datetime** |  | [optional] 
**status** | [**BackgroundJobStatus**](BackgroundJobStatus.md) |  | [optional] [default to BackgroundJobStatus.UNSPECIFIED]
**retry_count** | **int** |  | [optional] 
**max_retries** | **int** |  | [optional] 
**alert_emails** | **str** |  | [optional] 
**org_id** | **str** |  | [optional] 
**tenant_id** | **str** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 
**deleted_at** | **datetime** |  | [optional] 
**jobs** | [**List[ScrapingJob]**](ScrapingJob.md) |  | [optional] 
**workspace** | [**Workspace**](Workspace.md) |  | [optional] 
**geo_fencing_radius** | **float** |  | [optional] 
**geo_fencing_lat** | **float** |  | [optional] 
**geo_fencing_lon** | **float** |  | [optional] 
**geo_fencing_zoom_min** | **int** |  | [optional] 
**geo_fencing_zoom_max** | **int** |  | [optional] 
**include_reviews** | **bool** |  | [optional] 
**include_photos** | **bool** |  | [optional] 
**include_business_hours** | **bool** |  | [optional] 
**max_reviews_per_business** | **int** |  | [optional] 
**output_format** | [**OutputFormat**](OutputFormat.md) |  | [optional] [default to OutputFormat.UNSPECIFIED]
**output_destination** | **str** |  | [optional] 
**data_retention** | **str** |  | [optional] 
**anonymize_pii** | **bool** |  | [optional] 
**notification_webhook_url** | **str** |  | [optional] 
**notification_slack_channel** | **str** |  | [optional] 
**notification_email_group** | **str** |  | [optional] 
**notification_notify_on_start** | **bool** |  | [optional] 
**notification_notify_on_complete** | **bool** |  | [optional] 
**notification_notify_on_failure** | **bool** |  | [optional] 
**content_filter_allowed_countries** | **List[str]** |  | [optional] 
**content_filter_excluded_types** | **List[str]** |  | [optional] 
**content_filter_minimum_rating** | **float** |  | [optional] 
**content_filter_minimum_reviews** | **int** |  | [optional] 
**qos_max_concurrent_requests** | **int** |  | [optional] 
**qos_max_retries** | **int** |  | [optional] 
**qos_request_timeout** | **str** |  | [optional] 
**qos_enable_javascript** | **bool** |  | [optional] 
**respect_robots_txt** | **bool** |  | [optional] 
**accept_terms_of_service** | **bool** |  | [optional] 
**user_agent** | **str** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.scraping_workflow import ScrapingWorkflow

# TODO update the JSON string below
json = "{}"
# create an instance of ScrapingWorkflow from a JSON string
scraping_workflow_instance = ScrapingWorkflow.from_json(json)
# print the JSON string representation of the object
print(ScrapingWorkflow.to_json())

# convert the object into a dict
scraping_workflow_dict = scraping_workflow_instance.to_dict()
# create an instance of ScrapingWorkflow from a dict
scraping_workflow_from_dict = ScrapingWorkflow.from_dict(scraping_workflow_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


