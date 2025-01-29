# PaymentInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AmountDue** | Pointer to **float64** |  | [optional] 
**Currency** | Pointer to **string** |  | [optional] 
**DueDate** | Pointer to **time.Time** |  | [optional] 
**PaymentUrl** | Pointer to **string** |  | [optional] 

## Methods

### NewPaymentInfo

`func NewPaymentInfo() *PaymentInfo`

NewPaymentInfo instantiates a new PaymentInfo object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPaymentInfoWithDefaults

`func NewPaymentInfoWithDefaults() *PaymentInfo`

NewPaymentInfoWithDefaults instantiates a new PaymentInfo object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAmountDue

`func (o *PaymentInfo) GetAmountDue() float64`

GetAmountDue returns the AmountDue field if non-nil, zero value otherwise.

### GetAmountDueOk

`func (o *PaymentInfo) GetAmountDueOk() (*float64, bool)`

GetAmountDueOk returns a tuple with the AmountDue field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAmountDue

`func (o *PaymentInfo) SetAmountDue(v float64)`

SetAmountDue sets AmountDue field to given value.

### HasAmountDue

`func (o *PaymentInfo) HasAmountDue() bool`

HasAmountDue returns a boolean if a field has been set.

### GetCurrency

`func (o *PaymentInfo) GetCurrency() string`

GetCurrency returns the Currency field if non-nil, zero value otherwise.

### GetCurrencyOk

`func (o *PaymentInfo) GetCurrencyOk() (*string, bool)`

GetCurrencyOk returns a tuple with the Currency field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCurrency

`func (o *PaymentInfo) SetCurrency(v string)`

SetCurrency sets Currency field to given value.

### HasCurrency

`func (o *PaymentInfo) HasCurrency() bool`

HasCurrency returns a boolean if a field has been set.

### GetDueDate

`func (o *PaymentInfo) GetDueDate() time.Time`

GetDueDate returns the DueDate field if non-nil, zero value otherwise.

### GetDueDateOk

`func (o *PaymentInfo) GetDueDateOk() (*time.Time, bool)`

GetDueDateOk returns a tuple with the DueDate field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDueDate

`func (o *PaymentInfo) SetDueDate(v time.Time)`

SetDueDate sets DueDate field to given value.

### HasDueDate

`func (o *PaymentInfo) HasDueDate() bool`

HasDueDate returns a boolean if a field has been set.

### GetPaymentUrl

`func (o *PaymentInfo) GetPaymentUrl() string`

GetPaymentUrl returns the PaymentUrl field if non-nil, zero value otherwise.

### GetPaymentUrlOk

`func (o *PaymentInfo) GetPaymentUrlOk() (*string, bool)`

GetPaymentUrlOk returns a tuple with the PaymentUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPaymentUrl

`func (o *PaymentInfo) SetPaymentUrl(v string)`

SetPaymentUrl sets PaymentUrl field to given value.

### HasPaymentUrl

`func (o *PaymentInfo) HasPaymentUrl() bool`

HasPaymentUrl returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


