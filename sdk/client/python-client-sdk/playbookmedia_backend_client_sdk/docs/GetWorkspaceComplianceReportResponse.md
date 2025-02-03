# GetWorkspaceComplianceReportResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**overall_score** | **float** |  | [optional] 
**compliance_status** | **str** |  | [optional] 
**violations** | [**List[ComplianceViolation]**](ComplianceViolation.md) |  | [optional] 
**category_scores** | [**List[ComplianceScore]**](ComplianceScore.md) |  | [optional] 
**report_generated_at** | **datetime** |  | [optional] 
**certification_id** | **str** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.get_workspace_compliance_report_response import GetWorkspaceComplianceReportResponse

# TODO update the JSON string below
json = "{}"
# create an instance of GetWorkspaceComplianceReportResponse from a JSON string
get_workspace_compliance_report_response_instance = GetWorkspaceComplianceReportResponse.from_json(json)
# print the JSON string representation of the object
print(GetWorkspaceComplianceReportResponse.to_json())

# convert the object into a dict
get_workspace_compliance_report_response_dict = get_workspace_compliance_report_response_instance.to_dict()
# create an instance of GetWorkspaceComplianceReportResponse from a dict
get_workspace_compliance_report_response_from_dict = GetWorkspaceComplianceReportResponse.from_dict(get_workspace_compliance_report_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


