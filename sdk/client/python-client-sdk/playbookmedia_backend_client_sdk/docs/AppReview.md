# AppReview


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**app_id** | **str** |  | [optional] 
**reviewer_id** | **str** |  | [optional] 
**rating** | **float** |  | [optional] 
**review_text** | **str** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.app_review import AppReview

# TODO update the JSON string below
json = "{}"
# create an instance of AppReview from a JSON string
app_review_instance = AppReview.from_json(json)
# print the JSON string representation of the object
print(AppReview.to_json())

# convert the object into a dict
app_review_dict = app_review_instance.to_dict()
# create an instance of AppReview from a dict
app_review_from_dict = AppReview.from_dict(app_review_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


