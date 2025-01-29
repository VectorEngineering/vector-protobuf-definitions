# TokenInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**TokenId** | Pointer to **string** |  | [optional] 
**TokenType** | Pointer to **string** |  | [optional] 
**Expiry** | Pointer to **time.Time** |  | [optional] 
**Issuer** | Pointer to **string** |  | [optional] 
**Audiences** | Pointer to **[]string** |  | [optional] 

## Methods

### NewTokenInfo

`func NewTokenInfo() *TokenInfo`

NewTokenInfo instantiates a new TokenInfo object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewTokenInfoWithDefaults

`func NewTokenInfoWithDefaults() *TokenInfo`

NewTokenInfoWithDefaults instantiates a new TokenInfo object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetTokenId

`func (o *TokenInfo) GetTokenId() string`

GetTokenId returns the TokenId field if non-nil, zero value otherwise.

### GetTokenIdOk

`func (o *TokenInfo) GetTokenIdOk() (*string, bool)`

GetTokenIdOk returns a tuple with the TokenId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTokenId

`func (o *TokenInfo) SetTokenId(v string)`

SetTokenId sets TokenId field to given value.

### HasTokenId

`func (o *TokenInfo) HasTokenId() bool`

HasTokenId returns a boolean if a field has been set.

### GetTokenType

`func (o *TokenInfo) GetTokenType() string`

GetTokenType returns the TokenType field if non-nil, zero value otherwise.

### GetTokenTypeOk

`func (o *TokenInfo) GetTokenTypeOk() (*string, bool)`

GetTokenTypeOk returns a tuple with the TokenType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTokenType

`func (o *TokenInfo) SetTokenType(v string)`

SetTokenType sets TokenType field to given value.

### HasTokenType

`func (o *TokenInfo) HasTokenType() bool`

HasTokenType returns a boolean if a field has been set.

### GetExpiry

`func (o *TokenInfo) GetExpiry() time.Time`

GetExpiry returns the Expiry field if non-nil, zero value otherwise.

### GetExpiryOk

`func (o *TokenInfo) GetExpiryOk() (*time.Time, bool)`

GetExpiryOk returns a tuple with the Expiry field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiry

`func (o *TokenInfo) SetExpiry(v time.Time)`

SetExpiry sets Expiry field to given value.

### HasExpiry

`func (o *TokenInfo) HasExpiry() bool`

HasExpiry returns a boolean if a field has been set.

### GetIssuer

`func (o *TokenInfo) GetIssuer() string`

GetIssuer returns the Issuer field if non-nil, zero value otherwise.

### GetIssuerOk

`func (o *TokenInfo) GetIssuerOk() (*string, bool)`

GetIssuerOk returns a tuple with the Issuer field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIssuer

`func (o *TokenInfo) SetIssuer(v string)`

SetIssuer sets Issuer field to given value.

### HasIssuer

`func (o *TokenInfo) HasIssuer() bool`

HasIssuer returns a boolean if a field has been set.

### GetAudiences

`func (o *TokenInfo) GetAudiences() []string`

GetAudiences returns the Audiences field if non-nil, zero value otherwise.

### GetAudiencesOk

`func (o *TokenInfo) GetAudiencesOk() (*[]string, bool)`

GetAudiencesOk returns a tuple with the Audiences field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAudiences

`func (o *TokenInfo) SetAudiences(v []string)`

SetAudiences sets Audiences field to given value.

### HasAudiences

`func (o *TokenInfo) HasAudiences() bool`

HasAudiences returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


