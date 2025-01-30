# DownloadScrapingResultsResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content** | **bytearray** |  | [optional] 
**filename** | **str** |  | [optional] 
**content_type** | **str** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.download_scraping_results_response import DownloadScrapingResultsResponse

# TODO update the JSON string below
json = "{}"
# create an instance of DownloadScrapingResultsResponse from a JSON string
download_scraping_results_response_instance = DownloadScrapingResultsResponse.from_json(json)
# print the JSON string representation of the object
print(DownloadScrapingResultsResponse.to_json())

# convert the object into a dict
download_scraping_results_response_dict = download_scraping_results_response_instance.to_dict()
# create an instance of DownloadScrapingResultsResponse from a dict
download_scraping_results_response_from_dict = DownloadScrapingResultsResponse.from_dict(download_scraping_results_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


