# TemplateVersion


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**template_id** | **str** |  | [optional] 
**version** | **str** |  | [optional] 
**base_content** | **str** |  | [optional] 
**metadata** | **Dict[str, str]** |  | [optional] 
**author_id** | **str** |  | [optional] 
**change_description** | **str** |  | [optional] 
**created_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.template_version import TemplateVersion

# TODO update the JSON string below
json = "{}"
# create an instance of TemplateVersion from a JSON string
template_version_instance = TemplateVersion.from_json(json)
# print the JSON string representation of the object
print(TemplateVersion.to_json())

# convert the object into a dict
template_version_dict = template_version_instance.to_dict()
# create an instance of TemplateVersion from a dict
template_version_from_dict = TemplateVersion.from_dict(template_version_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


