from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk.models.workspace1 import Workspace1
from playbookmedia_backend_server_sdk import util

from playbookmedia_backend_server_sdk.models.workspace1 import Workspace1  # noqa: E501

class ListWorkspacesResponse1(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, workspaces=None, next_page_token=None, total_size=None):  # noqa: E501
        """ListWorkspacesResponse1 - a model defined in OpenAPI

        :param workspaces: The workspaces of this ListWorkspacesResponse1.  # noqa: E501
        :type workspaces: List[Workspace1]
        :param next_page_token: The next_page_token of this ListWorkspacesResponse1.  # noqa: E501
        :type next_page_token: str
        :param total_size: The total_size of this ListWorkspacesResponse1.  # noqa: E501
        :type total_size: int
        """
        self.openapi_types = {
            'workspaces': List[Workspace1],
            'next_page_token': str,
            'total_size': int
        }

        self.attribute_map = {
            'workspaces': 'workspaces',
            'next_page_token': 'nextPageToken',
            'total_size': 'totalSize'
        }

        self._workspaces = workspaces
        self._next_page_token = next_page_token
        self._total_size = total_size

    @classmethod
    def from_dict(cls, dikt) -> 'ListWorkspacesResponse1':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The ListWorkspacesResponse1 of this ListWorkspacesResponse1.  # noqa: E501
        :rtype: ListWorkspacesResponse1
        """
        return util.deserialize_model(dikt, cls)

    @property
    def workspaces(self) -> List[Workspace1]:
        """Gets the workspaces of this ListWorkspacesResponse1.


        :return: The workspaces of this ListWorkspacesResponse1.
        :rtype: List[Workspace1]
        """
        return self._workspaces

    @workspaces.setter
    def workspaces(self, workspaces: List[Workspace1]):
        """Sets the workspaces of this ListWorkspacesResponse1.


        :param workspaces: The workspaces of this ListWorkspacesResponse1.
        :type workspaces: List[Workspace1]
        """

        self._workspaces = workspaces

    @property
    def next_page_token(self) -> str:
        """Gets the next_page_token of this ListWorkspacesResponse1.


        :return: The next_page_token of this ListWorkspacesResponse1.
        :rtype: str
        """
        return self._next_page_token

    @next_page_token.setter
    def next_page_token(self, next_page_token: str):
        """Sets the next_page_token of this ListWorkspacesResponse1.


        :param next_page_token: The next_page_token of this ListWorkspacesResponse1.
        :type next_page_token: str
        """

        self._next_page_token = next_page_token

    @property
    def total_size(self) -> int:
        """Gets the total_size of this ListWorkspacesResponse1.


        :return: The total_size of this ListWorkspacesResponse1.
        :rtype: int
        """
        return self._total_size

    @total_size.setter
    def total_size(self, total_size: int):
        """Sets the total_size of this ListWorkspacesResponse1.


        :param total_size: The total_size of this ListWorkspacesResponse1.
        :type total_size: int
        """

        self._total_size = total_size
