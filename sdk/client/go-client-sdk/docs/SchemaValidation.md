# SchemaValidation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**SchemaVersion** | Pointer to **string** |  | [optional] 
**InvalidPatterns** | Pointer to **[]string** |  | [optional] 
**TypeMismatches** | Pointer to **[]string** |  | [optional] 

## Methods

### NewSchemaValidation

`func NewSchemaValidation() *SchemaValidation`

NewSchemaValidation instantiates a new SchemaValidation object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSchemaValidationWithDefaults

`func NewSchemaValidationWithDefaults() *SchemaValidation`

NewSchemaValidationWithDefaults instantiates a new SchemaValidation object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetSchemaVersion

`func (o *SchemaValidation) GetSchemaVersion() string`

GetSchemaVersion returns the SchemaVersion field if non-nil, zero value otherwise.

### GetSchemaVersionOk

`func (o *SchemaValidation) GetSchemaVersionOk() (*string, bool)`

GetSchemaVersionOk returns a tuple with the SchemaVersion field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSchemaVersion

`func (o *SchemaValidation) SetSchemaVersion(v string)`

SetSchemaVersion sets SchemaVersion field to given value.

### HasSchemaVersion

`func (o *SchemaValidation) HasSchemaVersion() bool`

HasSchemaVersion returns a boolean if a field has been set.

### GetInvalidPatterns

`func (o *SchemaValidation) GetInvalidPatterns() []string`

GetInvalidPatterns returns the InvalidPatterns field if non-nil, zero value otherwise.

### GetInvalidPatternsOk

`func (o *SchemaValidation) GetInvalidPatternsOk() (*[]string, bool)`

GetInvalidPatternsOk returns a tuple with the InvalidPatterns field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInvalidPatterns

`func (o *SchemaValidation) SetInvalidPatterns(v []string)`

SetInvalidPatterns sets InvalidPatterns field to given value.

### HasInvalidPatterns

`func (o *SchemaValidation) HasInvalidPatterns() bool`

HasInvalidPatterns returns a boolean if a field has been set.

### GetTypeMismatches

`func (o *SchemaValidation) GetTypeMismatches() []string`

GetTypeMismatches returns the TypeMismatches field if non-nil, zero value otherwise.

### GetTypeMismatchesOk

`func (o *SchemaValidation) GetTypeMismatchesOk() (*[]string, bool)`

GetTypeMismatchesOk returns a tuple with the TypeMismatches field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTypeMismatches

`func (o *SchemaValidation) SetTypeMismatches(v []string)`

SetTypeMismatches sets TypeMismatches field to given value.

### HasTypeMismatches

`func (o *SchemaValidation) HasTypeMismatches() bool`

HasTypeMismatches returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


