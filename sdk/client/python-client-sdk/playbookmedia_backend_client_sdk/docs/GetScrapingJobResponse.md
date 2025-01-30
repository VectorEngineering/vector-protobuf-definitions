# GetScrapingJobResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**job** | [**ScrapingJob**](ScrapingJob.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.get_scraping_job_response import GetScrapingJobResponse

# TODO update the JSON string below
json = "{}"
# create an instance of GetScrapingJobResponse from a JSON string
get_scraping_job_response_instance = GetScrapingJobResponse.from_json(json)
# print the JSON string representation of the object
print(GetScrapingJobResponse.to_json())

# convert the object into a dict
get_scraping_job_response_dict = get_scraping_job_response_instance.to_dict()
# create an instance of GetScrapingJobResponse from a dict
get_scraping_job_response_from_dict = GetScrapingJobResponse.from_dict(get_scraping_job_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


