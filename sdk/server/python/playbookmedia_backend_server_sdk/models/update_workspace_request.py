from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk.models.workspace1 import Workspace1
from playbookmedia_backend_server_sdk import util

from playbookmedia_backend_server_sdk.models.workspace1 import Workspace1  # noqa: E501

class UpdateWorkspaceRequest(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, workspace=None):  # noqa: E501
        """UpdateWorkspaceRequest - a model defined in OpenAPI

        :param workspace: The workspace of this UpdateWorkspaceRequest.  # noqa: E501
        :type workspace: Workspace1
        """
        self.openapi_types = {
            'workspace': Workspace1
        }

        self.attribute_map = {
            'workspace': 'workspace'
        }

        self._workspace = workspace

    @classmethod
    def from_dict(cls, dikt) -> 'UpdateWorkspaceRequest':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The UpdateWorkspaceRequest of this UpdateWorkspaceRequest.  # noqa: E501
        :rtype: UpdateWorkspaceRequest
        """
        return util.deserialize_model(dikt, cls)

    @property
    def workspace(self) -> Workspace1:
        """Gets the workspace of this UpdateWorkspaceRequest.


        :return: The workspace of this UpdateWorkspaceRequest.
        :rtype: Workspace1
        """
        return self._workspace

    @workspace.setter
    def workspace(self, workspace: Workspace1):
        """Sets the workspace of this UpdateWorkspaceRequest.


        :param workspace: The workspace of this UpdateWorkspaceRequest.
        :type workspace: Workspace1
        """

        self._workspace = workspace
