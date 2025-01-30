# LimitInfo


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current** | **int** |  | [optional] 
**limit** | **int** |  | [optional] 
**reset_time** | **datetime** |  | [optional] 
**window_size** | **str** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.limit_info import LimitInfo

# TODO update the JSON string below
json = "{}"
# create an instance of LimitInfo from a JSON string
limit_info_instance = LimitInfo.from_json(json)
# print the JSON string representation of the object
print(LimitInfo.to_json())

# convert the object into a dict
limit_info_dict = limit_info_instance.to_dict()
# create an instance of LimitInfo from a dict
limit_info_from_dict = LimitInfo.from_dict(limit_info_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


