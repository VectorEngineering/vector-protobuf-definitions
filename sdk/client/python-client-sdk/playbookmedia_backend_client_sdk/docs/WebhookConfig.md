# WebhookConfig


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**url** | **str** |  | [optional] 
**auth_type** | **str** | e.g., \&quot;basic\&quot;, \&quot;bearer\&quot;, \&quot;custom\&quot; | [optional] 
**auth_token** | **str** |  | [optional] 
**custom_headers** | **Dict[str, str]** |  | [optional] 
**max_retries** | **int** |  | [optional] 
**retry_interval** | **str** |  | [optional] 
**trigger_events** | [**List[TriggerEvent]**](TriggerEvent.md) |  | [optional] 
**included_fields** | [**List[IncludedField]**](IncludedField.md) |  | [optional] 
**include_full_results** | **bool** |  | [optional] 
**payload_format** | [**PayloadFormat**](PayloadFormat.md) |  | [optional] [default to PayloadFormat.UNSPECIFIED]
**verify_ssl** | **bool** |  | [optional] 
**signing_secret** | **str** |  | [optional] 
**rate_limit** | **int** |  | [optional] 
**rate_limit_interval** | **str** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 
**last_triggered_at** | **datetime** |  | [optional] 
**successful_calls** | **int** |  | [optional] 
**failed_calls** | **int** |  | [optional] 
**metadata** | **object** |  | [optional] 
**webhook_name** | **str** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.webhook_config import WebhookConfig

# TODO update the JSON string below
json = "{}"
# create an instance of WebhookConfig from a JSON string
webhook_config_instance = WebhookConfig.from_json(json)
# print the JSON string representation of the object
print(WebhookConfig.to_json())

# convert the object into a dict
webhook_config_dict = webhook_config_instance.to_dict()
# create an instance of WebhookConfig from a dict
webhook_config_from_dict = WebhookConfig.from_dict(webhook_config_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


