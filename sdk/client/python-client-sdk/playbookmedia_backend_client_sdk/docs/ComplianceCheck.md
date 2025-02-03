# ComplianceCheck


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**intelligence_id** | **str** |  | [optional] 
**compliance_standard** | **str** |  | [optional] 
**check_result** | **str** |  | [optional] 
**violations** | **List[str]** |  | [optional] 
**remediation_steps** | **str** |  | [optional] 
**check_date** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.compliance_check import ComplianceCheck

# TODO update the JSON string below
json = "{}"
# create an instance of ComplianceCheck from a JSON string
compliance_check_instance = ComplianceCheck.from_json(json)
# print the JSON string representation of the object
print(ComplianceCheck.to_json())

# convert the object into a dict
compliance_check_dict = compliance_check_instance.to_dict()
# create an instance of ComplianceCheck from a dict
compliance_check_from_dict = ComplianceCheck.from_dict(compliance_check_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


