# ScrapingJob

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Id** | Pointer to **string** |  | [optional] 
**Name** | Pointer to **string** |  | [optional] 
**CreatedAt** | Pointer to **time.Time** |  | [optional] 
**Status** | Pointer to [**BackgroundJobStatus**](BackgroundJobStatus.md) |  | [optional] [default to UNSPECIFIED]
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
**UpdatedAt** | Pointer to **time.Time** |  | [optional] 
**DeletedAt** | Pointer to **time.Time** |  | [optional] 
**PayloadType** | Pointer to **string** |  | [optional] 
**Priority** | Pointer to **int32** |  | [optional] 
**Payload** | Pointer to **string** |  | [optional] 

## Methods

### NewScrapingJob

`func NewScrapingJob() *ScrapingJob`

NewScrapingJob instantiates a new ScrapingJob object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewScrapingJobWithDefaults

`func NewScrapingJobWithDefaults() *ScrapingJob`

NewScrapingJobWithDefaults instantiates a new ScrapingJob object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetId

`func (o *ScrapingJob) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *ScrapingJob) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *ScrapingJob) SetId(v string)`

SetId sets Id field to given value.

### HasId

`func (o *ScrapingJob) HasId() bool`

HasId returns a boolean if a field has been set.

### GetName

`func (o *ScrapingJob) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ScrapingJob) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ScrapingJob) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *ScrapingJob) HasName() bool`

HasName returns a boolean if a field has been set.

### GetCreatedAt

`func (o *ScrapingJob) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *ScrapingJob) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *ScrapingJob) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *ScrapingJob) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### GetStatus

`func (o *ScrapingJob) GetStatus() BackgroundJobStatus`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *ScrapingJob) GetStatusOk() (*BackgroundJobStatus, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *ScrapingJob) SetStatus(v BackgroundJobStatus)`

SetStatus sets Status field to given value.

### HasStatus

`func (o *ScrapingJob) HasStatus() bool`

HasStatus returns a boolean if a field has been set.

### GetKeywords

`func (o *ScrapingJob) GetKeywords() []string`

GetKeywords returns the Keywords field if non-nil, zero value otherwise.

### GetKeywordsOk

`func (o *ScrapingJob) GetKeywordsOk() (*[]string, bool)`

GetKeywordsOk returns a tuple with the Keywords field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKeywords

`func (o *ScrapingJob) SetKeywords(v []string)`

SetKeywords sets Keywords field to given value.

### HasKeywords

`func (o *ScrapingJob) HasKeywords() bool`

HasKeywords returns a boolean if a field has been set.

### GetLang

`func (o *ScrapingJob) GetLang() string`

GetLang returns the Lang field if non-nil, zero value otherwise.

### GetLangOk

`func (o *ScrapingJob) GetLangOk() (*string, bool)`

GetLangOk returns a tuple with the Lang field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLang

`func (o *ScrapingJob) SetLang(v string)`

SetLang sets Lang field to given value.

### HasLang

`func (o *ScrapingJob) HasLang() bool`

HasLang returns a boolean if a field has been set.

### GetZoom

`func (o *ScrapingJob) GetZoom() int32`

GetZoom returns the Zoom field if non-nil, zero value otherwise.

### GetZoomOk

`func (o *ScrapingJob) GetZoomOk() (*int32, bool)`

GetZoomOk returns a tuple with the Zoom field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZoom

`func (o *ScrapingJob) SetZoom(v int32)`

SetZoom sets Zoom field to given value.

### HasZoom

`func (o *ScrapingJob) HasZoom() bool`

HasZoom returns a boolean if a field has been set.

### GetLat

`func (o *ScrapingJob) GetLat() string`

GetLat returns the Lat field if non-nil, zero value otherwise.

### GetLatOk

`func (o *ScrapingJob) GetLatOk() (*string, bool)`

GetLatOk returns a tuple with the Lat field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLat

`func (o *ScrapingJob) SetLat(v string)`

SetLat sets Lat field to given value.

### HasLat

`func (o *ScrapingJob) HasLat() bool`

HasLat returns a boolean if a field has been set.

### GetLon

`func (o *ScrapingJob) GetLon() string`

GetLon returns the Lon field if non-nil, zero value otherwise.

### GetLonOk

`func (o *ScrapingJob) GetLonOk() (*string, bool)`

GetLonOk returns a tuple with the Lon field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLon

`func (o *ScrapingJob) SetLon(v string)`

SetLon sets Lon field to given value.

### HasLon

`func (o *ScrapingJob) HasLon() bool`

HasLon returns a boolean if a field has been set.

### GetFastMode

`func (o *ScrapingJob) GetFastMode() bool`

GetFastMode returns the FastMode field if non-nil, zero value otherwise.

### GetFastModeOk

`func (o *ScrapingJob) GetFastModeOk() (*bool, bool)`

GetFastModeOk returns a tuple with the FastMode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFastMode

`func (o *ScrapingJob) SetFastMode(v bool)`

SetFastMode sets FastMode field to given value.

### HasFastMode

`func (o *ScrapingJob) HasFastMode() bool`

HasFastMode returns a boolean if a field has been set.

### GetRadius

`func (o *ScrapingJob) GetRadius() int32`

GetRadius returns the Radius field if non-nil, zero value otherwise.

### GetRadiusOk

`func (o *ScrapingJob) GetRadiusOk() (*int32, bool)`

GetRadiusOk returns a tuple with the Radius field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRadius

`func (o *ScrapingJob) SetRadius(v int32)`

SetRadius sets Radius field to given value.

### HasRadius

`func (o *ScrapingJob) HasRadius() bool`

HasRadius returns a boolean if a field has been set.

### GetDepth

`func (o *ScrapingJob) GetDepth() int32`

GetDepth returns the Depth field if non-nil, zero value otherwise.

### GetDepthOk

`func (o *ScrapingJob) GetDepthOk() (*int32, bool)`

GetDepthOk returns a tuple with the Depth field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDepth

`func (o *ScrapingJob) SetDepth(v int32)`

SetDepth sets Depth field to given value.

### HasDepth

`func (o *ScrapingJob) HasDepth() bool`

HasDepth returns a boolean if a field has been set.

### GetEmail

`func (o *ScrapingJob) GetEmail() bool`

GetEmail returns the Email field if non-nil, zero value otherwise.

### GetEmailOk

`func (o *ScrapingJob) GetEmailOk() (*bool, bool)`

GetEmailOk returns a tuple with the Email field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmail

`func (o *ScrapingJob) SetEmail(v bool)`

SetEmail sets Email field to given value.

### HasEmail

`func (o *ScrapingJob) HasEmail() bool`

HasEmail returns a boolean if a field has been set.

### GetMaxTime

`func (o *ScrapingJob) GetMaxTime() int32`

GetMaxTime returns the MaxTime field if non-nil, zero value otherwise.

### GetMaxTimeOk

`func (o *ScrapingJob) GetMaxTimeOk() (*int32, bool)`

GetMaxTimeOk returns a tuple with the MaxTime field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxTime

`func (o *ScrapingJob) SetMaxTime(v int32)`

SetMaxTime sets MaxTime field to given value.

### HasMaxTime

`func (o *ScrapingJob) HasMaxTime() bool`

HasMaxTime returns a boolean if a field has been set.

### GetProxies

`func (o *ScrapingJob) GetProxies() []string`

GetProxies returns the Proxies field if non-nil, zero value otherwise.

### GetProxiesOk

`func (o *ScrapingJob) GetProxiesOk() (*[]string, bool)`

GetProxiesOk returns a tuple with the Proxies field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProxies

`func (o *ScrapingJob) SetProxies(v []string)`

SetProxies sets Proxies field to given value.

### HasProxies

`func (o *ScrapingJob) HasProxies() bool`

HasProxies returns a boolean if a field has been set.

### GetUpdatedAt

`func (o *ScrapingJob) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *ScrapingJob) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *ScrapingJob) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *ScrapingJob) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### GetDeletedAt

`func (o *ScrapingJob) GetDeletedAt() time.Time`

GetDeletedAt returns the DeletedAt field if non-nil, zero value otherwise.

### GetDeletedAtOk

`func (o *ScrapingJob) GetDeletedAtOk() (*time.Time, bool)`

GetDeletedAtOk returns a tuple with the DeletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeletedAt

`func (o *ScrapingJob) SetDeletedAt(v time.Time)`

SetDeletedAt sets DeletedAt field to given value.

### HasDeletedAt

`func (o *ScrapingJob) HasDeletedAt() bool`

HasDeletedAt returns a boolean if a field has been set.

### GetPayloadType

`func (o *ScrapingJob) GetPayloadType() string`

GetPayloadType returns the PayloadType field if non-nil, zero value otherwise.

### GetPayloadTypeOk

`func (o *ScrapingJob) GetPayloadTypeOk() (*string, bool)`

GetPayloadTypeOk returns a tuple with the PayloadType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPayloadType

`func (o *ScrapingJob) SetPayloadType(v string)`

SetPayloadType sets PayloadType field to given value.

### HasPayloadType

`func (o *ScrapingJob) HasPayloadType() bool`

HasPayloadType returns a boolean if a field has been set.

### GetPriority

`func (o *ScrapingJob) GetPriority() int32`

GetPriority returns the Priority field if non-nil, zero value otherwise.

### GetPriorityOk

`func (o *ScrapingJob) GetPriorityOk() (*int32, bool)`

GetPriorityOk returns a tuple with the Priority field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPriority

`func (o *ScrapingJob) SetPriority(v int32)`

SetPriority sets Priority field to given value.

### HasPriority

`func (o *ScrapingJob) HasPriority() bool`

HasPriority returns a boolean if a field has been set.

### GetPayload

`func (o *ScrapingJob) GetPayload() string`

GetPayload returns the Payload field if non-nil, zero value otherwise.

### GetPayloadOk

`func (o *ScrapingJob) GetPayloadOk() (*string, bool)`

GetPayloadOk returns a tuple with the Payload field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPayload

`func (o *ScrapingJob) SetPayload(v string)`

SetPayload sets Payload field to given value.

### HasPayload

`func (o *ScrapingJob) HasPayload() bool`

HasPayload returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


