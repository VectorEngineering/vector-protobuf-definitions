# coding: utf-8

"""
    Lead Scraping Service API

    Vector Lead Scraping Service API - Manages Lead Scraping Jobs

    The version of the OpenAPI document: 1.0
    Contact: yoanyomba@vector.ai
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


from __future__ import annotations
import pprint
import re  # noqa: F401
import json

from datetime import datetime
from pydantic import BaseModel, ConfigDict, Field, StrictBool, StrictStr
from typing import Any, ClassVar, Dict, List, Optional
from playbookmedia_backend_client_sdk.models.contract_intelligence import ContractIntelligence
from playbookmedia_backend_client_sdk.models.document_instance import DocumentInstance
from playbookmedia_backend_client_sdk.models.explanation_block import ExplanationBlock
from playbookmedia_backend_client_sdk.models.template_type import TemplateType
from playbookmedia_backend_client_sdk.models.template_variable import TemplateVariable
from playbookmedia_backend_client_sdk.models.template_version import TemplateVersion
from typing import Optional, Set
from typing_extensions import Self

class DocumentTemplate(BaseModel):
    """
    DocumentTemplate
    """ # noqa: E501
    id: Optional[StrictStr] = None
    name: Optional[StrictStr] = None
    description: Optional[StrictStr] = None
    industry_type: Optional[StrictStr] = Field(default=None, alias="industryType")
    document_type: Optional[StrictStr] = Field(default=None, alias="documentType")
    base_content: Optional[StrictStr] = Field(default=None, alias="baseContent")
    is_adaptive: Optional[StrictBool] = Field(default=None, alias="isAdaptive")
    metadata: Optional[Dict[str, StrictStr]] = None
    supported_languages: Optional[List[StrictStr]] = Field(default=None, alias="supportedLanguages")
    required_data_fields: Optional[List[StrictStr]] = Field(default=None, alias="requiredDataFields")
    version: Optional[StrictStr] = None
    created_at: Optional[datetime] = Field(default=None, alias="createdAt")
    updated_at: Optional[datetime] = Field(default=None, alias="updatedAt")
    template_type: Optional[TemplateType] = Field(default=TemplateType.UNSPECIFIED, alias="templateType")
    content: Optional[StrictStr] = None
    detected_variables: Optional[List[StrictStr]] = Field(default=None, alias="detectedVariables")
    variable_mappings: Optional[Dict[str, StrictStr]] = Field(default=None, alias="variableMappings")
    is_context_aware: Optional[StrictBool] = Field(default=None, alias="isContextAware")
    supported_contexts: Optional[List[StrictStr]] = Field(default=None, alias="supportedContexts")
    formatting_rules: Optional[Dict[str, StrictStr]] = Field(default=None, alias="formattingRules")
    conditional_sections: Optional[List[StrictStr]] = Field(default=None, alias="conditionalSections")
    versions: Optional[List[TemplateVersion]] = None
    variables: Optional[List[TemplateVariable]] = None
    instances: Optional[List[DocumentInstance]] = None
    explanations: Optional[List[ExplanationBlock]] = None
    intelligence: Optional[List[ContractIntelligence]] = None
    __properties: ClassVar[List[str]] = ["id", "name", "description", "industryType", "documentType", "baseContent", "isAdaptive", "metadata", "supportedLanguages", "requiredDataFields", "version", "createdAt", "updatedAt", "templateType", "content", "detectedVariables", "variableMappings", "isContextAware", "supportedContexts", "formattingRules", "conditionalSections", "versions", "variables", "instances", "explanations", "intelligence"]

    model_config = ConfigDict(
        populate_by_name=True,
        validate_assignment=True,
        protected_namespaces=(),
    )


    def to_str(self) -> str:
        """Returns the string representation of the model using alias"""
        return pprint.pformat(self.model_dump(by_alias=True))

    def to_json(self) -> str:
        """Returns the JSON representation of the model using alias"""
        # TODO: pydantic v2: use .model_dump_json(by_alias=True, exclude_unset=True) instead
        return json.dumps(self.to_dict())

    @classmethod
    def from_json(cls, json_str: str) -> Optional[Self]:
        """Create an instance of DocumentTemplate from a JSON string"""
        return cls.from_dict(json.loads(json_str))

    def to_dict(self) -> Dict[str, Any]:
        """Return the dictionary representation of the model using alias.

        This has the following differences from calling pydantic's
        `self.model_dump(by_alias=True)`:

        * `None` is only added to the output dict for nullable fields that
          were set at model initialization. Other fields with value `None`
          are ignored.
        """
        excluded_fields: Set[str] = set([
        ])

        _dict = self.model_dump(
            by_alias=True,
            exclude=excluded_fields,
            exclude_none=True,
        )
        # override the default output from pydantic by calling `to_dict()` of each item in versions (list)
        _items = []
        if self.versions:
            for _item in self.versions:
                if _item:
                    _items.append(_item.to_dict())
            _dict['versions'] = _items
        # override the default output from pydantic by calling `to_dict()` of each item in variables (list)
        _items = []
        if self.variables:
            for _item in self.variables:
                if _item:
                    _items.append(_item.to_dict())
            _dict['variables'] = _items
        # override the default output from pydantic by calling `to_dict()` of each item in instances (list)
        _items = []
        if self.instances:
            for _item in self.instances:
                if _item:
                    _items.append(_item.to_dict())
            _dict['instances'] = _items
        # override the default output from pydantic by calling `to_dict()` of each item in explanations (list)
        _items = []
        if self.explanations:
            for _item in self.explanations:
                if _item:
                    _items.append(_item.to_dict())
            _dict['explanations'] = _items
        # override the default output from pydantic by calling `to_dict()` of each item in intelligence (list)
        _items = []
        if self.intelligence:
            for _item in self.intelligence:
                if _item:
                    _items.append(_item.to_dict())
            _dict['intelligence'] = _items
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of DocumentTemplate from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "id": obj.get("id"),
            "name": obj.get("name"),
            "description": obj.get("description"),
            "industryType": obj.get("industryType"),
            "documentType": obj.get("documentType"),
            "baseContent": obj.get("baseContent"),
            "isAdaptive": obj.get("isAdaptive"),
            "metadata": obj.get("metadata"),
            "supportedLanguages": obj.get("supportedLanguages"),
            "requiredDataFields": obj.get("requiredDataFields"),
            "version": obj.get("version"),
            "createdAt": obj.get("createdAt"),
            "updatedAt": obj.get("updatedAt"),
            "templateType": obj.get("templateType") if obj.get("templateType") is not None else TemplateType.UNSPECIFIED,
            "content": obj.get("content"),
            "detectedVariables": obj.get("detectedVariables"),
            "variableMappings": obj.get("variableMappings"),
            "isContextAware": obj.get("isContextAware"),
            "supportedContexts": obj.get("supportedContexts"),
            "formattingRules": obj.get("formattingRules"),
            "conditionalSections": obj.get("conditionalSections"),
            "versions": [TemplateVersion.from_dict(_item) for _item in obj["versions"]] if obj.get("versions") is not None else None,
            "variables": [TemplateVariable.from_dict(_item) for _item in obj["variables"]] if obj.get("variables") is not None else None,
            "instances": [DocumentInstance.from_dict(_item) for _item in obj["instances"]] if obj.get("instances") is not None else None,
            "explanations": [ExplanationBlock.from_dict(_item) for _item in obj["explanations"]] if obj.get("explanations") is not None else None,
            "intelligence": [ContractIntelligence.from_dict(_item) for _item in obj["intelligence"]] if obj.get("intelligence") is not None else None
        })
        return _obj


