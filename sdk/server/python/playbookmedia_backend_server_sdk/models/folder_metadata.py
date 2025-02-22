from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk.models.file_metadata import FileMetadata
from playbookmedia_backend_server_sdk import util

from playbookmedia_backend_server_sdk.models.file_metadata import FileMetadata  # noqa: E501

class FolderMetadata(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, id=None, name=None, s3_bucket_name=None, s3_folder_path=None, is_deleted=None, parent_folder_id=None, created_at=None, updated_at=None, deleted_at=None, files=None):  # noqa: E501
        """FolderMetadata - a model defined in OpenAPI

        :param id: The id of this FolderMetadata.  # noqa: E501
        :type id: str
        :param name: The name of this FolderMetadata.  # noqa: E501
        :type name: str
        :param s3_bucket_name: The s3_bucket_name of this FolderMetadata.  # noqa: E501
        :type s3_bucket_name: str
        :param s3_folder_path: The s3_folder_path of this FolderMetadata.  # noqa: E501
        :type s3_folder_path: str
        :param is_deleted: The is_deleted of this FolderMetadata.  # noqa: E501
        :type is_deleted: bool
        :param parent_folder_id: The parent_folder_id of this FolderMetadata.  # noqa: E501
        :type parent_folder_id: str
        :param created_at: The created_at of this FolderMetadata.  # noqa: E501
        :type created_at: datetime
        :param updated_at: The updated_at of this FolderMetadata.  # noqa: E501
        :type updated_at: datetime
        :param deleted_at: The deleted_at of this FolderMetadata.  # noqa: E501
        :type deleted_at: datetime
        :param files: The files of this FolderMetadata.  # noqa: E501
        :type files: List[FileMetadata]
        """
        self.openapi_types = {
            'id': str,
            'name': str,
            's3_bucket_name': str,
            's3_folder_path': str,
            'is_deleted': bool,
            'parent_folder_id': str,
            'created_at': datetime,
            'updated_at': datetime,
            'deleted_at': datetime,
            'files': List[FileMetadata]
        }

        self.attribute_map = {
            'id': 'id',
            'name': 'name',
            's3_bucket_name': 's3BucketName',
            's3_folder_path': 's3FolderPath',
            'is_deleted': 'isDeleted',
            'parent_folder_id': 'parentFolderId',
            'created_at': 'createdAt',
            'updated_at': 'updatedAt',
            'deleted_at': 'deletedAt',
            'files': 'files'
        }

        self._id = id
        self._name = name
        self._s3_bucket_name = s3_bucket_name
        self._s3_folder_path = s3_folder_path
        self._is_deleted = is_deleted
        self._parent_folder_id = parent_folder_id
        self._created_at = created_at
        self._updated_at = updated_at
        self._deleted_at = deleted_at
        self._files = files

    @classmethod
    def from_dict(cls, dikt) -> 'FolderMetadata':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The FolderMetadata of this FolderMetadata.  # noqa: E501
        :rtype: FolderMetadata
        """
        return util.deserialize_model(dikt, cls)

    @property
    def id(self) -> str:
        """Gets the id of this FolderMetadata.


        :return: The id of this FolderMetadata.
        :rtype: str
        """
        return self._id

    @id.setter
    def id(self, id: str):
        """Sets the id of this FolderMetadata.


        :param id: The id of this FolderMetadata.
        :type id: str
        """

        self._id = id

    @property
    def name(self) -> str:
        """Gets the name of this FolderMetadata.


        :return: The name of this FolderMetadata.
        :rtype: str
        """
        return self._name

    @name.setter
    def name(self, name: str):
        """Sets the name of this FolderMetadata.


        :param name: The name of this FolderMetadata.
        :type name: str
        """

        self._name = name

    @property
    def s3_bucket_name(self) -> str:
        """Gets the s3_bucket_name of this FolderMetadata.


        :return: The s3_bucket_name of this FolderMetadata.
        :rtype: str
        """
        return self._s3_bucket_name

    @s3_bucket_name.setter
    def s3_bucket_name(self, s3_bucket_name: str):
        """Sets the s3_bucket_name of this FolderMetadata.


        :param s3_bucket_name: The s3_bucket_name of this FolderMetadata.
        :type s3_bucket_name: str
        """

        self._s3_bucket_name = s3_bucket_name

    @property
    def s3_folder_path(self) -> str:
        """Gets the s3_folder_path of this FolderMetadata.


        :return: The s3_folder_path of this FolderMetadata.
        :rtype: str
        """
        return self._s3_folder_path

    @s3_folder_path.setter
    def s3_folder_path(self, s3_folder_path: str):
        """Sets the s3_folder_path of this FolderMetadata.


        :param s3_folder_path: The s3_folder_path of this FolderMetadata.
        :type s3_folder_path: str
        """

        self._s3_folder_path = s3_folder_path

    @property
    def is_deleted(self) -> bool:
        """Gets the is_deleted of this FolderMetadata.


        :return: The is_deleted of this FolderMetadata.
        :rtype: bool
        """
        return self._is_deleted

    @is_deleted.setter
    def is_deleted(self, is_deleted: bool):
        """Sets the is_deleted of this FolderMetadata.


        :param is_deleted: The is_deleted of this FolderMetadata.
        :type is_deleted: bool
        """

        self._is_deleted = is_deleted

    @property
    def parent_folder_id(self) -> str:
        """Gets the parent_folder_id of this FolderMetadata.


        :return: The parent_folder_id of this FolderMetadata.
        :rtype: str
        """
        return self._parent_folder_id

    @parent_folder_id.setter
    def parent_folder_id(self, parent_folder_id: str):
        """Sets the parent_folder_id of this FolderMetadata.


        :param parent_folder_id: The parent_folder_id of this FolderMetadata.
        :type parent_folder_id: str
        """

        self._parent_folder_id = parent_folder_id

    @property
    def created_at(self) -> datetime:
        """Gets the created_at of this FolderMetadata.


        :return: The created_at of this FolderMetadata.
        :rtype: datetime
        """
        return self._created_at

    @created_at.setter
    def created_at(self, created_at: datetime):
        """Sets the created_at of this FolderMetadata.


        :param created_at: The created_at of this FolderMetadata.
        :type created_at: datetime
        """

        self._created_at = created_at

    @property
    def updated_at(self) -> datetime:
        """Gets the updated_at of this FolderMetadata.


        :return: The updated_at of this FolderMetadata.
        :rtype: datetime
        """
        return self._updated_at

    @updated_at.setter
    def updated_at(self, updated_at: datetime):
        """Sets the updated_at of this FolderMetadata.


        :param updated_at: The updated_at of this FolderMetadata.
        :type updated_at: datetime
        """

        self._updated_at = updated_at

    @property
    def deleted_at(self) -> datetime:
        """Gets the deleted_at of this FolderMetadata.


        :return: The deleted_at of this FolderMetadata.
        :rtype: datetime
        """
        return self._deleted_at

    @deleted_at.setter
    def deleted_at(self, deleted_at: datetime):
        """Sets the deleted_at of this FolderMetadata.


        :param deleted_at: The deleted_at of this FolderMetadata.
        :type deleted_at: datetime
        """

        self._deleted_at = deleted_at

    @property
    def files(self) -> List[FileMetadata]:
        """Gets the files of this FolderMetadata.


        :return: The files of this FolderMetadata.
        :rtype: List[FileMetadata]
        """
        return self._files

    @files.setter
    def files(self, files: List[FileMetadata]):
        """Sets the files of this FolderMetadata.


        :param files: The files of this FolderMetadata.
        :type files: List[FileMetadata]
        """

        self._files = files
