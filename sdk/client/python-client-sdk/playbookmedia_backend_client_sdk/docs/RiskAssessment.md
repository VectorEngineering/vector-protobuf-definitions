# RiskAssessment


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**intelligence_id** | **str** |  | [optional] 
**risk_type** | **str** |  | [optional] 
**risk_score** | **float** |  | [optional] 
**assessment_details** | **str** |  | [optional] 
**mitigation_suggestions** | **List[str]** |  | [optional] 
**assessor_id** | **str** |  | [optional] 
**created_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.risk_assessment import RiskAssessment

# TODO update the JSON string below
json = "{}"
# create an instance of RiskAssessment from a JSON string
risk_assessment_instance = RiskAssessment.from_json(json)
# print the JSON string representation of the object
print(RiskAssessment.to_json())

# convert the object into a dict
risk_assessment_dict = risk_assessment_instance.to_dict()
# create an instance of RiskAssessment from a dict
risk_assessment_from_dict = RiskAssessment.from_dict(risk_assessment_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


