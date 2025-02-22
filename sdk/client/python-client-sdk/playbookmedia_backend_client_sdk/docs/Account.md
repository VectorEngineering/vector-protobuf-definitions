# Account

Account represents a user account within the workspace service. It serves as the top-level container for all user-specific workspaces and settings.  Key features: - Unique identification via Auth0 - Organization and tenant context - Audit timestamps - Associated scraping jobs  Database considerations: - Uses GORM for ORM mapping - Includes indexes for efficient querying - Supports soft deletes via deleted_at  Usage example: ```go account := &Account{     AuthPlatformUserId: \"auth0|123\",     OrgId: \"org_123\",     TenantId: \"tenant_456\", } ```

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**auth_platform_user_id** | **str** |  | [optional] 
**email** | **str** |  | [optional] 
**deleted_at** | **datetime** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**account_status** | [**AccountStatus**](AccountStatus.md) |  | [optional] [default to AccountStatus.UNSPECIFIED]
**roles** | [**List[Role]**](Role.md) |  | [optional] 
**permissions** | [**List[Permission]**](Permission.md) |  | [optional] 
**mfa_enabled** | **bool** |  | [optional] 
**last_login_at** | **datetime** |  | [optional] 
**timezone** | [**Timezone**](Timezone.md) |  | [optional] [default to Timezone.UNSPECIFIED]
**total_jobs_run** | **int** |  | [optional] 
**monthly_job_limit** | **int** |  | [optional] 
**concurrent_job_limit** | **int** |  | [optional] 
**workspaces** | [**List[Workspace]**](Workspace.md) |  | [optional] 
**settings** | [**AccountSettings**](AccountSettings.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.account import Account

# TODO update the JSON string below
json = "{}"
# create an instance of Account from a JSON string
account_instance = Account.from_json(json)
# print the JSON string representation of the object
print(Account.to_json())

# convert the object into a dict
account_dict = account_instance.to_dict()
# create an instance of Account from a dict
account_from_dict = Account.from_dict(account_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


