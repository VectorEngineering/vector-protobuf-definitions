# WorkspaceCompliance


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**compliance_type** | **str** |  | [optional] 
**status** | **str** |  | [optional] 
**certification_id** | **str** |  | [optional] 
**valid_until** | **datetime** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.workspace_compliance import WorkspaceCompliance

# TODO update the JSON string below
json = "{}"
# create an instance of WorkspaceCompliance from a JSON string
workspace_compliance_instance = WorkspaceCompliance.from_json(json)
# print the JSON string representation of the object
print(WorkspaceCompliance.to_json())

# convert the object into a dict
workspace_compliance_dict = workspace_compliance_instance.to_dict()
# create an instance of WorkspaceCompliance from a dict
workspace_compliance_from_dict = WorkspaceCompliance.from_dict(workspace_compliance_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


