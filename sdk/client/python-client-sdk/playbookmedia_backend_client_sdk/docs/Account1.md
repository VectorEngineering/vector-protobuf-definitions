# Account1


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**auth0_user_id** | **str** |  | [optional] 
**email** | **str** |  | [optional] 
**base_directory** | **str** |  | [optional] 
**bucket_name** | **str** |  | [optional] 
**region** | **str** |  | [optional] 
**org_id** | **str** |  | [optional] 
**tenant_id** | **str** |  | [optional] 
**roles** | **List[str]** |  | [optional] 
**permissions** | **List[str]** |  | [optional] 
**mfa_enabled** | **bool** |  | [optional] 
**compliance_level** | [**ComplianceLevel**](ComplianceLevel.md) |  | [optional] [default to ComplianceLevel.UNSPECIFIED]
**preferences** | **Dict[str, str]** |  | [optional] 
**api_keys** | **List[str]** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 
**deleted_at** | **datetime** |  | [optional] 
**workspaces** | [**List[Workspace1]**](Workspace1.md) |  | [optional] 
**data_profiles** | [**List[DataProfile]**](DataProfile.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.account1 import Account1

# TODO update the JSON string below
json = "{}"
# create an instance of Account1 from a JSON string
account1_instance = Account1.from_json(json)
# print the JSON string representation of the object
print(Account1.to_json())

# convert the object into a dict
account1_dict = account1_instance.to_dict()
# create an instance of Account1 from a dict
account1_from_dict = Account1.from_dict(account1_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


