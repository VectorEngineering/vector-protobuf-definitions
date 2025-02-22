from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk.models.account1 import Account1
from playbookmedia_backend_server_sdk import util

from playbookmedia_backend_server_sdk.models.account1 import Account1  # noqa: E501

class ListAccountsResponse1(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, accounts=None, next_page_token=None, total_size=None):  # noqa: E501
        """ListAccountsResponse1 - a model defined in OpenAPI

        :param accounts: The accounts of this ListAccountsResponse1.  # noqa: E501
        :type accounts: List[Account1]
        :param next_page_token: The next_page_token of this ListAccountsResponse1.  # noqa: E501
        :type next_page_token: str
        :param total_size: The total_size of this ListAccountsResponse1.  # noqa: E501
        :type total_size: int
        """
        self.openapi_types = {
            'accounts': List[Account1],
            'next_page_token': str,
            'total_size': int
        }

        self.attribute_map = {
            'accounts': 'accounts',
            'next_page_token': 'nextPageToken',
            'total_size': 'totalSize'
        }

        self._accounts = accounts
        self._next_page_token = next_page_token
        self._total_size = total_size

    @classmethod
    def from_dict(cls, dikt) -> 'ListAccountsResponse1':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The ListAccountsResponse1 of this ListAccountsResponse1.  # noqa: E501
        :rtype: ListAccountsResponse1
        """
        return util.deserialize_model(dikt, cls)

    @property
    def accounts(self) -> List[Account1]:
        """Gets the accounts of this ListAccountsResponse1.


        :return: The accounts of this ListAccountsResponse1.
        :rtype: List[Account1]
        """
        return self._accounts

    @accounts.setter
    def accounts(self, accounts: List[Account1]):
        """Sets the accounts of this ListAccountsResponse1.


        :param accounts: The accounts of this ListAccountsResponse1.
        :type accounts: List[Account1]
        """

        self._accounts = accounts

    @property
    def next_page_token(self) -> str:
        """Gets the next_page_token of this ListAccountsResponse1.


        :return: The next_page_token of this ListAccountsResponse1.
        :rtype: str
        """
        return self._next_page_token

    @next_page_token.setter
    def next_page_token(self, next_page_token: str):
        """Sets the next_page_token of this ListAccountsResponse1.


        :param next_page_token: The next_page_token of this ListAccountsResponse1.
        :type next_page_token: str
        """

        self._next_page_token = next_page_token

    @property
    def total_size(self) -> int:
        """Gets the total_size of this ListAccountsResponse1.


        :return: The total_size of this ListAccountsResponse1.
        :rtype: int
        """
        return self._total_size

    @total_size.setter
    def total_size(self, total_size: int):
        """Sets the total_size of this ListAccountsResponse1.


        :param total_size: The total_size of this ListAccountsResponse1.
        :type total_size: int
        """

        self._total_size = total_size
