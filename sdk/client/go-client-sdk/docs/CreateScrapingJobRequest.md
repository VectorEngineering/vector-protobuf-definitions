# CreateScrapingJobRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**UserId** | **string** |  | 
**OrgId** | **string** |  | 
**TenantId** | **string** |  | 
**Name** | Pointer to **string** |  | [optional] 
**Keywords** | Pointer to **[]string** |  | [optional] 
**Lang** | Pointer to **string** |  | [optional] 
**Zoom** | Pointer to **int32** |  | [optional] 
**Lat** | Pointer to **string** |  | [optional] 
**Lon** | Pointer to **string** |  | [optional] 
**FastMode** | Pointer to **bool** |  | [optional] 
**Radius** | Pointer to **int32** |  | [optional] 
**Depth** | Pointer to **int32** |  | [optional] 
**Email** | Pointer to **bool** |  | [optional] 
**MaxTime** | Pointer to **int32** |  | [optional] 
**Proxies** | Pointer to **[]string** |  | [optional] 

## Methods

### NewCreateScrapingJobRequest

`func NewCreateScrapingJobRequest(userId string, orgId string, tenantId string, ) *CreateScrapingJobRequest`

NewCreateScrapingJobRequest instantiates a new CreateScrapingJobRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateScrapingJobRequestWithDefaults

`func NewCreateScrapingJobRequestWithDefaults() *CreateScrapingJobRequest`

NewCreateScrapingJobRequestWithDefaults instantiates a new CreateScrapingJobRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetUserId

`func (o *CreateScrapingJobRequest) GetUserId() string`

GetUserId returns the UserId field if non-nil, zero value otherwise.

### GetUserIdOk

`func (o *CreateScrapingJobRequest) GetUserIdOk() (*string, bool)`

GetUserIdOk returns a tuple with the UserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserId

`func (o *CreateScrapingJobRequest) SetUserId(v string)`

SetUserId sets UserId field to given value.


### GetOrgId

`func (o *CreateScrapingJobRequest) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *CreateScrapingJobRequest) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *CreateScrapingJobRequest) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetTenantId

`func (o *CreateScrapingJobRequest) GetTenantId() string`

GetTenantId returns the TenantId field if non-nil, zero value otherwise.

### GetTenantIdOk

`func (o *CreateScrapingJobRequest) GetTenantIdOk() (*string, bool)`

GetTenantIdOk returns a tuple with the TenantId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTenantId

`func (o *CreateScrapingJobRequest) SetTenantId(v string)`

SetTenantId sets TenantId field to given value.


### GetName

`func (o *CreateScrapingJobRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreateScrapingJobRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreateScrapingJobRequest) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *CreateScrapingJobRequest) HasName() bool`

HasName returns a boolean if a field has been set.

### GetKeywords

`func (o *CreateScrapingJobRequest) GetKeywords() []string`

GetKeywords returns the Keywords field if non-nil, zero value otherwise.

### GetKeywordsOk

`func (o *CreateScrapingJobRequest) GetKeywordsOk() (*[]string, bool)`

GetKeywordsOk returns a tuple with the Keywords field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKeywords

`func (o *CreateScrapingJobRequest) SetKeywords(v []string)`

SetKeywords sets Keywords field to given value.

### HasKeywords

`func (o *CreateScrapingJobRequest) HasKeywords() bool`

HasKeywords returns a boolean if a field has been set.

### GetLang

`func (o *CreateScrapingJobRequest) GetLang() string`

GetLang returns the Lang field if non-nil, zero value otherwise.

### GetLangOk

`func (o *CreateScrapingJobRequest) GetLangOk() (*string, bool)`

GetLangOk returns a tuple with the Lang field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLang

`func (o *CreateScrapingJobRequest) SetLang(v string)`

SetLang sets Lang field to given value.

### HasLang

`func (o *CreateScrapingJobRequest) HasLang() bool`

HasLang returns a boolean if a field has been set.

### GetZoom

`func (o *CreateScrapingJobRequest) GetZoom() int32`

GetZoom returns the Zoom field if non-nil, zero value otherwise.

### GetZoomOk

`func (o *CreateScrapingJobRequest) GetZoomOk() (*int32, bool)`

GetZoomOk returns a tuple with the Zoom field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZoom

`func (o *CreateScrapingJobRequest) SetZoom(v int32)`

SetZoom sets Zoom field to given value.

### HasZoom

`func (o *CreateScrapingJobRequest) HasZoom() bool`

HasZoom returns a boolean if a field has been set.

### GetLat

`func (o *CreateScrapingJobRequest) GetLat() string`

GetLat returns the Lat field if non-nil, zero value otherwise.

### GetLatOk

`func (o *CreateScrapingJobRequest) GetLatOk() (*string, bool)`

GetLatOk returns a tuple with the Lat field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLat

`func (o *CreateScrapingJobRequest) SetLat(v string)`

SetLat sets Lat field to given value.

### HasLat

`func (o *CreateScrapingJobRequest) HasLat() bool`

HasLat returns a boolean if a field has been set.

### GetLon

`func (o *CreateScrapingJobRequest) GetLon() string`

GetLon returns the Lon field if non-nil, zero value otherwise.

### GetLonOk

`func (o *CreateScrapingJobRequest) GetLonOk() (*string, bool)`

GetLonOk returns a tuple with the Lon field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLon

`func (o *CreateScrapingJobRequest) SetLon(v string)`

SetLon sets Lon field to given value.

### HasLon

`func (o *CreateScrapingJobRequest) HasLon() bool`

HasLon returns a boolean if a field has been set.

### GetFastMode

`func (o *CreateScrapingJobRequest) GetFastMode() bool`

GetFastMode returns the FastMode field if non-nil, zero value otherwise.

### GetFastModeOk

`func (o *CreateScrapingJobRequest) GetFastModeOk() (*bool, bool)`

GetFastModeOk returns a tuple with the FastMode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFastMode

`func (o *CreateScrapingJobRequest) SetFastMode(v bool)`

SetFastMode sets FastMode field to given value.

### HasFastMode

`func (o *CreateScrapingJobRequest) HasFastMode() bool`

HasFastMode returns a boolean if a field has been set.

### GetRadius

`func (o *CreateScrapingJobRequest) GetRadius() int32`

GetRadius returns the Radius field if non-nil, zero value otherwise.

### GetRadiusOk

`func (o *CreateScrapingJobRequest) GetRadiusOk() (*int32, bool)`

GetRadiusOk returns a tuple with the Radius field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRadius

`func (o *CreateScrapingJobRequest) SetRadius(v int32)`

SetRadius sets Radius field to given value.

### HasRadius

`func (o *CreateScrapingJobRequest) HasRadius() bool`

HasRadius returns a boolean if a field has been set.

### GetDepth

`func (o *CreateScrapingJobRequest) GetDepth() int32`

GetDepth returns the Depth field if non-nil, zero value otherwise.

### GetDepthOk

`func (o *CreateScrapingJobRequest) GetDepthOk() (*int32, bool)`

GetDepthOk returns a tuple with the Depth field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDepth

`func (o *CreateScrapingJobRequest) SetDepth(v int32)`

SetDepth sets Depth field to given value.

### HasDepth

`func (o *CreateScrapingJobRequest) HasDepth() bool`

HasDepth returns a boolean if a field has been set.

### GetEmail

`func (o *CreateScrapingJobRequest) GetEmail() bool`

GetEmail returns the Email field if non-nil, zero value otherwise.

### GetEmailOk

`func (o *CreateScrapingJobRequest) GetEmailOk() (*bool, bool)`

GetEmailOk returns a tuple with the Email field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmail

`func (o *CreateScrapingJobRequest) SetEmail(v bool)`

SetEmail sets Email field to given value.

### HasEmail

`func (o *CreateScrapingJobRequest) HasEmail() bool`

HasEmail returns a boolean if a field has been set.

### GetMaxTime

`func (o *CreateScrapingJobRequest) GetMaxTime() int32`

GetMaxTime returns the MaxTime field if non-nil, zero value otherwise.

### GetMaxTimeOk

`func (o *CreateScrapingJobRequest) GetMaxTimeOk() (*int32, bool)`

GetMaxTimeOk returns a tuple with the MaxTime field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxTime

`func (o *CreateScrapingJobRequest) SetMaxTime(v int32)`

SetMaxTime sets MaxTime field to given value.

### HasMaxTime

`func (o *CreateScrapingJobRequest) HasMaxTime() bool`

HasMaxTime returns a boolean if a field has been set.

### GetProxies

`func (o *CreateScrapingJobRequest) GetProxies() []string`

GetProxies returns the Proxies field if non-nil, zero value otherwise.

### GetProxiesOk

`func (o *CreateScrapingJobRequest) GetProxiesOk() (*[]string, bool)`

GetProxiesOk returns a tuple with the Proxies field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProxies

`func (o *CreateScrapingJobRequest) SetProxies(v []string)`

SetProxies sets Proxies field to given value.

### HasProxies

`func (o *CreateScrapingJobRequest) HasProxies() bool`

HasProxies returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


