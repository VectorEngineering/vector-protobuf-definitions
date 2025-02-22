from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk import util


class TemplateVariable(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, id=None, name=None, description=None, variable_type=None, default_value=None, is_required=None, validation_rules=None, data_source=None, ai_extraction_rules=None, alternatives=None, template_id=None):  # noqa: E501
        """TemplateVariable - a model defined in OpenAPI

        :param id: The id of this TemplateVariable.  # noqa: E501
        :type id: str
        :param name: The name of this TemplateVariable.  # noqa: E501
        :type name: str
        :param description: The description of this TemplateVariable.  # noqa: E501
        :type description: str
        :param variable_type: The variable_type of this TemplateVariable.  # noqa: E501
        :type variable_type: str
        :param default_value: The default_value of this TemplateVariable.  # noqa: E501
        :type default_value: str
        :param is_required: The is_required of this TemplateVariable.  # noqa: E501
        :type is_required: bool
        :param validation_rules: The validation_rules of this TemplateVariable.  # noqa: E501
        :type validation_rules: str
        :param data_source: The data_source of this TemplateVariable.  # noqa: E501
        :type data_source: str
        :param ai_extraction_rules: The ai_extraction_rules of this TemplateVariable.  # noqa: E501
        :type ai_extraction_rules: str
        :param alternatives: The alternatives of this TemplateVariable.  # noqa: E501
        :type alternatives: List[str]
        :param template_id: The template_id of this TemplateVariable.  # noqa: E501
        :type template_id: str
        """
        self.openapi_types = {
            'id': str,
            'name': str,
            'description': str,
            'variable_type': str,
            'default_value': str,
            'is_required': bool,
            'validation_rules': str,
            'data_source': str,
            'ai_extraction_rules': str,
            'alternatives': List[str],
            'template_id': str
        }

        self.attribute_map = {
            'id': 'id',
            'name': 'name',
            'description': 'description',
            'variable_type': 'variableType',
            'default_value': 'defaultValue',
            'is_required': 'isRequired',
            'validation_rules': 'validationRules',
            'data_source': 'dataSource',
            'ai_extraction_rules': 'aiExtractionRules',
            'alternatives': 'alternatives',
            'template_id': 'templateId'
        }

        self._id = id
        self._name = name
        self._description = description
        self._variable_type = variable_type
        self._default_value = default_value
        self._is_required = is_required
        self._validation_rules = validation_rules
        self._data_source = data_source
        self._ai_extraction_rules = ai_extraction_rules
        self._alternatives = alternatives
        self._template_id = template_id

    @classmethod
    def from_dict(cls, dikt) -> 'TemplateVariable':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The TemplateVariable of this TemplateVariable.  # noqa: E501
        :rtype: TemplateVariable
        """
        return util.deserialize_model(dikt, cls)

    @property
    def id(self) -> str:
        """Gets the id of this TemplateVariable.


        :return: The id of this TemplateVariable.
        :rtype: str
        """
        return self._id

    @id.setter
    def id(self, id: str):
        """Sets the id of this TemplateVariable.


        :param id: The id of this TemplateVariable.
        :type id: str
        """

        self._id = id

    @property
    def name(self) -> str:
        """Gets the name of this TemplateVariable.


        :return: The name of this TemplateVariable.
        :rtype: str
        """
        return self._name

    @name.setter
    def name(self, name: str):
        """Sets the name of this TemplateVariable.


        :param name: The name of this TemplateVariable.
        :type name: str
        """

        self._name = name

    @property
    def description(self) -> str:
        """Gets the description of this TemplateVariable.


        :return: The description of this TemplateVariable.
        :rtype: str
        """
        return self._description

    @description.setter
    def description(self, description: str):
        """Sets the description of this TemplateVariable.


        :param description: The description of this TemplateVariable.
        :type description: str
        """

        self._description = description

    @property
    def variable_type(self) -> str:
        """Gets the variable_type of this TemplateVariable.


        :return: The variable_type of this TemplateVariable.
        :rtype: str
        """
        return self._variable_type

    @variable_type.setter
    def variable_type(self, variable_type: str):
        """Sets the variable_type of this TemplateVariable.


        :param variable_type: The variable_type of this TemplateVariable.
        :type variable_type: str
        """

        self._variable_type = variable_type

    @property
    def default_value(self) -> str:
        """Gets the default_value of this TemplateVariable.


        :return: The default_value of this TemplateVariable.
        :rtype: str
        """
        return self._default_value

    @default_value.setter
    def default_value(self, default_value: str):
        """Sets the default_value of this TemplateVariable.


        :param default_value: The default_value of this TemplateVariable.
        :type default_value: str
        """

        self._default_value = default_value

    @property
    def is_required(self) -> bool:
        """Gets the is_required of this TemplateVariable.


        :return: The is_required of this TemplateVariable.
        :rtype: bool
        """
        return self._is_required

    @is_required.setter
    def is_required(self, is_required: bool):
        """Sets the is_required of this TemplateVariable.


        :param is_required: The is_required of this TemplateVariable.
        :type is_required: bool
        """

        self._is_required = is_required

    @property
    def validation_rules(self) -> str:
        """Gets the validation_rules of this TemplateVariable.


        :return: The validation_rules of this TemplateVariable.
        :rtype: str
        """
        return self._validation_rules

    @validation_rules.setter
    def validation_rules(self, validation_rules: str):
        """Sets the validation_rules of this TemplateVariable.


        :param validation_rules: The validation_rules of this TemplateVariable.
        :type validation_rules: str
        """

        self._validation_rules = validation_rules

    @property
    def data_source(self) -> str:
        """Gets the data_source of this TemplateVariable.


        :return: The data_source of this TemplateVariable.
        :rtype: str
        """
        return self._data_source

    @data_source.setter
    def data_source(self, data_source: str):
        """Sets the data_source of this TemplateVariable.


        :param data_source: The data_source of this TemplateVariable.
        :type data_source: str
        """

        self._data_source = data_source

    @property
    def ai_extraction_rules(self) -> str:
        """Gets the ai_extraction_rules of this TemplateVariable.


        :return: The ai_extraction_rules of this TemplateVariable.
        :rtype: str
        """
        return self._ai_extraction_rules

    @ai_extraction_rules.setter
    def ai_extraction_rules(self, ai_extraction_rules: str):
        """Sets the ai_extraction_rules of this TemplateVariable.


        :param ai_extraction_rules: The ai_extraction_rules of this TemplateVariable.
        :type ai_extraction_rules: str
        """

        self._ai_extraction_rules = ai_extraction_rules

    @property
    def alternatives(self) -> List[str]:
        """Gets the alternatives of this TemplateVariable.


        :return: The alternatives of this TemplateVariable.
        :rtype: List[str]
        """
        return self._alternatives

    @alternatives.setter
    def alternatives(self, alternatives: List[str]):
        """Sets the alternatives of this TemplateVariable.


        :param alternatives: The alternatives of this TemplateVariable.
        :type alternatives: List[str]
        """

        self._alternatives = alternatives

    @property
    def template_id(self) -> str:
        """Gets the template_id of this TemplateVariable.


        :return: The template_id of this TemplateVariable.
        :rtype: str
        """
        return self._template_id

    @template_id.setter
    def template_id(self, template_id: str):
        """Sets the template_id of this TemplateVariable.


        :param template_id: The template_id of this TemplateVariable.
        :type template_id: str
        """

        self._template_id = template_id
