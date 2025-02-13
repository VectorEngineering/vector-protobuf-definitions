# APIKey

APIKey represents an authentication token for accessing the API. It includes features for rate limiting, permissions, and usage tracking.  Key features: - Unique key identification - Scope-based access control - Rate limiting - Usage tracking - Expiration management - Multi-tenant support  Database considerations: - Uses GORM for ORM mapping - Includes indexes for efficient querying - Supports soft deletes  Usage example: ```go apiKey := &APIKey{     Name: \"Production API Key\",     Scopes: []string{\"leads:read\", \"leads:write\"},     ExpiresAt: timestamppb.New(time.Now().AddDate(1, 0, 0)), } ```

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**name** | **str** |  | [optional] 
**key_hash** | **str** |  | [optional] 
**key_prefix** | **str** |  | [optional] 
**scopes** | **List[str]** |  | [optional] 
**allowed_ips** | **List[str]** |  | [optional] 
**allowed_domains** | **List[str]** |  | [optional] 
**allowed_environments** | **List[str]** |  | [optional] 
**is_test_key** | **bool** |  | [optional] 
**requests_per_second** | **int** |  | [optional] 
**requests_per_day** | **int** |  | [optional] 
**concurrent_requests** | **int** |  | [optional] 
**monthly_request_quota** | **str** |  | [optional] 
**cost_per_request** | **float** |  | [optional] 
**billing_tier** | **str** |  | [optional] 
**total_requests** | **str** |  | [optional] 
**total_errors** | **str** |  | [optional] 
**last_used_at** | **datetime** |  | [optional] 
**average_response_time** | **float** |  | [optional] 
**endpoint_usage_json** | **bytearray** | Tracks usage per endpoint | [optional] 
**error_rates_json** | **bytearray** |  | [optional] 
**recent_errors** | **bytearray** |  | [optional] 
**successful_requests_count** | **int** |  | [optional] 
**success_rate** | **float** |  | [optional] 
**status** | [**V1Status**](V1Status.md) |  | [optional] [default to V1Status.UNSPECIFIED]
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 
**expires_at** | **datetime** |  | [optional] 
**deleted_at** | **datetime** |  | [optional] 
**last_rotated_at** | **datetime** |  | [optional] 
**last_security_review_at** | **datetime** |  | [optional] 
**requires_client_secret** | **bool** |  | [optional] 
**client_secret_hash** | **str** |  | [optional] 
**enforce_https** | **bool** |  | [optional] 
**enforce_signing** | **bool** |  | [optional] 
**allowed_signature_algorithms** | **List[str]** |  | [optional] 
**enforce_mutual_tls** | **bool** |  | [optional] 
**client_certificate_hash** | **str** |  | [optional] 
**require_request_signing** | **bool** |  | [optional] 
**description** | **str** | Detailed description of key usage | [optional] 
**metadata_json** | **bytearray** |  | [optional] 
**tags** | **List[str]** |  | [optional] 
**api_version** | **str** |  | [optional] 
**supported_features** | **List[str]** |  | [optional] 
**documentation_url** | **str** |  | [optional] 
**support_contact** | **str** |  | [optional] 
**log_all_requests** | **bool** |  | [optional] 
**last_rotation_reason** | **str** |  | [optional] 
**last_rotation_date** | **datetime** |  | [optional] 
**rotation_frequency_days** | **int** |  | [optional] 
**compliance_standards** | **List[str]** |  | [optional] 
**requires_audit_logging** | **bool** |  | [optional] 
**data_residency** | **str** |  | [optional] 
**approved_integrations** | **List[str]** |  | [optional] 
**alert_emails** | **List[str]** |  | [optional] 
**webhook_url** | **str** |  | [optional] 
**alert_on_quota_threshold** | **bool** |  | [optional] 
**quota_alert_threshold** | **float** |  | [optional] 
**alert_on_error_spike** | **bool** |  | [optional] 
**error_alert_threshold** | **float** |  | [optional] 
**monitoring_integrations** | **List[str]** |  | [optional] 
**encrypted** | **bool** |  | [optional] 
**data_classification** | **str** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.api_key import APIKey

# TODO update the JSON string below
json = "{}"
# create an instance of APIKey from a JSON string
api_key_instance = APIKey.from_json(json)
# print the JSON string representation of the object
print(APIKey.to_json())

# convert the object into a dict
api_key_dict = api_key_instance.to_dict()
# create an instance of APIKey from a dict
api_key_from_dict = APIKey.from_dict(api_key_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


