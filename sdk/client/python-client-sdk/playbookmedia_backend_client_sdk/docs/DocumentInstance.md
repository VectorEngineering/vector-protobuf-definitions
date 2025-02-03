# DocumentInstance


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**template_id** | **str** |  | [optional] 
**status** | [**DocumentStatus**](DocumentStatus.md) |  | [optional] [default to DocumentStatus.UNSPECIFIED]
**field_values** | **Dict[str, str]** |  | [optional] 
**signers** | **List[str]** |  | [optional] 
**language** | **str** |  | [optional] 
**is_completed** | **bool** |  | [optional] 
**due_date** | **datetime** |  | [optional] 
**generated_content** | **str** |  | [optional] 
**attachments** | **List[str]** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 
**signature_requests** | [**List[SignatureRequest]**](SignatureRequest.md) |  | [optional] 
**ai_assistance** | [**List[AIAssistanceLog]**](AIAssistanceLog.md) |  | [optional] 
**negotiation** | [**NegotiationHistory**](NegotiationHistory.md) |  | [optional] 
**versions** | [**List[DocumentVersion]**](DocumentVersion.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.document_instance import DocumentInstance

# TODO update the JSON string below
json = "{}"
# create an instance of DocumentInstance from a JSON string
document_instance_instance = DocumentInstance.from_json(json)
# print the JSON string representation of the object
print(DocumentInstance.to_json())

# convert the object into a dict
document_instance_dict = document_instance_instance.to_dict()
# create an instance of DocumentInstance from a dict
document_instance_from_dict = DocumentInstance.from_dict(document_instance_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


