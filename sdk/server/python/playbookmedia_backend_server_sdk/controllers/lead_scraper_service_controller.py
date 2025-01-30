import connexion
from typing import Dict
from typing import Tuple
from typing import Union

from playbookmedia_backend_server_sdk.models.authentication_error_message_response import AuthenticationErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.bad_gateway_error_message_response import BadGatewayErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.conflict_error_message_response import ConflictErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_account_request import CreateAccountRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_account_response import CreateAccountResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_scraping_job_request import CreateScrapingJobRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_scraping_job_response import CreateScrapingJobResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.delete_account_response import DeleteAccountResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.delete_scraping_job_response import DeleteScrapingJobResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.download_scraping_results_response import DownloadScrapingResultsResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.forbidden_error_message_response import ForbiddenErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.gateway_timeout_error_message_response import GatewayTimeoutErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.get_account_response import GetAccountResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.get_scraping_job_response import GetScrapingJobResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.gone_error_message_response import GoneErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.internal_error_message_response import InternalErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.list_scraping_jobs_response import ListScrapingJobsResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.method_not_allowed_error_message_response import MethodNotAllowedErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.not_found_error_message_response import NotFoundErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.not_implemented_error_message_response import NotImplementedErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.payment_required_error_message_response import PaymentRequiredErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.precondition_failed_error_message_response import PreconditionFailedErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.rate_limit_error_message_response import RateLimitErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.service_unavailable_error_message_response import ServiceUnavailableErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.status import Status  # noqa: E501
from playbookmedia_backend_server_sdk.models.too_early_error_message_response import TooEarlyErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.unprocessable_entity_error_message_response import UnprocessableEntityErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_account_request import UpdateAccountRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_account_response import UpdateAccountResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.validation_error_message_response import ValidationErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk import util


def create_account(create_account_request):  # noqa: E501
    """Create a new account

    Creates a new user account in the workspace service # noqa: E501

    :param create_account_request: 
    :type create_account_request: dict | bytes

    :rtype: Union[CreateAccountResponse, Tuple[CreateAccountResponse, int], Tuple[CreateAccountResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        create_account_request = CreateAccountRequest.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def create_scraping_job(create_scraping_job_request):  # noqa: E501
    """Create a new job scraping task

    This endpoint creates a new Google Maps scraping job # noqa: E501

    :param create_scraping_job_request: 
    :type create_scraping_job_request: dict | bytes

    :rtype: Union[CreateScrapingJobResponse, Tuple[CreateScrapingJobResponse, int], Tuple[CreateScrapingJobResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        create_scraping_job_request = CreateScrapingJobRequest.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def delete_account(id):  # noqa: E501
    """Delete account

    Permanently deletes an account and associated resources # noqa: E501

    :param id: 
    :type id: str

    :rtype: Union[DeleteAccountResponse, Tuple[DeleteAccountResponse, int], Tuple[DeleteAccountResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def delete_scraping_job(job_id, user_id, org_id, tenant_id):  # noqa: E501
    """Delete a specific job

    This endpoint deletes a specific Google Maps scraping job # noqa: E501

    :param job_id: 
    :type job_id: str
    :param user_id: 
    :type user_id: str
    :param org_id: 
    :type org_id: str
    :param tenant_id: 
    :type tenant_id: str

    :rtype: Union[DeleteScrapingJobResponse, Tuple[DeleteScrapingJobResponse, int], Tuple[DeleteScrapingJobResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def download_scraping_results(job_id, user_id, org_id, tenant_id):  # noqa: E501
    """Download job results as CSV

    This endpoint downloads the results of a Google Maps scraping job as CSV # noqa: E501

    :param job_id: 
    :type job_id: str
    :param user_id: 
    :type user_id: str
    :param org_id: 
    :type org_id: str
    :param tenant_id: 
    :type tenant_id: str

    :rtype: Union[DownloadScrapingResultsResponse, Tuple[DownloadScrapingResultsResponse, int], Tuple[DownloadScrapingResultsResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def get_account(id):  # noqa: E501
    """Get account details

    Retrieves details of a specific account # noqa: E501

    :param id: 
    :type id: str

    :rtype: Union[GetAccountResponse, Tuple[GetAccountResponse, int], Tuple[GetAccountResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def get_scraping_job(job_id, user_id, org_id, tenant_id):  # noqa: E501
    """Get a specific job

    This endpoint retrieves a specific Google Maps scraping job # noqa: E501

    :param job_id: 
    :type job_id: str
    :param user_id: 
    :type user_id: str
    :param org_id: 
    :type org_id: str
    :param tenant_id: 
    :type tenant_id: str

    :rtype: Union[GetScrapingJobResponse, Tuple[GetScrapingJobResponse, int], Tuple[GetScrapingJobResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def list_scraping_jobs(user_id, org_id, tenant_id):  # noqa: E501
    """Get all jobs

    This endpoint retrieves all Google Maps scraping jobs # noqa: E501

    :param user_id: 
    :type user_id: str
    :param org_id: 
    :type org_id: str
    :param tenant_id: 
    :type tenant_id: str

    :rtype: Union[ListScrapingJobsResponse, Tuple[ListScrapingJobsResponse, int], Tuple[ListScrapingJobsResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def update_account(update_account_request):  # noqa: E501
    """Update account details

    Updates specified fields of an existing account # noqa: E501

    :param update_account_request: 
    :type update_account_request: dict | bytes

    :rtype: Union[UpdateAccountResponse, Tuple[UpdateAccountResponse, int], Tuple[UpdateAccountResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        update_account_request = UpdateAccountRequest.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'
