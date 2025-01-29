# ResourceUtilization


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cpu_usage** | **float** |  | [optional] 
**memory_usage** | **float** |  | [optional] 
**active_connections** | **int** |  | [optional] 
**quotas** | **Dict[str, float]** |  | [optional] 

## Example

```python
from backend_client.models.resource_utilization import ResourceUtilization

# TODO update the JSON string below
json = "{}"
# create an instance of ResourceUtilization from a JSON string
resource_utilization_instance = ResourceUtilization.from_json(json)
# print the JSON string representation of the object
print(ResourceUtilization.to_json())

# convert the object into a dict
resource_utilization_dict = resource_utilization_instance.to_dict()
# create an instance of ResourceUtilization from a dict
resource_utilization_from_dict = ResourceUtilization.from_dict(resource_utilization_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


