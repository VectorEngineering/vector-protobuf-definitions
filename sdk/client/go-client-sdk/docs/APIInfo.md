# APIInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Version** | Pointer to **string** |  | [optional] 
**SupportedVersions** | Pointer to **[]string** |  | [optional] 
**IsDeprecated** | Pointer to **bool** |  | [optional] 
**SunsetDate** | Pointer to **time.Time** |  | [optional] 

## Methods

### NewAPIInfo

`func NewAPIInfo() *APIInfo`

NewAPIInfo instantiates a new APIInfo object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAPIInfoWithDefaults

`func NewAPIInfoWithDefaults() *APIInfo`

NewAPIInfoWithDefaults instantiates a new APIInfo object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetVersion

`func (o *APIInfo) GetVersion() string`

GetVersion returns the Version field if non-nil, zero value otherwise.

### GetVersionOk

`func (o *APIInfo) GetVersionOk() (*string, bool)`

GetVersionOk returns a tuple with the Version field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVersion

`func (o *APIInfo) SetVersion(v string)`

SetVersion sets Version field to given value.

### HasVersion

`func (o *APIInfo) HasVersion() bool`

HasVersion returns a boolean if a field has been set.

### GetSupportedVersions

`func (o *APIInfo) GetSupportedVersions() []string`

GetSupportedVersions returns the SupportedVersions field if non-nil, zero value otherwise.

### GetSupportedVersionsOk

`func (o *APIInfo) GetSupportedVersionsOk() (*[]string, bool)`

GetSupportedVersionsOk returns a tuple with the SupportedVersions field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSupportedVersions

`func (o *APIInfo) SetSupportedVersions(v []string)`

SetSupportedVersions sets SupportedVersions field to given value.

### HasSupportedVersions

`func (o *APIInfo) HasSupportedVersions() bool`

HasSupportedVersions returns a boolean if a field has been set.

### GetIsDeprecated

`func (o *APIInfo) GetIsDeprecated() bool`

GetIsDeprecated returns the IsDeprecated field if non-nil, zero value otherwise.

### GetIsDeprecatedOk

`func (o *APIInfo) GetIsDeprecatedOk() (*bool, bool)`

GetIsDeprecatedOk returns a tuple with the IsDeprecated field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsDeprecated

`func (o *APIInfo) SetIsDeprecated(v bool)`

SetIsDeprecated sets IsDeprecated field to given value.

### HasIsDeprecated

`func (o *APIInfo) HasIsDeprecated() bool`

HasIsDeprecated returns a boolean if a field has been set.

### GetSunsetDate

`func (o *APIInfo) GetSunsetDate() time.Time`

GetSunsetDate returns the SunsetDate field if non-nil, zero value otherwise.

### GetSunsetDateOk

`func (o *APIInfo) GetSunsetDateOk() (*time.Time, bool)`

GetSunsetDateOk returns a tuple with the SunsetDate field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSunsetDate

`func (o *APIInfo) SetSunsetDate(v time.Time)`

SetSunsetDate sets SunsetDate field to given value.

### HasSunsetDate

`func (o *APIInfo) HasSunsetDate() bool`

HasSunsetDate returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


