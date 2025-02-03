# MarketplaceApp


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**name** | **str** |  | [optional] 
**description** | **str** |  | [optional] 
**developer_id** | **str** |  | [optional] 
**category** | [**AppCategory**](AppCategory.md) |  | [optional] [default to AppCategory.UNSPECIFIED]
**supported_contract_types** | **List[str]** |  | [optional] 
**features** | **List[str]** |  | [optional] 
**pricing_model** | [**PricingModel**](PricingModel.md) |  | [optional] [default to PricingModel.UNSPECIFIED]
**pricing_tiers** | **Dict[str, float]** |  | [optional] 
**supported_integrations** | **List[str]** |  | [optional] 
**is_verified** | **bool** |  | [optional] 
**rating** | **float** |  | [optional] 
**installation_count** | **int** |  | [optional] 
**required_permissions** | **List[str]** |  | [optional] 
**metadata** | **Dict[str, str]** |  | [optional] 
**documentation_url** | **str** |  | [optional] 
**support_email** | **str** |  | [optional] 
**screenshots** | **List[str]** |  | [optional] 
**privacy_policy_url** | **str** |  | [optional] 
**terms_url** | **str** |  | [optional] 
**published_at** | **datetime** |  | [optional] 
**last_updated** | **datetime** |  | [optional] 
**versions** | [**List[AppVersion]**](AppVersion.md) |  | [optional] 
**installations** | [**List[AppInstallation]**](AppInstallation.md) |  | [optional] 
**analytics** | [**List[AppAnalytics]**](AppAnalytics.md) |  | [optional] 
**reviews** | [**List[AppReview]**](AppReview.md) |  | [optional] 
**info** | [**AppDevelopmentInfo**](AppDevelopmentInfo.md) |  | [optional] 
**webhooks** | [**List[AppWebhook]**](AppWebhook.md) |  | [optional] 
**permissions** | [**List[AppPermission]**](AppPermission.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.marketplace_app import MarketplaceApp

# TODO update the JSON string below
json = "{}"
# create an instance of MarketplaceApp from a JSON string
marketplace_app_instance = MarketplaceApp.from_json(json)
# print the JSON string representation of the object
print(MarketplaceApp.to_json())

# convert the object into a dict
marketplace_app_dict = marketplace_app_instance.to_dict()
# create an instance of MarketplaceApp from a dict
marketplace_app_from_dict = MarketplaceApp.from_dict(marketplace_app_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


