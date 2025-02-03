# ContractIntelligence


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**contract_type** | **str** |  | [optional] 
**risk_scores** | **Dict[str, float]** |  | [optional] 
**detected_clauses** | **Dict[str, str]** |  | [optional] 
**key_obligations** | **List[str]** |  | [optional] 
**compliance_scores** | **Dict[str, float]** |  | [optional] 
**jurisdiction** | **str** |  | [optional] 
**governing_laws** | **List[str]** |  | [optional] 
**semantic_analysis** | **Dict[str, str]** |  | [optional] 
**analysis_date** | **datetime** |  | [optional] 
**risk_assessments** | [**List[RiskAssessment]**](RiskAssessment.md) |  | [optional] 
**compliance_checks** | [**List[ComplianceCheck]**](ComplianceCheck.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.contract_intelligence import ContractIntelligence

# TODO update the JSON string below
json = "{}"
# create an instance of ContractIntelligence from a JSON string
contract_intelligence_instance = ContractIntelligence.from_json(json)
# print the JSON string representation of the object
print(ContractIntelligence.to_json())

# convert the object into a dict
contract_intelligence_dict = contract_intelligence_instance.to_dict()
# create an instance of ContractIntelligence from a dict
contract_intelligence_from_dict = ContractIntelligence.from_dict(contract_intelligence_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


