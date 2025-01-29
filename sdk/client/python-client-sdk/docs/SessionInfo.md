# SessionInfo


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**session_id** | **str** |  | [optional] 
**last_active** | **datetime** |  | [optional] 
**device_id** | **str** |  | [optional] 
**ip_address** | **str** |  | [optional] 

## Example

```python
from backend_client.models.session_info import SessionInfo

# TODO update the JSON string below
json = "{}"
# create an instance of SessionInfo from a JSON string
session_info_instance = SessionInfo.from_json(json)
# print the JSON string representation of the object
print(SessionInfo.to_json())

# convert the object into a dict
session_info_dict = session_info_instance.to_dict()
# create an instance of SessionInfo from a dict
session_info_from_dict = SessionInfo.from_dict(session_info_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


