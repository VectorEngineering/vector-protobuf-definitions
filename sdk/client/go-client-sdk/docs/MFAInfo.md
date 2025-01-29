# MFAInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**MfaRequired** | Pointer to **bool** |  | [optional] 
**AllowedMethods** | Pointer to **[]string** |  | [optional] 
**FailedReason** | Pointer to **string** |  | [optional] 

## Methods

### NewMFAInfo

`func NewMFAInfo() *MFAInfo`

NewMFAInfo instantiates a new MFAInfo object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMFAInfoWithDefaults

`func NewMFAInfoWithDefaults() *MFAInfo`

NewMFAInfoWithDefaults instantiates a new MFAInfo object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetMfaRequired

`func (o *MFAInfo) GetMfaRequired() bool`

GetMfaRequired returns the MfaRequired field if non-nil, zero value otherwise.

### GetMfaRequiredOk

`func (o *MFAInfo) GetMfaRequiredOk() (*bool, bool)`

GetMfaRequiredOk returns a tuple with the MfaRequired field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMfaRequired

`func (o *MFAInfo) SetMfaRequired(v bool)`

SetMfaRequired sets MfaRequired field to given value.

### HasMfaRequired

`func (o *MFAInfo) HasMfaRequired() bool`

HasMfaRequired returns a boolean if a field has been set.

### GetAllowedMethods

`func (o *MFAInfo) GetAllowedMethods() []string`

GetAllowedMethods returns the AllowedMethods field if non-nil, zero value otherwise.

### GetAllowedMethodsOk

`func (o *MFAInfo) GetAllowedMethodsOk() (*[]string, bool)`

GetAllowedMethodsOk returns a tuple with the AllowedMethods field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedMethods

`func (o *MFAInfo) SetAllowedMethods(v []string)`

SetAllowedMethods sets AllowedMethods field to given value.

### HasAllowedMethods

`func (o *MFAInfo) HasAllowedMethods() bool`

HasAllowedMethods returns a boolean if a field has been set.

### GetFailedReason

`func (o *MFAInfo) GetFailedReason() string`

GetFailedReason returns the FailedReason field if non-nil, zero value otherwise.

### GetFailedReasonOk

`func (o *MFAInfo) GetFailedReasonOk() (*string, bool)`

GetFailedReasonOk returns a tuple with the FailedReason field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFailedReason

`func (o *MFAInfo) SetFailedReason(v string)`

SetFailedReason sets FailedReason field to given value.

### HasFailedReason

`func (o *MFAInfo) HasFailedReason() bool`

HasFailedReason returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


