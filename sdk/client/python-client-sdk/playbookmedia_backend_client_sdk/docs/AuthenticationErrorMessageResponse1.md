# AuthenticationErrorMessageResponse1

Represents authentication and authorization failures

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | [**AuthErrorCode**](AuthErrorCode.md) |  | [optional] [default to AuthErrorCode.NO_AUTH_ERROR]
**message** | **str** |  | [optional] 
**token_info** | [**TokenInfo**](TokenInfo.md) |  | [optional] 
**auth_context** | [**AuthContext**](AuthContext.md) |  | [optional] 
**session_info** | [**SessionInfo**](SessionInfo.md) |  | [optional] 
**mfa_info** | [**MFAInfo**](MFAInfo.md) |  | [optional] 
**error_response** | [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.authentication_error_message_response1 import AuthenticationErrorMessageResponse1

# TODO update the JSON string below
json = "{}"
# create an instance of AuthenticationErrorMessageResponse1 from a JSON string
authentication_error_message_response1_instance = AuthenticationErrorMessageResponse1.from_json(json)
# print the JSON string representation of the object
print(AuthenticationErrorMessageResponse1.to_json())

# convert the object into a dict
authentication_error_message_response1_dict = authentication_error_message_response1_instance.to_dict()
# create an instance of AuthenticationErrorMessageResponse1 from a dict
authentication_error_message_response1_from_dict = AuthenticationErrorMessageResponse1.from_dict(authentication_error_message_response1_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


