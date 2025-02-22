from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk.models.webhook_config import WebhookConfig
from playbookmedia_backend_server_sdk import util

from playbookmedia_backend_server_sdk.models.webhook_config import WebhookConfig  # noqa: E501

class ListWebhooksResponse(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, webhooks=None, next_page_number=None, total_count=None):  # noqa: E501
        """ListWebhooksResponse - a model defined in OpenAPI

        :param webhooks: The webhooks of this ListWebhooksResponse.  # noqa: E501
        :type webhooks: List[WebhookConfig]
        :param next_page_number: The next_page_number of this ListWebhooksResponse.  # noqa: E501
        :type next_page_number: int
        :param total_count: The total_count of this ListWebhooksResponse.  # noqa: E501
        :type total_count: int
        """
        self.openapi_types = {
            'webhooks': List[WebhookConfig],
            'next_page_number': int,
            'total_count': int
        }

        self.attribute_map = {
            'webhooks': 'webhooks',
            'next_page_number': 'nextPageNumber',
            'total_count': 'totalCount'
        }

        self._webhooks = webhooks
        self._next_page_number = next_page_number
        self._total_count = total_count

    @classmethod
    def from_dict(cls, dikt) -> 'ListWebhooksResponse':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The ListWebhooksResponse of this ListWebhooksResponse.  # noqa: E501
        :rtype: ListWebhooksResponse
        """
        return util.deserialize_model(dikt, cls)

    @property
    def webhooks(self) -> List[WebhookConfig]:
        """Gets the webhooks of this ListWebhooksResponse.


        :return: The webhooks of this ListWebhooksResponse.
        :rtype: List[WebhookConfig]
        """
        return self._webhooks

    @webhooks.setter
    def webhooks(self, webhooks: List[WebhookConfig]):
        """Sets the webhooks of this ListWebhooksResponse.


        :param webhooks: The webhooks of this ListWebhooksResponse.
        :type webhooks: List[WebhookConfig]
        """

        self._webhooks = webhooks

    @property
    def next_page_number(self) -> int:
        """Gets the next_page_number of this ListWebhooksResponse.


        :return: The next_page_number of this ListWebhooksResponse.
        :rtype: int
        """
        return self._next_page_number

    @next_page_number.setter
    def next_page_number(self, next_page_number: int):
        """Sets the next_page_number of this ListWebhooksResponse.


        :param next_page_number: The next_page_number of this ListWebhooksResponse.
        :type next_page_number: int
        """

        self._next_page_number = next_page_number

    @property
    def total_count(self) -> int:
        """Gets the total_count of this ListWebhooksResponse.


        :return: The total_count of this ListWebhooksResponse.
        :rtype: int
        """
        return self._total_count

    @total_count.setter
    def total_count(self, total_count: int):
        """Sets the total_count of this ListWebhooksResponse.


        :param total_count: The total_count of this ListWebhooksResponse.
        :type total_count: int
        """

        self._total_count = total_count
