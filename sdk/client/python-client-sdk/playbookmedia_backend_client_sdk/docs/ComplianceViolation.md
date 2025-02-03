# ComplianceViolation


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rule_id** | **str** |  | [optional] 
**severity** | **str** |  | [optional] 
**description** | **str** |  | [optional] 
**affected_resources** | **List[str]** |  | [optional] 
**remediation_steps** | **str** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.compliance_violation import ComplianceViolation

# TODO update the JSON string below
json = "{}"
# create an instance of ComplianceViolation from a JSON string
compliance_violation_instance = ComplianceViolation.from_json(json)
# print the JSON string representation of the object
print(ComplianceViolation.to_json())

# convert the object into a dict
compliance_violation_dict = compliance_violation_instance.to_dict()
# create an instance of ComplianceViolation from a dict
compliance_violation_from_dict = ComplianceViolation.from_dict(compliance_violation_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


