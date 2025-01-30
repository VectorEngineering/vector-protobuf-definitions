# OperationDetails


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**operation_id** | **str** |  | [optional] 
**start_time** | **datetime** |  | [optional] 
**end_time** | **datetime** |  | [optional] 
**stack_trace** | **List[str]** |  | [optional] 
**parameters** | **Dict[str, str]** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.operation_details import OperationDetails

# TODO update the JSON string below
json = "{}"
# create an instance of OperationDetails from a JSON string
operation_details_instance = OperationDetails.from_json(json)
# print the JSON string representation of the object
print(OperationDetails.to_json())

# convert the object into a dict
operation_details_dict = operation_details_instance.to_dict()
# create an instance of OperationDetails from a dict
operation_details_from_dict = OperationDetails.from_dict(operation_details_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


