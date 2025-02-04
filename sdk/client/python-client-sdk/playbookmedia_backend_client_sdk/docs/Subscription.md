# Subscription

Key features: - Stripe integration - Usage-based billing - Metered pricing - Feature flags  Database considerations: - Syncs with Stripe webhooks - Tracks usage for metered billing - Maintains subscription lifecycle

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**stripe_customer_id** | **str** |  | [optional] 
**stripe_subscription_id** | **str** |  | [optional] 
**stripe_price_id** | **str** |  | [optional] 
**stripe_product_id** | **str** |  | [optional] 
**plan_tier** | [**PlanTier**](PlanTier.md) |  | [optional] [default to PlanTier.UNSPECIFIED]
**billing_mode** | [**BillingMode**](BillingMode.md) |  | [optional] [default to BillingMode.UNSPECIFIED]
**included_jobs** | **int** | Jobs included in base price | [optional] 
**per_job_rate** | **float** |  | [optional] 
**max_concurrent_jobs** | **int** |  | [optional] 
**included_storage** | **str** |  | [optional] 
**per_gb_rate** | **float** |  | [optional] 
**advanced_filtering_enabled** | **bool** |  | [optional] 
**priority_support_enabled** | **bool** |  | [optional] 
**custom_exports_enabled** | **bool** |  | [optional] 
**api_access_enabled** | **bool** |  | [optional] 
**custom_proxies_enabled** | **bool** |  | [optional] 
**advanced_analytics_enabled** | **bool** |  | [optional] 
**retention_days** | **int** |  | [optional] 
**max_results_per_job** | **int** |  | [optional] 
**current_period_start** | **datetime** |  | [optional] 
**current_period_end** | **datetime** |  | [optional] 
**current_period_usage** | **float** |  | [optional] 
**currency** | **str** | e.g., \&quot;usd\&quot; | [optional] 
**base_price** | **float** |  | [optional] 
**billing_interval** | [**Interval**](Interval.md) |  | [optional] [default to Interval.UNSPECIFIED]
**auto_renew** | **bool** |  | [optional] 
**payment_status** | [**PaymentStatus**](PaymentStatus.md) |  | [optional] [default to PaymentStatus.UNSPECIFIED]
**is_trial** | **bool** |  | [optional] 
**trial_start** | **datetime** |  | [optional] 
**trial_end** | **datetime** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 
**canceled_at** | **datetime** |  | [optional] 
**ended_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.subscription import Subscription

# TODO update the JSON string below
json = "{}"
# create an instance of Subscription from a JSON string
subscription_instance = Subscription.from_json(json)
# print the JSON string representation of the object
print(Subscription.to_json())

# convert the object into a dict
subscription_dict = subscription_instance.to_dict()
# create an instance of Subscription from a dict
subscription_from_dict = Subscription.from_dict(subscription_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


