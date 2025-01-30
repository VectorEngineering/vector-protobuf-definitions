# PaymentRequiredErrorMessageResponse

Indicates that payment is required to proceed

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **int** |  | [optional] 
**message** | **str** |  | [optional] 
**payment_info** | [**PaymentInfo**](PaymentInfo.md) |  | [optional] 
**error_response** | [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.payment_required_error_message_response import PaymentRequiredErrorMessageResponse

# TODO update the JSON string below
json = "{}"
# create an instance of PaymentRequiredErrorMessageResponse from a JSON string
payment_required_error_message_response_instance = PaymentRequiredErrorMessageResponse.from_json(json)
# print the JSON string representation of the object
print(PaymentRequiredErrorMessageResponse.to_json())

# convert the object into a dict
payment_required_error_message_response_dict = payment_required_error_message_response_instance.to_dict()
# create an instance of PaymentRequiredErrorMessageResponse from a dict
payment_required_error_message_response_from_dict = PaymentRequiredErrorMessageResponse.from_dict(payment_required_error_message_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


