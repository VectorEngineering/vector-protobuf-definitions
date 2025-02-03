# ComplianceMetrics


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**gdpr_compliance_score** | **float** |  | [optional] 
**hipaa_compliance_score** | **float** |  | [optional] 
**pending_approvals** | **int** |  | [optional] 
**compliance_violations** | **int** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.compliance_metrics import ComplianceMetrics

# TODO update the JSON string below
json = "{}"
# create an instance of ComplianceMetrics from a JSON string
compliance_metrics_instance = ComplianceMetrics.from_json(json)
# print the JSON string representation of the object
print(ComplianceMetrics.to_json())

# convert the object into a dict
compliance_metrics_dict = compliance_metrics_instance.to_dict()
# create an instance of ComplianceMetrics from a dict
compliance_metrics_from_dict = ComplianceMetrics.from_dict(compliance_metrics_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


