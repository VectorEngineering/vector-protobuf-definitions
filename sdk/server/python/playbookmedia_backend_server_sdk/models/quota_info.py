from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk import util


class QuotaInfo(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, resource=None, used=None, limit=None, reset_time=None, resource_usage=None):  # noqa: E501
        """QuotaInfo - a model defined in OpenAPI

        :param resource: The resource of this QuotaInfo.  # noqa: E501
        :type resource: str
        :param used: The used of this QuotaInfo.  # noqa: E501
        :type used: str
        :param limit: The limit of this QuotaInfo.  # noqa: E501
        :type limit: str
        :param reset_time: The reset_time of this QuotaInfo.  # noqa: E501
        :type reset_time: datetime
        :param resource_usage: The resource_usage of this QuotaInfo.  # noqa: E501
        :type resource_usage: Dict[str, str]
        """
        self.openapi_types = {
            'resource': str,
            'used': str,
            'limit': str,
            'reset_time': datetime,
            'resource_usage': Dict[str, str]
        }

        self.attribute_map = {
            'resource': 'resource',
            'used': 'used',
            'limit': 'limit',
            'reset_time': 'resetTime',
            'resource_usage': 'resourceUsage'
        }

        self._resource = resource
        self._used = used
        self._limit = limit
        self._reset_time = reset_time
        self._resource_usage = resource_usage

    @classmethod
    def from_dict(cls, dikt) -> 'QuotaInfo':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The QuotaInfo of this QuotaInfo.  # noqa: E501
        :rtype: QuotaInfo
        """
        return util.deserialize_model(dikt, cls)

    @property
    def resource(self) -> str:
        """Gets the resource of this QuotaInfo.


        :return: The resource of this QuotaInfo.
        :rtype: str
        """
        return self._resource

    @resource.setter
    def resource(self, resource: str):
        """Sets the resource of this QuotaInfo.


        :param resource: The resource of this QuotaInfo.
        :type resource: str
        """

        self._resource = resource

    @property
    def used(self) -> str:
        """Gets the used of this QuotaInfo.


        :return: The used of this QuotaInfo.
        :rtype: str
        """
        return self._used

    @used.setter
    def used(self, used: str):
        """Sets the used of this QuotaInfo.


        :param used: The used of this QuotaInfo.
        :type used: str
        """

        self._used = used

    @property
    def limit(self) -> str:
        """Gets the limit of this QuotaInfo.


        :return: The limit of this QuotaInfo.
        :rtype: str
        """
        return self._limit

    @limit.setter
    def limit(self, limit: str):
        """Sets the limit of this QuotaInfo.


        :param limit: The limit of this QuotaInfo.
        :type limit: str
        """

        self._limit = limit

    @property
    def reset_time(self) -> datetime:
        """Gets the reset_time of this QuotaInfo.


        :return: The reset_time of this QuotaInfo.
        :rtype: datetime
        """
        return self._reset_time

    @reset_time.setter
    def reset_time(self, reset_time: datetime):
        """Sets the reset_time of this QuotaInfo.


        :param reset_time: The reset_time of this QuotaInfo.
        :type reset_time: datetime
        """

        self._reset_time = reset_time

    @property
    def resource_usage(self) -> Dict[str, str]:
        """Gets the resource_usage of this QuotaInfo.


        :return: The resource_usage of this QuotaInfo.
        :rtype: Dict[str, str]
        """
        return self._resource_usage

    @resource_usage.setter
    def resource_usage(self, resource_usage: Dict[str, str]):
        """Sets the resource_usage of this QuotaInfo.


        :param resource_usage: The resource_usage of this QuotaInfo.
        :type resource_usage: Dict[str, str]
        """

        self._resource_usage = resource_usage
