# SignatureBlock


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**request_id** | **str** |  | [optional] 
**block_type** | **str** |  | [optional] 
**page_number** | **int** |  | [optional] 
**x_position** | **float** |  | [optional] 
**y_position** | **float** |  | [optional] 
**width** | **float** |  | [optional] 
**height** | **float** |  | [optional] 
**is_required** | **bool** |  | [optional] 
**signature_data** | **str** |  | [optional] 
**signed_at** | **datetime** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.signature_block import SignatureBlock

# TODO update the JSON string below
json = "{}"
# create an instance of SignatureBlock from a JSON string
signature_block_instance = SignatureBlock.from_json(json)
# print the JSON string representation of the object
print(SignatureBlock.to_json())

# convert the object into a dict
signature_block_dict = signature_block_instance.to_dict()
# create an instance of SignatureBlock from a dict
signature_block_from_dict = SignatureBlock.from_dict(signature_block_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


