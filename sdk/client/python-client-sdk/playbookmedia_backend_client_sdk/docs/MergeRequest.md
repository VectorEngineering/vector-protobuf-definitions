# MergeRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**source_branch_id** | **str** |  | [optional] 
**target_branch_id** | **str** |  | [optional] 
**title** | **str** |  | [optional] 
**description** | **str** |  | [optional] 
**author_id** | **str** |  | [optional] 
**reviewers** | **List[str]** |  | [optional] 
**status** | **str** |  | [optional] 
**labels** | **List[str]** |  | [optional] 
**has_conflicts** | **bool** |  | [optional] 
**automated_check_results** | **List[str]** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.merge_request import MergeRequest

# TODO update the JSON string below
json = "{}"
# create an instance of MergeRequest from a JSON string
merge_request_instance = MergeRequest.from_json(json)
# print the JSON string representation of the object
print(MergeRequest.to_json())

# convert the object into a dict
merge_request_dict = merge_request_instance.to_dict()
# create an instance of MergeRequest from a dict
merge_request_from_dict = MergeRequest.from_dict(merge_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


