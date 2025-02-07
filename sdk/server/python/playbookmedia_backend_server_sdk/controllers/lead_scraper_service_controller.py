import connexion
from typing import Dict
from typing import Tuple
from typing import Union

from playbookmedia_backend_server_sdk.models.authentication_error_message_response import AuthenticationErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.bad_gateway_error_message_response import BadGatewayErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.conflict_error_message_response import ConflictErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_organization_request import CreateOrganizationRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_organization_response import CreateOrganizationResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_tenant_body import CreateTenantBody  # noqa: E501
from playbookmedia_backend_server_sdk.models.create_tenant_response import CreateTenantResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.delete_organization_response import DeleteOrganizationResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.delete_tenant_response import DeleteTenantResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.forbidden_error_message_response import ForbiddenErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.gateway_timeout_error_message_response import GatewayTimeoutErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.get_organization_response import GetOrganizationResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.get_tenant_response import GetTenantResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.gone_error_message_response import GoneErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.internal_error_message_response import InternalErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.list_organizations_response import ListOrganizationsResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.list_tenants_response import ListTenantsResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.method_not_allowed_error_message_response import MethodNotAllowedErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.not_found_error_message_response import NotFoundErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.not_implemented_error_message_response import NotImplementedErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.payment_required_error_message_response import PaymentRequiredErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.precondition_failed_error_message_response import PreconditionFailedErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.rate_limit_error_message_response import RateLimitErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.rpc_status import RpcStatus  # noqa: E501
from playbookmedia_backend_server_sdk.models.service_unavailable_error_message_response import ServiceUnavailableErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.too_early_error_message_response import TooEarlyErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.unprocessable_entity_error_message_response import UnprocessableEntityErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_organization_request import UpdateOrganizationRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_organization_response import UpdateOrganizationResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_tenant_request import UpdateTenantRequest  # noqa: E501
from playbookmedia_backend_server_sdk.models.update_tenant_response import UpdateTenantResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.validation_error_message_response import ValidationErrorMessageResponse  # noqa: E501
from playbookmedia_backend_server_sdk import util


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
