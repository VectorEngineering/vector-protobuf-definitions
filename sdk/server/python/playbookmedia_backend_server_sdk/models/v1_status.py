from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk import util


class V1Status(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    """
    allowed enum values
    """
    STATUS_UNSPECIFIED = 'STATUS_UNSPECIFIED'
    STATUS_ACTIVE = 'STATUS_ACTIVE'
    STATUS_SUSPENDED = 'STATUS_SUSPENDED'
    STATUS_PENDING_VERIFICATION = 'STATUS_PENDING_VERIFICATION'
    STATUS_REVOKED = 'STATUS_REVOKED'
    STATUS_EXPIRED = 'STATUS_EXPIRED'
    STATUS_RATE_LIMITED = 'STATUS_RATE_LIMITED'
    STATUS_PENDING_REVIEW = 'STATUS_PENDING_REVIEW'
    STATUS_DEPRECATED = 'STATUS_DEPRECATED'
    STATUS_MAINTENANCE = 'STATUS_MAINTENANCE'
    def __init__(self):  # noqa: E501
        """V1Status - a model defined in OpenAPI

        """
        self.openapi_types = {
        }

        self.attribute_map = {
        }

    @classmethod
    def from_dict(cls, dikt) -> 'V1Status':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The v1.Status of this V1Status.  # noqa: E501
        :rtype: V1Status
        """
        return util.deserialize_model(dikt, cls)
