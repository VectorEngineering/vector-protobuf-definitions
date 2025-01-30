# Review


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**author** | **str** |  | [optional] 
**rating** | **float** |  | [optional] 
**text** | **str** |  | [optional] 
**time** | **datetime** |  | [optional] 
**language** | **str** |  | [optional] 
**profile_photo_url** | **str** |  | [optional] 
**review_count** | **int** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.review import Review

# TODO update the JSON string below
json = "{}"
# create an instance of Review from a JSON string
review_instance = Review.from_json(json)
# print the JSON string representation of the object
print(Review.to_json())

# convert the object into a dict
review_dict = review_instance.to_dict()
# create an instance of Review from a dict
review_from_dict = Review.from_dict(review_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


