import connexion
from typing import Dict
from typing import Tuple
from typing import Union

from playbookmedia_backend_server_sdk.models.authentication_error_message_response import AuthenticationErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.bad_gateway_error_message_response import BadGatewayErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.conflict_error_message_response import ConflictErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_api_key_request import CreateAPIKeyRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_api_key_response import CreateAPIKeyResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_organization_request import CreateOrganizationRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_organization_response import CreateOrganizationResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_tenant_api_key_request import CreateTenantAPIKeyRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_tenant_api_key_response import CreateTenantAPIKeyResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_tenant_body import CreateTenantBody  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_tenant_response import CreateTenantResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_webhook_request import CreateWebhookRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_webhook_response import CreateWebhookResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.delete_api_key_response import DeleteAPIKeyResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.delete_organization_response import DeleteOrganizationResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.delete_tenant_api_key_response import DeleteTenantAPIKeyResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.delete_tenant_response import DeleteTenantResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.delete_webhook_response import DeleteWebhookResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.delete_workspace_response import DeleteWorkspaceResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.forbidden_error_message_response import ForbiddenErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.gateway_timeout_error_message_response import GatewayTimeoutErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.get_api_key_response import GetAPIKeyResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.get_lead_response import GetLeadResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.get_organization_response import GetOrganizationResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.get_tenant_api_key_response import GetTenantAPIKeyResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.get_tenant_response import GetTenantResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.get_webhook_response import GetWebhookResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.get_workflow_response import GetWorkflowResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.get_workspace_analytics_response import GetWorkspaceAnalyticsResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.get_workspace_response import GetWorkspaceResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.gone_error_message_response import GoneErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.internal_error_message_response import InternalErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.list_api_keys_response import ListAPIKeysResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.list_leads_response import ListLeadsResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.list_organizations_response import ListOrganizationsResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.list_tenant_api_keys_response import ListTenantAPIKeysResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.list_tenants_response import ListTenantsResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.list_webhooks_response import ListWebhooksResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.list_workflows_response import ListWorkflowsResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.method_not_allowed_error_message_response import MethodNotAllowedErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.not_found_error_message_response import NotFoundErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.not_implemented_error_message_response import NotImplementedErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.pause_workflow_body import PauseWorkflowBody  # noqa: E501
from playbookmedia_backend_server_sdk.models.pause_workflow_response import PauseWorkflowResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.payment_required_error_message_response import PaymentRequiredErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.precondition_failed_error_message_response import PreconditionFailedErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.rate_limit_error_message_response import RateLimitErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.rotate_api_key_request import RotateAPIKeyRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.rotate_api_key_response import RotateAPIKeyResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.rotate_tenant_api_key_request import RotateTenantAPIKeyRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.rotate_tenant_api_key_response import RotateTenantAPIKeyResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.rpc_status import RpcStatus  # noqa: E501
from playbookmedia_backend_server_sdk.models.service_unavailable_error_message_response import ServiceUnavailableErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.too_early_error_message_response import TooEarlyErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.trigger_workflow_body import TriggerWorkflowBody  # noqa: E501
from playbookmedia_backend_server_sdk.models.trigger_workflow_response import TriggerWorkflowResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.unprocessable_entity_error_message_response import UnprocessableEntityErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_api_key_request import UpdateAPIKeyRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_api_key_response import UpdateAPIKeyResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_organization_request import UpdateOrganizationRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_organization_response import UpdateOrganizationResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_tenant_api_key_request import UpdateTenantAPIKeyRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_tenant_api_key_response import UpdateTenantAPIKeyResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_tenant_request import UpdateTenantRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_tenant_response import UpdateTenantResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_webhook_request import UpdateWebhookRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_webhook_response import UpdateWebhookResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_workflow_request import UpdateWorkflowRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_workflow_response import UpdateWorkflowResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_workspace_request import UpdateWorkspaceRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_workspace_response import UpdateWorkspaceResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.validation_error_message_response import ValidationErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk import util


def create_api_key(create_api_key_request):  # noqa: E501
    """Create a new API key

    Creates a new API key with specified permissions and settings # noqa: E501

    :param create_api_key_request: 
    :type create_api_key_request: dict | bytes

    :rtype: Union[CreateAPIKeyResponse, Tuple[CreateAPIKeyResponse, int], Tuple[CreateAPIKeyResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        create_api_key_request = CreateAPIKeyRequest.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def create_organization(create_organization_request):  # noqa: E501
    """Create a new organization

    Creates a new organization within a tenant # noqa: E501

    :param create_organization_request: 
    :type create_organization_request: dict | bytes

    :rtype: Union[CreateOrganizationResponse, Tuple[CreateOrganizationResponse, int], Tuple[CreateOrganizationResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        create_organization_request = CreateOrganizationRequest.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def create_tenant(organization_id, create_tenant_body):  # noqa: E501
    """Create a new tenant

    Creates a new tenant in the system # noqa: E501

    :param organization_id: 
    :type organization_id: str
    :param create_tenant_body: 
    :type create_tenant_body: dict | bytes

    :rtype: Union[CreateTenantResponse, Tuple[CreateTenantResponse, int], Tuple[CreateTenantResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        create_tenant_body = CreateTenantBody.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def create_tenant_api_key(create_tenant_api_key_request):  # noqa: E501
    """Create a new tenant API key

    Creates a new API key for a tenant with specified permissions # noqa: E501

    :param create_tenant_api_key_request: 
    :type create_tenant_api_key_request: dict | bytes

    :rtype: Union[CreateTenantAPIKeyResponse, Tuple[CreateTenantAPIKeyResponse, int], Tuple[CreateTenantAPIKeyResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        create_tenant_api_key_request = CreateTenantAPIKeyRequest.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def create_webhook(create_webhook_request):  # noqa: E501
    """Create webhook

    Creates a new webhook configuration for event notifications # noqa: E501

    :param create_webhook_request: 
    :type create_webhook_request: dict | bytes

    :rtype: Union[CreateWebhookResponse, Tuple[CreateWebhookResponse, int], Tuple[CreateWebhookResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        create_webhook_request = CreateWebhookRequest.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def delete_api_key(key_id, organization_id=None, workspace_id=None, tenant_id=None, account_id=None):  # noqa: E501
    """Delete API key

    Permanently deletes an API key # noqa: E501

    :param key_id: 
    :type key_id: str
    :param organization_id: 
    :type organization_id: str
    :param workspace_id: 
    :type workspace_id: str
    :param tenant_id: 
    :type tenant_id: str
    :param account_id: 
    :type account_id: str

    :rtype: Union[DeleteAPIKeyResponse, Tuple[DeleteAPIKeyResponse, int], Tuple[DeleteAPIKeyResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def delete_organization(id):  # noqa: E501
    """Delete an organization

    Permanently deletes an organization and all associated resources # noqa: E501

    :param id: 
    :type id: str

    :rtype: Union[DeleteOrganizationResponse, Tuple[DeleteOrganizationResponse, int], Tuple[DeleteOrganizationResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def delete_tenant(organization_id, tenant_id):  # noqa: E501
    """Delete a tenant

    Permanently deletes a tenant and all associated resources # noqa: E501

    :param organization_id: 
    :type organization_id: str
    :param tenant_id: 
    :type tenant_id: str

    :rtype: Union[DeleteTenantResponse, Tuple[DeleteTenantResponse, int], Tuple[DeleteTenantResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def delete_tenant_api_key(key_id, organization_id=None, tenant_id=None):  # noqa: E501
    """Delete tenant API key

    Permanently deletes a tenant API key # noqa: E501

    :param key_id: 
    :type key_id: str
    :param organization_id: 
    :type organization_id: str
    :param tenant_id: 
    :type tenant_id: str

    :rtype: Union[DeleteTenantAPIKeyResponse, Tuple[DeleteTenantAPIKeyResponse, int], Tuple[DeleteTenantAPIKeyResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def delete_webhook(webhook_id, organization_id=None, workspace_id=None, tenant_id=None, account_id=None):  # noqa: E501
    """Delete webhook

    Deletes a webhook configuration # noqa: E501

    :param webhook_id: 
    :type webhook_id: str
    :param organization_id: 
    :type organization_id: str
    :param workspace_id: 
    :type workspace_id: str
    :param tenant_id: 
    :type tenant_id: str
    :param account_id: 
    :type account_id: str

    :rtype: Union[DeleteWebhookResponse, Tuple[DeleteWebhookResponse, int], Tuple[DeleteWebhookResponse, int, Dict[str, str]]
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


def get_api_key(key_id, organization_id=None, tenant_id=None, account_id=None, workspace_id=None):  # noqa: E501
    """Get API key details

    Retrieves details of a specific API key # noqa: E501

    :param key_id: 
    :type key_id: str
    :param organization_id: 
    :type organization_id: str
    :param tenant_id: 
    :type tenant_id: str
    :param account_id: 
    :type account_id: str
    :param workspace_id: 
    :type workspace_id: str

    :rtype: Union[GetAPIKeyResponse, Tuple[GetAPIKeyResponse, int], Tuple[GetAPIKeyResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def get_lead(lead_id, organization_id=None, workspace_id=None, tenant_id=None, account_id=None):  # noqa: E501
    """Get lead details

    Retrieves detailed information about a specific lead # noqa: E501

    :param lead_id: 
    :type lead_id: str
    :param organization_id: 
    :type organization_id: str
    :param workspace_id: 
    :type workspace_id: str
    :param tenant_id: 
    :type tenant_id: str
    :param account_id: 
    :type account_id: str

    :rtype: Union[GetLeadResponse, Tuple[GetLeadResponse, int], Tuple[GetLeadResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def get_organization(id):  # noqa: E501
    """Get organization details

    Retrieves details of a specific organization # noqa: E501

    :param id: 
    :type id: str

    :rtype: Union[GetOrganizationResponse, Tuple[GetOrganizationResponse, int], Tuple[GetOrganizationResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def get_tenant(organization_id, tenant_id):  # noqa: E501
    """Get tenant details

    Retrieves details of a specific tenant # noqa: E501

    :param organization_id: 
    :type organization_id: str
    :param tenant_id: 
    :type tenant_id: str

    :rtype: Union[GetTenantResponse, Tuple[GetTenantResponse, int], Tuple[GetTenantResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def get_tenant_api_key(key_id, organization_id=None, tenant_id=None):  # noqa: E501
    """Get tenant API key details

    Retrieves details of a specific tenant API key # noqa: E501

    :param key_id: 
    :type key_id: str
    :param organization_id: 
    :type organization_id: str
    :param tenant_id: 
    :type tenant_id: str

    :rtype: Union[GetTenantAPIKeyResponse, Tuple[GetTenantAPIKeyResponse, int], Tuple[GetTenantAPIKeyResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def get_webhook(webhook_id, organization_id=None, workspace_id=None, tenant_id=None, account_id=None):  # noqa: E501
    """Get webhook

    Retrieves details of a specific webhook configuration # noqa: E501

    :param webhook_id: 
    :type webhook_id: str
    :param organization_id: 
    :type organization_id: str
    :param workspace_id: 
    :type workspace_id: str
    :param tenant_id: 
    :type tenant_id: str
    :param account_id: 
    :type account_id: str

    :rtype: Union[GetWebhookResponse, Tuple[GetWebhookResponse, int], Tuple[GetWebhookResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def get_workflow(workspace_id, id):  # noqa: E501
    """Get workflow details

    Retrieves details of a specific workflow # noqa: E501

    :param workspace_id: 
    :type workspace_id: str
    :param id: 
    :type id: str

    :rtype: Union[GetWorkflowResponse, Tuple[GetWorkflowResponse, int], Tuple[GetWorkflowResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def get_workspace(id, organization_id=None, tenant_id=None, account_id=None):  # noqa: E501
    """Get workspace details

    Retrieves details of a specific workspace # noqa: E501

    :param id: 
    :type id: str
    :param organization_id: 
    :type organization_id: str
    :param tenant_id: 
    :type tenant_id: str
    :param account_id: 
    :type account_id: str

    :rtype: Union[GetWorkspaceResponse, Tuple[GetWorkspaceResponse, int], Tuple[GetWorkspaceResponse, int, Dict[str, str]]
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


def list_api_keys(organization_id=None, tenant_id=None, account_id=None, workspace_id=None, page_size=None, page_number=None, status=None, search=None):  # noqa: E501
    """List API keys

    Retrieves a list of API keys with optional filtering # noqa: E501

    :param organization_id: 
    :type organization_id: str
    :param tenant_id: 
    :type tenant_id: str
    :param account_id: 
    :type account_id: str
    :param workspace_id: 
    :type workspace_id: str
    :param page_size: 
    :type page_size: int
    :param page_number: 
    :type page_number: int
    :param status: 
    :type status: str
    :param search: 
    :type search: str

    :rtype: Union[ListAPIKeysResponse, Tuple[ListAPIKeysResponse, int], Tuple[ListAPIKeysResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def list_leads(organization_id=None, workspace_id=None, tenant_id=None, account_id=None, page_size=None, page_number=None):  # noqa: E501
    """List leads

    Retrieves a paginated list of leads with comprehensive filtering options # noqa: E501

    :param organization_id: Context identifiers
    :type organization_id: str
    :param workspace_id: 
    :type workspace_id: str
    :param tenant_id: 
    :type tenant_id: str
    :param account_id: 
    :type account_id: str
    :param page_size: Pagination
    :type page_size: int
    :param page_number: 
    :type page_number: int

    :rtype: Union[ListLeadsResponse, Tuple[ListLeadsResponse, int], Tuple[ListLeadsResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def list_organizations(page_size=None, page_number=None):  # noqa: E501
    """List all organizations

    Retrieves a list of all organizations in a tenant # noqa: E501

    :param page_size: 
    :type page_size: int
    :param page_number: 
    :type page_number: int

    :rtype: Union[ListOrganizationsResponse, Tuple[ListOrganizationsResponse, int], Tuple[ListOrganizationsResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def list_tenant_api_keys(organization_id=None, tenant_id=None, page_size=None, page_number=None, status=None):  # noqa: E501
    """List tenant API keys

    Retrieves all API keys for a specific tenant # noqa: E501

    :param organization_id: 
    :type organization_id: str
    :param tenant_id: 
    :type tenant_id: str
    :param page_size: 
    :type page_size: int
    :param page_number: 
    :type page_number: int
    :param status: 
    :type status: str

    :rtype: Union[ListTenantAPIKeysResponse, Tuple[ListTenantAPIKeysResponse, int], Tuple[ListTenantAPIKeysResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def list_tenants(organization_id, page_size=None, page_number=None):  # noqa: E501
    """List all tenants

    Retrieves a list of all tenants in the system # noqa: E501

    :param organization_id: 
    :type organization_id: str
    :param page_size: 
    :type page_size: int
    :param page_number: 
    :type page_number: int

    :rtype: Union[ListTenantsResponse, Tuple[ListTenantsResponse, int], Tuple[ListTenantsResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def list_webhooks(organization_id=None, workspace_id=None, tenant_id=None, account_id=None, page_size=None, page_number=None, status=None, search=None):  # noqa: E501
    """List webhooks

    Lists all webhook configurations with pagination # noqa: E501

    :param organization_id: 
    :type organization_id: str
    :param workspace_id: 
    :type workspace_id: str
    :param tenant_id: 
    :type tenant_id: str
    :param account_id: 
    :type account_id: str
    :param page_size: 
    :type page_size: int
    :param page_number: 
    :type page_number: int
    :param status: 
    :type status: str
    :param search: 
    :type search: str

    :rtype: Union[ListWebhooksResponse, Tuple[ListWebhooksResponse, int], Tuple[ListWebhooksResponse, int, Dict[str, str]]
    """
    return 'do some magic!'


def list_workflows(workspace_id, page_size=None, page_number=None, filter=None, organization_id=None, tenant_id=None, account_id=None):  # noqa: E501
    """List workflows

    Retrieves a list of workflows for a given workspace # noqa: E501

    :param workspace_id: 
    :type workspace_id: str
    :param page_size: 
    :type page_size: int
    :param page_number: 
    :type page_number: int
    :param filter: 
    :type filter: str
    :param organization_id: 
    :type organization_id: str
    :param tenant_id: 
    :type tenant_id: str
    :param account_id: 
    :type account_id: str

    :rtype: Union[ListWorkflowsResponse, Tuple[ListWorkflowsResponse, int], Tuple[ListWorkflowsResponse, int, Dict[str, str]]
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


def rotate_api_key(rotate_api_key_request):  # noqa: E501
    """Rotate API key

    Rotates an API key while maintaining its configuration # noqa: E501

    :param rotate_api_key_request: 
    :type rotate_api_key_request: dict | bytes

    :rtype: Union[RotateAPIKeyResponse, Tuple[RotateAPIKeyResponse, int], Tuple[RotateAPIKeyResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        rotate_api_key_request = RotateAPIKeyRequest.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def rotate_tenant_api_key(rotate_tenant_api_key_request):  # noqa: E501
    """Rotate tenant API key

    Rotates a tenant API key while maintaining its configuration # noqa: E501

    :param rotate_tenant_api_key_request: 
    :type rotate_tenant_api_key_request: dict | bytes

    :rtype: Union[RotateTenantAPIKeyResponse, Tuple[RotateTenantAPIKeyResponse, int], Tuple[RotateTenantAPIKeyResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        rotate_tenant_api_key_request = RotateTenantAPIKeyRequest.from_dict(connexion.request.get_json())  # noqa: E501
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


def update_api_key(update_api_key_request):  # noqa: E501
    """Update API key

    Updates an existing API key&#39;s configuration # noqa: E501

    :param update_api_key_request: 
    :type update_api_key_request: dict | bytes

    :rtype: Union[UpdateAPIKeyResponse, Tuple[UpdateAPIKeyResponse, int], Tuple[UpdateAPIKeyResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        update_api_key_request = UpdateAPIKeyRequest.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def update_organization(update_organization_request):  # noqa: E501
    """Update organization details

    Updates the configuration of a specific organization # noqa: E501

    :param update_organization_request: 
    :type update_organization_request: dict | bytes

    :rtype: Union[UpdateOrganizationResponse, Tuple[UpdateOrganizationResponse, int], Tuple[UpdateOrganizationResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        update_organization_request = UpdateOrganizationRequest.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def update_tenant(update_tenant_request):  # noqa: E501
    """Update tenant details

    Updates the configuration of a specific tenant # noqa: E501

    :param update_tenant_request: 
    :type update_tenant_request: dict | bytes

    :rtype: Union[UpdateTenantResponse, Tuple[UpdateTenantResponse, int], Tuple[UpdateTenantResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        update_tenant_request = UpdateTenantRequest.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def update_tenant_api_key(update_tenant_api_key_request):  # noqa: E501
    """Update tenant API key

    Updates an existing tenant API key&#39;s configuration # noqa: E501

    :param update_tenant_api_key_request: 
    :type update_tenant_api_key_request: dict | bytes

    :rtype: Union[UpdateTenantAPIKeyResponse, Tuple[UpdateTenantAPIKeyResponse, int], Tuple[UpdateTenantAPIKeyResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        update_tenant_api_key_request = UpdateTenantAPIKeyRequest.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def update_webhook(update_webhook_request):  # noqa: E501
    """Update webhook

    Updates an existing webhook configuration # noqa: E501

    :param update_webhook_request: 
    :type update_webhook_request: dict | bytes

    :rtype: Union[UpdateWebhookResponse, Tuple[UpdateWebhookResponse, int], Tuple[UpdateWebhookResponse, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        update_webhook_request = UpdateWebhookRequest.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def update_workflow(update_workflow_request):  # noqa: E501
    """Update workflow details

    Updates the details of a specific workflow # noqa: E501

    :param update_workflow_request: 
    :type update_workflow_request: dict | bytes

    :rtype: Union[UpdateWorkflowResponse, Tuple[UpdateWorkflowResponse, int], Tuple[UpdateWorkflowResponse, int, Dict[str, str]]
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
