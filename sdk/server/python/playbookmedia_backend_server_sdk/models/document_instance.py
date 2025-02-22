from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk.models.ai_assistance_log import AIAssistanceLog
from playbookmedia_backend_server_sdk.models.document_status import DocumentStatus
from playbookmedia_backend_server_sdk.models.document_version import DocumentVersion
from playbookmedia_backend_server_sdk.models.negotiation_history import NegotiationHistory
from playbookmedia_backend_server_sdk.models.signature_request import SignatureRequest
from playbookmedia_backend_server_sdk import util

from playbookmedia_backend_server_sdk.models.ai_assistance_log import AIAssistanceLog  # noqa: E501
from playbookmedia_backend_server_sdk.models.document_status import DocumentStatus  # noqa: E501
from playbookmedia_backend_server_sdk.models.document_version import DocumentVersion  # noqa: E501
from playbookmedia_backend_server_sdk.models.negotiation_history import NegotiationHistory  # noqa: E501
from playbookmedia_backend_server_sdk.models.signature_request import SignatureRequest  # noqa: E501

class DocumentInstance(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, id=None, template_id=None, status=DocumentStatus.UNSPECIFIED, field_values=None, signers=None, language=None, is_completed=None, due_date=None, generated_content=None, attachments=None, created_at=None, updated_at=None, signature_requests=None, ai_assistance=None, negotiation=None, versions=None):  # noqa: E501
        """DocumentInstance - a model defined in OpenAPI

        :param id: The id of this DocumentInstance.  # noqa: E501
        :type id: str
        :param template_id: The template_id of this DocumentInstance.  # noqa: E501
        :type template_id: str
        :param status: The status of this DocumentInstance.  # noqa: E501
        :type status: DocumentStatus
        :param field_values: The field_values of this DocumentInstance.  # noqa: E501
        :type field_values: Dict[str, str]
        :param signers: The signers of this DocumentInstance.  # noqa: E501
        :type signers: List[str]
        :param language: The language of this DocumentInstance.  # noqa: E501
        :type language: str
        :param is_completed: The is_completed of this DocumentInstance.  # noqa: E501
        :type is_completed: bool
        :param due_date: The due_date of this DocumentInstance.  # noqa: E501
        :type due_date: datetime
        :param generated_content: The generated_content of this DocumentInstance.  # noqa: E501
        :type generated_content: str
        :param attachments: The attachments of this DocumentInstance.  # noqa: E501
        :type attachments: List[str]
        :param created_at: The created_at of this DocumentInstance.  # noqa: E501
        :type created_at: datetime
        :param updated_at: The updated_at of this DocumentInstance.  # noqa: E501
        :type updated_at: datetime
        :param signature_requests: The signature_requests of this DocumentInstance.  # noqa: E501
        :type signature_requests: List[SignatureRequest]
        :param ai_assistance: The ai_assistance of this DocumentInstance.  # noqa: E501
        :type ai_assistance: List[AIAssistanceLog]
        :param negotiation: The negotiation of this DocumentInstance.  # noqa: E501
        :type negotiation: NegotiationHistory
        :param versions: The versions of this DocumentInstance.  # noqa: E501
        :type versions: List[DocumentVersion]
        """
        self.openapi_types = {
            'id': str,
            'template_id': str,
            'status': DocumentStatus,
            'field_values': Dict[str, str],
            'signers': List[str],
            'language': str,
            'is_completed': bool,
            'due_date': datetime,
            'generated_content': str,
            'attachments': List[str],
            'created_at': datetime,
            'updated_at': datetime,
            'signature_requests': List[SignatureRequest],
            'ai_assistance': List[AIAssistanceLog],
            'negotiation': NegotiationHistory,
            'versions': List[DocumentVersion]
        }

        self.attribute_map = {
            'id': 'id',
            'template_id': 'templateId',
            'status': 'status',
            'field_values': 'fieldValues',
            'signers': 'signers',
            'language': 'language',
            'is_completed': 'isCompleted',
            'due_date': 'dueDate',
            'generated_content': 'generatedContent',
            'attachments': 'attachments',
            'created_at': 'createdAt',
            'updated_at': 'updatedAt',
            'signature_requests': 'signatureRequests',
            'ai_assistance': 'aiAssistance',
            'negotiation': 'negotiation',
            'versions': 'versions'
        }

        self._id = id
        self._template_id = template_id
        self._status = status
        self._field_values = field_values
        self._signers = signers
        self._language = language
        self._is_completed = is_completed
        self._due_date = due_date
        self._generated_content = generated_content
        self._attachments = attachments
        self._created_at = created_at
        self._updated_at = updated_at
        self._signature_requests = signature_requests
        self._ai_assistance = ai_assistance
        self._negotiation = negotiation
        self._versions = versions

    @classmethod
    def from_dict(cls, dikt) -> 'DocumentInstance':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The DocumentInstance of this DocumentInstance.  # noqa: E501
        :rtype: DocumentInstance
        """
        return util.deserialize_model(dikt, cls)

    @property
    def id(self) -> str:
        """Gets the id of this DocumentInstance.


        :return: The id of this DocumentInstance.
        :rtype: str
        """
        return self._id

    @id.setter
    def id(self, id: str):
        """Sets the id of this DocumentInstance.


        :param id: The id of this DocumentInstance.
        :type id: str
        """

        self._id = id

    @property
    def template_id(self) -> str:
        """Gets the template_id of this DocumentInstance.


        :return: The template_id of this DocumentInstance.
        :rtype: str
        """
        return self._template_id

    @template_id.setter
    def template_id(self, template_id: str):
        """Sets the template_id of this DocumentInstance.


        :param template_id: The template_id of this DocumentInstance.
        :type template_id: str
        """

        self._template_id = template_id

    @property
    def status(self) -> DocumentStatus:
        """Gets the status of this DocumentInstance.


        :return: The status of this DocumentInstance.
        :rtype: DocumentStatus
        """
        return self._status

    @status.setter
    def status(self, status: DocumentStatus):
        """Sets the status of this DocumentInstance.


        :param status: The status of this DocumentInstance.
        :type status: DocumentStatus
        """

        self._status = status

    @property
    def field_values(self) -> Dict[str, str]:
        """Gets the field_values of this DocumentInstance.


        :return: The field_values of this DocumentInstance.
        :rtype: Dict[str, str]
        """
        return self._field_values

    @field_values.setter
    def field_values(self, field_values: Dict[str, str]):
        """Sets the field_values of this DocumentInstance.


        :param field_values: The field_values of this DocumentInstance.
        :type field_values: Dict[str, str]
        """

        self._field_values = field_values

    @property
    def signers(self) -> List[str]:
        """Gets the signers of this DocumentInstance.


        :return: The signers of this DocumentInstance.
        :rtype: List[str]
        """
        return self._signers

    @signers.setter
    def signers(self, signers: List[str]):
        """Sets the signers of this DocumentInstance.


        :param signers: The signers of this DocumentInstance.
        :type signers: List[str]
        """

        self._signers = signers

    @property
    def language(self) -> str:
        """Gets the language of this DocumentInstance.


        :return: The language of this DocumentInstance.
        :rtype: str
        """
        return self._language

    @language.setter
    def language(self, language: str):
        """Sets the language of this DocumentInstance.


        :param language: The language of this DocumentInstance.
        :type language: str
        """

        self._language = language

    @property
    def is_completed(self) -> bool:
        """Gets the is_completed of this DocumentInstance.


        :return: The is_completed of this DocumentInstance.
        :rtype: bool
        """
        return self._is_completed

    @is_completed.setter
    def is_completed(self, is_completed: bool):
        """Sets the is_completed of this DocumentInstance.


        :param is_completed: The is_completed of this DocumentInstance.
        :type is_completed: bool
        """

        self._is_completed = is_completed

    @property
    def due_date(self) -> datetime:
        """Gets the due_date of this DocumentInstance.


        :return: The due_date of this DocumentInstance.
        :rtype: datetime
        """
        return self._due_date

    @due_date.setter
    def due_date(self, due_date: datetime):
        """Sets the due_date of this DocumentInstance.


        :param due_date: The due_date of this DocumentInstance.
        :type due_date: datetime
        """

        self._due_date = due_date

    @property
    def generated_content(self) -> str:
        """Gets the generated_content of this DocumentInstance.


        :return: The generated_content of this DocumentInstance.
        :rtype: str
        """
        return self._generated_content

    @generated_content.setter
    def generated_content(self, generated_content: str):
        """Sets the generated_content of this DocumentInstance.


        :param generated_content: The generated_content of this DocumentInstance.
        :type generated_content: str
        """

        self._generated_content = generated_content

    @property
    def attachments(self) -> List[str]:
        """Gets the attachments of this DocumentInstance.


        :return: The attachments of this DocumentInstance.
        :rtype: List[str]
        """
        return self._attachments

    @attachments.setter
    def attachments(self, attachments: List[str]):
        """Sets the attachments of this DocumentInstance.


        :param attachments: The attachments of this DocumentInstance.
        :type attachments: List[str]
        """

        self._attachments = attachments

    @property
    def created_at(self) -> datetime:
        """Gets the created_at of this DocumentInstance.


        :return: The created_at of this DocumentInstance.
        :rtype: datetime
        """
        return self._created_at

    @created_at.setter
    def created_at(self, created_at: datetime):
        """Sets the created_at of this DocumentInstance.


        :param created_at: The created_at of this DocumentInstance.
        :type created_at: datetime
        """

        self._created_at = created_at

    @property
    def updated_at(self) -> datetime:
        """Gets the updated_at of this DocumentInstance.


        :return: The updated_at of this DocumentInstance.
        :rtype: datetime
        """
        return self._updated_at

    @updated_at.setter
    def updated_at(self, updated_at: datetime):
        """Sets the updated_at of this DocumentInstance.


        :param updated_at: The updated_at of this DocumentInstance.
        :type updated_at: datetime
        """

        self._updated_at = updated_at

    @property
    def signature_requests(self) -> List[SignatureRequest]:
        """Gets the signature_requests of this DocumentInstance.


        :return: The signature_requests of this DocumentInstance.
        :rtype: List[SignatureRequest]
        """
        return self._signature_requests

    @signature_requests.setter
    def signature_requests(self, signature_requests: List[SignatureRequest]):
        """Sets the signature_requests of this DocumentInstance.


        :param signature_requests: The signature_requests of this DocumentInstance.
        :type signature_requests: List[SignatureRequest]
        """

        self._signature_requests = signature_requests

    @property
    def ai_assistance(self) -> List[AIAssistanceLog]:
        """Gets the ai_assistance of this DocumentInstance.


        :return: The ai_assistance of this DocumentInstance.
        :rtype: List[AIAssistanceLog]
        """
        return self._ai_assistance

    @ai_assistance.setter
    def ai_assistance(self, ai_assistance: List[AIAssistanceLog]):
        """Sets the ai_assistance of this DocumentInstance.


        :param ai_assistance: The ai_assistance of this DocumentInstance.
        :type ai_assistance: List[AIAssistanceLog]
        """

        self._ai_assistance = ai_assistance

    @property
    def negotiation(self) -> NegotiationHistory:
        """Gets the negotiation of this DocumentInstance.


        :return: The negotiation of this DocumentInstance.
        :rtype: NegotiationHistory
        """
        return self._negotiation

    @negotiation.setter
    def negotiation(self, negotiation: NegotiationHistory):
        """Sets the negotiation of this DocumentInstance.


        :param negotiation: The negotiation of this DocumentInstance.
        :type negotiation: NegotiationHistory
        """

        self._negotiation = negotiation

    @property
    def versions(self) -> List[DocumentVersion]:
        """Gets the versions of this DocumentInstance.


        :return: The versions of this DocumentInstance.
        :rtype: List[DocumentVersion]
        """
        return self._versions

    @versions.setter
    def versions(self, versions: List[DocumentVersion]):
        """Sets the versions of this DocumentInstance.


        :param versions: The versions of this DocumentInstance.
        :type versions: List[DocumentVersion]
        """

        self._versions = versions
