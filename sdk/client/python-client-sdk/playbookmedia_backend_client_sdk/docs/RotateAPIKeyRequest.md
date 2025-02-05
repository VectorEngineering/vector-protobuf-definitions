# RotateAPIKeyRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | **str** |  | [optional] 
**tenant_id** | **str** |  | [optional] 
**account_id** | **str** |  | [optional] 
**workspace_id** | **str** |  | [optional] 
**key_id** | **str** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.rotate_api_key_request import RotateAPIKeyRequest

# TODO update the JSON string below
json = "{}"
# create an instance of RotateAPIKeyRequest from a JSON string
rotate_api_key_request_instance = RotateAPIKeyRequest.from_json(json)
# print the JSON string representation of the object
print(RotateAPIKeyRequest.to_json())

# convert the object into a dict
rotate_api_key_request_dict = rotate_api_key_request_instance.to_dict()
# create an instance of RotateAPIKeyRequest from a dict
rotate_api_key_request_from_dict = RotateAPIKeyRequest.from_dict(rotate_api_key_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


