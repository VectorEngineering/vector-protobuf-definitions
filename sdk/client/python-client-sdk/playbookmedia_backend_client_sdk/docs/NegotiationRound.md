# NegotiationRound


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**history_id** | **str** |  | [optional] 
**round_number** | **int** |  | [optional] 
**proposer_id** | **str** |  | [optional] 
**proposal_content** | **str** |  | [optional] 
**response_content** | **str** |  | [optional] 
**status** | **str** |  | [optional] 
**created_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.negotiation_round import NegotiationRound

# TODO update the JSON string below
json = "{}"
# create an instance of NegotiationRound from a JSON string
negotiation_round_instance = NegotiationRound.from_json(json)
# print the JSON string representation of the object
print(NegotiationRound.to_json())

# convert the object into a dict
negotiation_round_dict = negotiation_round_instance.to_dict()
# create an instance of NegotiationRound from a dict
negotiation_round_from_dict = NegotiationRound.from_dict(negotiation_round_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


