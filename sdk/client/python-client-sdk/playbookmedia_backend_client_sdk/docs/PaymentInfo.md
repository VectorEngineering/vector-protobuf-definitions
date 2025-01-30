# PaymentInfo


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount_due** | **float** |  | [optional] 
**currency** | **str** |  | [optional] 
**due_date** | **datetime** |  | [optional] 
**payment_url** | **str** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.payment_info import PaymentInfo

# TODO update the JSON string below
json = "{}"
# create an instance of PaymentInfo from a JSON string
payment_info_instance = PaymentInfo.from_json(json)
# print the JSON string representation of the object
print(PaymentInfo.to_json())

# convert the object into a dict
payment_info_dict = payment_info_instance.to_dict()
# create an instance of PaymentInfo from a dict
payment_info_from_dict = PaymentInfo.from_dict(payment_info_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


