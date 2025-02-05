# Organization

Key features: - Multi-tenant support - Hierarchical structure - Enterprise features - Billing management  Use cases: - Company creates multiple apps/tenants - Enterprise manages business units - Usage tracking across tenants

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**name** | **str** |  | [optional] 
**display_name** | **str** |  | [optional] 
**website** | **str** |  | [optional] 
**description** | **str** |  | [optional] 
**billing_email** | **str** |  | [optional] 
**technical_email** | **str** |  | [optional] 
**phone** | **str** |  | [optional] 
**address** | **str** |  | [optional] 
**sso_enabled** | **bool** |  | [optional] 
**sso_provider** | **str** |  | [optional] 
**sso_domain** | **str** |  | [optional] 
**allowed_domains** | **List[str]** |  | [optional] 
**enforce2fa** | **bool** |  | [optional] 
**billing_plan** | [**BillingPlan**](BillingPlan.md) |  | [optional] [default to BillingPlan.UNSPECIFIED]
**billing_currency** | **str** |  | [optional] 
**auto_billing** | **bool** |  | [optional] 
**tax_id** | **str** |  | [optional] 
**compliance_frameworks** | **List[str]** | SOC2, GDPR, etc | [optional] 
**data_processing_agreement** | **bool** |  | [optional] 
**data_region** | **str** |  | [optional] 
**max_tenants** | **int** |  | [optional] 
**total_storage_limit** | **str** |  | [optional] 
**max_api_keys** | **int** |  | [optional] 
**max_users** | **int** |  | [optional] 
**tenants** | [**List[Tenant]**](Tenant.md) |  | [optional] 
**subscriptions** | [**Subscription**](Subscription.md) |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 
**deleted_at** | **datetime** |  | [optional] 
**created_by** | **str** |  | [optional] 
**status** | [**V1Status**](V1Status.md) |  | [optional] [default to V1Status.UNSPECIFIED]

## Example

```python
from playbookmedia_backend_client_sdk.models.organization import Organization

# TODO update the JSON string below
json = "{}"
# create an instance of Organization from a JSON string
organization_instance = Organization.from_json(json)
# print the JSON string representation of the object
print(Organization.to_json())

# convert the object into a dict
organization_dict = organization_instance.to_dict()
# create an instance of Organization from a dict
organization_from_dict = Organization.from_dict(organization_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


