# BusinessHours


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**day** | [**DayOfWeek**](DayOfWeek.md) |  | [optional] [default to DayOfWeek.UNSPECIFIED]
**open_time** | **str** |  | [optional] 
**close_time** | **str** |  | [optional] 
**closed** | **bool** |  | [optional] 
**lead_id** | **str** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.business_hours import BusinessHours

# TODO update the JSON string below
json = "{}"
# create an instance of BusinessHours from a JSON string
business_hours_instance = BusinessHours.from_json(json)
# print the JSON string representation of the object
print(BusinessHours.to_json())

# convert the object into a dict
business_hours_dict = business_hours_instance.to_dict()
# create an instance of BusinessHours from a dict
business_hours_from_dict = BusinessHours.from_dict(business_hours_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


