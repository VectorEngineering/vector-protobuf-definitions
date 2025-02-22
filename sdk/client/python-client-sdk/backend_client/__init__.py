# coding: utf-8

# flake8: noqa

"""
    Lead Scraping Service API

    Vector Lead Scraping Service API - Manages Lead Scraping Jobs

    The version of the OpenAPI document: 1.0
    Contact: yoanyomba@vector.ai
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


__version__ = "1.0.0"

# import apis into sdk package
from backend_client.api.lead_scraper_service_api import LeadScraperServiceApi

# import ApiClient
from backend_client.api_response import ApiResponse
from backend_client.api_client import ApiClient
from backend_client.configuration import Configuration
from backend_client.exceptions import OpenApiException
from backend_client.exceptions import ApiTypeError
from backend_client.exceptions import ApiValueError
from backend_client.exceptions import ApiKeyError
from backend_client.exceptions import ApiAttributeError
from backend_client.exceptions import ApiException

# import models into sdk package
from backend_client.models.api_info import APIInfo
from backend_client.models.account import Account
from backend_client.models.any import Any
from backend_client.models.auth_context import AuthContext
from backend_client.models.auth_error_code import AuthErrorCode
from backend_client.models.authentication_error_message_response import AuthenticationErrorMessageResponse
from backend_client.models.availability_info import AvailabilityInfo
from backend_client.models.background_job_status import BackgroundJobStatus
from backend_client.models.bad_gateway_error_message_response import BadGatewayErrorMessageResponse
from backend_client.models.conflict_error_message_response import ConflictErrorMessageResponse
from backend_client.models.conflict_info import ConflictInfo
from backend_client.models.create_account_request import CreateAccountRequest
from backend_client.models.create_account_response import CreateAccountResponse
from backend_client.models.create_scraping_job_request import CreateScrapingJobRequest
from backend_client.models.create_scraping_job_response import CreateScrapingJobResponse
from backend_client.models.delete_account_response import DeleteAccountResponse
from backend_client.models.delete_scraping_job_response import DeleteScrapingJobResponse
from backend_client.models.dependency import Dependency
from backend_client.models.download_scraping_results_response import DownloadScrapingResultsResponse
from backend_client.models.error_response import ErrorResponse
from backend_client.models.field_error import FieldError
from backend_client.models.field_violation import FieldViolation
from backend_client.models.forbidden_error_message_response import ForbiddenErrorMessageResponse
from backend_client.models.gateway_timeout_error_message_response import GatewayTimeoutErrorMessageResponse
from backend_client.models.get_account_response import GetAccountResponse
from backend_client.models.get_scraping_job_response import GetScrapingJobResponse
from backend_client.models.gone_error_message_response import GoneErrorMessageResponse
from backend_client.models.internal_error_code import InternalErrorCode
from backend_client.models.internal_error_message_response import InternalErrorMessageResponse
from backend_client.models.limit_info import LimitInfo
from backend_client.models.list_accounts_response import ListAccountsResponse
from backend_client.models.list_scraping_jobs_response import ListScrapingJobsResponse
from backend_client.models.mfa_info import MFAInfo
from backend_client.models.method_not_allowed_error_message_response import MethodNotAllowedErrorMessageResponse
from backend_client.models.not_found_error_code import NotFoundErrorCode
from backend_client.models.not_found_error_message_response import NotFoundErrorMessageResponse
from backend_client.models.not_implemented_error_message_response import NotImplementedErrorMessageResponse
from backend_client.models.operation_details import OperationDetails
from backend_client.models.payment_info import PaymentInfo
from backend_client.models.payment_required_error_message_response import PaymentRequiredErrorMessageResponse
from backend_client.models.precondition_failed_error_message_response import PreconditionFailedErrorMessageResponse
from backend_client.models.quota_info import QuotaInfo
from backend_client.models.rate_limit_context import RateLimitContext
from backend_client.models.rate_limit_error_message_response import RateLimitErrorMessageResponse
from backend_client.models.resource_info import ResourceInfo
from backend_client.models.resource_utilization import ResourceUtilization
from backend_client.models.resource_validation import ResourceValidation
from backend_client.models.schema_validation import SchemaValidation
from backend_client.models.scraping_job import ScrapingJob
from backend_client.models.service_status import ServiceStatus
from backend_client.models.service_unavailable_error_message_response import ServiceUnavailableErrorMessageResponse
from backend_client.models.session_info import SessionInfo
from backend_client.models.status import Status
from backend_client.models.suggestions import Suggestions
from backend_client.models.token_info import TokenInfo
from backend_client.models.too_early_error_message_response import TooEarlyErrorMessageResponse
from backend_client.models.unprocessable_entity_error_message_response import UnprocessableEntityErrorMessageResponse
from backend_client.models.update_account_request import UpdateAccountRequest
from backend_client.models.update_account_response import UpdateAccountResponse
from backend_client.models.validation_error_code import ValidationErrorCode
from backend_client.models.validation_error_message_response import ValidationErrorMessageResponse
