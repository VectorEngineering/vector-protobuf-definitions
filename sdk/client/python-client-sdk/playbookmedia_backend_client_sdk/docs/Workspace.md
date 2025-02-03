# Workspace


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**name** | **str** |  | [optional] 
**industry** | **str** |  | [optional] 
**domain** | **str** |  | [optional] 
**gdpr_compliant** | **bool** |  | [optional] 
**hipaa_compliant** | **bool** |  | [optional] 
**soc2_compliant** | **bool** |  | [optional] 
**storage_quota** | **str** | in bytes | [optional] 
**used_storage** | **str** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 
**deleted_at** | **datetime** |  | [optional] 
**workflows** | [**List[ScrapingWorkflow]**](ScrapingWorkflow.md) |  | [optional] 
**jobs_run_this_month** | **int** |  | [optional] 
**workspace_job_limit** | **int** |  | [optional] 
**daily_job_quota** | **int** |  | [optional] 
**active_scrapers** | **int** |  | [optional] 
**total_leads_collected** | **int** |  | [optional] 
**last_job_run** | **datetime** |  | [optional] 
**scraping_jobs** | [**List[ScrapingJob]**](ScrapingJob.md) |  | [optional] 
**api_keys** | [**List[APIKey]**](APIKey.md) |  | [optional] 
**webhooks** | [**List[WebhookConfig]**](WebhookConfig.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.workspace import Workspace

# TODO update the JSON string below
json = "{}"
# create an instance of Workspace from a JSON string
workspace_instance = Workspace.from_json(json)
# print the JSON string representation of the object
print(Workspace.to_json())

# convert the object into a dict
workspace_dict = workspace_instance.to_dict()
# create an instance of Workspace from a dict
workspace_from_dict = Workspace.from_dict(workspace_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


