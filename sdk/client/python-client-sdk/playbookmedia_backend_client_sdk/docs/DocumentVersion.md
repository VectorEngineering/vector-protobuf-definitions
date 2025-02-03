# DocumentVersion


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**version_hash** | **str** |  | [optional] 
**parent_hash** | **str** |  | [optional] 
**commit_message** | **str** |  | [optional] 
**author_id** | **str** |  | [optional] 
**branch_name** | **str** |  | [optional] 
**metadata** | **Dict[str, str]** |  | [optional] 
**tags** | **List[str]** |  | [optional] 
**is_approved** | **bool** |  | [optional] 
**approval_chain** | **str** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**branches** | [**List[DocumentBranch]**](DocumentBranch.md) |  | [optional] 
**changes** | [**List[ChangeSet]**](ChangeSet.md) |  | [optional] 
**summaries** | [**List[ContextualSummary]**](ContextualSummary.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.document_version import DocumentVersion

# TODO update the JSON string below
json = "{}"
# create an instance of DocumentVersion from a JSON string
document_version_instance = DocumentVersion.from_json(json)
# print the JSON string representation of the object
print(DocumentVersion.to_json())

# convert the object into a dict
document_version_dict = document_version_instance.to_dict()
# create an instance of DocumentVersion from a dict
document_version_from_dict = DocumentVersion.from_dict(document_version_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


