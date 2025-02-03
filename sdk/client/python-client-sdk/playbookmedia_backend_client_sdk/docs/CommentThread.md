# CommentThread


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**author_id** | **str** |  | [optional] 
**content** | **str** |  | [optional] 
**start_position** | **int** |  | [optional] 
**end_position** | **int** |  | [optional] 
**resolved** | **bool** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.comment_thread import CommentThread

# TODO update the JSON string below
json = "{}"
# create an instance of CommentThread from a JSON string
comment_thread_instance = CommentThread.from_json(json)
# print the JSON string representation of the object
print(CommentThread.to_json())

# convert the object into a dict
comment_thread_dict = comment_thread_instance.to_dict()
# create an instance of CommentThread from a dict
comment_thread_from_dict = CommentThread.from_dict(comment_thread_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


