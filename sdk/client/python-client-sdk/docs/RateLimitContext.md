# RateLimitContext


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | **str** |  | [optional] 
**ip_address** | **str** |  | [optional] 
**affected_endpoints** | **List[str]** |  | [optional] 
**headers** | **Dict[str, str]** |  | [optional] 

## Example

```python
from backend_client.models.rate_limit_context import RateLimitContext

# TODO update the JSON string below
json = "{}"
# create an instance of RateLimitContext from a JSON string
rate_limit_context_instance = RateLimitContext.from_json(json)
# print the JSON string representation of the object
print(RateLimitContext.to_json())

# convert the object into a dict
rate_limit_context_dict = rate_limit_context_instance.to_dict()
# create an instance of RateLimitContext from a dict
rate_limit_context_from_dict = RateLimitContext.from_dict(rate_limit_context_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


