# APIInfo


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | **str** |  | [optional] 
**supported_versions** | **List[str]** |  | [optional] 
**is_deprecated** | **bool** |  | [optional] 
**sunset_date** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.api_info import APIInfo

# TODO update the JSON string below
json = "{}"
# create an instance of APIInfo from a JSON string
api_info_instance = APIInfo.from_json(json)
# print the JSON string representation of the object
print(APIInfo.to_json())

# convert the object into a dict
api_info_dict = api_info_instance.to_dict()
# create an instance of APIInfo from a dict
api_info_from_dict = APIInfo.from_dict(api_info_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


