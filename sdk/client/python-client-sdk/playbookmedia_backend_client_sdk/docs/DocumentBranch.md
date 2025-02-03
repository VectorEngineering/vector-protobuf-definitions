# DocumentBranch


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**name** | **str** |  | [optional] 
**base_version_hash** | **str** |  | [optional] 
**current_head_hash** | **str** |  | [optional] 
**purpose** | **str** |  | [optional] 
**status** | **str** |  | [optional] 
**reviewers** | **List[str]** |  | [optional] 
**branch_metadata** | **Dict[str, str]** |  | [optional] 
**is_locked** | **bool** |  | [optional] 
**last_updated** | **datetime** |  | [optional] 
**merges** | [**List[BranchMerge]**](BranchMerge.md) |  | [optional] 
**policy** | [**BranchPolicy**](BranchPolicy.md) |  | [optional] 
**merge_requests** | [**List[MergeRequest]**](MergeRequest.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.document_branch import DocumentBranch

# TODO update the JSON string below
json = "{}"
# create an instance of DocumentBranch from a JSON string
document_branch_instance = DocumentBranch.from_json(json)
# print the JSON string representation of the object
print(DocumentBranch.to_json())

# convert the object into a dict
document_branch_dict = document_branch_instance.to_dict()
# create an instance of DocumentBranch from a dict
document_branch_from_dict = DocumentBranch.from_dict(document_branch_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


