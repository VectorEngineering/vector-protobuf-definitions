# AuthContext


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**required_scopes** | **List[str]** |  | [optional] 
**provided_scopes** | **List[str]** |  | [optional] 
**missing_permissions** | **List[str]** |  | [optional] 
**tenant_id** | **str** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.auth_context import AuthContext

# TODO update the JSON string below
json = "{}"
# create an instance of AuthContext from a JSON string
auth_context_instance = AuthContext.from_json(json)
# print the JSON string representation of the object
print(AuthContext.to_json())

# convert the object into a dict
auth_context_dict = auth_context_instance.to_dict()
# create an instance of AuthContext from a dict
auth_context_from_dict = AuthContext.from_dict(auth_context_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


