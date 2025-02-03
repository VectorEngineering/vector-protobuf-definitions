# MarketplaceApp

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [optional] [default to None]
**name** | **String** |  | [optional] [default to None]
**description** | **String** |  | [optional] [default to None]
**developer_id** | **String** |  | [optional] [default to None]
**category** | [***models::AppCategory**](AppCategory.md) |  | [optional] [default to None]
**supported_contract_types** | **Vec<String>** |  | [optional] [default to None]
**features** | **Vec<String>** |  | [optional] [default to None]
**pricing_model** | [***models::PricingModel**](PricingModel.md) |  | [optional] [default to None]
**pricing_tiers** | **std::collections::HashMap<String, f32>** |  | [optional] [default to None]
**supported_integrations** | **Vec<String>** |  | [optional] [default to None]
**is_verified** | **bool** |  | [optional] [default to None]
**rating** | **f32** |  | [optional] [default to None]
**installation_count** | **i32** |  | [optional] [default to None]
**required_permissions** | **Vec<String>** |  | [optional] [default to None]
**metadata** | **std::collections::HashMap<String, String>** |  | [optional] [default to None]
**documentation_url** | **String** |  | [optional] [default to None]
**support_email** | **String** |  | [optional] [default to None]
**screenshots** | **Vec<String>** |  | [optional] [default to None]
**privacy_policy_url** | **String** |  | [optional] [default to None]
**terms_url** | **String** |  | [optional] [default to None]
**published_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**last_updated** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**versions** | [**Vec<models::AppVersion>**](AppVersion.md) |  | [optional] [default to None]
**installations** | [**Vec<models::AppInstallation>**](AppInstallation.md) |  | [optional] [default to None]
**analytics** | [**Vec<models::AppAnalytics>**](AppAnalytics.md) |  | [optional] [default to None]
**reviews** | [**Vec<models::AppReview>**](AppReview.md) |  | [optional] [default to None]
**info** | [***models::AppDevelopmentInfo**](AppDevelopmentInfo.md) |  | [optional] [default to None]
**webhooks** | [**Vec<models::AppWebhook>**](AppWebhook.md) |  | [optional] [default to None]
**permissions** | [**Vec<models::AppPermission>**](AppPermission.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


