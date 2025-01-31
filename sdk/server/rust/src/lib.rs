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
pub enum CreateScrapingJobResponse {
    /// A successful response.
    ASuccessfulResponse
    (models::CreateScrapingJobResponse)
    ,
    /// Job created successfully
    JobCreatedSuccessfully
    (models::CreateScrapingJobResponse)
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
pub enum DeleteScrapingJobResponse {
    /// Job deleted successfully
    JobDeletedSuccessfully
    (models::DeleteScrapingJobResponse)
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
pub enum DownloadScrapingResultsResponse {
    /// Successful response
    SuccessfulResponse
    (models::DownloadScrapingResultsResponse)
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
pub enum GetScrapingJobResponse {
    /// Successful response
    SuccessfulResponse
    (models::GetScrapingJobResponse)
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
pub enum ListScrapingJobsResponse {
    /// Successful response
    SuccessfulResponse
    (models::ListScrapingJobsResponse)
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

/// API
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Create a new account
    async fn create_account(
        &self,
        create_account_request: models::CreateAccountRequest,
        context: &C) -> Result<CreateAccountResponse, ApiError>;

    /// Create a new job scraping task
    async fn create_scraping_job(
        &self,
        create_scraping_job_request: models::CreateScrapingJobRequest,
        context: &C) -> Result<CreateScrapingJobResponse, ApiError>;

    /// Delete account
    async fn delete_account(
        &self,
        id: String,
        context: &C) -> Result<DeleteAccountResponse, ApiError>;

    /// Delete a specific job
    async fn delete_scraping_job(
        &self,
        job_id: String,
        user_id: String,
        org_id: String,
        tenant_id: String,
        context: &C) -> Result<DeleteScrapingJobResponse, ApiError>;

    /// Download job results as CSV
    async fn download_scraping_results(
        &self,
        job_id: String,
        user_id: String,
        org_id: String,
        tenant_id: String,
        context: &C) -> Result<DownloadScrapingResultsResponse, ApiError>;

    /// Get account details
    async fn get_account(
        &self,
        id: String,
        context: &C) -> Result<GetAccountResponse, ApiError>;

    /// Get a specific job
    async fn get_scraping_job(
        &self,
        job_id: String,
        user_id: String,
        org_id: String,
        tenant_id: String,
        context: &C) -> Result<GetScrapingJobResponse, ApiError>;

    /// Get all jobs
    async fn list_scraping_jobs(
        &self,
        user_id: String,
        org_id: String,
        tenant_id: String,
        context: &C) -> Result<ListScrapingJobsResponse, ApiError>;

    /// Update account details
    async fn update_account(
        &self,
        update_account_request: models::UpdateAccountRequest,
        context: &C) -> Result<UpdateAccountResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Create a new account
    async fn create_account(
        &self,
        create_account_request: models::CreateAccountRequest,
        ) -> Result<CreateAccountResponse, ApiError>;

    /// Create a new job scraping task
    async fn create_scraping_job(
        &self,
        create_scraping_job_request: models::CreateScrapingJobRequest,
        ) -> Result<CreateScrapingJobResponse, ApiError>;

    /// Delete account
    async fn delete_account(
        &self,
        id: String,
        ) -> Result<DeleteAccountResponse, ApiError>;

    /// Delete a specific job
    async fn delete_scraping_job(
        &self,
        job_id: String,
        user_id: String,
        org_id: String,
        tenant_id: String,
        ) -> Result<DeleteScrapingJobResponse, ApiError>;

    /// Download job results as CSV
    async fn download_scraping_results(
        &self,
        job_id: String,
        user_id: String,
        org_id: String,
        tenant_id: String,
        ) -> Result<DownloadScrapingResultsResponse, ApiError>;

    /// Get account details
    async fn get_account(
        &self,
        id: String,
        ) -> Result<GetAccountResponse, ApiError>;

    /// Get a specific job
    async fn get_scraping_job(
        &self,
        job_id: String,
        user_id: String,
        org_id: String,
        tenant_id: String,
        ) -> Result<GetScrapingJobResponse, ApiError>;

    /// Get all jobs
    async fn list_scraping_jobs(
        &self,
        user_id: String,
        org_id: String,
        tenant_id: String,
        ) -> Result<ListScrapingJobsResponse, ApiError>;

    /// Update account details
    async fn update_account(
        &self,
        update_account_request: models::UpdateAccountRequest,
        ) -> Result<UpdateAccountResponse, ApiError>;

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

    /// Create a new account
    async fn create_account(
        &self,
        create_account_request: models::CreateAccountRequest,
        ) -> Result<CreateAccountResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_account(create_account_request, &context).await
    }

    /// Create a new job scraping task
    async fn create_scraping_job(
        &self,
        create_scraping_job_request: models::CreateScrapingJobRequest,
        ) -> Result<CreateScrapingJobResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_scraping_job(create_scraping_job_request, &context).await
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

    /// Delete a specific job
    async fn delete_scraping_job(
        &self,
        job_id: String,
        user_id: String,
        org_id: String,
        tenant_id: String,
        ) -> Result<DeleteScrapingJobResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_scraping_job(job_id, user_id, org_id, tenant_id, &context).await
    }

    /// Download job results as CSV
    async fn download_scraping_results(
        &self,
        job_id: String,
        user_id: String,
        org_id: String,
        tenant_id: String,
        ) -> Result<DownloadScrapingResultsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().download_scraping_results(job_id, user_id, org_id, tenant_id, &context).await
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

    /// Get a specific job
    async fn get_scraping_job(
        &self,
        job_id: String,
        user_id: String,
        org_id: String,
        tenant_id: String,
        ) -> Result<GetScrapingJobResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_scraping_job(job_id, user_id, org_id, tenant_id, &context).await
    }

    /// Get all jobs
    async fn list_scraping_jobs(
        &self,
        user_id: String,
        org_id: String,
        tenant_id: String,
        ) -> Result<ListScrapingJobsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().list_scraping_jobs(user_id, org_id, tenant_id, &context).await
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
