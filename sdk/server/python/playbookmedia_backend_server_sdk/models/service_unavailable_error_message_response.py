from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk.models.availability_info import AvailabilityInfo
from playbookmedia_backend_server_sdk.models.error_response import ErrorResponse
from playbookmedia_backend_server_sdk.models.internal_error_code import InternalErrorCode
from playbookmedia_backend_server_sdk import util

from playbookmedia_backend_server_sdk.models.availability_info import AvailabilityInfo  # noqa: E501
from playbookmedia_backend_server_sdk.models.error_response import ErrorResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.internal_error_code import InternalErrorCode  # noqa: E501

class ServiceUnavailableErrorMessageResponse(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, code=InternalErrorCode.NO_INTERNAL_ERROR, message=None, availability_info=None, error_response=None):  # noqa: E501
        """ServiceUnavailableErrorMessageResponse - a model defined in OpenAPI

        :param code: The code of this ServiceUnavailableErrorMessageResponse.  # noqa: E501
        :type code: InternalErrorCode
        :param message: The message of this ServiceUnavailableErrorMessageResponse.  # noqa: E501
        :type message: str
        :param availability_info: The availability_info of this ServiceUnavailableErrorMessageResponse.  # noqa: E501
        :type availability_info: AvailabilityInfo
        :param error_response: The error_response of this ServiceUnavailableErrorMessageResponse.  # noqa: E501
        :type error_response: ErrorResponse
        """
        self.openapi_types = {
            'code': InternalErrorCode,
            'message': str,
            'availability_info': AvailabilityInfo,
            'error_response': ErrorResponse
        }

        self.attribute_map = {
            'code': 'code',
            'message': 'message',
            'availability_info': 'availabilityInfo',
            'error_response': 'errorResponse'
        }

        self._code = code
        self._message = message
        self._availability_info = availability_info
        self._error_response = error_response

    @classmethod
    def from_dict(cls, dikt) -> 'ServiceUnavailableErrorMessageResponse':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The ServiceUnavailableErrorMessageResponse of this ServiceUnavailableErrorMessageResponse.  # noqa: E501
        :rtype: ServiceUnavailableErrorMessageResponse
        """
        return util.deserialize_model(dikt, cls)

    @property
    def code(self) -> InternalErrorCode:
        """Gets the code of this ServiceUnavailableErrorMessageResponse.


        :return: The code of this ServiceUnavailableErrorMessageResponse.
        :rtype: InternalErrorCode
        """
        return self._code

    @code.setter
    def code(self, code: InternalErrorCode):
        """Sets the code of this ServiceUnavailableErrorMessageResponse.


        :param code: The code of this ServiceUnavailableErrorMessageResponse.
        :type code: InternalErrorCode
        """

        self._code = code

    @property
    def message(self) -> str:
        """Gets the message of this ServiceUnavailableErrorMessageResponse.


        :return: The message of this ServiceUnavailableErrorMessageResponse.
        :rtype: str
        """
        return self._message

    @message.setter
    def message(self, message: str):
        """Sets the message of this ServiceUnavailableErrorMessageResponse.


        :param message: The message of this ServiceUnavailableErrorMessageResponse.
        :type message: str
        """

        self._message = message

    @property
    def availability_info(self) -> AvailabilityInfo:
        """Gets the availability_info of this ServiceUnavailableErrorMessageResponse.


        :return: The availability_info of this ServiceUnavailableErrorMessageResponse.
        :rtype: AvailabilityInfo
        """
        return self._availability_info

    @availability_info.setter
    def availability_info(self, availability_info: AvailabilityInfo):
        """Sets the availability_info of this ServiceUnavailableErrorMessageResponse.


        :param availability_info: The availability_info of this ServiceUnavailableErrorMessageResponse.
        :type availability_info: AvailabilityInfo
        """

        self._availability_info = availability_info

    @property
    def error_response(self) -> ErrorResponse:
        """Gets the error_response of this ServiceUnavailableErrorMessageResponse.


        :return: The error_response of this ServiceUnavailableErrorMessageResponse.
        :rtype: ErrorResponse
        """
        return self._error_response

    @error_response.setter
    def error_response(self, error_response: ErrorResponse):
        """Sets the error_response of this ServiceUnavailableErrorMessageResponse.


        :param error_response: The error_response of this ServiceUnavailableErrorMessageResponse.
        :type error_response: ErrorResponse
        """

        self._error_response = error_response
