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
from pydantic import BaseModel, ConfigDict, Field, StrictFloat, StrictInt, StrictStr
from typing import Any, ClassVar, Dict, List, Optional, Union
from playbookmedia_backend_client_sdk.models.compliance_check import ComplianceCheck
from playbookmedia_backend_client_sdk.models.risk_assessment import RiskAssessment
from typing import Optional, Set
from typing_extensions import Self

class ContractIntelligence(BaseModel):
    """
    ContractIntelligence
    """ # noqa: E501
    id: Optional[StrictStr] = None
    contract_type: Optional[StrictStr] = Field(default=None, alias="contractType")
    risk_scores: Optional[Dict[str, Union[StrictFloat, StrictInt]]] = Field(default=None, alias="riskScores")
    detected_clauses: Optional[Dict[str, StrictStr]] = Field(default=None, alias="detectedClauses")
    key_obligations: Optional[List[StrictStr]] = Field(default=None, alias="keyObligations")
    compliance_scores: Optional[Dict[str, Union[StrictFloat, StrictInt]]] = Field(default=None, alias="complianceScores")
    jurisdiction: Optional[StrictStr] = None
    governing_laws: Optional[List[StrictStr]] = Field(default=None, alias="governingLaws")
    semantic_analysis: Optional[Dict[str, StrictStr]] = Field(default=None, alias="semanticAnalysis")
    analysis_date: Optional[datetime] = Field(default=None, alias="analysisDate")
    risk_assessments: Optional[List[RiskAssessment]] = Field(default=None, alias="riskAssessments")
    compliance_checks: Optional[List[ComplianceCheck]] = Field(default=None, alias="complianceChecks")
    __properties: ClassVar[List[str]] = ["id", "contractType", "riskScores", "detectedClauses", "keyObligations", "complianceScores", "jurisdiction", "governingLaws", "semanticAnalysis", "analysisDate", "riskAssessments", "complianceChecks"]

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
        """Create an instance of ContractIntelligence from a JSON string"""
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
        # override the default output from pydantic by calling `to_dict()` of each item in risk_assessments (list)
        _items = []
        if self.risk_assessments:
            for _item in self.risk_assessments:
                if _item:
                    _items.append(_item.to_dict())
            _dict['riskAssessments'] = _items
        # override the default output from pydantic by calling `to_dict()` of each item in compliance_checks (list)
        _items = []
        if self.compliance_checks:
            for _item in self.compliance_checks:
                if _item:
                    _items.append(_item.to_dict())
            _dict['complianceChecks'] = _items
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of ContractIntelligence from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "id": obj.get("id"),
            "contractType": obj.get("contractType"),
            "riskScores": obj.get("riskScores"),
            "detectedClauses": obj.get("detectedClauses"),
            "keyObligations": obj.get("keyObligations"),
            "complianceScores": obj.get("complianceScores"),
            "jurisdiction": obj.get("jurisdiction"),
            "governingLaws": obj.get("governingLaws"),
            "semanticAnalysis": obj.get("semanticAnalysis"),
            "analysisDate": obj.get("analysisDate"),
            "riskAssessments": [RiskAssessment.from_dict(_item) for _item in obj["riskAssessments"]] if obj.get("riskAssessments") is not None else None,
            "complianceChecks": [ComplianceCheck.from_dict(_item) for _item in obj["complianceChecks"]] if obj.get("complianceChecks") is not None else None
        })
        return _obj


