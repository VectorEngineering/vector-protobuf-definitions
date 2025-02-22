from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk import util


class AppPermission(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, id=None, app_id=None, scope=None, description=None, is_required=None, access_levels=None, constraints=None, requires_approval=None, created_at=None, updated_at=None):  # noqa: E501
        """AppPermission - a model defined in OpenAPI

        :param id: The id of this AppPermission.  # noqa: E501
        :type id: str
        :param app_id: The app_id of this AppPermission.  # noqa: E501
        :type app_id: str
        :param scope: The scope of this AppPermission.  # noqa: E501
        :type scope: str
        :param description: The description of this AppPermission.  # noqa: E501
        :type description: str
        :param is_required: The is_required of this AppPermission.  # noqa: E501
        :type is_required: bool
        :param access_levels: The access_levels of this AppPermission.  # noqa: E501
        :type access_levels: List[str]
        :param constraints: The constraints of this AppPermission.  # noqa: E501
        :type constraints: Dict[str, str]
        :param requires_approval: The requires_approval of this AppPermission.  # noqa: E501
        :type requires_approval: bool
        :param created_at: The created_at of this AppPermission.  # noqa: E501
        :type created_at: datetime
        :param updated_at: The updated_at of this AppPermission.  # noqa: E501
        :type updated_at: datetime
        """
        self.openapi_types = {
            'id': str,
            'app_id': str,
            'scope': str,
            'description': str,
            'is_required': bool,
            'access_levels': List[str],
            'constraints': Dict[str, str],
            'requires_approval': bool,
            'created_at': datetime,
            'updated_at': datetime
        }

        self.attribute_map = {
            'id': 'id',
            'app_id': 'appId',
            'scope': 'scope',
            'description': 'description',
            'is_required': 'isRequired',
            'access_levels': 'accessLevels',
            'constraints': 'constraints',
            'requires_approval': 'requiresApproval',
            'created_at': 'createdAt',
            'updated_at': 'updatedAt'
        }

        self._id = id
        self._app_id = app_id
        self._scope = scope
        self._description = description
        self._is_required = is_required
        self._access_levels = access_levels
        self._constraints = constraints
        self._requires_approval = requires_approval
        self._created_at = created_at
        self._updated_at = updated_at

    @classmethod
    def from_dict(cls, dikt) -> 'AppPermission':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The AppPermission of this AppPermission.  # noqa: E501
        :rtype: AppPermission
        """
        return util.deserialize_model(dikt, cls)

    @property
    def id(self) -> str:
        """Gets the id of this AppPermission.


        :return: The id of this AppPermission.
        :rtype: str
        """
        return self._id

    @id.setter
    def id(self, id: str):
        """Sets the id of this AppPermission.


        :param id: The id of this AppPermission.
        :type id: str
        """

        self._id = id

    @property
    def app_id(self) -> str:
        """Gets the app_id of this AppPermission.


        :return: The app_id of this AppPermission.
        :rtype: str
        """
        return self._app_id

    @app_id.setter
    def app_id(self, app_id: str):
        """Sets the app_id of this AppPermission.


        :param app_id: The app_id of this AppPermission.
        :type app_id: str
        """

        self._app_id = app_id

    @property
    def scope(self) -> str:
        """Gets the scope of this AppPermission.


        :return: The scope of this AppPermission.
        :rtype: str
        """
        return self._scope

    @scope.setter
    def scope(self, scope: str):
        """Sets the scope of this AppPermission.


        :param scope: The scope of this AppPermission.
        :type scope: str
        """

        self._scope = scope

    @property
    def description(self) -> str:
        """Gets the description of this AppPermission.


        :return: The description of this AppPermission.
        :rtype: str
        """
        return self._description

    @description.setter
    def description(self, description: str):
        """Sets the description of this AppPermission.


        :param description: The description of this AppPermission.
        :type description: str
        """

        self._description = description

    @property
    def is_required(self) -> bool:
        """Gets the is_required of this AppPermission.


        :return: The is_required of this AppPermission.
        :rtype: bool
        """
        return self._is_required

    @is_required.setter
    def is_required(self, is_required: bool):
        """Sets the is_required of this AppPermission.


        :param is_required: The is_required of this AppPermission.
        :type is_required: bool
        """

        self._is_required = is_required

    @property
    def access_levels(self) -> List[str]:
        """Gets the access_levels of this AppPermission.


        :return: The access_levels of this AppPermission.
        :rtype: List[str]
        """
        return self._access_levels

    @access_levels.setter
    def access_levels(self, access_levels: List[str]):
        """Sets the access_levels of this AppPermission.


        :param access_levels: The access_levels of this AppPermission.
        :type access_levels: List[str]
        """

        self._access_levels = access_levels

    @property
    def constraints(self) -> Dict[str, str]:
        """Gets the constraints of this AppPermission.


        :return: The constraints of this AppPermission.
        :rtype: Dict[str, str]
        """
        return self._constraints

    @constraints.setter
    def constraints(self, constraints: Dict[str, str]):
        """Sets the constraints of this AppPermission.


        :param constraints: The constraints of this AppPermission.
        :type constraints: Dict[str, str]
        """

        self._constraints = constraints

    @property
    def requires_approval(self) -> bool:
        """Gets the requires_approval of this AppPermission.


        :return: The requires_approval of this AppPermission.
        :rtype: bool
        """
        return self._requires_approval

    @requires_approval.setter
    def requires_approval(self, requires_approval: bool):
        """Sets the requires_approval of this AppPermission.


        :param requires_approval: The requires_approval of this AppPermission.
        :type requires_approval: bool
        """

        self._requires_approval = requires_approval

    @property
    def created_at(self) -> datetime:
        """Gets the created_at of this AppPermission.


        :return: The created_at of this AppPermission.
        :rtype: datetime
        """
        return self._created_at

    @created_at.setter
    def created_at(self, created_at: datetime):
        """Sets the created_at of this AppPermission.


        :param created_at: The created_at of this AppPermission.
        :type created_at: datetime
        """

        self._created_at = created_at

    @property
    def updated_at(self) -> datetime:
        """Gets the updated_at of this AppPermission.


        :return: The updated_at of this AppPermission.
        :rtype: datetime
        """
        return self._updated_at

    @updated_at.setter
    def updated_at(self, updated_at: datetime):
        """Sets the updated_at of this AppPermission.


        :param updated_at: The updated_at of this AppPermission.
        :type updated_at: datetime
        """

        self._updated_at = updated_at
