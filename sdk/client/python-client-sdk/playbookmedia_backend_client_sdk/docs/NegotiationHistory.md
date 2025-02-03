# NegotiationHistory


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**document_id** | **str** |  | [optional] 
**rounds** | [**List[NegotiationRound]**](NegotiationRound.md) |  | [optional] 
**current_status** | **str** |  | [optional] 
**started_at** | **datetime** |  | [optional] 
**last_updated** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.negotiation_history import NegotiationHistory

# TODO update the JSON string below
json = "{}"
# create an instance of NegotiationHistory from a JSON string
negotiation_history_instance = NegotiationHistory.from_json(json)
# print the JSON string representation of the object
print(NegotiationHistory.to_json())

# convert the object into a dict
negotiation_history_dict = negotiation_history_instance.to_dict()
# create an instance of NegotiationHistory from a dict
negotiation_history_from_dict = NegotiationHistory.from_dict(negotiation_history_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


