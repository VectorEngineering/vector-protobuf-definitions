# CreateWebhookResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**webhook** | [**WebhookConfig**](WebhookConfig.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.create_webhook_response import CreateWebhookResponse

# TODO update the JSON string below
json = "{}"
# create an instance of CreateWebhookResponse from a JSON string
create_webhook_response_instance = CreateWebhookResponse.from_json(json)
# print the JSON string representation of the object
print(CreateWebhookResponse.to_json())

# convert the object into a dict
create_webhook_response_dict = create_webhook_response_instance.to_dict()
# create an instance of CreateWebhookResponse from a dict
create_webhook_response_from_dict = CreateWebhookResponse.from_dict(create_webhook_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


