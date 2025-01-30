# JobSuccessRate


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**workflow_id** | **str** |  | [optional] 
**success_rate** | **float** |  | [optional] 
**total_runs** | **int** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.job_success_rate import JobSuccessRate

# TODO update the JSON string below
json = "{}"
# create an instance of JobSuccessRate from a JSON string
job_success_rate_instance = JobSuccessRate.from_json(json)
# print the JSON string representation of the object
print(JobSuccessRate.to_json())

# convert the object into a dict
job_success_rate_dict = job_success_rate_instance.to_dict()
# create an instance of JobSuccessRate from a dict
job_success_rate_from_dict = JobSuccessRate.from_dict(job_success_rate_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


