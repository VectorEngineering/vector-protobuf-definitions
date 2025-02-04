# Subscription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [optional] [default to None]
**stripe_customer_id** | **String** |  | [optional] [default to None]
**stripe_subscription_id** | **String** |  | [optional] [default to None]
**stripe_price_id** | **String** |  | [optional] [default to None]
**stripe_product_id** | **String** |  | [optional] [default to None]
**plan_tier** | [***models::PlanTier**](PlanTier.md) |  | [optional] [default to None]
**billing_mode** | [***models::BillingMode**](BillingMode.md) |  | [optional] [default to None]
**included_jobs** | **i32** | Jobs included in base price | [optional] [default to None]
**per_job_rate** | **f32** |  | [optional] [default to None]
**max_concurrent_jobs** | **i32** |  | [optional] [default to None]
**included_storage** | **String** |  | [optional] [default to None]
**per_gb_rate** | **f32** |  | [optional] [default to None]
**advanced_filtering_enabled** | **bool** |  | [optional] [default to None]
**priority_support_enabled** | **bool** |  | [optional] [default to None]
**custom_exports_enabled** | **bool** |  | [optional] [default to None]
**api_access_enabled** | **bool** |  | [optional] [default to None]
**custom_proxies_enabled** | **bool** |  | [optional] [default to None]
**advanced_analytics_enabled** | **bool** |  | [optional] [default to None]
**retention_days** | **i32** |  | [optional] [default to None]
**max_results_per_job** | **i32** |  | [optional] [default to None]
**current_period_start** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**current_period_end** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**current_period_usage** | **f32** |  | [optional] [default to None]
**currency** | **String** | e.g., \"usd\" | [optional] [default to None]
**base_price** | **f32** |  | [optional] [default to None]
**billing_interval** | [***models::Interval**](Interval.md) |  | [optional] [default to None]
**auto_renew** | **bool** |  | [optional] [default to None]
**payment_status** | [***models::PaymentStatus**](PaymentStatus.md) |  | [optional] [default to None]
**is_trial** | **bool** |  | [optional] [default to None]
**trial_start** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**trial_end** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**created_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**updated_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**canceled_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**ended_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


