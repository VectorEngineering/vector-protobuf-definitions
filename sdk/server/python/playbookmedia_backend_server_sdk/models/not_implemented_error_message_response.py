from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk.models.error_response import ErrorResponse
from playbookmedia_backend_server_sdk import util

from playbookmedia_backend_server_sdk.models.error_response import ErrorResponse  # noqa: E501

class NotImplementedErrorMessageResponse(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, code=None, message=None, feature=None, error_response=None):  # noqa: E501
        """NotImplementedErrorMessageResponse - a model defined in OpenAPI

        :param code: The code of this NotImplementedErrorMessageResponse.  # noqa: E501
        :type code: int
        :param message: The message of this NotImplementedErrorMessageResponse.  # noqa: E501
        :type message: str
        :param feature: The feature of this NotImplementedErrorMessageResponse.  # noqa: E501
        :type feature: str
        :param error_response: The error_response of this NotImplementedErrorMessageResponse.  # noqa: E501
        :type error_response: ErrorResponse
        """
        self.openapi_types = {
            'code': int,
            'message': str,
            'feature': str,
            'error_response': ErrorResponse
        }

        self.attribute_map = {
            'code': 'code',
            'message': 'message',
            'feature': 'feature',
            'error_response': 'errorResponse'
        }

        self._code = code
        self._message = message
        self._feature = feature
        self._error_response = error_response

    @classmethod
    def from_dict(cls, dikt) -> 'NotImplementedErrorMessageResponse':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The NotImplementedErrorMessageResponse of this NotImplementedErrorMessageResponse.  # noqa: E501
        :rtype: NotImplementedErrorMessageResponse
        """
        return util.deserialize_model(dikt, cls)

    @property
    def code(self) -> int:
        """Gets the code of this NotImplementedErrorMessageResponse.


        :return: The code of this NotImplementedErrorMessageResponse.
        :rtype: int
        """
        return self._code

    @code.setter
    def code(self, code: int):
        """Sets the code of this NotImplementedErrorMessageResponse.


        :param code: The code of this NotImplementedErrorMessageResponse.
        :type code: int
        """

        self._code = code

    @property
    def message(self) -> str:
        """Gets the message of this NotImplementedErrorMessageResponse.


        :return: The message of this NotImplementedErrorMessageResponse.
        :rtype: str
        """
        return self._message

    @message.setter
    def message(self, message: str):
        """Sets the message of this NotImplementedErrorMessageResponse.


        :param message: The message of this NotImplementedErrorMessageResponse.
        :type message: str
        """

        self._message = message

    @property
    def feature(self) -> str:
        """Gets the feature of this NotImplementedErrorMessageResponse.


        :return: The feature of this NotImplementedErrorMessageResponse.
        :rtype: str
        """
        return self._feature

    @feature.setter
    def feature(self, feature: str):
        """Sets the feature of this NotImplementedErrorMessageResponse.


        :param feature: The feature of this NotImplementedErrorMessageResponse.
        :type feature: str
        """

        self._feature = feature

    @property
    def error_response(self) -> ErrorResponse:
        """Gets the error_response of this NotImplementedErrorMessageResponse.


        :return: The error_response of this NotImplementedErrorMessageResponse.
        :rtype: ErrorResponse
        """
        return self._error_response

    @error_response.setter
    def error_response(self, error_response: ErrorResponse):
        """Sets the error_response of this NotImplementedErrorMessageResponse.


        :param error_response: The error_response of this NotImplementedErrorMessageResponse.
        :type error_response: ErrorResponse
        """

        self._error_response = error_response
