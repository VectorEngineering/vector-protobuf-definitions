# AuthenticationErrorMessageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Code** | Pointer to [**AuthErrorCode**](AuthErrorCode.md) |  | [optional] [default to NO_AUTH_ERROR]
**Message** | Pointer to **string** |  | [optional] 
**TokenInfo** | Pointer to [**TokenInfo**](TokenInfo.md) |  | [optional] 
**AuthContext** | Pointer to [**AuthContext**](AuthContext.md) |  | [optional] 
**SessionInfo** | Pointer to [**SessionInfo**](SessionInfo.md) |  | [optional] 
**MfaInfo** | Pointer to [**MFAInfo**](MFAInfo.md) |  | [optional] 
**ErrorResponse** | Pointer to [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Methods

### NewAuthenticationErrorMessageResponse

`func NewAuthenticationErrorMessageResponse() *AuthenticationErrorMessageResponse`

NewAuthenticationErrorMessageResponse instantiates a new AuthenticationErrorMessageResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAuthenticationErrorMessageResponseWithDefaults

`func NewAuthenticationErrorMessageResponseWithDefaults() *AuthenticationErrorMessageResponse`

NewAuthenticationErrorMessageResponseWithDefaults instantiates a new AuthenticationErrorMessageResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCode

`func (o *AuthenticationErrorMessageResponse) GetCode() AuthErrorCode`

GetCode returns the Code field if non-nil, zero value otherwise.

### GetCodeOk

`func (o *AuthenticationErrorMessageResponse) GetCodeOk() (*AuthErrorCode, bool)`

GetCodeOk returns a tuple with the Code field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCode

`func (o *AuthenticationErrorMessageResponse) SetCode(v AuthErrorCode)`

SetCode sets Code field to given value.

### HasCode

`func (o *AuthenticationErrorMessageResponse) HasCode() bool`

HasCode returns a boolean if a field has been set.

### GetMessage

`func (o *AuthenticationErrorMessageResponse) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *AuthenticationErrorMessageResponse) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *AuthenticationErrorMessageResponse) SetMessage(v string)`

SetMessage sets Message field to given value.

### HasMessage

`func (o *AuthenticationErrorMessageResponse) HasMessage() bool`

HasMessage returns a boolean if a field has been set.

### GetTokenInfo

`func (o *AuthenticationErrorMessageResponse) GetTokenInfo() TokenInfo`

GetTokenInfo returns the TokenInfo field if non-nil, zero value otherwise.

### GetTokenInfoOk

`func (o *AuthenticationErrorMessageResponse) GetTokenInfoOk() (*TokenInfo, bool)`

GetTokenInfoOk returns a tuple with the TokenInfo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTokenInfo

`func (o *AuthenticationErrorMessageResponse) SetTokenInfo(v TokenInfo)`

SetTokenInfo sets TokenInfo field to given value.

### HasTokenInfo

`func (o *AuthenticationErrorMessageResponse) HasTokenInfo() bool`

HasTokenInfo returns a boolean if a field has been set.

### GetAuthContext

`func (o *AuthenticationErrorMessageResponse) GetAuthContext() AuthContext`

GetAuthContext returns the AuthContext field if non-nil, zero value otherwise.

### GetAuthContextOk

`func (o *AuthenticationErrorMessageResponse) GetAuthContextOk() (*AuthContext, bool)`

GetAuthContextOk returns a tuple with the AuthContext field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAuthContext

`func (o *AuthenticationErrorMessageResponse) SetAuthContext(v AuthContext)`

SetAuthContext sets AuthContext field to given value.

### HasAuthContext

`func (o *AuthenticationErrorMessageResponse) HasAuthContext() bool`

HasAuthContext returns a boolean if a field has been set.

### GetSessionInfo

`func (o *AuthenticationErrorMessageResponse) GetSessionInfo() SessionInfo`

GetSessionInfo returns the SessionInfo field if non-nil, zero value otherwise.

### GetSessionInfoOk

`func (o *AuthenticationErrorMessageResponse) GetSessionInfoOk() (*SessionInfo, bool)`

GetSessionInfoOk returns a tuple with the SessionInfo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSessionInfo

`func (o *AuthenticationErrorMessageResponse) SetSessionInfo(v SessionInfo)`

SetSessionInfo sets SessionInfo field to given value.

### HasSessionInfo

`func (o *AuthenticationErrorMessageResponse) HasSessionInfo() bool`

HasSessionInfo returns a boolean if a field has been set.

### GetMfaInfo

`func (o *AuthenticationErrorMessageResponse) GetMfaInfo() MFAInfo`

GetMfaInfo returns the MfaInfo field if non-nil, zero value otherwise.

### GetMfaInfoOk

`func (o *AuthenticationErrorMessageResponse) GetMfaInfoOk() (*MFAInfo, bool)`

GetMfaInfoOk returns a tuple with the MfaInfo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMfaInfo

`func (o *AuthenticationErrorMessageResponse) SetMfaInfo(v MFAInfo)`

SetMfaInfo sets MfaInfo field to given value.

### HasMfaInfo

`func (o *AuthenticationErrorMessageResponse) HasMfaInfo() bool`

HasMfaInfo returns a boolean if a field has been set.

### GetErrorResponse

`func (o *AuthenticationErrorMessageResponse) GetErrorResponse() ErrorResponse`

GetErrorResponse returns the ErrorResponse field if non-nil, zero value otherwise.

### GetErrorResponseOk

`func (o *AuthenticationErrorMessageResponse) GetErrorResponseOk() (*ErrorResponse, bool)`

GetErrorResponseOk returns a tuple with the ErrorResponse field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetErrorResponse

`func (o *AuthenticationErrorMessageResponse) SetErrorResponse(v ErrorResponse)`

SetErrorResponse sets ErrorResponse field to given value.

### HasErrorResponse

`func (o *AuthenticationErrorMessageResponse) HasErrorResponse() bool`

HasErrorResponse returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


