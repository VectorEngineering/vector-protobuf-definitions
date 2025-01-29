# ListAccountsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Accounts** | Pointer to [**[]Account**](Account.md) |  | [optional] 
**Total** | Pointer to **int32** |  | [optional] 

## Methods

### NewListAccountsResponse

`func NewListAccountsResponse() *ListAccountsResponse`

NewListAccountsResponse instantiates a new ListAccountsResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewListAccountsResponseWithDefaults

`func NewListAccountsResponseWithDefaults() *ListAccountsResponse`

NewListAccountsResponseWithDefaults instantiates a new ListAccountsResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAccounts

`func (o *ListAccountsResponse) GetAccounts() []Account`

GetAccounts returns the Accounts field if non-nil, zero value otherwise.

### GetAccountsOk

`func (o *ListAccountsResponse) GetAccountsOk() (*[]Account, bool)`

GetAccountsOk returns a tuple with the Accounts field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAccounts

`func (o *ListAccountsResponse) SetAccounts(v []Account)`

SetAccounts sets Accounts field to given value.

### HasAccounts

`func (o *ListAccountsResponse) HasAccounts() bool`

HasAccounts returns a boolean if a field has been set.

### GetTotal

`func (o *ListAccountsResponse) GetTotal() int32`

GetTotal returns the Total field if non-nil, zero value otherwise.

### GetTotalOk

`func (o *ListAccountsResponse) GetTotalOk() (*int32, bool)`

GetTotalOk returns a tuple with the Total field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotal

`func (o *ListAccountsResponse) SetTotal(v int32)`

SetTotal sets Total field to given value.

### HasTotal

`func (o *ListAccountsResponse) HasTotal() bool`

HasTotal returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


