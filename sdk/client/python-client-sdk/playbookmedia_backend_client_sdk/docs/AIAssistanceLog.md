# AIAssistanceLog


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**document_id** | **str** |  | [optional] 
**interaction_type** | **str** |  | [optional] 
**user_query** | **str** |  | [optional] 
**ai_response** | **str** |  | [optional] 
**context** | **Dict[str, str]** |  | [optional] 
**created_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.ai_assistance_log import AIAssistanceLog

# TODO update the JSON string below
json = "{}"
# create an instance of AIAssistanceLog from a JSON string
ai_assistance_log_instance = AIAssistanceLog.from_json(json)
# print the JSON string representation of the object
print(AIAssistanceLog.to_json())

# convert the object into a dict
ai_assistance_log_dict = ai_assistance_log_instance.to_dict()
# create an instance of AIAssistanceLog from a dict
ai_assistance_log_from_dict = AIAssistanceLog.from_dict(ai_assistance_log_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


