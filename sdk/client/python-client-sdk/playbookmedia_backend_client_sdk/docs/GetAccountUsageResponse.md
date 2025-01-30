# GetAccountUsageResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_jobs_run** | **int** |  | [optional] 
**monthly_job_limit** | **int** |  | [optional] 
**remaining_jobs** | **int** |  | [optional] 
**reset_time** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.get_account_usage_response import GetAccountUsageResponse

# TODO update the JSON string below
json = "{}"
# create an instance of GetAccountUsageResponse from a JSON string
get_account_usage_response_instance = GetAccountUsageResponse.from_json(json)
# print the JSON string representation of the object
print(GetAccountUsageResponse.to_json())

# convert the object into a dict
get_account_usage_response_dict = get_account_usage_response_instance.to_dict()
# create an instance of GetAccountUsageResponse from a dict
get_account_usage_response_from_dict = GetAccountUsageResponse.from_dict(get_account_usage_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


