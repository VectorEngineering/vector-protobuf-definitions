# Tenant

Key features: - Resource isolation - Independent configuration - Usage tracking - Service subscriptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**name** | **str** |  | [optional] 
**display_name** | **str** |  | [optional] 
**description** | **str** |  | [optional] 
**organization** | [**Organization**](Organization.md) |  | [optional] 
**api_base_url** | **str** |  | [optional] 
**environment_variables** | **Dict[str, str]** |  | [optional] 
**allowed_origins** | **List[str]** |  | [optional] 
**storage_quota** | **str** |  | [optional] 
**monthly_request_limit** | **str** |  | [optional] 
**max_concurrent_jobs** | **int** |  | [optional] 
**enable_caching** | **bool** |  | [optional] 
**enable_rate_limiting** | **bool** |  | [optional] 
**enable_request_logging** | **bool** |  | [optional] 
**accounts** | [**List[Account]**](Account.md) |  | [optional] 
**api_keys** | [**List[TenantAPIKey]**](TenantAPIKey.md) |  | [optional] 
**total_requests** | **str** |  | [optional] 
**total_storage_used** | **str** |  | [optional] 
**average_response_time** | **float** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 
**deleted_at** | **datetime** |  | [optional] 
**status** | [**TenantStatus**](TenantStatus.md) |  | [optional] [default to TenantStatus.UNSPECIFIED]

## Example

```python
from playbookmedia_backend_client_sdk.models.tenant import Tenant

# TODO update the JSON string below
json = "{}"
# create an instance of Tenant from a JSON string
tenant_instance = Tenant.from_json(json)
# print the JSON string representation of the object
print(Tenant.to_json())

# convert the object into a dict
tenant_dict = tenant_instance.to_dict()
# create an instance of Tenant from a dict
tenant_from_dict = Tenant.from_dict(tenant_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


