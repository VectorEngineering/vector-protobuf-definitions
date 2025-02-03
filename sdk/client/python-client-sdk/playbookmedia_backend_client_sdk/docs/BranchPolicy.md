# BranchPolicy


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**branch_id** | **str** |  | [optional] 
**required_approvers** | **List[str]** |  | [optional] 
**minimum_approvals** | **int** |  | [optional] 
**enforce_linear_history** | **bool** |  | [optional] 
**allow_force_push** | **bool** |  | [optional] 
**protected_paths** | **List[str]** |  | [optional] 
**merge_rules** | **Dict[str, str]** |  | [optional] 
**automated_checks** | **List[str]** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.branch_policy import BranchPolicy

# TODO update the JSON string below
json = "{}"
# create an instance of BranchPolicy from a JSON string
branch_policy_instance = BranchPolicy.from_json(json)
# print the JSON string representation of the object
print(BranchPolicy.to_json())

# convert the object into a dict
branch_policy_dict = branch_policy_instance.to_dict()
# create an instance of BranchPolicy from a dict
branch_policy_from_dict = BranchPolicy.from_dict(branch_policy_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


