from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk import util


class AppAnalytics(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, id=None, app_id=None, metric_name=None, metric_value=None, dimensions=None, recorded_at=None):  # noqa: E501
        """AppAnalytics - a model defined in OpenAPI

        :param id: The id of this AppAnalytics.  # noqa: E501
        :type id: str
        :param app_id: The app_id of this AppAnalytics.  # noqa: E501
        :type app_id: str
        :param metric_name: The metric_name of this AppAnalytics.  # noqa: E501
        :type metric_name: str
        :param metric_value: The metric_value of this AppAnalytics.  # noqa: E501
        :type metric_value: float
        :param dimensions: The dimensions of this AppAnalytics.  # noqa: E501
        :type dimensions: Dict[str, str]
        :param recorded_at: The recorded_at of this AppAnalytics.  # noqa: E501
        :type recorded_at: datetime
        """
        self.openapi_types = {
            'id': str,
            'app_id': str,
            'metric_name': str,
            'metric_value': float,
            'dimensions': Dict[str, str],
            'recorded_at': datetime
        }

        self.attribute_map = {
            'id': 'id',
            'app_id': 'appId',
            'metric_name': 'metricName',
            'metric_value': 'metricValue',
            'dimensions': 'dimensions',
            'recorded_at': 'recordedAt'
        }

        self._id = id
        self._app_id = app_id
        self._metric_name = metric_name
        self._metric_value = metric_value
        self._dimensions = dimensions
        self._recorded_at = recorded_at

    @classmethod
    def from_dict(cls, dikt) -> 'AppAnalytics':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The AppAnalytics of this AppAnalytics.  # noqa: E501
        :rtype: AppAnalytics
        """
        return util.deserialize_model(dikt, cls)

    @property
    def id(self) -> str:
        """Gets the id of this AppAnalytics.


        :return: The id of this AppAnalytics.
        :rtype: str
        """
        return self._id

    @id.setter
    def id(self, id: str):
        """Sets the id of this AppAnalytics.


        :param id: The id of this AppAnalytics.
        :type id: str
        """

        self._id = id

    @property
    def app_id(self) -> str:
        """Gets the app_id of this AppAnalytics.


        :return: The app_id of this AppAnalytics.
        :rtype: str
        """
        return self._app_id

    @app_id.setter
    def app_id(self, app_id: str):
        """Sets the app_id of this AppAnalytics.


        :param app_id: The app_id of this AppAnalytics.
        :type app_id: str
        """

        self._app_id = app_id

    @property
    def metric_name(self) -> str:
        """Gets the metric_name of this AppAnalytics.


        :return: The metric_name of this AppAnalytics.
        :rtype: str
        """
        return self._metric_name

    @metric_name.setter
    def metric_name(self, metric_name: str):
        """Sets the metric_name of this AppAnalytics.


        :param metric_name: The metric_name of this AppAnalytics.
        :type metric_name: str
        """

        self._metric_name = metric_name

    @property
    def metric_value(self) -> float:
        """Gets the metric_value of this AppAnalytics.


        :return: The metric_value of this AppAnalytics.
        :rtype: float
        """
        return self._metric_value

    @metric_value.setter
    def metric_value(self, metric_value: float):
        """Sets the metric_value of this AppAnalytics.


        :param metric_value: The metric_value of this AppAnalytics.
        :type metric_value: float
        """

        self._metric_value = metric_value

    @property
    def dimensions(self) -> Dict[str, str]:
        """Gets the dimensions of this AppAnalytics.


        :return: The dimensions of this AppAnalytics.
        :rtype: Dict[str, str]
        """
        return self._dimensions

    @dimensions.setter
    def dimensions(self, dimensions: Dict[str, str]):
        """Sets the dimensions of this AppAnalytics.


        :param dimensions: The dimensions of this AppAnalytics.
        :type dimensions: Dict[str, str]
        """

        self._dimensions = dimensions

    @property
    def recorded_at(self) -> datetime:
        """Gets the recorded_at of this AppAnalytics.


        :return: The recorded_at of this AppAnalytics.
        :rtype: datetime
        """
        return self._recorded_at

    @recorded_at.setter
    def recorded_at(self, recorded_at: datetime):
        """Sets the recorded_at of this AppAnalytics.


        :param recorded_at: The recorded_at of this AppAnalytics.
        :type recorded_at: datetime
        """

        self._recorded_at = recorded_at
