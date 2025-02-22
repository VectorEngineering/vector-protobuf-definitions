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
from pydantic import BaseModel, ConfigDict, Field, StrictStr
from typing import Any, ClassVar, Dict, List, Optional
from playbookmedia_backend_client_sdk.models.api_key import APIKey
from typing import Optional, Set
from typing_extensions import Self

class RotateAPIKeyResponse(BaseModel):
    """
    RotateAPIKeyResponse
    """ # noqa: E501
    new_api_key: Optional[APIKey] = Field(default=None, alias="newApiKey")
    new_key_value: Optional[StrictStr] = Field(default=None, alias="newKeyValue")
    old_key_expiry: Optional[datetime] = Field(default=None, alias="oldKeyExpiry")
    __properties: ClassVar[List[str]] = ["newApiKey", "newKeyValue", "oldKeyExpiry"]

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
        """Create an instance of RotateAPIKeyResponse from a JSON string"""
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
        # override the default output from pydantic by calling `to_dict()` of new_api_key
        if self.new_api_key:
            _dict['newApiKey'] = self.new_api_key.to_dict()
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of RotateAPIKeyResponse from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "newApiKey": APIKey.from_dict(obj["newApiKey"]) if obj.get("newApiKey") is not None else None,
            "newKeyValue": obj.get("newKeyValue"),
            "oldKeyExpiry": obj.get("oldKeyExpiry")
        })
        return _obj


