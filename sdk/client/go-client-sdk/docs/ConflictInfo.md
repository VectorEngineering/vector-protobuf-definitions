# ConflictInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ResourceType** | Pointer to **string** |  | [optional] 
**Identifier** | Pointer to **string** |  | [optional] 
**ConflictReason** | Pointer to **string** |  | [optional] 
**CreatedAt** | Pointer to **time.Time** |  | [optional] 
**CreatedBy** | Pointer to **string** |  | [optional] 

## Methods

### NewConflictInfo

`func NewConflictInfo() *ConflictInfo`

NewConflictInfo instantiates a new ConflictInfo object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewConflictInfoWithDefaults

`func NewConflictInfoWithDefaults() *ConflictInfo`

NewConflictInfoWithDefaults instantiates a new ConflictInfo object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetResourceType

`func (o *ConflictInfo) GetResourceType() string`

GetResourceType returns the ResourceType field if non-nil, zero value otherwise.

### GetResourceTypeOk

`func (o *ConflictInfo) GetResourceTypeOk() (*string, bool)`

GetResourceTypeOk returns a tuple with the ResourceType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResourceType

`func (o *ConflictInfo) SetResourceType(v string)`

SetResourceType sets ResourceType field to given value.

### HasResourceType

`func (o *ConflictInfo) HasResourceType() bool`

HasResourceType returns a boolean if a field has been set.

### GetIdentifier

`func (o *ConflictInfo) GetIdentifier() string`

GetIdentifier returns the Identifier field if non-nil, zero value otherwise.

### GetIdentifierOk

`func (o *ConflictInfo) GetIdentifierOk() (*string, bool)`

GetIdentifierOk returns a tuple with the Identifier field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIdentifier

`func (o *ConflictInfo) SetIdentifier(v string)`

SetIdentifier sets Identifier field to given value.

### HasIdentifier

`func (o *ConflictInfo) HasIdentifier() bool`

HasIdentifier returns a boolean if a field has been set.

### GetConflictReason

`func (o *ConflictInfo) GetConflictReason() string`

GetConflictReason returns the ConflictReason field if non-nil, zero value otherwise.

### GetConflictReasonOk

`func (o *ConflictInfo) GetConflictReasonOk() (*string, bool)`

GetConflictReasonOk returns a tuple with the ConflictReason field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConflictReason

`func (o *ConflictInfo) SetConflictReason(v string)`

SetConflictReason sets ConflictReason field to given value.

### HasConflictReason

`func (o *ConflictInfo) HasConflictReason() bool`

HasConflictReason returns a boolean if a field has been set.

### GetCreatedAt

`func (o *ConflictInfo) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *ConflictInfo) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *ConflictInfo) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *ConflictInfo) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### GetCreatedBy

`func (o *ConflictInfo) GetCreatedBy() string`

GetCreatedBy returns the CreatedBy field if non-nil, zero value otherwise.

### GetCreatedByOk

`func (o *ConflictInfo) GetCreatedByOk() (*string, bool)`

GetCreatedByOk returns a tuple with the CreatedBy field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedBy

`func (o *ConflictInfo) SetCreatedBy(v string)`

SetCreatedBy sets CreatedBy field to given value.

### HasCreatedBy

`func (o *ConflictInfo) HasCreatedBy() bool`

HasCreatedBy returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


