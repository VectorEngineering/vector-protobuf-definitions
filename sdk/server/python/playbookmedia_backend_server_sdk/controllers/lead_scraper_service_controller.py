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
from playbookmedia_backend_server_sdk.models.create_workflow_body import CreateWorkflowBody  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_workflow_response import CreateWorkflowResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_workspace_request import CreateWorkspaceRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_workspace_response import CreateWorkspaceResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.delete_account_response import DeleteAccountResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.delete_scraping_job_response import DeleteScrapingJobResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.delete_workspace_response import DeleteWorkspaceResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.download_scraping_results_response import DownloadScrapingResultsResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.forbidden_error_message_response import ForbiddenErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.gateway_timeout_error_message_response import GatewayTimeoutErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.get_account_response import GetAccountResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.get_account_usage_response import GetAccountUsageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.get_scraping_job_response import GetScrapingJobResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.get_workspace_analytics_response import GetWorkspaceAnalyticsResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.gone_error_message_response import GoneErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.internal_error_message_response import InternalErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.list_accounts_response import ListAccountsResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.list_scraping_jobs_response import ListScrapingJobsResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.list_workflows_response import ListWorkflowsResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.list_workspaces_response import ListWorkspacesResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.method_not_allowed_error_message_response import MethodNotAllowedErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.not_found_error_message_response import NotFoundErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.not_implemented_error_message_response import NotImplementedErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.pause_workflow_body import PauseWorkflowBody  # noqa: E501
from playbookmedia_backend_server_sdk.models.pause_workflow_response import PauseWorkflowResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.payment_required_error_message_response import PaymentRequiredErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.precondition_failed_error_message_response import PreconditionFailedErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.rate_limit_error_message_response import RateLimitErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.rpc_status import RpcStatus  # noqa: E501
from playbookmedia_backend_server_sdk.models.scraping_workflow import ScrapingWorkflow  # noqa: E501
from playbookmedia_backend_server_sdk.models.service_unavailable_error_message_response import ServiceUnavailableErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.too_early_error_message_response import TooEarlyErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.trigger_workflow_body import TriggerWorkflowBody  # noqa: E501
from playbookmedia_backend_server_sdk.models.trigger_workflow_response import TriggerWorkflowResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.unprocessable_entity_error_message_response import UnprocessableEntityErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_account_request import UpdateAccountRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_account_response import UpdateAccountResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_account_settings_request import UpdateAccountSettingsRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_account_settings_response import UpdateAccountSettingsResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_workflow_request import UpdateWorkflowRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_workspace_request import UpdateWorkspaceRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_workspace_response import UpdateWorkspaceResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.validation_error_message_response import ValidationErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.workspace import Workspace  # noqa: E501
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


def create_workspace(create_workspace_request):  # noqa: E501
    """Create a new workspace

    Creates a new workspace for a given account # noqa: E501

    :param create_workspace_request: 
    :type create_workspace_request: dict | bytes

    :rtype: Union[CreateWorkspaceResponse, Tuple[CreateWorkspaceResponse, int], Tuple[CreateWorkspaceResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        create_workspace_request = CreateWorkspaceRequest.from_dict(connexion.request.get_json())  # noqa: E501
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


def delete_workspace(id):  # noqa: E501
    """Delete a workspace

    Deletes a specific workspace # noqa: E501

    :param id: 
    :type id: str

    :rtype: Union[DeleteWorkspaceResponse, Tuple[DeleteWorkspaceResponse, int], Tuple[DeleteWorkspaceResponse, int, Dict[str, str]]
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


def get_account_usage(id):  # noqa: E501
    """Get account usage

    Retrieves usage details for a given account # noqa: E501

    :param id: 
    :type id: str

    :rtype: Union[GetAccountUsageResponse, Tuple[GetAccountUsageResponse, int], Tuple[GetAccountUsageResponse, int, Dict[str, str]]
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


def get_workflow(workspace_id, id):  # noqa: E501
    """Get workflow details

    Retrieves details of a specific workflow # noqa: E501

    :param workspace_id: 
    :type workspace_id: str
    :param id: 
    :type id: str

    :rtype: Union[ScrapingWorkflow, Tuple[ScrapingWorkflow, int], Tuple[ScrapingWorkflow, int, Dict[str, str]]
    """
    return 'do some magic!'


def get_workspace(id):  # noqa: E501
    """Get workspace details

    Retrieves details of a specific workspace # noqa: E501

    :param id: 
    :type id: str

    :rtype: Union[Workspace, Tuple[Workspace, int], Tuple[Workspace, int, Dict[str, str]]
    """
    return 'do some magic!'


def get_workspace_analytics(workspace_id, start_time=None, end_time=None):  # noqa: E501
    """Get workspace analytics

    Retrieves analytics data for a specific workspace # noqa: E501

    :param workspace_id: 
    :type workspace_id: str
    :param start_time: 
    :type start_time: str
    :param end_time: 
    :type end_time: str

    :rtype: Union[GetWorkspaceAnalyticsResponse, Tuple[GetWorkspaceAnalyticsResponse, int], Tuple[GetWorkspaceAnalyticsResponse, int, Dict[str, str]]
    """
    start_time = util.deserialize_datetime(start_time)
    end_time = util.deserialize_datetime(end_time)
    return 'do some magic!'


def lead_scraper_service_create_workflow(workspace_id, create_workflow_body):  # noqa: E501
    """Create a new workflow

    This endpoint creates a new workflow configuration for a specific workspace  Required parameters: - workspace_id: Parent workspace ID - name: Human-readable workflow name - description: Optional workflow description - parameters: Job parameters for the workflow  Example: &#x60;&#x60;&#x60;json # noqa: E501

    :param workspace_id: 
    :type workspace_id: str
    :param create_workflow_body: 
    :type create_workflow_body: dict | bytes

    :rtype: Union[CreateWorkflowResponse, Tuple[CreateWorkflowResponse, int], Tuple[CreateWorkflowResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        create_workflow_body = CreateWorkflowBody.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def list_accounts(page_size=None, page_token=None, filter=None):  # noqa: E501
    """List all accounts

    Retrieves a list of accounts # noqa: E501

    :param page_size: 
    :type page_size: int
    :param page_token: 
    :type page_token: str
    :param filter: 
    :type filter: str

    :rtype: Union[ListAccountsResponse, Tuple[ListAccountsResponse, int], Tuple[ListAccountsResponse, int, Dict[str, str]]
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


def list_workflows(workspace_id, page_size=None, page_token=None, filter=None):  # noqa: E501
    """List workflows

    Retrieves a list of workflows for a given workspace # noqa: E501

    :param workspace_id: 
    :type workspace_id: str
    :param page_size: 
    :type page_size: int
    :param page_token: 
    :type page_token: str
    :param filter: 
    :type filter: str

    :rtype: Union[ListWorkflowsResponse, Tuple[ListWorkflowsResponse, int], Tuple[ListWorkflowsResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def list_workspaces(account_id=None, page_size=None, page_token=None):  # noqa: E501
    """List workspaces

    Retrieves a list of workspaces for a given account # noqa: E501

    :param account_id: 
    :type account_id: str
    :param page_size: 
    :type page_size: int
    :param page_token: 
    :type page_token: str

    :rtype: Union[ListWorkspacesResponse, Tuple[ListWorkspacesResponse, int], Tuple[ListWorkspacesResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def pause_workflow(workspace_id, id, pause_workflow_body):  # noqa: E501
    """Pause workflow execution

    Pauses the execution of a specific workflow # noqa: E501

    :param workspace_id: 
    :type workspace_id: str
    :param id: 
    :type id: str
    :param pause_workflow_body: 
    :type pause_workflow_body: dict | bytes

    :rtype: Union[PauseWorkflowResponse, Tuple[PauseWorkflowResponse, int], Tuple[PauseWorkflowResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        pause_workflow_body = PauseWorkflowBody.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def trigger_workflow(workspace_id, id, trigger_workflow_body):  # noqa: E501
    """Trigger workflow execution

    Triggers the execution of a specific workflow # noqa: E501

    :param workspace_id: 
    :type workspace_id: str
    :param id: 
    :type id: str
    :param trigger_workflow_body: 
    :type trigger_workflow_body: dict | bytes

    :rtype: Union[TriggerWorkflowResponse, Tuple[TriggerWorkflowResponse, int], Tuple[TriggerWorkflowResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        trigger_workflow_body = TriggerWorkflowBody.from_dict(connexion.request.get_json())  # noqa: E501
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


def update_account_settings(update_account_settings_request):  # noqa: E501
    """Update account settings

    Updates the settings for a given account # noqa: E501

    :param update_account_settings_request: 
    :type update_account_settings_request: dict | bytes

    :rtype: Union[UpdateAccountSettingsResponse, Tuple[UpdateAccountSettingsResponse, int], Tuple[UpdateAccountSettingsResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        update_account_settings_request = UpdateAccountSettingsRequest.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def update_workflow(update_workflow_request):  # noqa: E501
    """Update workflow details

    Updates the details of a specific workflow # noqa: E501

    :param update_workflow_request: 
    :type update_workflow_request: dict | bytes

    :rtype: Union[ScrapingWorkflow, Tuple[ScrapingWorkflow, int], Tuple[ScrapingWorkflow, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        update_workflow_request = UpdateWorkflowRequest.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def update_workspace(update_workspace_request):  # noqa: E501
    """Update workspace details

    Updates the details of a specific workspace # noqa: E501

    :param update_workspace_request: 
    :type update_workspace_request: dict | bytes

    :rtype: Union[UpdateWorkspaceResponse, Tuple[UpdateWorkspaceResponse, int], Tuple[UpdateWorkspaceResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        update_workspace_request = UpdateWorkspaceRequest.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'
