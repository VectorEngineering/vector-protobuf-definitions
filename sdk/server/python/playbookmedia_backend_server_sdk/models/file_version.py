from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk import util


class FileVersion(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, id=None, file_id=None, version_number=None, s3_key=None, size=None, commit_message=None, author_id=None, created_at=None):  # noqa: E501
        """FileVersion - a model defined in OpenAPI

        :param id: The id of this FileVersion.  # noqa: E501
        :type id: str
        :param file_id: The file_id of this FileVersion.  # noqa: E501
        :type file_id: str
        :param version_number: The version_number of this FileVersion.  # noqa: E501
        :type version_number: int
        :param s3_key: The s3_key of this FileVersion.  # noqa: E501
        :type s3_key: str
        :param size: The size of this FileVersion.  # noqa: E501
        :type size: str
        :param commit_message: The commit_message of this FileVersion.  # noqa: E501
        :type commit_message: str
        :param author_id: The author_id of this FileVersion.  # noqa: E501
        :type author_id: str
        :param created_at: The created_at of this FileVersion.  # noqa: E501
        :type created_at: datetime
        """
        self.openapi_types = {
            'id': str,
            'file_id': str,
            'version_number': int,
            's3_key': str,
            'size': str,
            'commit_message': str,
            'author_id': str,
            'created_at': datetime
        }

        self.attribute_map = {
            'id': 'id',
            'file_id': 'fileId',
            'version_number': 'versionNumber',
            's3_key': 's3Key',
            'size': 'size',
            'commit_message': 'commitMessage',
            'author_id': 'authorId',
            'created_at': 'createdAt'
        }

        self._id = id
        self._file_id = file_id
        self._version_number = version_number
        self._s3_key = s3_key
        self._size = size
        self._commit_message = commit_message
        self._author_id = author_id
        self._created_at = created_at

    @classmethod
    def from_dict(cls, dikt) -> 'FileVersion':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The FileVersion of this FileVersion.  # noqa: E501
        :rtype: FileVersion
        """
        return util.deserialize_model(dikt, cls)

    @property
    def id(self) -> str:
        """Gets the id of this FileVersion.


        :return: The id of this FileVersion.
        :rtype: str
        """
        return self._id

    @id.setter
    def id(self, id: str):
        """Sets the id of this FileVersion.


        :param id: The id of this FileVersion.
        :type id: str
        """

        self._id = id

    @property
    def file_id(self) -> str:
        """Gets the file_id of this FileVersion.


        :return: The file_id of this FileVersion.
        :rtype: str
        """
        return self._file_id

    @file_id.setter
    def file_id(self, file_id: str):
        """Sets the file_id of this FileVersion.


        :param file_id: The file_id of this FileVersion.
        :type file_id: str
        """

        self._file_id = file_id

    @property
    def version_number(self) -> int:
        """Gets the version_number of this FileVersion.


        :return: The version_number of this FileVersion.
        :rtype: int
        """
        return self._version_number

    @version_number.setter
    def version_number(self, version_number: int):
        """Sets the version_number of this FileVersion.


        :param version_number: The version_number of this FileVersion.
        :type version_number: int
        """

        self._version_number = version_number

    @property
    def s3_key(self) -> str:
        """Gets the s3_key of this FileVersion.


        :return: The s3_key of this FileVersion.
        :rtype: str
        """
        return self._s3_key

    @s3_key.setter
    def s3_key(self, s3_key: str):
        """Sets the s3_key of this FileVersion.


        :param s3_key: The s3_key of this FileVersion.
        :type s3_key: str
        """

        self._s3_key = s3_key

    @property
    def size(self) -> str:
        """Gets the size of this FileVersion.


        :return: The size of this FileVersion.
        :rtype: str
        """
        return self._size

    @size.setter
    def size(self, size: str):
        """Sets the size of this FileVersion.


        :param size: The size of this FileVersion.
        :type size: str
        """

        self._size = size

    @property
    def commit_message(self) -> str:
        """Gets the commit_message of this FileVersion.


        :return: The commit_message of this FileVersion.
        :rtype: str
        """
        return self._commit_message

    @commit_message.setter
    def commit_message(self, commit_message: str):
        """Sets the commit_message of this FileVersion.


        :param commit_message: The commit_message of this FileVersion.
        :type commit_message: str
        """

        self._commit_message = commit_message

    @property
    def author_id(self) -> str:
        """Gets the author_id of this FileVersion.


        :return: The author_id of this FileVersion.
        :rtype: str
        """
        return self._author_id

    @author_id.setter
    def author_id(self, author_id: str):
        """Sets the author_id of this FileVersion.


        :param author_id: The author_id of this FileVersion.
        :type author_id: str
        """

        self._author_id = author_id

    @property
    def created_at(self) -> datetime:
        """Gets the created_at of this FileVersion.


        :return: The created_at of this FileVersion.
        :rtype: datetime
        """
        return self._created_at

    @created_at.setter
    def created_at(self, created_at: datetime):
        """Sets the created_at of this FileVersion.


        :param created_at: The created_at of this FileVersion.
        :type created_at: datetime
        """

        self._created_at = created_at
