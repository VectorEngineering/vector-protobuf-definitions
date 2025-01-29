# CreateScrapingJobResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**job_id** | **str** |  | [optional] 
**status** | [**BackgroundJobStatus**](BackgroundJobStatus.md) |  | [optional] [default to BackgroundJobStatus.UNSPECIFIED]

## Example

```python
from backend_client.models.create_scraping_job_response import CreateScrapingJobResponse

# TODO update the JSON string below
json = "{}"
# create an instance of CreateScrapingJobResponse from a JSON string
create_scraping_job_response_instance = CreateScrapingJobResponse.from_json(json)
# print the JSON string representation of the object
print(CreateScrapingJobResponse.to_json())

# convert the object into a dict
create_scraping_job_response_dict = create_scraping_job_response_instance.to_dict()
# create an instance of CreateScrapingJobResponse from a dict
create_scraping_job_response_from_dict = CreateScrapingJobResponse.from_dict(create_scraping_job_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


