from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk.models.status import Status
from playbookmedia_backend_server_sdk import util

from playbookmedia_backend_server_sdk.models.status import Status  # noqa: E501

class ErrorResponse(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, status=None):  # noqa: E501
        """ErrorResponse - a model defined in OpenAPI

        :param status: The status of this ErrorResponse.  # noqa: E501
        :type status: Status
        """
        self.openapi_types = {
            'status': Status
        }

        self.attribute_map = {
            'status': 'status'
        }

        self._status = status

    @classmethod
    def from_dict(cls, dikt) -> 'ErrorResponse':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The ErrorResponse of this ErrorResponse.  # noqa: E501
        :rtype: ErrorResponse
        """
        return util.deserialize_model(dikt, cls)

    @property
    def status(self) -> Status:
        """Gets the status of this ErrorResponse.


        :return: The status of this ErrorResponse.
        :rtype: Status
        """
        return self._status

    @status.setter
    def status(self, status: Status):
        """Sets the status of this ErrorResponse.


        :param status: The status of this ErrorResponse.
        :type status: Status
        """

        self._status = status
