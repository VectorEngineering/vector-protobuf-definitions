# GetWorkspaceComplianceReportResponse

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**overall_score** | **f32** |  | [optional] [default to None]
**compliance_status** | **String** |  | [optional] [default to None]
**violations** | [**Vec<models::ComplianceViolation>**](ComplianceViolation.md) |  | [optional] [default to None]
**category_scores** | [**Vec<models::ComplianceScore>**](ComplianceScore.md) |  | [optional] [default to None]
**report_generated_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**certification_id** | **String** |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


