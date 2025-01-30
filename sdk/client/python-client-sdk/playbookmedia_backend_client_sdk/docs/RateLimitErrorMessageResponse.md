# RateLimitErrorMessageResponse

Represents rate limiting and quota exceeded errors

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | [**InternalErrorCode**](InternalErrorCode.md) |  | [optional] [default to InternalErrorCode.NO_INTERNAL_ERROR]
**message** | **str** |  | [optional] 
**limit_info** | [**LimitInfo**](LimitInfo.md) |  | [optional] 
**quota_info** | [**QuotaInfo**](QuotaInfo.md) |  | [optional] 
**context** | [**RateLimitContext**](RateLimitContext.md) |  | [optional] 
**error_response** | [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.rate_limit_error_message_response import RateLimitErrorMessageResponse

# TODO update the JSON string below
json = "{}"
# create an instance of RateLimitErrorMessageResponse from a JSON string
rate_limit_error_message_response_instance = RateLimitErrorMessageResponse.from_json(json)
# print the JSON string representation of the object
print(RateLimitErrorMessageResponse.to_json())

# convert the object into a dict
rate_limit_error_message_response_dict = rate_limit_error_message_response_instance.to_dict()
# create an instance of RateLimitErrorMessageResponse from a dict
rate_limit_error_message_response_from_dict = RateLimitErrorMessageResponse.from_dict(rate_limit_error_message_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


