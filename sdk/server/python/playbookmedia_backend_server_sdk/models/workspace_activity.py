from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk import util


class WorkspaceActivity(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, id=None, activity_type=None, user_id=None, description=None, metadata=None, created_at=None):  # noqa: E501
        """WorkspaceActivity - a model defined in OpenAPI

        :param id: The id of this WorkspaceActivity.  # noqa: E501
        :type id: str
        :param activity_type: The activity_type of this WorkspaceActivity.  # noqa: E501
        :type activity_type: str
        :param user_id: The user_id of this WorkspaceActivity.  # noqa: E501
        :type user_id: str
        :param description: The description of this WorkspaceActivity.  # noqa: E501
        :type description: str
        :param metadata: The metadata of this WorkspaceActivity.  # noqa: E501
        :type metadata: Dict[str, str]
        :param created_at: The created_at of this WorkspaceActivity.  # noqa: E501
        :type created_at: datetime
        """
        self.openapi_types = {
            'id': str,
            'activity_type': str,
            'user_id': str,
            'description': str,
            'metadata': Dict[str, str],
            'created_at': datetime
        }

        self.attribute_map = {
            'id': 'id',
            'activity_type': 'activityType',
            'user_id': 'userId',
            'description': 'description',
            'metadata': 'metadata',
            'created_at': 'createdAt'
        }

        self._id = id
        self._activity_type = activity_type
        self._user_id = user_id
        self._description = description
        self._metadata = metadata
        self._created_at = created_at

    @classmethod
    def from_dict(cls, dikt) -> 'WorkspaceActivity':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The WorkspaceActivity of this WorkspaceActivity.  # noqa: E501
        :rtype: WorkspaceActivity
        """
        return util.deserialize_model(dikt, cls)

    @property
    def id(self) -> str:
        """Gets the id of this WorkspaceActivity.


        :return: The id of this WorkspaceActivity.
        :rtype: str
        """
        return self._id

    @id.setter
    def id(self, id: str):
        """Sets the id of this WorkspaceActivity.


        :param id: The id of this WorkspaceActivity.
        :type id: str
        """

        self._id = id

    @property
    def activity_type(self) -> str:
        """Gets the activity_type of this WorkspaceActivity.


        :return: The activity_type of this WorkspaceActivity.
        :rtype: str
        """
        return self._activity_type

    @activity_type.setter
    def activity_type(self, activity_type: str):
        """Sets the activity_type of this WorkspaceActivity.


        :param activity_type: The activity_type of this WorkspaceActivity.
        :type activity_type: str
        """

        self._activity_type = activity_type

    @property
    def user_id(self) -> str:
        """Gets the user_id of this WorkspaceActivity.


        :return: The user_id of this WorkspaceActivity.
        :rtype: str
        """
        return self._user_id

    @user_id.setter
    def user_id(self, user_id: str):
        """Sets the user_id of this WorkspaceActivity.


        :param user_id: The user_id of this WorkspaceActivity.
        :type user_id: str
        """

        self._user_id = user_id

    @property
    def description(self) -> str:
        """Gets the description of this WorkspaceActivity.


        :return: The description of this WorkspaceActivity.
        :rtype: str
        """
        return self._description

    @description.setter
    def description(self, description: str):
        """Sets the description of this WorkspaceActivity.


        :param description: The description of this WorkspaceActivity.
        :type description: str
        """

        self._description = description

    @property
    def metadata(self) -> Dict[str, str]:
        """Gets the metadata of this WorkspaceActivity.


        :return: The metadata of this WorkspaceActivity.
        :rtype: Dict[str, str]
        """
        return self._metadata

    @metadata.setter
    def metadata(self, metadata: Dict[str, str]):
        """Sets the metadata of this WorkspaceActivity.


        :param metadata: The metadata of this WorkspaceActivity.
        :type metadata: Dict[str, str]
        """

        self._metadata = metadata

    @property
    def created_at(self) -> datetime:
        """Gets the created_at of this WorkspaceActivity.


        :return: The created_at of this WorkspaceActivity.
        :rtype: datetime
        """
        return self._created_at

    @created_at.setter
    def created_at(self, created_at: datetime):
        """Sets the created_at of this WorkspaceActivity.


        :param created_at: The created_at of this WorkspaceActivity.
        :type created_at: datetime
        """

        self._created_at = created_at
