# ComplianceScore


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category** | **str** |  | [optional] 
**score** | **float** |  | [optional] 
**status** | **str** |  | [optional] 
**improvements** | **List[str]** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.compliance_score import ComplianceScore

# TODO update the JSON string below
json = "{}"
# create an instance of ComplianceScore from a JSON string
compliance_score_instance = ComplianceScore.from_json(json)
# print the JSON string representation of the object
print(ComplianceScore.to_json())

# convert the object into a dict
compliance_score_dict = compliance_score_instance.to_dict()
# create an instance of ComplianceScore from a dict
compliance_score_from_dict = ComplianceScore.from_dict(compliance_score_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


