from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk import util


class AvailabilityInfo(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, retry_after=None, maintenance_window=None, affected_services=None, status_page_url=None):  # noqa: E501
        """AvailabilityInfo - a model defined in OpenAPI

        :param retry_after: The retry_after of this AvailabilityInfo.  # noqa: E501
        :type retry_after: int
        :param maintenance_window: The maintenance_window of this AvailabilityInfo.  # noqa: E501
        :type maintenance_window: str
        :param affected_services: The affected_services of this AvailabilityInfo.  # noqa: E501
        :type affected_services: List[str]
        :param status_page_url: The status_page_url of this AvailabilityInfo.  # noqa: E501
        :type status_page_url: str
        """
        self.openapi_types = {
            'retry_after': int,
            'maintenance_window': str,
            'affected_services': List[str],
            'status_page_url': str
        }

        self.attribute_map = {
            'retry_after': 'retryAfter',
            'maintenance_window': 'maintenanceWindow',
            'affected_services': 'affectedServices',
            'status_page_url': 'statusPageUrl'
        }

        self._retry_after = retry_after
        self._maintenance_window = maintenance_window
        self._affected_services = affected_services
        self._status_page_url = status_page_url

    @classmethod
    def from_dict(cls, dikt) -> 'AvailabilityInfo':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The AvailabilityInfo of this AvailabilityInfo.  # noqa: E501
        :rtype: AvailabilityInfo
        """
        return util.deserialize_model(dikt, cls)

    @property
    def retry_after(self) -> int:
        """Gets the retry_after of this AvailabilityInfo.


        :return: The retry_after of this AvailabilityInfo.
        :rtype: int
        """
        return self._retry_after

    @retry_after.setter
    def retry_after(self, retry_after: int):
        """Sets the retry_after of this AvailabilityInfo.


        :param retry_after: The retry_after of this AvailabilityInfo.
        :type retry_after: int
        """

        self._retry_after = retry_after

    @property
    def maintenance_window(self) -> str:
        """Gets the maintenance_window of this AvailabilityInfo.


        :return: The maintenance_window of this AvailabilityInfo.
        :rtype: str
        """
        return self._maintenance_window

    @maintenance_window.setter
    def maintenance_window(self, maintenance_window: str):
        """Sets the maintenance_window of this AvailabilityInfo.


        :param maintenance_window: The maintenance_window of this AvailabilityInfo.
        :type maintenance_window: str
        """

        self._maintenance_window = maintenance_window

    @property
    def affected_services(self) -> List[str]:
        """Gets the affected_services of this AvailabilityInfo.


        :return: The affected_services of this AvailabilityInfo.
        :rtype: List[str]
        """
        return self._affected_services

    @affected_services.setter
    def affected_services(self, affected_services: List[str]):
        """Sets the affected_services of this AvailabilityInfo.


        :param affected_services: The affected_services of this AvailabilityInfo.
        :type affected_services: List[str]
        """

        self._affected_services = affected_services

    @property
    def status_page_url(self) -> str:
        """Gets the status_page_url of this AvailabilityInfo.


        :return: The status_page_url of this AvailabilityInfo.
        :rtype: str
        """
        return self._status_page_url

    @status_page_url.setter
    def status_page_url(self, status_page_url: str):
        """Sets the status_page_url of this AvailabilityInfo.


        :param status_page_url: The status_page_url of this AvailabilityInfo.
        :type status_page_url: str
        """

        self._status_page_url = status_page_url
