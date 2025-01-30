# MFAInfo


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mfa_required** | **bool** |  | [optional] 
**allowed_methods** | **List[str]** |  | [optional] 
**failed_reason** | **str** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.mfa_info import MFAInfo

# TODO update the JSON string below
json = "{}"
# create an instance of MFAInfo from a JSON string
mfa_info_instance = MFAInfo.from_json(json)
# print the JSON string representation of the object
print(MFAInfo.to_json())

# convert the object into a dict
mfa_info_dict = mfa_info_instance.to_dict()
# create an instance of MFAInfo from a dict
mfa_info_from_dict = MFAInfo.from_dict(mfa_info_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


