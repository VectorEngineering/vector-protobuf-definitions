# AvailabilityInfo


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**retry_after** | **int** |  | [optional] 
**maintenance_window** | **str** |  | [optional] 
**affected_services** | **List[str]** |  | [optional] 
**status_page_url** | **str** |  | [optional] 

## Example

```python
from backend_client.models.availability_info import AvailabilityInfo

# TODO update the JSON string below
json = "{}"
# create an instance of AvailabilityInfo from a JSON string
availability_info_instance = AvailabilityInfo.from_json(json)
# print the JSON string representation of the object
print(AvailabilityInfo.to_json())

# convert the object into a dict
availability_info_dict = availability_info_instance.to_dict()
# create an instance of AvailabilityInfo from a dict
availability_info_from_dict = AvailabilityInfo.from_dict(availability_info_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


