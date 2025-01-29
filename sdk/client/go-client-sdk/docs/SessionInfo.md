# SessionInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**SessionId** | Pointer to **string** |  | [optional] 
**LastActive** | Pointer to **time.Time** |  | [optional] 
**DeviceId** | Pointer to **string** |  | [optional] 
**IpAddress** | Pointer to **string** |  | [optional] 

## Methods

### NewSessionInfo

`func NewSessionInfo() *SessionInfo`

NewSessionInfo instantiates a new SessionInfo object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSessionInfoWithDefaults

`func NewSessionInfoWithDefaults() *SessionInfo`

NewSessionInfoWithDefaults instantiates a new SessionInfo object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetSessionId

`func (o *SessionInfo) GetSessionId() string`

GetSessionId returns the SessionId field if non-nil, zero value otherwise.

### GetSessionIdOk

`func (o *SessionInfo) GetSessionIdOk() (*string, bool)`

GetSessionIdOk returns a tuple with the SessionId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSessionId

`func (o *SessionInfo) SetSessionId(v string)`

SetSessionId sets SessionId field to given value.

### HasSessionId

`func (o *SessionInfo) HasSessionId() bool`

HasSessionId returns a boolean if a field has been set.

### GetLastActive

`func (o *SessionInfo) GetLastActive() time.Time`

GetLastActive returns the LastActive field if non-nil, zero value otherwise.

### GetLastActiveOk

`func (o *SessionInfo) GetLastActiveOk() (*time.Time, bool)`

GetLastActiveOk returns a tuple with the LastActive field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastActive

`func (o *SessionInfo) SetLastActive(v time.Time)`

SetLastActive sets LastActive field to given value.

### HasLastActive

`func (o *SessionInfo) HasLastActive() bool`

HasLastActive returns a boolean if a field has been set.

### GetDeviceId

`func (o *SessionInfo) GetDeviceId() string`

GetDeviceId returns the DeviceId field if non-nil, zero value otherwise.

### GetDeviceIdOk

`func (o *SessionInfo) GetDeviceIdOk() (*string, bool)`

GetDeviceIdOk returns a tuple with the DeviceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeviceId

`func (o *SessionInfo) SetDeviceId(v string)`

SetDeviceId sets DeviceId field to given value.

### HasDeviceId

`func (o *SessionInfo) HasDeviceId() bool`

HasDeviceId returns a boolean if a field has been set.

### GetIpAddress

`func (o *SessionInfo) GetIpAddress() string`

GetIpAddress returns the IpAddress field if non-nil, zero value otherwise.

### GetIpAddressOk

`func (o *SessionInfo) GetIpAddressOk() (*string, bool)`

GetIpAddressOk returns a tuple with the IpAddress field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIpAddress

`func (o *SessionInfo) SetIpAddress(v string)`

SetIpAddress sets IpAddress field to given value.

### HasIpAddress

`func (o *SessionInfo) HasIpAddress() bool`

HasIpAddress returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


