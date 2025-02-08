#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, unused_attributes, non_camel_case_types)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::disallowed_names)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::collections::BTreeSet;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};
use crate::server::Authorization;


type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &str = "";
pub const API_VERSION: &str = "1.0";

mod auth;
pub use auth::{AuthenticationApi, Claims};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CreateApiKeyResponse {
    /// API key created successfully
    APIKeyCreatedSuccessfully
    (models::CreateApiKeyResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CreateOrganizationResponse {
    /// A successful response.
    ASuccessfulResponse
    (models::CreateOrganizationResponse)
    ,
    /// Organization created successfully
    OrganizationCreatedSuccessfully
    (models::CreateOrganizationResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CreateTenantResponse {
    /// A successful response.
    ASuccessfulResponse
    (models::CreateTenantResponse)
    ,
    /// Tenant created successfully
    TenantCreatedSuccessfully
    (models::CreateTenantResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CreateTenantApiKeyResponse {
    /// Organizations retrieved successfully
    OrganizationsRetrievedSuccessfully
    (models::CreateTenantApiKeyResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CreateWebhookResponse {
    /// Webhook created successfully
    WebhookCreatedSuccessfully
    (models::CreateWebhookResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DeleteApiKeyResponse {
    /// API key deleted successfully
    APIKeyDeletedSuccessfully
    (models::DeleteApiKeyResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DeleteOrganizationResponse {
    /// Organization deleted successfully
    OrganizationDeletedSuccessfully
    (models::DeleteOrganizationResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DeleteTenantResponse {
    /// Tenant deleted successfully
    TenantDeletedSuccessfully
    (models::DeleteTenantResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DeleteTenantApiKeyResponse {
    /// Tenant API key deleted successfully
    TenantAPIKeyDeletedSuccessfully
    (models::DeleteTenantApiKeyResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DeleteWebhookResponse {
    /// Webhook deleted successfully
    WebhookDeletedSuccessfully
    (models::DeleteWebhookResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DeleteWorkspaceResponse {
    /// Workspace deleted successfully
    WorkspaceDeletedSuccessfully
    (models::DeleteWorkspaceResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetApiKeyResponse {
    /// API key retrieved successfully
    APIKeyRetrievedSuccessfully
    (models::GetApiKeyResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetLeadResponse {
    /// Lead retrieved successfully
    LeadRetrievedSuccessfully
    (models::GetLeadResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetOrganizationResponse {
    /// Organization retrieved successfully
    OrganizationRetrievedSuccessfully
    (models::GetOrganizationResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetTenantResponse {
    /// Tenant retrieved successfully
    TenantRetrievedSuccessfully
    (models::GetTenantResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetTenantApiKeyResponse {
    /// Tenant API key retrieved successfully
    TenantAPIKeyRetrievedSuccessfully
    (models::GetTenantApiKeyResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetWebhookResponse {
    /// Webhook retrieved successfully
    WebhookRetrievedSuccessfully
    (models::GetWebhookResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ListApiKeysResponse {
    /// API keys retrieved successfully
    APIKeysRetrievedSuccessfully
    (models::ListApiKeysResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ListLeadsResponse {
    /// Leads retrieved successfully
    LeadsRetrievedSuccessfully
    (models::ListLeadsResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ListOrganizationsResponse {
    /// Organizations retrieved successfully
    OrganizationsRetrievedSuccessfully
    (models::ListOrganizationsResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ListTenantApiKeysResponse {
    /// Tenant API keys retrieved successfully
    TenantAPIKeysRetrievedSuccessfully
    (models::ListTenantApiKeysResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ListTenantsResponse {
    /// Tenants retrieved successfully
    TenantsRetrievedSuccessfully
    (models::ListTenantsResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ListWebhooksResponse {
    /// Webhooks retrieved successfully
    WebhooksRetrievedSuccessfully
    (models::ListWebhooksResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum RotateApiKeyResponse {
    /// API key rotated successfully
    APIKeyRotatedSuccessfully
    (models::RotateApiKeyResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum RotateTenantApiKeyResponse {
    /// Tenant API key rotated successfully
    TenantAPIKeyRotatedSuccessfully
    (models::RotateTenantApiKeyResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum UpdateApiKeyResponse {
    /// API key updated successfully
    APIKeyUpdatedSuccessfully
    (models::UpdateApiKeyResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum UpdateOrganizationResponse {
    /// Organization updated successfully
    OrganizationUpdatedSuccessfully
    (models::UpdateOrganizationResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum UpdateTenantResponse {
    /// Tenant updated successfully
    TenantUpdatedSuccessfully
    (models::UpdateTenantResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum UpdateTenantApiKeyResponse {
    /// Tenant API key updated successfully
    TenantAPIKeyUpdatedSuccessfully
    (models::UpdateTenantApiKeyResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum UpdateWebhookResponse {
    /// Webhook updated successfully
    WebhookUpdatedSuccessfully
    (models::UpdateWebhookResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum UpdateWorkspaceResponse {
    /// Workspace updated successfully
    WorkspaceUpdatedSuccessfully
    (models::UpdateWorkspaceResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse)
    ,
    /// Payment Required - Payment is necessary to proceed
    PaymentRequired
    (models::PaymentRequiredErrorMessageResponse)
    ,
    /// Forbidden - Access denied
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Method Not Allowed - HTTP method not supported
    MethodNotAllowed
    (models::MethodNotAllowedErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Gone - Resource is no longer available
    Gone
    (models::GoneErrorMessageResponse)
    ,
    /// Precondition Failed - Preconditions in headers did not match
    PreconditionFailed
    (models::PreconditionFailedErrorMessageResponse)
    ,
    /// Unprocessable Entity - Semantic errors in the request
    UnprocessableEntity
    (models::UnprocessableEntityErrorMessageResponse)
    ,
    /// Too Early - Request is being replayed
    TooEarly
    (models::TooEarlyErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// Not Implemented - Functionality not supported
    NotImplemented
    (models::NotImplementedErrorMessageResponse)
    ,
    /// Bad Gateway - Invalid response from upstream server
    BadGateway
    (models::BadGatewayErrorMessageResponse)
    ,
    /// Service Unavailable - Try again later
    ServiceUnavailable
    (models::ServiceUnavailableErrorMessageResponse)
    ,
    /// Gateway Timeout - Upstream server timed out
    GatewayTimeout
    (models::GatewayTimeoutErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::RpcPeriodStatus)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CreateAccountResponse {
    /// A successful response.
    ASuccessfulResponse
    (models::CreateAccountResponse)
    ,
    /// Account created successfully
    AccountCreatedSuccessfully
    (models::CreateAccountResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse1)
    ,
    /// Forbidden - Insufficient permissions
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::Status)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CreateWorkspaceResponse {
    /// A successful response.
    ASuccessfulResponse
    (models::CreateWorkspaceResponse)
    ,
    /// Workspace created successfully
    WorkspaceCreatedSuccessfully
    (models::CreateWorkspaceResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse1)
    ,
    /// Forbidden - Insufficient permissions
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::Status)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DeleteAccountResponse {
    /// Account deleted successfully
    AccountDeletedSuccessfully
    (models::DeleteAccountResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse1)
    ,
    /// Forbidden - Insufficient permissions
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::Status)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DeleteWorkspace1Response {
    /// Workspace deleted successfully
    WorkspaceDeletedSuccessfully
    (models::DeleteWorkspaceResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse1)
    ,
    /// Forbidden - Insufficient permissions
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::Status)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetAccountResponse {
    /// Account details retrieved successfully
    AccountDetailsRetrievedSuccessfully
    (models::GetAccountResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse1)
    ,
    /// Forbidden - Insufficient permissions
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::Status)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetWorkspaceResponse {
    /// Workspace retrieved successfully
    WorkspaceRetrievedSuccessfully
    (models::GetWorkspaceResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse1)
    ,
    /// Forbidden - Insufficient permissions
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::Status)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetWorkspaceAnalyticsResponse {
    /// Analytics data retrieved successfully
    AnalyticsDataRetrievedSuccessfully
    (models::GetWorkspaceAnalyticsResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse1)
    ,
    /// Forbidden - Insufficient permissions
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::Status)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetWorkspaceComplianceReportResponse {
    /// Compliance report retrieved successfully
    ComplianceReportRetrievedSuccessfully
    (models::GetWorkspaceComplianceReportResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse1)
    ,
    /// Forbidden - Insufficient permissions
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::Status)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetWorkspaceStorageStatsResponse {
    /// Storage statistics retrieved successfully
    StorageStatisticsRetrievedSuccessfully
    (models::GetWorkspaceStorageStatsResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse1)
    ,
    /// Forbidden - Insufficient permissions
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::Status)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ListWorkspaceSharingsResponse {
    /// Workspace sharings retrieved successfully
    WorkspaceSharingsRetrievedSuccessfully
    (models::ListWorkspaceSharingsResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse1)
    ,
    /// Forbidden - Insufficient permissions
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::Status)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ListWorkspacesResponse {
    /// Workspaces retrieved successfully
    WorkspacesRetrievedSuccessfully
    (models::ListWorkspacesResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse1)
    ,
    /// Forbidden - Insufficient permissions
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::Status)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum RemoveWorkspaceSharingResponse {
    /// Workspace sharing removed successfully
    WorkspaceSharingRemovedSuccessfully
    (models::RemoveWorkspaceSharingResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse1)
    ,
    /// Forbidden - Insufficient permissions
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::Status)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ShareWorkspaceResponse {
    /// Workspace shared successfully
    WorkspaceSharedSuccessfully
    (models::ShareWorkspaceResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse1)
    ,
    /// Forbidden - Insufficient permissions
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::Status)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum UpdateAccountResponse {
    /// Account updated successfully
    AccountUpdatedSuccessfully
    (models::UpdateAccountResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse1)
    ,
    /// Forbidden - Insufficient permissions
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::Status)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum UpdateWorkspace1Response {
    /// Workspace updated successfully
    WorkspaceUpdatedSuccessfully
    (models::UpdateWorkspaceResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse1)
    ,
    /// Forbidden - Insufficient permissions
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::Status)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum UpdateWorkspaceSharingResponse {
    /// Workspace sharing updated successfully
    WorkspaceSharingUpdatedSuccessfully
    (models::UpdateWorkspaceSharingResponse)
    ,
    /// Bad Request - Invalid input parameters
    BadRequest
    (models::ValidationErrorMessageResponse)
    ,
    /// Unauthorized - Authentication required
    Unauthorized
    (models::AuthenticationErrorMessageResponse1)
    ,
    /// Forbidden - Insufficient permissions
    Forbidden
    (models::ForbiddenErrorMessageResponse)
    ,
    /// Not Found - Resource not found
    NotFound
    (models::NotFoundErrorMessageResponse)
    ,
    /// Conflict - Resource already exists
    Conflict
    (models::ConflictErrorMessageResponse)
    ,
    /// Too Many Requests - Rate limit exceeded
    TooManyRequests
    (models::RateLimitErrorMessageResponse)
    ,
    /// Internal Server Error
    InternalServerError
    (models::InternalErrorMessageResponse)
    ,
    /// An unexpected error response.
    AnUnexpectedErrorResponse
    (models::Status)
}

/// API
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Create a new API key
    async fn create_api_key(
        &self,
        create_api_key_request: models::CreateApiKeyRequest,
        context: &C) -> Result<CreateApiKeyResponse, ApiError>;

    /// Create a new organization
    async fn create_organization(
        &self,
        create_organization_request: models::CreateOrganizationRequest,
        context: &C) -> Result<CreateOrganizationResponse, ApiError>;

    /// Create a new tenant
    async fn create_tenant(
        &self,
        organization_id: String,
        create_tenant_body: models::CreateTenantBody,
        context: &C) -> Result<CreateTenantResponse, ApiError>;

    /// Create a new tenant API key
    async fn create_tenant_api_key(
        &self,
        create_tenant_api_key_request: models::CreateTenantApiKeyRequest,
        context: &C) -> Result<CreateTenantApiKeyResponse, ApiError>;

    /// Create webhook
    async fn create_webhook(
        &self,
        create_webhook_request: models::CreateWebhookRequest,
        context: &C) -> Result<CreateWebhookResponse, ApiError>;

    /// Delete API key
    async fn delete_api_key(
        &self,
        key_id: String,
        organization_id: Option<String>,
        workspace_id: Option<String>,
        tenant_id: Option<String>,
        account_id: Option<String>,
        context: &C) -> Result<DeleteApiKeyResponse, ApiError>;

    /// Delete an organization
    async fn delete_organization(
        &self,
        id: String,
        context: &C) -> Result<DeleteOrganizationResponse, ApiError>;

    /// Delete a tenant
    async fn delete_tenant(
        &self,
        organization_id: String,
        tenant_id: String,
        context: &C) -> Result<DeleteTenantResponse, ApiError>;

    /// Delete tenant API key
    async fn delete_tenant_api_key(
        &self,
        key_id: String,
        organization_id: Option<String>,
        tenant_id: Option<String>,
        context: &C) -> Result<DeleteTenantApiKeyResponse, ApiError>;

    /// Delete webhook
    async fn delete_webhook(
        &self,
        webhook_id: String,
        organization_id: Option<String>,
        workspace_id: Option<String>,
        tenant_id: Option<String>,
        account_id: Option<String>,
        context: &C) -> Result<DeleteWebhookResponse, ApiError>;

    /// Delete a workspace
    async fn delete_workspace(
        &self,
        id: String,
        context: &C) -> Result<DeleteWorkspaceResponse, ApiError>;

    /// Get API key details
    async fn get_api_key(
        &self,
        key_id: String,
        organization_id: Option<String>,
        tenant_id: Option<String>,
        account_id: Option<String>,
        workspace_id: Option<String>,
        context: &C) -> Result<GetApiKeyResponse, ApiError>;

    /// Get lead details
    async fn get_lead(
        &self,
        lead_id: String,
        organization_id: Option<String>,
        workspace_id: Option<String>,
        tenant_id: Option<String>,
        account_id: Option<String>,
        context: &C) -> Result<GetLeadResponse, ApiError>;

    /// Get organization details
    async fn get_organization(
        &self,
        id: String,
        context: &C) -> Result<GetOrganizationResponse, ApiError>;

    /// Get tenant details
    async fn get_tenant(
        &self,
        organization_id: String,
        tenant_id: String,
        context: &C) -> Result<GetTenantResponse, ApiError>;

    /// Get tenant API key details
    async fn get_tenant_api_key(
        &self,
        key_id: String,
        organization_id: Option<String>,
        tenant_id: Option<String>,
        context: &C) -> Result<GetTenantApiKeyResponse, ApiError>;

    /// Get webhook
    async fn get_webhook(
        &self,
        webhook_id: String,
        organization_id: Option<String>,
        workspace_id: Option<String>,
        tenant_id: Option<String>,
        account_id: Option<String>,
        context: &C) -> Result<GetWebhookResponse, ApiError>;

    /// List API keys
    async fn list_api_keys(
        &self,
        organization_id: Option<String>,
        tenant_id: Option<String>,
        account_id: Option<String>,
        workspace_id: Option<String>,
        page_size: Option<i32>,
        page_number: Option<i32>,
        status: Option<String>,
        search: Option<String>,
        context: &C) -> Result<ListApiKeysResponse, ApiError>;

    /// List leads
    async fn list_leads(
        &self,
        organization_id: Option<String>,
        workspace_id: Option<String>,
        tenant_id: Option<String>,
        account_id: Option<String>,
        page_size: Option<i32>,
        page_number: Option<i32>,
        context: &C) -> Result<ListLeadsResponse, ApiError>;

    /// List all organizations
    async fn list_organizations(
        &self,
        page_size: Option<i32>,
        page_number: Option<i32>,
        context: &C) -> Result<ListOrganizationsResponse, ApiError>;

    /// List tenant API keys
    async fn list_tenant_api_keys(
        &self,
        organization_id: Option<String>,
        tenant_id: Option<String>,
        page_size: Option<i32>,
        page_number: Option<i32>,
        status: Option<String>,
        context: &C) -> Result<ListTenantApiKeysResponse, ApiError>;

    /// List all tenants
    async fn list_tenants(
        &self,
        organization_id: String,
        page_size: Option<i32>,
        page_number: Option<i32>,
        context: &C) -> Result<ListTenantsResponse, ApiError>;

    /// List webhooks
    async fn list_webhooks(
        &self,
        organization_id: Option<String>,
        workspace_id: Option<String>,
        tenant_id: Option<String>,
        account_id: Option<String>,
        page_size: Option<i32>,
        page_number: Option<i32>,
        status: Option<String>,
        search: Option<String>,
        context: &C) -> Result<ListWebhooksResponse, ApiError>;

    /// Rotate API key
    async fn rotate_api_key(
        &self,
        rotate_api_key_request: models::RotateApiKeyRequest,
        context: &C) -> Result<RotateApiKeyResponse, ApiError>;

    /// Rotate tenant API key
    async fn rotate_tenant_api_key(
        &self,
        rotate_tenant_api_key_request: models::RotateTenantApiKeyRequest,
        context: &C) -> Result<RotateTenantApiKeyResponse, ApiError>;

    /// Update API key
    async fn update_api_key(
        &self,
        update_api_key_request: models::UpdateApiKeyRequest,
        context: &C) -> Result<UpdateApiKeyResponse, ApiError>;

    /// Update organization details
    async fn update_organization(
        &self,
        update_organization_request: models::UpdateOrganizationRequest,
        context: &C) -> Result<UpdateOrganizationResponse, ApiError>;

    /// Update tenant details
    async fn update_tenant(
        &self,
        update_tenant_request: models::UpdateTenantRequest,
        context: &C) -> Result<UpdateTenantResponse, ApiError>;

    /// Update tenant API key
    async fn update_tenant_api_key(
        &self,
        update_tenant_api_key_request: models::UpdateTenantApiKeyRequest,
        context: &C) -> Result<UpdateTenantApiKeyResponse, ApiError>;

    /// Update webhook
    async fn update_webhook(
        &self,
        update_webhook_request: models::UpdateWebhookRequest,
        context: &C) -> Result<UpdateWebhookResponse, ApiError>;

    /// Update workspace details
    async fn update_workspace(
        &self,
        update_workspace_request: models::UpdateWorkspaceRequest,
        context: &C) -> Result<UpdateWorkspaceResponse, ApiError>;

    /// Create a new account
    async fn create_account(
        &self,
        create_account_request: models::CreateAccountRequest,
        context: &C) -> Result<CreateAccountResponse, ApiError>;

    /// Create workspace
    async fn create_workspace(
        &self,
        create_workspace_request: models::CreateWorkspaceRequest,
        context: &C) -> Result<CreateWorkspaceResponse, ApiError>;

    /// Delete account
    async fn delete_account(
        &self,
        id: String,
        context: &C) -> Result<DeleteAccountResponse, ApiError>;

    /// Delete workspace
    async fn delete_workspace1(
        &self,
        id: String,
        context: &C) -> Result<DeleteWorkspace1Response, ApiError>;

    /// Get account details
    async fn get_account(
        &self,
        id: String,
        context: &C) -> Result<GetAccountResponse, ApiError>;

    /// Get workspace details
    async fn get_workspace(
        &self,
        id: String,
        context: &C) -> Result<GetWorkspaceResponse, ApiError>;

    /// Get workspace analytics
    async fn get_workspace_analytics(
        &self,
        workspace_id: String,
        start_time: Option<chrono::DateTime::<chrono::Utc>>,
        end_time: Option<chrono::DateTime::<chrono::Utc>>,
        context: &C) -> Result<GetWorkspaceAnalyticsResponse, ApiError>;

    /// Get compliance report
    async fn get_workspace_compliance_report(
        &self,
        workspace_id: String,
        compliance_type: Option<String>,
        context: &C) -> Result<GetWorkspaceComplianceReportResponse, ApiError>;

    /// Get storage statistics
    async fn get_workspace_storage_stats(
        &self,
        workspace_id: String,
        context: &C) -> Result<GetWorkspaceStorageStatsResponse, ApiError>;

    /// List workspace sharings
    async fn list_workspace_sharings(
        &self,
        workspace_id: String,
        page_size: Option<i32>,
        page_token: Option<String>,
        context: &C) -> Result<ListWorkspaceSharingsResponse, ApiError>;

    /// List workspaces
    async fn list_workspaces(
        &self,
        account_id: Option<String>,
        page_size: Option<i32>,
        page_token: Option<String>,
        filter: Option<String>,
        context: &C) -> Result<ListWorkspacesResponse, ApiError>;

    /// Remove workspace sharing
    async fn remove_workspace_sharing(
        &self,
        sharing_id: String,
        context: &C) -> Result<RemoveWorkspaceSharingResponse, ApiError>;

    /// Share workspace
    async fn share_workspace(
        &self,
        workspace_id: String,
        share_workspace_body: models::ShareWorkspaceBody,
        context: &C) -> Result<ShareWorkspaceResponse, ApiError>;

    /// Update account details
    async fn update_account(
        &self,
        update_account_request: models::UpdateAccountRequest,
        context: &C) -> Result<UpdateAccountResponse, ApiError>;

    /// Update workspace
    async fn update_workspace1(
        &self,
        update_workspace_request1: models::UpdateWorkspaceRequest1,
        context: &C) -> Result<UpdateWorkspace1Response, ApiError>;

    /// Update workspace sharing
    async fn update_workspace_sharing(
        &self,
        update_workspace_sharing_request: models::UpdateWorkspaceSharingRequest,
        context: &C) -> Result<UpdateWorkspaceSharingResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Create a new API key
    async fn create_api_key(
        &self,
        create_api_key_request: models::CreateApiKeyRequest,
        ) -> Result<CreateApiKeyResponse, ApiError>;

    /// Create a new organization
    async fn create_organization(
        &self,
        create_organization_request: models::CreateOrganizationRequest,
        ) -> Result<CreateOrganizationResponse, ApiError>;

    /// Create a new tenant
    async fn create_tenant(
        &self,
        organization_id: String,
        create_tenant_body: models::CreateTenantBody,
        ) -> Result<CreateTenantResponse, ApiError>;

    /// Create a new tenant API key
    async fn create_tenant_api_key(
        &self,
        create_tenant_api_key_request: models::CreateTenantApiKeyRequest,
        ) -> Result<CreateTenantApiKeyResponse, ApiError>;

    /// Create webhook
    async fn create_webhook(
        &self,
        create_webhook_request: models::CreateWebhookRequest,
        ) -> Result<CreateWebhookResponse, ApiError>;

    /// Delete API key
    async fn delete_api_key(
        &self,
        key_id: String,
        organization_id: Option<String>,
        workspace_id: Option<String>,
        tenant_id: Option<String>,
        account_id: Option<String>,
        ) -> Result<DeleteApiKeyResponse, ApiError>;

    /// Delete an organization
    async fn delete_organization(
        &self,
        id: String,
        ) -> Result<DeleteOrganizationResponse, ApiError>;

    /// Delete a tenant
    async fn delete_tenant(
        &self,
        organization_id: String,
        tenant_id: String,
        ) -> Result<DeleteTenantResponse, ApiError>;

    /// Delete tenant API key
    async fn delete_tenant_api_key(
        &self,
        key_id: String,
        organization_id: Option<String>,
        tenant_id: Option<String>,
        ) -> Result<DeleteTenantApiKeyResponse, ApiError>;

    /// Delete webhook
    async fn delete_webhook(
        &self,
        webhook_id: String,
        organization_id: Option<String>,
        workspace_id: Option<String>,
        tenant_id: Option<String>,
        account_id: Option<String>,
        ) -> Result<DeleteWebhookResponse, ApiError>;

    /// Delete a workspace
    async fn delete_workspace(
        &self,
        id: String,
        ) -> Result<DeleteWorkspaceResponse, ApiError>;

    /// Get API key details
    async fn get_api_key(
        &self,
        key_id: String,
        organization_id: Option<String>,
        tenant_id: Option<String>,
        account_id: Option<String>,
        workspace_id: Option<String>,
        ) -> Result<GetApiKeyResponse, ApiError>;

    /// Get lead details
    async fn get_lead(
        &self,
        lead_id: String,
        organization_id: Option<String>,
        workspace_id: Option<String>,
        tenant_id: Option<String>,
        account_id: Option<String>,
        ) -> Result<GetLeadResponse, ApiError>;

    /// Get organization details
    async fn get_organization(
        &self,
        id: String,
        ) -> Result<GetOrganizationResponse, ApiError>;

    /// Get tenant details
    async fn get_tenant(
        &self,
        organization_id: String,
        tenant_id: String,
        ) -> Result<GetTenantResponse, ApiError>;

    /// Get tenant API key details
    async fn get_tenant_api_key(
        &self,
        key_id: String,
        organization_id: Option<String>,
        tenant_id: Option<String>,
        ) -> Result<GetTenantApiKeyResponse, ApiError>;

    /// Get webhook
    async fn get_webhook(
        &self,
        webhook_id: String,
        organization_id: Option<String>,
        workspace_id: Option<String>,
        tenant_id: Option<String>,
        account_id: Option<String>,
        ) -> Result<GetWebhookResponse, ApiError>;

    /// List API keys
    async fn list_api_keys(
        &self,
        organization_id: Option<String>,
        tenant_id: Option<String>,
        account_id: Option<String>,
        workspace_id: Option<String>,
        page_size: Option<i32>,
        page_number: Option<i32>,
        status: Option<String>,
        search: Option<String>,
        ) -> Result<ListApiKeysResponse, ApiError>;

    /// List leads
    async fn list_leads(
        &self,
        organization_id: Option<String>,
        workspace_id: Option<String>,
        tenant_id: Option<String>,
        account_id: Option<String>,
        page_size: Option<i32>,
        page_number: Option<i32>,
        ) -> Result<ListLeadsResponse, ApiError>;

    /// List all organizations
    async fn list_organizations(
        &self,
        page_size: Option<i32>,
        page_number: Option<i32>,
        ) -> Result<ListOrganizationsResponse, ApiError>;

    /// List tenant API keys
    async fn list_tenant_api_keys(
        &self,
        organization_id: Option<String>,
        tenant_id: Option<String>,
        page_size: Option<i32>,
        page_number: Option<i32>,
        status: Option<String>,
        ) -> Result<ListTenantApiKeysResponse, ApiError>;

    /// List all tenants
    async fn list_tenants(
        &self,
        organization_id: String,
        page_size: Option<i32>,
        page_number: Option<i32>,
        ) -> Result<ListTenantsResponse, ApiError>;

    /// List webhooks
    async fn list_webhooks(
        &self,
        organization_id: Option<String>,
        workspace_id: Option<String>,
        tenant_id: Option<String>,
        account_id: Option<String>,
        page_size: Option<i32>,
        page_number: Option<i32>,
        status: Option<String>,
        search: Option<String>,
        ) -> Result<ListWebhooksResponse, ApiError>;

    /// Rotate API key
    async fn rotate_api_key(
        &self,
        rotate_api_key_request: models::RotateApiKeyRequest,
        ) -> Result<RotateApiKeyResponse, ApiError>;

    /// Rotate tenant API key
    async fn rotate_tenant_api_key(
        &self,
        rotate_tenant_api_key_request: models::RotateTenantApiKeyRequest,
        ) -> Result<RotateTenantApiKeyResponse, ApiError>;

    /// Update API key
    async fn update_api_key(
        &self,
        update_api_key_request: models::UpdateApiKeyRequest,
        ) -> Result<UpdateApiKeyResponse, ApiError>;

    /// Update organization details
    async fn update_organization(
        &self,
        update_organization_request: models::UpdateOrganizationRequest,
        ) -> Result<UpdateOrganizationResponse, ApiError>;

    /// Update tenant details
    async fn update_tenant(
        &self,
        update_tenant_request: models::UpdateTenantRequest,
        ) -> Result<UpdateTenantResponse, ApiError>;

    /// Update tenant API key
    async fn update_tenant_api_key(
        &self,
        update_tenant_api_key_request: models::UpdateTenantApiKeyRequest,
        ) -> Result<UpdateTenantApiKeyResponse, ApiError>;

    /// Update webhook
    async fn update_webhook(
        &self,
        update_webhook_request: models::UpdateWebhookRequest,
        ) -> Result<UpdateWebhookResponse, ApiError>;

    /// Update workspace details
    async fn update_workspace(
        &self,
        update_workspace_request: models::UpdateWorkspaceRequest,
        ) -> Result<UpdateWorkspaceResponse, ApiError>;

    /// Create a new account
    async fn create_account(
        &self,
        create_account_request: models::CreateAccountRequest,
        ) -> Result<CreateAccountResponse, ApiError>;

    /// Create workspace
    async fn create_workspace(
        &self,
        create_workspace_request: models::CreateWorkspaceRequest,
        ) -> Result<CreateWorkspaceResponse, ApiError>;

    /// Delete account
    async fn delete_account(
        &self,
        id: String,
        ) -> Result<DeleteAccountResponse, ApiError>;

    /// Delete workspace
    async fn delete_workspace1(
        &self,
        id: String,
        ) -> Result<DeleteWorkspace1Response, ApiError>;

    /// Get account details
    async fn get_account(
        &self,
        id: String,
        ) -> Result<GetAccountResponse, ApiError>;

    /// Get workspace details
    async fn get_workspace(
        &self,
        id: String,
        ) -> Result<GetWorkspaceResponse, ApiError>;

    /// Get workspace analytics
    async fn get_workspace_analytics(
        &self,
        workspace_id: String,
        start_time: Option<chrono::DateTime::<chrono::Utc>>,
        end_time: Option<chrono::DateTime::<chrono::Utc>>,
        ) -> Result<GetWorkspaceAnalyticsResponse, ApiError>;

    /// Get compliance report
    async fn get_workspace_compliance_report(
        &self,
        workspace_id: String,
        compliance_type: Option<String>,
        ) -> Result<GetWorkspaceComplianceReportResponse, ApiError>;

    /// Get storage statistics
    async fn get_workspace_storage_stats(
        &self,
        workspace_id: String,
        ) -> Result<GetWorkspaceStorageStatsResponse, ApiError>;

    /// List workspace sharings
    async fn list_workspace_sharings(
        &self,
        workspace_id: String,
        page_size: Option<i32>,
        page_token: Option<String>,
        ) -> Result<ListWorkspaceSharingsResponse, ApiError>;

    /// List workspaces
    async fn list_workspaces(
        &self,
        account_id: Option<String>,
        page_size: Option<i32>,
        page_token: Option<String>,
        filter: Option<String>,
        ) -> Result<ListWorkspacesResponse, ApiError>;

    /// Remove workspace sharing
    async fn remove_workspace_sharing(
        &self,
        sharing_id: String,
        ) -> Result<RemoveWorkspaceSharingResponse, ApiError>;

    /// Share workspace
    async fn share_workspace(
        &self,
        workspace_id: String,
        share_workspace_body: models::ShareWorkspaceBody,
        ) -> Result<ShareWorkspaceResponse, ApiError>;

    /// Update account details
    async fn update_account(
        &self,
        update_account_request: models::UpdateAccountRequest,
        ) -> Result<UpdateAccountResponse, ApiError>;

    /// Update workspace
    async fn update_workspace1(
        &self,
        update_workspace_request1: models::UpdateWorkspaceRequest1,
        ) -> Result<UpdateWorkspace1Response, ApiError>;

    /// Update workspace sharing
    async fn update_workspace_sharing(
        &self,
        update_workspace_sharing_request: models::UpdateWorkspaceSharingRequest,
        ) -> Result<UpdateWorkspaceSharingResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// Create a new API key
    async fn create_api_key(
        &self,
        create_api_key_request: models::CreateApiKeyRequest,
        ) -> Result<CreateApiKeyResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_api_key(create_api_key_request, &context).await
    }

    /// Create a new organization
    async fn create_organization(
        &self,
        create_organization_request: models::CreateOrganizationRequest,
        ) -> Result<CreateOrganizationResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_organization(create_organization_request, &context).await
    }

    /// Create a new tenant
    async fn create_tenant(
        &self,
        organization_id: String,
        create_tenant_body: models::CreateTenantBody,
        ) -> Result<CreateTenantResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_tenant(organization_id, create_tenant_body, &context).await
    }

    /// Create a new tenant API key
    async fn create_tenant_api_key(
        &self,
        create_tenant_api_key_request: models::CreateTenantApiKeyRequest,
        ) -> Result<CreateTenantApiKeyResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_tenant_api_key(create_tenant_api_key_request, &context).await
    }

    /// Create webhook
    async fn create_webhook(
        &self,
        create_webhook_request: models::CreateWebhookRequest,
        ) -> Result<CreateWebhookResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_webhook(create_webhook_request, &context).await
    }

    /// Delete API key
    async fn delete_api_key(
        &self,
        key_id: String,
        organization_id: Option<String>,
        workspace_id: Option<String>,
        tenant_id: Option<String>,
        account_id: Option<String>,
        ) -> Result<DeleteApiKeyResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_api_key(key_id, organization_id, workspace_id, tenant_id, account_id, &context).await
    }

    /// Delete an organization
    async fn delete_organization(
        &self,
        id: String,
        ) -> Result<DeleteOrganizationResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_organization(id, &context).await
    }

    /// Delete a tenant
    async fn delete_tenant(
        &self,
        organization_id: String,
        tenant_id: String,
        ) -> Result<DeleteTenantResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_tenant(organization_id, tenant_id, &context).await
    }

    /// Delete tenant API key
    async fn delete_tenant_api_key(
        &self,
        key_id: String,
        organization_id: Option<String>,
        tenant_id: Option<String>,
        ) -> Result<DeleteTenantApiKeyResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_tenant_api_key(key_id, organization_id, tenant_id, &context).await
    }

    /// Delete webhook
    async fn delete_webhook(
        &self,
        webhook_id: String,
        organization_id: Option<String>,
        workspace_id: Option<String>,
        tenant_id: Option<String>,
        account_id: Option<String>,
        ) -> Result<DeleteWebhookResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_webhook(webhook_id, organization_id, workspace_id, tenant_id, account_id, &context).await
    }

    /// Delete a workspace
    async fn delete_workspace(
        &self,
        id: String,
        ) -> Result<DeleteWorkspaceResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_workspace(id, &context).await
    }

    /// Get API key details
    async fn get_api_key(
        &self,
        key_id: String,
        organization_id: Option<String>,
        tenant_id: Option<String>,
        account_id: Option<String>,
        workspace_id: Option<String>,
        ) -> Result<GetApiKeyResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_api_key(key_id, organization_id, tenant_id, account_id, workspace_id, &context).await
    }

    /// Get lead details
    async fn get_lead(
        &self,
        lead_id: String,
        organization_id: Option<String>,
        workspace_id: Option<String>,
        tenant_id: Option<String>,
        account_id: Option<String>,
        ) -> Result<GetLeadResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_lead(lead_id, organization_id, workspace_id, tenant_id, account_id, &context).await
    }

    /// Get organization details
    async fn get_organization(
        &self,
        id: String,
        ) -> Result<GetOrganizationResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_organization(id, &context).await
    }

    /// Get tenant details
    async fn get_tenant(
        &self,
        organization_id: String,
        tenant_id: String,
        ) -> Result<GetTenantResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_tenant(organization_id, tenant_id, &context).await
    }

    /// Get tenant API key details
    async fn get_tenant_api_key(
        &self,
        key_id: String,
        organization_id: Option<String>,
        tenant_id: Option<String>,
        ) -> Result<GetTenantApiKeyResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_tenant_api_key(key_id, organization_id, tenant_id, &context).await
    }

    /// Get webhook
    async fn get_webhook(
        &self,
        webhook_id: String,
        organization_id: Option<String>,
        workspace_id: Option<String>,
        tenant_id: Option<String>,
        account_id: Option<String>,
        ) -> Result<GetWebhookResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_webhook(webhook_id, organization_id, workspace_id, tenant_id, account_id, &context).await
    }

    /// List API keys
    async fn list_api_keys(
        &self,
        organization_id: Option<String>,
        tenant_id: Option<String>,
        account_id: Option<String>,
        workspace_id: Option<String>,
        page_size: Option<i32>,
        page_number: Option<i32>,
        status: Option<String>,
        search: Option<String>,
        ) -> Result<ListApiKeysResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().list_api_keys(organization_id, tenant_id, account_id, workspace_id, page_size, page_number, status, search, &context).await
    }

    /// List leads
    async fn list_leads(
        &self,
        organization_id: Option<String>,
        workspace_id: Option<String>,
        tenant_id: Option<String>,
        account_id: Option<String>,
        page_size: Option<i32>,
        page_number: Option<i32>,
        ) -> Result<ListLeadsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().list_leads(organization_id, workspace_id, tenant_id, account_id, page_size, page_number, &context).await
    }

    /// List all organizations
    async fn list_organizations(
        &self,
        page_size: Option<i32>,
        page_number: Option<i32>,
        ) -> Result<ListOrganizationsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().list_organizations(page_size, page_number, &context).await
    }

    /// List tenant API keys
    async fn list_tenant_api_keys(
        &self,
        organization_id: Option<String>,
        tenant_id: Option<String>,
        page_size: Option<i32>,
        page_number: Option<i32>,
        status: Option<String>,
        ) -> Result<ListTenantApiKeysResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().list_tenant_api_keys(organization_id, tenant_id, page_size, page_number, status, &context).await
    }

    /// List all tenants
    async fn list_tenants(
        &self,
        organization_id: String,
        page_size: Option<i32>,
        page_number: Option<i32>,
        ) -> Result<ListTenantsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().list_tenants(organization_id, page_size, page_number, &context).await
    }

    /// List webhooks
    async fn list_webhooks(
        &self,
        organization_id: Option<String>,
        workspace_id: Option<String>,
        tenant_id: Option<String>,
        account_id: Option<String>,
        page_size: Option<i32>,
        page_number: Option<i32>,
        status: Option<String>,
        search: Option<String>,
        ) -> Result<ListWebhooksResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().list_webhooks(organization_id, workspace_id, tenant_id, account_id, page_size, page_number, status, search, &context).await
    }

    /// Rotate API key
    async fn rotate_api_key(
        &self,
        rotate_api_key_request: models::RotateApiKeyRequest,
        ) -> Result<RotateApiKeyResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().rotate_api_key(rotate_api_key_request, &context).await
    }

    /// Rotate tenant API key
    async fn rotate_tenant_api_key(
        &self,
        rotate_tenant_api_key_request: models::RotateTenantApiKeyRequest,
        ) -> Result<RotateTenantApiKeyResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().rotate_tenant_api_key(rotate_tenant_api_key_request, &context).await
    }

    /// Update API key
    async fn update_api_key(
        &self,
        update_api_key_request: models::UpdateApiKeyRequest,
        ) -> Result<UpdateApiKeyResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().update_api_key(update_api_key_request, &context).await
    }

    /// Update organization details
    async fn update_organization(
        &self,
        update_organization_request: models::UpdateOrganizationRequest,
        ) -> Result<UpdateOrganizationResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().update_organization(update_organization_request, &context).await
    }

    /// Update tenant details
    async fn update_tenant(
        &self,
        update_tenant_request: models::UpdateTenantRequest,
        ) -> Result<UpdateTenantResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().update_tenant(update_tenant_request, &context).await
    }

    /// Update tenant API key
    async fn update_tenant_api_key(
        &self,
        update_tenant_api_key_request: models::UpdateTenantApiKeyRequest,
        ) -> Result<UpdateTenantApiKeyResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().update_tenant_api_key(update_tenant_api_key_request, &context).await
    }

    /// Update webhook
    async fn update_webhook(
        &self,
        update_webhook_request: models::UpdateWebhookRequest,
        ) -> Result<UpdateWebhookResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().update_webhook(update_webhook_request, &context).await
    }

    /// Update workspace details
    async fn update_workspace(
        &self,
        update_workspace_request: models::UpdateWorkspaceRequest,
        ) -> Result<UpdateWorkspaceResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().update_workspace(update_workspace_request, &context).await
    }

    /// Create a new account
    async fn create_account(
        &self,
        create_account_request: models::CreateAccountRequest,
        ) -> Result<CreateAccountResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_account(create_account_request, &context).await
    }

    /// Create workspace
    async fn create_workspace(
        &self,
        create_workspace_request: models::CreateWorkspaceRequest,
        ) -> Result<CreateWorkspaceResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_workspace(create_workspace_request, &context).await
    }

    /// Delete account
    async fn delete_account(
        &self,
        id: String,
        ) -> Result<DeleteAccountResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_account(id, &context).await
    }

    /// Delete workspace
    async fn delete_workspace1(
        &self,
        id: String,
        ) -> Result<DeleteWorkspace1Response, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_workspace1(id, &context).await
    }

    /// Get account details
    async fn get_account(
        &self,
        id: String,
        ) -> Result<GetAccountResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_account(id, &context).await
    }

    /// Get workspace details
    async fn get_workspace(
        &self,
        id: String,
        ) -> Result<GetWorkspaceResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_workspace(id, &context).await
    }

    /// Get workspace analytics
    async fn get_workspace_analytics(
        &self,
        workspace_id: String,
        start_time: Option<chrono::DateTime::<chrono::Utc>>,
        end_time: Option<chrono::DateTime::<chrono::Utc>>,
        ) -> Result<GetWorkspaceAnalyticsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_workspace_analytics(workspace_id, start_time, end_time, &context).await
    }

    /// Get compliance report
    async fn get_workspace_compliance_report(
        &self,
        workspace_id: String,
        compliance_type: Option<String>,
        ) -> Result<GetWorkspaceComplianceReportResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_workspace_compliance_report(workspace_id, compliance_type, &context).await
    }

    /// Get storage statistics
    async fn get_workspace_storage_stats(
        &self,
        workspace_id: String,
        ) -> Result<GetWorkspaceStorageStatsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_workspace_storage_stats(workspace_id, &context).await
    }

    /// List workspace sharings
    async fn list_workspace_sharings(
        &self,
        workspace_id: String,
        page_size: Option<i32>,
        page_token: Option<String>,
        ) -> Result<ListWorkspaceSharingsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().list_workspace_sharings(workspace_id, page_size, page_token, &context).await
    }

    /// List workspaces
    async fn list_workspaces(
        &self,
        account_id: Option<String>,
        page_size: Option<i32>,
        page_token: Option<String>,
        filter: Option<String>,
        ) -> Result<ListWorkspacesResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().list_workspaces(account_id, page_size, page_token, filter, &context).await
    }

    /// Remove workspace sharing
    async fn remove_workspace_sharing(
        &self,
        sharing_id: String,
        ) -> Result<RemoveWorkspaceSharingResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().remove_workspace_sharing(sharing_id, &context).await
    }

    /// Share workspace
    async fn share_workspace(
        &self,
        workspace_id: String,
        share_workspace_body: models::ShareWorkspaceBody,
        ) -> Result<ShareWorkspaceResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().share_workspace(workspace_id, share_workspace_body, &context).await
    }

    /// Update account details
    async fn update_account(
        &self,
        update_account_request: models::UpdateAccountRequest,
        ) -> Result<UpdateAccountResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().update_account(update_account_request, &context).await
    }

    /// Update workspace
    async fn update_workspace1(
        &self,
        update_workspace_request1: models::UpdateWorkspaceRequest1,
        ) -> Result<UpdateWorkspace1Response, ApiError>
    {
        let context = self.context().clone();
        self.api().update_workspace1(update_workspace_request1, &context).await
    }

    /// Update workspace sharing
    async fn update_workspace_sharing(
        &self,
        update_workspace_sharing_request: models::UpdateWorkspaceSharingRequest,
        ) -> Result<UpdateWorkspaceSharingResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().update_workspace_sharing(update_workspace_sharing_request, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
