# UpdateAccountRequestPayload


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account** | [**Account**](Account.md) |  | [optional] 
**organization_id** | **str** |  | [optional] 
**tenant_id** | **str** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.update_account_request_payload import UpdateAccountRequestPayload

# TODO update the JSON string below
json = "{}"
# create an instance of UpdateAccountRequestPayload from a JSON string
update_account_request_payload_instance = UpdateAccountRequestPayload.from_json(json)
# print the JSON string representation of the object
print(UpdateAccountRequestPayload.to_json())

# convert the object into a dict
update_account_request_payload_dict = update_account_request_payload_instance.to_dict()
# create an instance of UpdateAccountRequestPayload from a dict
update_account_request_payload_from_dict = UpdateAccountRequestPayload.from_dict(update_account_request_payload_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


