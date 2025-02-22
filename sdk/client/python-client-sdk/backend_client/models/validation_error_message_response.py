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

from pydantic import BaseModel, ConfigDict, Field, StrictStr
from typing import Any, ClassVar, Dict, List, Optional
from backend_client.models.error_response import ErrorResponse
from backend_client.models.field_violation import FieldViolation
from backend_client.models.resource_validation import ResourceValidation
from backend_client.models.schema_validation import SchemaValidation
from backend_client.models.validation_error_code import ValidationErrorCode
from typing import Optional, Set
from typing_extensions import Self

class ValidationErrorMessageResponse(BaseModel):
    """
    Represents data validation and business logic errors
    """ # noqa: E501
    code: Optional[ValidationErrorCode] = ValidationErrorCode.NO_ERROR
    message: Optional[StrictStr] = None
    field_violations: Optional[List[FieldViolation]] = Field(default=None, alias="fieldViolations")
    schema_validation: Optional[SchemaValidation] = Field(default=None, alias="schemaValidation")
    resource_validation: Optional[ResourceValidation] = Field(default=None, alias="resourceValidation")
    error_response: Optional[ErrorResponse] = Field(default=None, alias="errorResponse")
    __properties: ClassVar[List[str]] = ["code", "message", "fieldViolations", "schemaValidation", "resourceValidation", "errorResponse"]

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
        """Create an instance of ValidationErrorMessageResponse from a JSON string"""
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
        # override the default output from pydantic by calling `to_dict()` of each item in field_violations (list)
        _items = []
        if self.field_violations:
            for _item in self.field_violations:
                if _item:
                    _items.append(_item.to_dict())
            _dict['fieldViolations'] = _items
        # override the default output from pydantic by calling `to_dict()` of schema_validation
        if self.schema_validation:
            _dict['schemaValidation'] = self.schema_validation.to_dict()
        # override the default output from pydantic by calling `to_dict()` of resource_validation
        if self.resource_validation:
            _dict['resourceValidation'] = self.resource_validation.to_dict()
        # override the default output from pydantic by calling `to_dict()` of error_response
        if self.error_response:
            _dict['errorResponse'] = self.error_response.to_dict()
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of ValidationErrorMessageResponse from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "code": obj.get("code") if obj.get("code") is not None else ValidationErrorCode.NO_ERROR,
            "message": obj.get("message"),
            "fieldViolations": [FieldViolation.from_dict(_item) for _item in obj["fieldViolations"]] if obj.get("fieldViolations") is not None else None,
            "schemaValidation": SchemaValidation.from_dict(obj["schemaValidation"]) if obj.get("schemaValidation") is not None else None,
            "resourceValidation": ResourceValidation.from_dict(obj["resourceValidation"]) if obj.get("resourceValidation") is not None else None,
            "errorResponse": ErrorResponse.from_dict(obj["errorResponse"]) if obj.get("errorResponse") is not None else None
        })
        return _obj


