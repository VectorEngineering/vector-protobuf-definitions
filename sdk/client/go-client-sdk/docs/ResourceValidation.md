# ResourceValidation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**MissingResources** | Pointer to **[]string** |  | [optional] 
**InvalidReferences** | Pointer to **[]string** |  | [optional] 
**IdViolations** | Pointer to **map[string]string** |  | [optional] 

## Methods

### NewResourceValidation

`func NewResourceValidation() *ResourceValidation`

NewResourceValidation instantiates a new ResourceValidation object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewResourceValidationWithDefaults

`func NewResourceValidationWithDefaults() *ResourceValidation`

NewResourceValidationWithDefaults instantiates a new ResourceValidation object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetMissingResources

`func (o *ResourceValidation) GetMissingResources() []string`

GetMissingResources returns the MissingResources field if non-nil, zero value otherwise.

### GetMissingResourcesOk

`func (o *ResourceValidation) GetMissingResourcesOk() (*[]string, bool)`

GetMissingResourcesOk returns a tuple with the MissingResources field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMissingResources

`func (o *ResourceValidation) SetMissingResources(v []string)`

SetMissingResources sets MissingResources field to given value.

### HasMissingResources

`func (o *ResourceValidation) HasMissingResources() bool`

HasMissingResources returns a boolean if a field has been set.

### GetInvalidReferences

`func (o *ResourceValidation) GetInvalidReferences() []string`

GetInvalidReferences returns the InvalidReferences field if non-nil, zero value otherwise.

### GetInvalidReferencesOk

`func (o *ResourceValidation) GetInvalidReferencesOk() (*[]string, bool)`

GetInvalidReferencesOk returns a tuple with the InvalidReferences field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInvalidReferences

`func (o *ResourceValidation) SetInvalidReferences(v []string)`

SetInvalidReferences sets InvalidReferences field to given value.

### HasInvalidReferences

`func (o *ResourceValidation) HasInvalidReferences() bool`

HasInvalidReferences returns a boolean if a field has been set.

### GetIdViolations

`func (o *ResourceValidation) GetIdViolations() map[string]string`

GetIdViolations returns the IdViolations field if non-nil, zero value otherwise.

### GetIdViolationsOk

`func (o *ResourceValidation) GetIdViolationsOk() (*map[string]string, bool)`

GetIdViolationsOk returns a tuple with the IdViolations field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIdViolations

`func (o *ResourceValidation) SetIdViolations(v map[string]string)`

SetIdViolations sets IdViolations field to given value.

### HasIdViolations

`func (o *ResourceValidation) HasIdViolations() bool`

HasIdViolations returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


