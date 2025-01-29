# ValidationErrorMessageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Code** | Pointer to [**ValidationErrorCode**](ValidationErrorCode.md) |  | [optional] [default to NO_ERROR]
**Message** | Pointer to **string** |  | [optional] 
**FieldViolations** | Pointer to [**[]FieldViolation**](FieldViolation.md) |  | [optional] 
**SchemaValidation** | Pointer to [**SchemaValidation**](SchemaValidation.md) |  | [optional] 
**ResourceValidation** | Pointer to [**ResourceValidation**](ResourceValidation.md) |  | [optional] 
**ErrorResponse** | Pointer to [**ErrorResponse**](ErrorResponse.md) |  | [optional] 

## Methods

### NewValidationErrorMessageResponse

`func NewValidationErrorMessageResponse() *ValidationErrorMessageResponse`

NewValidationErrorMessageResponse instantiates a new ValidationErrorMessageResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewValidationErrorMessageResponseWithDefaults

`func NewValidationErrorMessageResponseWithDefaults() *ValidationErrorMessageResponse`

NewValidationErrorMessageResponseWithDefaults instantiates a new ValidationErrorMessageResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCode

`func (o *ValidationErrorMessageResponse) GetCode() ValidationErrorCode`

GetCode returns the Code field if non-nil, zero value otherwise.

### GetCodeOk

`func (o *ValidationErrorMessageResponse) GetCodeOk() (*ValidationErrorCode, bool)`

GetCodeOk returns a tuple with the Code field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCode

`func (o *ValidationErrorMessageResponse) SetCode(v ValidationErrorCode)`

SetCode sets Code field to given value.

### HasCode

`func (o *ValidationErrorMessageResponse) HasCode() bool`

HasCode returns a boolean if a field has been set.

### GetMessage

`func (o *ValidationErrorMessageResponse) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *ValidationErrorMessageResponse) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *ValidationErrorMessageResponse) SetMessage(v string)`

SetMessage sets Message field to given value.

### HasMessage

`func (o *ValidationErrorMessageResponse) HasMessage() bool`

HasMessage returns a boolean if a field has been set.

### GetFieldViolations

`func (o *ValidationErrorMessageResponse) GetFieldViolations() []FieldViolation`

GetFieldViolations returns the FieldViolations field if non-nil, zero value otherwise.

### GetFieldViolationsOk

`func (o *ValidationErrorMessageResponse) GetFieldViolationsOk() (*[]FieldViolation, bool)`

GetFieldViolationsOk returns a tuple with the FieldViolations field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFieldViolations

`func (o *ValidationErrorMessageResponse) SetFieldViolations(v []FieldViolation)`

SetFieldViolations sets FieldViolations field to given value.

### HasFieldViolations

`func (o *ValidationErrorMessageResponse) HasFieldViolations() bool`

HasFieldViolations returns a boolean if a field has been set.

### GetSchemaValidation

`func (o *ValidationErrorMessageResponse) GetSchemaValidation() SchemaValidation`

GetSchemaValidation returns the SchemaValidation field if non-nil, zero value otherwise.

### GetSchemaValidationOk

`func (o *ValidationErrorMessageResponse) GetSchemaValidationOk() (*SchemaValidation, bool)`

GetSchemaValidationOk returns a tuple with the SchemaValidation field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSchemaValidation

`func (o *ValidationErrorMessageResponse) SetSchemaValidation(v SchemaValidation)`

SetSchemaValidation sets SchemaValidation field to given value.

### HasSchemaValidation

`func (o *ValidationErrorMessageResponse) HasSchemaValidation() bool`

HasSchemaValidation returns a boolean if a field has been set.

### GetResourceValidation

`func (o *ValidationErrorMessageResponse) GetResourceValidation() ResourceValidation`

GetResourceValidation returns the ResourceValidation field if non-nil, zero value otherwise.

### GetResourceValidationOk

`func (o *ValidationErrorMessageResponse) GetResourceValidationOk() (*ResourceValidation, bool)`

GetResourceValidationOk returns a tuple with the ResourceValidation field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResourceValidation

`func (o *ValidationErrorMessageResponse) SetResourceValidation(v ResourceValidation)`

SetResourceValidation sets ResourceValidation field to given value.

### HasResourceValidation

`func (o *ValidationErrorMessageResponse) HasResourceValidation() bool`

HasResourceValidation returns a boolean if a field has been set.

### GetErrorResponse

`func (o *ValidationErrorMessageResponse) GetErrorResponse() ErrorResponse`

GetErrorResponse returns the ErrorResponse field if non-nil, zero value otherwise.

### GetErrorResponseOk

`func (o *ValidationErrorMessageResponse) GetErrorResponseOk() (*ErrorResponse, bool)`

GetErrorResponseOk returns a tuple with the ErrorResponse field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetErrorResponse

`func (o *ValidationErrorMessageResponse) SetErrorResponse(v ErrorResponse)`

SetErrorResponse sets ErrorResponse field to given value.

### HasErrorResponse

`func (o *ValidationErrorMessageResponse) HasErrorResponse() bool`

HasErrorResponse returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


