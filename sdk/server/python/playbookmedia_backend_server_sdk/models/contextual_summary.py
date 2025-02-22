from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk import util


class ContextualSummary(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, id=None, version_id=None, summary_type=None, content=None, target_audience=None, key_points=None, metadata=None, created_at=None):  # noqa: E501
        """ContextualSummary - a model defined in OpenAPI

        :param id: The id of this ContextualSummary.  # noqa: E501
        :type id: str
        :param version_id: The version_id of this ContextualSummary.  # noqa: E501
        :type version_id: str
        :param summary_type: The summary_type of this ContextualSummary.  # noqa: E501
        :type summary_type: str
        :param content: The content of this ContextualSummary.  # noqa: E501
        :type content: str
        :param target_audience: The target_audience of this ContextualSummary.  # noqa: E501
        :type target_audience: str
        :param key_points: The key_points of this ContextualSummary.  # noqa: E501
        :type key_points: List[str]
        :param metadata: The metadata of this ContextualSummary.  # noqa: E501
        :type metadata: Dict[str, str]
        :param created_at: The created_at of this ContextualSummary.  # noqa: E501
        :type created_at: datetime
        """
        self.openapi_types = {
            'id': str,
            'version_id': str,
            'summary_type': str,
            'content': str,
            'target_audience': str,
            'key_points': List[str],
            'metadata': Dict[str, str],
            'created_at': datetime
        }

        self.attribute_map = {
            'id': 'id',
            'version_id': 'versionId',
            'summary_type': 'summaryType',
            'content': 'content',
            'target_audience': 'targetAudience',
            'key_points': 'keyPoints',
            'metadata': 'metadata',
            'created_at': 'createdAt'
        }

        self._id = id
        self._version_id = version_id
        self._summary_type = summary_type
        self._content = content
        self._target_audience = target_audience
        self._key_points = key_points
        self._metadata = metadata
        self._created_at = created_at

    @classmethod
    def from_dict(cls, dikt) -> 'ContextualSummary':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The ContextualSummary of this ContextualSummary.  # noqa: E501
        :rtype: ContextualSummary
        """
        return util.deserialize_model(dikt, cls)

    @property
    def id(self) -> str:
        """Gets the id of this ContextualSummary.


        :return: The id of this ContextualSummary.
        :rtype: str
        """
        return self._id

    @id.setter
    def id(self, id: str):
        """Sets the id of this ContextualSummary.


        :param id: The id of this ContextualSummary.
        :type id: str
        """

        self._id = id

    @property
    def version_id(self) -> str:
        """Gets the version_id of this ContextualSummary.


        :return: The version_id of this ContextualSummary.
        :rtype: str
        """
        return self._version_id

    @version_id.setter
    def version_id(self, version_id: str):
        """Sets the version_id of this ContextualSummary.


        :param version_id: The version_id of this ContextualSummary.
        :type version_id: str
        """

        self._version_id = version_id

    @property
    def summary_type(self) -> str:
        """Gets the summary_type of this ContextualSummary.


        :return: The summary_type of this ContextualSummary.
        :rtype: str
        """
        return self._summary_type

    @summary_type.setter
    def summary_type(self, summary_type: str):
        """Sets the summary_type of this ContextualSummary.


        :param summary_type: The summary_type of this ContextualSummary.
        :type summary_type: str
        """

        self._summary_type = summary_type

    @property
    def content(self) -> str:
        """Gets the content of this ContextualSummary.


        :return: The content of this ContextualSummary.
        :rtype: str
        """
        return self._content

    @content.setter
    def content(self, content: str):
        """Sets the content of this ContextualSummary.


        :param content: The content of this ContextualSummary.
        :type content: str
        """

        self._content = content

    @property
    def target_audience(self) -> str:
        """Gets the target_audience of this ContextualSummary.


        :return: The target_audience of this ContextualSummary.
        :rtype: str
        """
        return self._target_audience

    @target_audience.setter
    def target_audience(self, target_audience: str):
        """Sets the target_audience of this ContextualSummary.


        :param target_audience: The target_audience of this ContextualSummary.
        :type target_audience: str
        """

        self._target_audience = target_audience

    @property
    def key_points(self) -> List[str]:
        """Gets the key_points of this ContextualSummary.


        :return: The key_points of this ContextualSummary.
        :rtype: List[str]
        """
        return self._key_points

    @key_points.setter
    def key_points(self, key_points: List[str]):
        """Sets the key_points of this ContextualSummary.


        :param key_points: The key_points of this ContextualSummary.
        :type key_points: List[str]
        """

        self._key_points = key_points

    @property
    def metadata(self) -> Dict[str, str]:
        """Gets the metadata of this ContextualSummary.


        :return: The metadata of this ContextualSummary.
        :rtype: Dict[str, str]
        """
        return self._metadata

    @metadata.setter
    def metadata(self, metadata: Dict[str, str]):
        """Sets the metadata of this ContextualSummary.


        :param metadata: The metadata of this ContextualSummary.
        :type metadata: Dict[str, str]
        """

        self._metadata = metadata

    @property
    def created_at(self) -> datetime:
        """Gets the created_at of this ContextualSummary.


        :return: The created_at of this ContextualSummary.
        :rtype: datetime
        """
        return self._created_at

    @created_at.setter
    def created_at(self, created_at: datetime):
        """Sets the created_at of this ContextualSummary.


        :param created_at: The created_at of this ContextualSummary.
        :type created_at: datetime
        """

        self._created_at = created_at
