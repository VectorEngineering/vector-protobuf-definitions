# ScrapingJob

ScrapingJob represents a Google Maps scraping task. This message defines both the configuration and current state of a scraping operation.  Key components: - Basic metadata (id, name, timestamps) - Job status tracking - Search configuration parameters - Geographic settings - Performance options - Multi-tenant context  Database considerations: - Stored in \"scraping_jobs\" table - Uses GORM for ORM mapping - Includes foreign key to Account - Supports soft deletes  Usage example: ```go job := &ScrapingJob{     Name: \"Athens Cafes\",     Status: BackgroundJobStatus_BACKGROUND_JOB_STATUS_QUEUED,     Keywords: []string{\"cafe\", \"coffee\"},     Lang: \"el\",     Zoom: 15,     FastMode: true,     MaxTime: 3600, } ```

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**name** | **str** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**status** | [**BackgroundJobStatus**](BackgroundJobStatus.md) |  | [optional] [default to BackgroundJobStatus.UNSPECIFIED]
**keywords** | **List[str]** |  | [optional] 
**lang** | **str** |  | [optional] 
**zoom** | **int** |  | [optional] 
**lat** | **str** |  | [optional] 
**lon** | **str** |  | [optional] 
**fast_mode** | **bool** |  | [optional] 
**radius** | **int** |  | [optional] 
**depth** | **int** |  | [optional] 
**email** | **bool** |  | [optional] 
**max_time** | **int** |  | [optional] 
**proxies** | **List[str]** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 
**deleted_at** | **datetime** |  | [optional] 
**payload_type** | **str** |  | [optional] 
**priority** | **int** |  | [optional] 
**payload** | **bytearray** |  | [optional] 

## Example

```python
from backend_client.models.scraping_job import ScrapingJob

# TODO update the JSON string below
json = "{}"
# create an instance of ScrapingJob from a JSON string
scraping_job_instance = ScrapingJob.from_json(json)
# print the JSON string representation of the object
print(ScrapingJob.to_json())

# convert the object into a dict
scraping_job_dict = scraping_job_instance.to_dict()
# create an instance of ScrapingJob from a dict
scraping_job_from_dict = ScrapingJob.from_dict(scraping_job_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


