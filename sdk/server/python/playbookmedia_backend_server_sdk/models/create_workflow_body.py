from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk.models.scraping_workflow import ScrapingWorkflow
from playbookmedia_backend_server_sdk import util

from playbookmedia_backend_server_sdk.models.scraping_workflow import ScrapingWorkflow  # noqa: E501

class CreateWorkflowBody(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, workflow=None):  # noqa: E501
        """CreateWorkflowBody - a model defined in OpenAPI

        :param workflow: The workflow of this CreateWorkflowBody.  # noqa: E501
        :type workflow: ScrapingWorkflow
        """
        self.openapi_types = {
            'workflow': ScrapingWorkflow
        }

        self.attribute_map = {
            'workflow': 'workflow'
        }

        self._workflow = workflow

    @classmethod
    def from_dict(cls, dikt) -> 'CreateWorkflowBody':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The CreateWorkflowBody of this CreateWorkflowBody.  # noqa: E501
        :rtype: CreateWorkflowBody
        """
        return util.deserialize_model(dikt, cls)

    @property
    def workflow(self) -> ScrapingWorkflow:
        """Gets the workflow of this CreateWorkflowBody.


        :return: The workflow of this CreateWorkflowBody.
        :rtype: ScrapingWorkflow
        """
        return self._workflow

    @workflow.setter
    def workflow(self, workflow: ScrapingWorkflow):
        """Sets the workflow of this CreateWorkflowBody.


        :param workflow: The workflow of this CreateWorkflowBody.
        :type workflow: ScrapingWorkflow
        """

        self._workflow = workflow
