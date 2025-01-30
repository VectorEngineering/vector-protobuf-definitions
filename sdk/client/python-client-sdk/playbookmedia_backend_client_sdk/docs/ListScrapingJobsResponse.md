# ListScrapingJobsResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**jobs** | [**List[ScrapingJob]**](ScrapingJob.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.list_scraping_jobs_response import ListScrapingJobsResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ListScrapingJobsResponse from a JSON string
list_scraping_jobs_response_instance = ListScrapingJobsResponse.from_json(json)
# print the JSON string representation of the object
print(ListScrapingJobsResponse.to_json())

# convert the object into a dict
list_scraping_jobs_response_dict = list_scraping_jobs_response_instance.to_dict()
# create an instance of ListScrapingJobsResponse from a dict
list_scraping_jobs_response_from_dict = ListScrapingJobsResponse.from_dict(list_scraping_jobs_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


