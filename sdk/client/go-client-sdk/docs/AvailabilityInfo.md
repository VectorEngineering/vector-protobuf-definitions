# AvailabilityInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**RetryAfter** | Pointer to **int32** |  | [optional] 
**MaintenanceWindow** | Pointer to **string** |  | [optional] 
**AffectedServices** | Pointer to **[]string** |  | [optional] 
**StatusPageUrl** | Pointer to **string** |  | [optional] 

## Methods

### NewAvailabilityInfo

`func NewAvailabilityInfo() *AvailabilityInfo`

NewAvailabilityInfo instantiates a new AvailabilityInfo object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAvailabilityInfoWithDefaults

`func NewAvailabilityInfoWithDefaults() *AvailabilityInfo`

NewAvailabilityInfoWithDefaults instantiates a new AvailabilityInfo object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetRetryAfter

`func (o *AvailabilityInfo) GetRetryAfter() int32`

GetRetryAfter returns the RetryAfter field if non-nil, zero value otherwise.

### GetRetryAfterOk

`func (o *AvailabilityInfo) GetRetryAfterOk() (*int32, bool)`

GetRetryAfterOk returns a tuple with the RetryAfter field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRetryAfter

`func (o *AvailabilityInfo) SetRetryAfter(v int32)`

SetRetryAfter sets RetryAfter field to given value.

### HasRetryAfter

`func (o *AvailabilityInfo) HasRetryAfter() bool`

HasRetryAfter returns a boolean if a field has been set.

### GetMaintenanceWindow

`func (o *AvailabilityInfo) GetMaintenanceWindow() string`

GetMaintenanceWindow returns the MaintenanceWindow field if non-nil, zero value otherwise.

### GetMaintenanceWindowOk

`func (o *AvailabilityInfo) GetMaintenanceWindowOk() (*string, bool)`

GetMaintenanceWindowOk returns a tuple with the MaintenanceWindow field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaintenanceWindow

`func (o *AvailabilityInfo) SetMaintenanceWindow(v string)`

SetMaintenanceWindow sets MaintenanceWindow field to given value.

### HasMaintenanceWindow

`func (o *AvailabilityInfo) HasMaintenanceWindow() bool`

HasMaintenanceWindow returns a boolean if a field has been set.

### GetAffectedServices

`func (o *AvailabilityInfo) GetAffectedServices() []string`

GetAffectedServices returns the AffectedServices field if non-nil, zero value otherwise.

### GetAffectedServicesOk

`func (o *AvailabilityInfo) GetAffectedServicesOk() (*[]string, bool)`

GetAffectedServicesOk returns a tuple with the AffectedServices field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAffectedServices

`func (o *AvailabilityInfo) SetAffectedServices(v []string)`

SetAffectedServices sets AffectedServices field to given value.

### HasAffectedServices

`func (o *AvailabilityInfo) HasAffectedServices() bool`

HasAffectedServices returns a boolean if a field has been set.

### GetStatusPageUrl

`func (o *AvailabilityInfo) GetStatusPageUrl() string`

GetStatusPageUrl returns the StatusPageUrl field if non-nil, zero value otherwise.

### GetStatusPageUrlOk

`func (o *AvailabilityInfo) GetStatusPageUrlOk() (*string, bool)`

GetStatusPageUrlOk returns a tuple with the StatusPageUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatusPageUrl

`func (o *AvailabilityInfo) SetStatusPageUrl(v string)`

SetStatusPageUrl sets StatusPageUrl field to given value.

### HasStatusPageUrl

`func (o *AvailabilityInfo) HasStatusPageUrl() bool`

HasStatusPageUrl returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


