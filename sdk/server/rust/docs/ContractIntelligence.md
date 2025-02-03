# ContractIntelligence

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [optional] [default to None]
**contract_type** | **String** |  | [optional] [default to None]
**risk_scores** | **std::collections::HashMap<String, f32>** |  | [optional] [default to None]
**detected_clauses** | **std::collections::HashMap<String, String>** |  | [optional] [default to None]
**key_obligations** | **Vec<String>** |  | [optional] [default to None]
**compliance_scores** | **std::collections::HashMap<String, f32>** |  | [optional] [default to None]
**jurisdiction** | **String** |  | [optional] [default to None]
**governing_laws** | **Vec<String>** |  | [optional] [default to None]
**semantic_analysis** | **std::collections::HashMap<String, String>** |  | [optional] [default to None]
**analysis_date** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**risk_assessments** | [**Vec<models::RiskAssessment>**](RiskAssessment.md) |  | [optional] [default to None]
**compliance_checks** | [**Vec<models::ComplianceCheck>**](ComplianceCheck.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


