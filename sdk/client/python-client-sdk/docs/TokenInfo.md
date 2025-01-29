# TokenInfo


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**token_id** | **str** |  | [optional] 
**token_type** | **str** |  | [optional] 
**expiry** | **datetime** |  | [optional] 
**issuer** | **str** |  | [optional] 
**audiences** | **List[str]** |  | [optional] 

## Example

```python
from backend_client.models.token_info import TokenInfo

# TODO update the JSON string below
json = "{}"
# create an instance of TokenInfo from a JSON string
token_info_instance = TokenInfo.from_json(json)
# print the JSON string representation of the object
print(TokenInfo.to_json())

# convert the object into a dict
token_info_dict = token_info_instance.to_dict()
# create an instance of TokenInfo from a dict
token_info_from_dict = TokenInfo.from_dict(token_info_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


