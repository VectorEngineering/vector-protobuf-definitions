from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk.models.auth_context import AuthContext
from playbookmedia_backend_server_sdk.models.auth_error_code import AuthErrorCode
from playbookmedia_backend_server_sdk.models.error_response import ErrorResponse
from playbookmedia_backend_server_sdk.models.mfa_info import MFAInfo
from playbookmedia_backend_server_sdk.models.session_info import SessionInfo
from playbookmedia_backend_server_sdk.models.token_info import TokenInfo
from playbookmedia_backend_server_sdk import util

from playbookmedia_backend_server_sdk.models.auth_context import AuthContext  # noqa: E501
from playbookmedia_backend_server_sdk.models.auth_error_code import AuthErrorCode  # noqa: E501
from playbookmedia_backend_server_sdk.models.error_response import ErrorResponse  # noqa: E501
from playbookmedia_backend_server_sdk.models.mfa_info import MFAInfo  # noqa: E501
from playbookmedia_backend_server_sdk.models.session_info import SessionInfo  # noqa: E501
from playbookmedia_backend_server_sdk.models.token_info import TokenInfo  # noqa: E501

class AuthenticationErrorMessageResponse1(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, code=AuthErrorCode.NO_AUTH_ERROR, message=None, token_info=None, auth_context=None, session_info=None, mfa_info=None, error_response=None):  # noqa: E501
        """AuthenticationErrorMessageResponse1 - a model defined in OpenAPI

        :param code: The code of this AuthenticationErrorMessageResponse1.  # noqa: E501
        :type code: AuthErrorCode
        :param message: The message of this AuthenticationErrorMessageResponse1.  # noqa: E501
        :type message: str
        :param token_info: The token_info of this AuthenticationErrorMessageResponse1.  # noqa: E501
        :type token_info: TokenInfo
        :param auth_context: The auth_context of this AuthenticationErrorMessageResponse1.  # noqa: E501
        :type auth_context: AuthContext
        :param session_info: The session_info of this AuthenticationErrorMessageResponse1.  # noqa: E501
        :type session_info: SessionInfo
        :param mfa_info: The mfa_info of this AuthenticationErrorMessageResponse1.  # noqa: E501
        :type mfa_info: MFAInfo
        :param error_response: The error_response of this AuthenticationErrorMessageResponse1.  # noqa: E501
        :type error_response: ErrorResponse
        """
        self.openapi_types = {
            'code': AuthErrorCode,
            'message': str,
            'token_info': TokenInfo,
            'auth_context': AuthContext,
            'session_info': SessionInfo,
            'mfa_info': MFAInfo,
            'error_response': ErrorResponse
        }

        self.attribute_map = {
            'code': 'code',
            'message': 'message',
            'token_info': 'tokenInfo',
            'auth_context': 'authContext',
            'session_info': 'sessionInfo',
            'mfa_info': 'mfaInfo',
            'error_response': 'errorResponse'
        }

        self._code = code
        self._message = message
        self._token_info = token_info
        self._auth_context = auth_context
        self._session_info = session_info
        self._mfa_info = mfa_info
        self._error_response = error_response

    @classmethod
    def from_dict(cls, dikt) -> 'AuthenticationErrorMessageResponse1':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The AuthenticationErrorMessageResponse1 of this AuthenticationErrorMessageResponse1.  # noqa: E501
        :rtype: AuthenticationErrorMessageResponse1
        """
        return util.deserialize_model(dikt, cls)

    @property
    def code(self) -> AuthErrorCode:
        """Gets the code of this AuthenticationErrorMessageResponse1.


        :return: The code of this AuthenticationErrorMessageResponse1.
        :rtype: AuthErrorCode
        """
        return self._code

    @code.setter
    def code(self, code: AuthErrorCode):
        """Sets the code of this AuthenticationErrorMessageResponse1.


        :param code: The code of this AuthenticationErrorMessageResponse1.
        :type code: AuthErrorCode
        """

        self._code = code

    @property
    def message(self) -> str:
        """Gets the message of this AuthenticationErrorMessageResponse1.


        :return: The message of this AuthenticationErrorMessageResponse1.
        :rtype: str
        """
        return self._message

    @message.setter
    def message(self, message: str):
        """Sets the message of this AuthenticationErrorMessageResponse1.


        :param message: The message of this AuthenticationErrorMessageResponse1.
        :type message: str
        """

        self._message = message

    @property
    def token_info(self) -> TokenInfo:
        """Gets the token_info of this AuthenticationErrorMessageResponse1.


        :return: The token_info of this AuthenticationErrorMessageResponse1.
        :rtype: TokenInfo
        """
        return self._token_info

    @token_info.setter
    def token_info(self, token_info: TokenInfo):
        """Sets the token_info of this AuthenticationErrorMessageResponse1.


        :param token_info: The token_info of this AuthenticationErrorMessageResponse1.
        :type token_info: TokenInfo
        """

        self._token_info = token_info

    @property
    def auth_context(self) -> AuthContext:
        """Gets the auth_context of this AuthenticationErrorMessageResponse1.


        :return: The auth_context of this AuthenticationErrorMessageResponse1.
        :rtype: AuthContext
        """
        return self._auth_context

    @auth_context.setter
    def auth_context(self, auth_context: AuthContext):
        """Sets the auth_context of this AuthenticationErrorMessageResponse1.


        :param auth_context: The auth_context of this AuthenticationErrorMessageResponse1.
        :type auth_context: AuthContext
        """

        self._auth_context = auth_context

    @property
    def session_info(self) -> SessionInfo:
        """Gets the session_info of this AuthenticationErrorMessageResponse1.


        :return: The session_info of this AuthenticationErrorMessageResponse1.
        :rtype: SessionInfo
        """
        return self._session_info

    @session_info.setter
    def session_info(self, session_info: SessionInfo):
        """Sets the session_info of this AuthenticationErrorMessageResponse1.


        :param session_info: The session_info of this AuthenticationErrorMessageResponse1.
        :type session_info: SessionInfo
        """

        self._session_info = session_info

    @property
    def mfa_info(self) -> MFAInfo:
        """Gets the mfa_info of this AuthenticationErrorMessageResponse1.


        :return: The mfa_info of this AuthenticationErrorMessageResponse1.
        :rtype: MFAInfo
        """
        return self._mfa_info

    @mfa_info.setter
    def mfa_info(self, mfa_info: MFAInfo):
        """Sets the mfa_info of this AuthenticationErrorMessageResponse1.


        :param mfa_info: The mfa_info of this AuthenticationErrorMessageResponse1.
        :type mfa_info: MFAInfo
        """

        self._mfa_info = mfa_info

    @property
    def error_response(self) -> ErrorResponse:
        """Gets the error_response of this AuthenticationErrorMessageResponse1.


        :return: The error_response of this AuthenticationErrorMessageResponse1.
        :rtype: ErrorResponse
        """
        return self._error_response

    @error_response.setter
    def error_response(self, error_response: ErrorResponse):
        """Sets the error_response of this AuthenticationErrorMessageResponse1.


        :param error_response: The error_response of this AuthenticationErrorMessageResponse1.
        :type error_response: ErrorResponse
        """

        self._error_response = error_response
