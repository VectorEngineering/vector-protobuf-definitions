# CreateWebhookRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | **str** |  | [optional] 
**workspace_id** | **str** |  | [optional] 
**tenant_id** | **str** |  | [optional] 
**account_id** | **str** |  | [optional] 
**webhook** | [**WebhookConfig**](WebhookConfig.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.create_webhook_request import CreateWebhookRequest

# TODO update the JSON string below
json = "{}"
# create an instance of CreateWebhookRequest from a JSON string
create_webhook_request_instance = CreateWebhookRequest.from_json(json)
# print the JSON string representation of the object
print(CreateWebhookRequest.to_json())

# convert the object into a dict
create_webhook_request_dict = create_webhook_request_instance.to_dict()
# create an instance of CreateWebhookRequest from a dict
create_webhook_request_from_dict = CreateWebhookRequest.from_dict(create_webhook_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


