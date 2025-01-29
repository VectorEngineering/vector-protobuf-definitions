# DeleteScrapingJobResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**success** | **bool** |  | [optional] 

## Example

```python
from backend_client.models.delete_scraping_job_response import DeleteScrapingJobResponse

# TODO update the JSON string below
json = "{}"
# create an instance of DeleteScrapingJobResponse from a JSON string
delete_scraping_job_response_instance = DeleteScrapingJobResponse.from_json(json)
# print the JSON string representation of the object
print(DeleteScrapingJobResponse.to_json())

# convert the object into a dict
delete_scraping_job_response_dict = delete_scraping_job_response_instance.to_dict()
# create an instance of DeleteScrapingJobResponse from a dict
delete_scraping_job_response_from_dict = DeleteScrapingJobResponse.from_dict(delete_scraping_job_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


