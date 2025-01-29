# CreateScrapingJobResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**JobId** | Pointer to **string** |  | [optional] 
**Status** | Pointer to [**BackgroundJobStatus**](BackgroundJobStatus.md) |  | [optional] [default to UNSPECIFIED]

## Methods

### NewCreateScrapingJobResponse

`func NewCreateScrapingJobResponse() *CreateScrapingJobResponse`

NewCreateScrapingJobResponse instantiates a new CreateScrapingJobResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateScrapingJobResponseWithDefaults

`func NewCreateScrapingJobResponseWithDefaults() *CreateScrapingJobResponse`

NewCreateScrapingJobResponseWithDefaults instantiates a new CreateScrapingJobResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetJobId

`func (o *CreateScrapingJobResponse) GetJobId() string`

GetJobId returns the JobId field if non-nil, zero value otherwise.

### GetJobIdOk

`func (o *CreateScrapingJobResponse) GetJobIdOk() (*string, bool)`

GetJobIdOk returns a tuple with the JobId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetJobId

`func (o *CreateScrapingJobResponse) SetJobId(v string)`

SetJobId sets JobId field to given value.

### HasJobId

`func (o *CreateScrapingJobResponse) HasJobId() bool`

HasJobId returns a boolean if a field has been set.

### GetStatus

`func (o *CreateScrapingJobResponse) GetStatus() BackgroundJobStatus`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *CreateScrapingJobResponse) GetStatusOk() (*BackgroundJobStatus, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *CreateScrapingJobResponse) SetStatus(v BackgroundJobStatus)`

SetStatus sets Status field to given value.

### HasStatus

`func (o *CreateScrapingJobResponse) HasStatus() bool`

HasStatus returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


