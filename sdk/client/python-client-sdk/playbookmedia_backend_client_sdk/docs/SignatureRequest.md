# SignatureRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**status** | [**SignatureStatus**](SignatureStatus.md) |  | [optional] [default to SignatureStatus.UNSPECIFIED]
**signer_email** | **str** |  | [optional] 
**signer_name** | **str** |  | [optional] 
**role** | **str** |  | [optional] 
**authentication_method** | **str** |  | [optional] 
**expires_at** | **datetime** |  | [optional] 
**requires_mfa** | **bool** |  | [optional] 
**signature_type** | **str** |  | [optional] 
**reminder_schedule** | **List[str]** |  | [optional] 
**document_id** | **str** |  | [optional] 
**signature_blocks** | [**List[SignatureBlock]**](SignatureBlock.md) |  | [optional] 
**workflow** | [**SignatureWorkflow**](SignatureWorkflow.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.signature_request import SignatureRequest

# TODO update the JSON string below
json = "{}"
# create an instance of SignatureRequest from a JSON string
signature_request_instance = SignatureRequest.from_json(json)
# print the JSON string representation of the object
print(SignatureRequest.to_json())

# convert the object into a dict
signature_request_dict = signature_request_instance.to_dict()
# create an instance of SignatureRequest from a dict
signature_request_from_dict = SignatureRequest.from_dict(signature_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


