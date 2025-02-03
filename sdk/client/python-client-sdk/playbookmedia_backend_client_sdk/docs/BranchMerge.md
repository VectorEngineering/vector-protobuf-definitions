# BranchMerge


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**branch_id** | **str** |  | [optional] 
**source_branch** | **str** |  | [optional] 
**target_branch** | **str** |  | [optional] 
**merge_status** | **str** |  | [optional] 
**merge_strategy** | **str** |  | [optional] 
**conflicts** | **List[str]** |  | [optional] 
**merger_id** | **str** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**completed_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.branch_merge import BranchMerge

# TODO update the JSON string below
json = "{}"
# create an instance of BranchMerge from a JSON string
branch_merge_instance = BranchMerge.from_json(json)
# print the JSON string representation of the object
print(BranchMerge.to_json())

# convert the object into a dict
branch_merge_dict = branch_merge_instance.to_dict()
# create an instance of BranchMerge from a dict
branch_merge_from_dict = BranchMerge.from_dict(branch_merge_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


