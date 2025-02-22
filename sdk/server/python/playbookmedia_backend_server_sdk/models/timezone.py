from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk import util


class Timezone(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    """
    allowed enum values
    """
    TIMEZONE_UNSPECIFIED = 'TIMEZONE_UNSPECIFIED'
    TIMEZONE_UTC = 'TIMEZONE_UTC'
    TIMEZONE_EST = 'TIMEZONE_EST'
    TIMEZONE_CST = 'TIMEZONE_CST'
    TIMEZONE_MST = 'TIMEZONE_MST'
    TIMEZONE_PST = 'TIMEZONE_PST'
    TIMEZONE_GMT = 'TIMEZONE_GMT'
    TIMEZONE_CET = 'TIMEZONE_CET'
    TIMEZONE_IST = 'TIMEZONE_IST'
    TIMEZONE_JST = 'TIMEZONE_JST'
    TIMEZONE_AEST = 'TIMEZONE_AEST'
    def __init__(self):  # noqa: E501
        """Timezone - a model defined in OpenAPI

        """
        self.openapi_types = {
        }

        self.attribute_map = {
        }

    @classmethod
    def from_dict(cls, dikt) -> 'Timezone':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The Timezone of this Timezone.  # noqa: E501
        :rtype: Timezone
        """
        return util.deserialize_model(dikt, cls)
