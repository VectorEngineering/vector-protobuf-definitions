import connexion
from typing import Dict
from typing import Tuple
from typing import Union

from playbookmedia_backend_server_sdk.models.authentication_error_message_response1 import AuthenticationErrorMessageResponse1  # noqa: E501
from playbookmedia_backend_server_sdk.models.conflict_error_message_response import ConflictErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_account_request import CreateAccountRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_account_response import CreateAccountResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_workspace_request import CreateWorkspaceRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_workspace_response import CreateWorkspaceResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.delete_account_response import DeleteAccountResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.delete_workspace_response import DeleteWorkspaceResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.forbidden_error_message_response import ForbiddenErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.get_account_response import GetAccountResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.get_workspace_analytics_response import GetWorkspaceAnalyticsResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.get_workspace_compliance_report_response import GetWorkspaceComplianceReportResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.get_workspace_response import GetWorkspaceResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.get_workspace_storage_stats_response import GetWorkspaceStorageStatsResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.internal_error_message_response import InternalErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.list_workspace_sharings_response import ListWorkspaceSharingsResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.list_workspaces_response import ListWorkspacesResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.not_found_error_message_response import NotFoundErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.rate_limit_error_message_response import RateLimitErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.remove_workspace_sharing_response import RemoveWorkspaceSharingResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.share_workspace_body import ShareWorkspaceBody  # noqa: E501
from playbookmedia_backend_server_sdk.models.share_workspace_response import ShareWorkspaceResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.status import Status  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_account_request import UpdateAccountRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_account_response import UpdateAccountResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_workspace_request1 import UpdateWorkspaceRequest1  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_workspace_response import UpdateWorkspaceResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_workspace_sharing_request import UpdateWorkspaceSharingRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_workspace_sharing_response import UpdateWorkspaceSharingResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.validation_error_message_response import ValidationErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk import util


def create_account(create_account_request):  # noqa: E501
    """Create a new account

    Creates a new user account with initial workspace # noqa: E501

    :param create_account_request: 
    :type create_account_request: dict | bytes

    :rtype: Union[CreateAccountResponse, Tuple[CreateAccountResponse, int], Tuple[CreateAccountResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        create_account_request = CreateAccountRequest.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def create_workspace(create_workspace_request):  # noqa: E501
    """Create workspace

     # noqa: E501

    :param create_workspace_request: 
    :type create_workspace_request: dict | bytes

    :rtype: Union[CreateWorkspaceResponse, Tuple[CreateWorkspaceResponse, int], Tuple[CreateWorkspaceResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        create_workspace_request = CreateWorkspaceRequest.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def delete_account(id):  # noqa: E501
    """Delete account

     # noqa: E501

    :param id: 
    :type id: str

    :rtype: Union[DeleteAccountResponse, Tuple[DeleteAccountResponse, int], Tuple[DeleteAccountResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def delete_workspace1(id):  # noqa: E501
    """Delete workspace

     # noqa: E501

    :param id: 
    :type id: str

    :rtype: Union[DeleteWorkspaceResponse, Tuple[DeleteWorkspaceResponse, int], Tuple[DeleteWorkspaceResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def get_account(id):  # noqa: E501
    """Get account details

     # noqa: E501

    :param id: 
    :type id: str

    :rtype: Union[GetAccountResponse, Tuple[GetAccountResponse, int], Tuple[GetAccountResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def get_workspace(id):  # noqa: E501
    """Get workspace details

     # noqa: E501

    :param id: 
    :type id: str

    :rtype: Union[GetWorkspaceResponse, Tuple[GetWorkspaceResponse, int], Tuple[GetWorkspaceResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def get_workspace_analytics(workspace_id, start_time=None, end_time=None):  # noqa: E501
    """Get workspace analytics

     # noqa: E501

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


def get_workspace_compliance_report(workspace_id, compliance_type=None):  # noqa: E501
    """Get compliance report

     # noqa: E501

    :param workspace_id: 
    :type workspace_id: str
    :param compliance_type: 
    :type compliance_type: str

    :rtype: Union[GetWorkspaceComplianceReportResponse, Tuple[GetWorkspaceComplianceReportResponse, int], Tuple[GetWorkspaceComplianceReportResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def get_workspace_storage_stats(workspace_id):  # noqa: E501
    """Get storage statistics

     # noqa: E501

    :param workspace_id: 
    :type workspace_id: str

    :rtype: Union[GetWorkspaceStorageStatsResponse, Tuple[GetWorkspaceStorageStatsResponse, int], Tuple[GetWorkspaceStorageStatsResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def list_workspace_sharings(workspace_id, page_size=None, page_token=None):  # noqa: E501
    """List workspace sharings

     # noqa: E501

    :param workspace_id: 
    :type workspace_id: str
    :param page_size: 
    :type page_size: int
    :param page_token: 
    :type page_token: str

    :rtype: Union[ListWorkspaceSharingsResponse, Tuple[ListWorkspaceSharingsResponse, int], Tuple[ListWorkspaceSharingsResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def list_workspaces(account_id=None, page_size=None, page_token=None, filter=None):  # noqa: E501
    """List workspaces

     # noqa: E501

    :param account_id: 
    :type account_id: str
    :param page_size: 
    :type page_size: int
    :param page_token: 
    :type page_token: str
    :param filter: 
    :type filter: str

    :rtype: Union[ListWorkspacesResponse, Tuple[ListWorkspacesResponse, int], Tuple[ListWorkspacesResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def remove_workspace_sharing(sharing_id):  # noqa: E501
    """Remove workspace sharing

     # noqa: E501

    :param sharing_id: 
    :type sharing_id: str

    :rtype: Union[RemoveWorkspaceSharingResponse, Tuple[RemoveWorkspaceSharingResponse, int], Tuple[RemoveWorkspaceSharingResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def share_workspace(workspace_id, share_workspace_body):  # noqa: E501
    """Share workspace

     # noqa: E501

    :param workspace_id: 
    :type workspace_id: str
    :param share_workspace_body: 
    :type share_workspace_body: dict | bytes

    :rtype: Union[ShareWorkspaceResponse, Tuple[ShareWorkspaceResponse, int], Tuple[ShareWorkspaceResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        share_workspace_body = ShareWorkspaceBody.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def update_account(update_account_request):  # noqa: E501
    """Update account details

     # noqa: E501

    :param update_account_request: 
    :type update_account_request: dict | bytes

    :rtype: Union[UpdateAccountResponse, Tuple[UpdateAccountResponse, int], Tuple[UpdateAccountResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        update_account_request = UpdateAccountRequest.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def update_workspace1(update_workspace_request1):  # noqa: E501
    """Update workspace

     # noqa: E501

    :param update_workspace_request1: 
    :type update_workspace_request1: dict | bytes

    :rtype: Union[UpdateWorkspaceResponse, Tuple[UpdateWorkspaceResponse, int], Tuple[UpdateWorkspaceResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        update_workspace_request1 = UpdateWorkspaceRequest1.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def update_workspace_sharing(update_workspace_sharing_request):  # noqa: E501
    """Update workspace sharing

     # noqa: E501

    :param update_workspace_sharing_request: 
    :type update_workspace_sharing_request: dict | bytes

    :rtype: Union[UpdateWorkspaceSharingResponse, Tuple[UpdateWorkspaceSharingResponse, int], Tuple[UpdateWorkspaceSharingResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        update_workspace_sharing_request = UpdateWorkspaceSharingRequest.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'
