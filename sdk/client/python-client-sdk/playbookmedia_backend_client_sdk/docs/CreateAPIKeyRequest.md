# CreateAPIKeyRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | **str** |  | [optional] 
**workspace_id** | **str** |  | [optional] 
**tenant_id** | **str** |  | [optional] 
**account_id** | **str** |  | [optional] 
**name** | **str** |  | [optional] 
**description** | **str** |  | [optional] 
**scopes** | **List[str]** |  | [optional] 
**expires_at** | **datetime** |  | [optional] 
**max_uses** | **int** |  | [optional] 
**allowed_ips** | **List[str]** |  | [optional] 
**rate_limit** | **int** |  | [optional] 
**enforce_signing** | **bool** |  | [optional] 
**allowed_signature_algorithms** | **List[str]** |  | [optional] 
**enforce_mutual_tls** | **bool** |  | [optional] 
**alert_emails** | **List[str]** |  | [optional] 
**alert_on_quota_threshold** | **bool** |  | [optional] 
**quota_alert_threshold** | **float** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.create_api_key_request import CreateAPIKeyRequest

# TODO update the JSON string below
json = "{}"
# create an instance of CreateAPIKeyRequest from a JSON string
create_api_key_request_instance = CreateAPIKeyRequest.from_json(json)
# print the JSON string representation of the object
print(CreateAPIKeyRequest.to_json())

# convert the object into a dict
create_api_key_request_dict = create_api_key_request_instance.to_dict()
# create an instance of CreateAPIKeyRequest from a dict
create_api_key_request_from_dict = CreateAPIKeyRequest.from_dict(create_api_key_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


