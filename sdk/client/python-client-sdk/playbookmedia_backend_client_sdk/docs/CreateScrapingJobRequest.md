# CreateScrapingJobRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auth_platform_user_id** | **str** |  | 
**org_id** | **str** |  | 
**tenant_id** | **str** |  | 
**name** | **str** |  | [optional] 
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
**workspace_id** | **str** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.create_scraping_job_request import CreateScrapingJobRequest

# TODO update the JSON string below
json = "{}"
# create an instance of CreateScrapingJobRequest from a JSON string
create_scraping_job_request_instance = CreateScrapingJobRequest.from_json(json)
# print the JSON string representation of the object
print(CreateScrapingJobRequest.to_json())

# convert the object into a dict
create_scraping_job_request_dict = create_scraping_job_request_instance.to_dict()
# create an instance of CreateScrapingJobRequest from a dict
create_scraping_job_request_from_dict = CreateScrapingJobRequest.from_dict(create_scraping_job_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


