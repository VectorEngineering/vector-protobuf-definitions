from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk.models.tenant import Tenant
from playbookmedia_backend_server_sdk import util

from playbookmedia_backend_server_sdk.models.tenant import Tenant  # noqa: E501

class GetTenantResponse(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, tenant=None):  # noqa: E501
        """GetTenantResponse - a model defined in OpenAPI

        :param tenant: The tenant of this GetTenantResponse.  # noqa: E501
        :type tenant: Tenant
        """
        self.openapi_types = {
            'tenant': Tenant
        }

        self.attribute_map = {
            'tenant': 'tenant'
        }

        self._tenant = tenant

    @classmethod
    def from_dict(cls, dikt) -> 'GetTenantResponse':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The GetTenantResponse of this GetTenantResponse.  # noqa: E501
        :rtype: GetTenantResponse
        """
        return util.deserialize_model(dikt, cls)

    @property
    def tenant(self) -> Tenant:
        """Gets the tenant of this GetTenantResponse.


        :return: The tenant of this GetTenantResponse.
        :rtype: Tenant
        """
        return self._tenant

    @tenant.setter
    def tenant(self, tenant: Tenant):
        """Sets the tenant of this GetTenantResponse.


        :param tenant: The tenant of this GetTenantResponse.
        :type tenant: Tenant
        """

        self._tenant = tenant
