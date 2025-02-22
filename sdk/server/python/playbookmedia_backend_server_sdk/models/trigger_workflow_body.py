from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk import util


class TriggerWorkflowBody(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, parameters=None):  # noqa: E501
        """TriggerWorkflowBody - a model defined in OpenAPI

        :param parameters: The parameters of this TriggerWorkflowBody.  # noqa: E501
        :type parameters: Dict[str, str]
        """
        self.openapi_types = {
            'parameters': Dict[str, str]
        }

        self.attribute_map = {
            'parameters': 'parameters'
        }

        self._parameters = parameters

    @classmethod
    def from_dict(cls, dikt) -> 'TriggerWorkflowBody':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The TriggerWorkflowBody of this TriggerWorkflowBody.  # noqa: E501
        :rtype: TriggerWorkflowBody
        """
        return util.deserialize_model(dikt, cls)

    @property
    def parameters(self) -> Dict[str, str]:
        """Gets the parameters of this TriggerWorkflowBody.


        :return: The parameters of this TriggerWorkflowBody.
        :rtype: Dict[str, str]
        """
        return self._parameters

    @parameters.setter
    def parameters(self, parameters: Dict[str, str]):
        """Sets the parameters of this TriggerWorkflowBody.


        :param parameters: The parameters of this TriggerWorkflowBody.
        :type parameters: Dict[str, str]
        """

        self._parameters = parameters
