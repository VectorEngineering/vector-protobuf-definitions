# Organization

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [optional] [default to None]
**name** | **String** |  | [optional] [default to None]
**display_name** | **String** |  | [optional] [default to None]
**website** | **String** |  | [optional] [default to None]
**description** | **String** |  | [optional] [default to None]
**billing_email** | **String** |  | [optional] [default to None]
**technical_email** | **String** |  | [optional] [default to None]
**phone** | **String** |  | [optional] [default to None]
**address** | **String** |  | [optional] [default to None]
**sso_enabled** | **bool** |  | [optional] [default to None]
**sso_provider** | **String** |  | [optional] [default to None]
**sso_domain** | **String** |  | [optional] [default to None]
**allowed_domains** | **Vec<String>** |  | [optional] [default to None]
**enforce2fa** | **bool** |  | [optional] [default to None]
**billing_plan** | [***models::BillingPlan**](BillingPlan.md) |  | [optional] [default to None]
**billing_currency** | **String** |  | [optional] [default to None]
**auto_billing** | **bool** |  | [optional] [default to None]
**tax_id** | **String** |  | [optional] [default to None]
**compliance_frameworks** | **Vec<String>** | SOC2, GDPR, etc | [optional] [default to None]
**data_processing_agreement** | **bool** |  | [optional] [default to None]
**data_region** | **String** |  | [optional] [default to None]
**max_tenants** | **i32** |  | [optional] [default to None]
**total_storage_limit** | **String** |  | [optional] [default to None]
**max_api_keys** | **i32** |  | [optional] [default to None]
**max_users** | **i32** |  | [optional] [default to None]
**tenants** | [**Vec<models::Tenant>**](Tenant.md) |  | [optional] [default to None]
**subscriptions** | [***models::Subscription**](Subscription.md) |  | [optional] [default to None]
**created_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**updated_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**deleted_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**created_by** | **String** |  | [optional] [default to None]
**status** | [***models::V1PeriodStatus**](v1.Status.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


