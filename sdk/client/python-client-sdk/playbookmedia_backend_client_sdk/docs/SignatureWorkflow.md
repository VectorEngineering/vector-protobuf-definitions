# SignatureWorkflow


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**request_id** | **str** |  | [optional] 
**signing_order** | **List[str]** |  | [optional] 
**current_signer** | **str** |  | [optional] 
**workflow_status** | **str** |  | [optional] 
**parallel_signing** | **bool** |  | [optional] 
**require_all_signatures** | **bool** |  | [optional] 
**deadline** | **datetime** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.signature_workflow import SignatureWorkflow

# TODO update the JSON string below
json = "{}"
# create an instance of SignatureWorkflow from a JSON string
signature_workflow_instance = SignatureWorkflow.from_json(json)
# print the JSON string representation of the object
print(SignatureWorkflow.to_json())

# convert the object into a dict
signature_workflow_dict = signature_workflow_instance.to_dict()
# create an instance of SignatureWorkflow from a dict
signature_workflow_from_dict = SignatureWorkflow.from_dict(signature_workflow_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


