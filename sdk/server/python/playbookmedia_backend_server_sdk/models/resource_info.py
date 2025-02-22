from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk import util


class ResourceInfo(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, type=None, id=None, path=None, tenant_id=None, scopes=None):  # noqa: E501
        """ResourceInfo - a model defined in OpenAPI

        :param type: The type of this ResourceInfo.  # noqa: E501
        :type type: str
        :param id: The id of this ResourceInfo.  # noqa: E501
        :type id: str
        :param path: The path of this ResourceInfo.  # noqa: E501
        :type path: str
        :param tenant_id: The tenant_id of this ResourceInfo.  # noqa: E501
        :type tenant_id: str
        :param scopes: The scopes of this ResourceInfo.  # noqa: E501
        :type scopes: List[str]
        """
        self.openapi_types = {
            'type': str,
            'id': str,
            'path': str,
            'tenant_id': str,
            'scopes': List[str]
        }

        self.attribute_map = {
            'type': 'type',
            'id': 'id',
            'path': 'path',
            'tenant_id': 'tenantId',
            'scopes': 'scopes'
        }

        self._type = type
        self._id = id
        self._path = path
        self._tenant_id = tenant_id
        self._scopes = scopes

    @classmethod
    def from_dict(cls, dikt) -> 'ResourceInfo':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The ResourceInfo of this ResourceInfo.  # noqa: E501
        :rtype: ResourceInfo
        """
        return util.deserialize_model(dikt, cls)

    @property
    def type(self) -> str:
        """Gets the type of this ResourceInfo.


        :return: The type of this ResourceInfo.
        :rtype: str
        """
        return self._type

    @type.setter
    def type(self, type: str):
        """Sets the type of this ResourceInfo.


        :param type: The type of this ResourceInfo.
        :type type: str
        """

        self._type = type

    @property
    def id(self) -> str:
        """Gets the id of this ResourceInfo.


        :return: The id of this ResourceInfo.
        :rtype: str
        """
        return self._id

    @id.setter
    def id(self, id: str):
        """Sets the id of this ResourceInfo.


        :param id: The id of this ResourceInfo.
        :type id: str
        """

        self._id = id

    @property
    def path(self) -> str:
        """Gets the path of this ResourceInfo.


        :return: The path of this ResourceInfo.
        :rtype: str
        """
        return self._path

    @path.setter
    def path(self, path: str):
        """Sets the path of this ResourceInfo.


        :param path: The path of this ResourceInfo.
        :type path: str
        """

        self._path = path

    @property
    def tenant_id(self) -> str:
        """Gets the tenant_id of this ResourceInfo.


        :return: The tenant_id of this ResourceInfo.
        :rtype: str
        """
        return self._tenant_id

    @tenant_id.setter
    def tenant_id(self, tenant_id: str):
        """Sets the tenant_id of this ResourceInfo.


        :param tenant_id: The tenant_id of this ResourceInfo.
        :type tenant_id: str
        """

        self._tenant_id = tenant_id

    @property
    def scopes(self) -> List[str]:
        """Gets the scopes of this ResourceInfo.


        :return: The scopes of this ResourceInfo.
        :rtype: List[str]
        """
        return self._scopes

    @scopes.setter
    def scopes(self, scopes: List[str]):
        """Sets the scopes of this ResourceInfo.


        :param scopes: The scopes of this ResourceInfo.
        :type scopes: List[str]
        """

        self._scopes = scopes
