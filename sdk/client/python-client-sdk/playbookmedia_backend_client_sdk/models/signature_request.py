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
from playbookmedia_backend_client_sdk.models.signature_block import SignatureBlock
from playbookmedia_backend_client_sdk.models.signature_status import SignatureStatus
from playbookmedia_backend_client_sdk.models.signature_workflow import SignatureWorkflow
from typing import Optional, Set
from typing_extensions import Self

class SignatureRequest(BaseModel):
    """
    SignatureRequest
    """ # noqa: E501
    id: Optional[StrictStr] = None
    status: Optional[SignatureStatus] = SignatureStatus.UNSPECIFIED
    signer_email: Optional[StrictStr] = Field(default=None, alias="signerEmail")
    signer_name: Optional[StrictStr] = Field(default=None, alias="signerName")
    role: Optional[StrictStr] = None
    authentication_method: Optional[StrictStr] = Field(default=None, alias="authenticationMethod")
    expires_at: Optional[datetime] = Field(default=None, alias="expiresAt")
    requires_mfa: Optional[StrictBool] = Field(default=None, alias="requiresMfa")
    signature_type: Optional[StrictStr] = Field(default=None, alias="signatureType")
    reminder_schedule: Optional[List[StrictStr]] = Field(default=None, alias="reminderSchedule")
    document_id: Optional[StrictStr] = Field(default=None, alias="documentId")
    signature_blocks: Optional[List[SignatureBlock]] = Field(default=None, alias="signatureBlocks")
    workflow: Optional[SignatureWorkflow] = None
    __properties: ClassVar[List[str]] = ["id", "status", "signerEmail", "signerName", "role", "authenticationMethod", "expiresAt", "requiresMfa", "signatureType", "reminderSchedule", "documentId", "signatureBlocks", "workflow"]

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
        """Create an instance of SignatureRequest from a JSON string"""
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
        # override the default output from pydantic by calling `to_dict()` of each item in signature_blocks (list)
        _items = []
        if self.signature_blocks:
            for _item in self.signature_blocks:
                if _item:
                    _items.append(_item.to_dict())
            _dict['signatureBlocks'] = _items
        # override the default output from pydantic by calling `to_dict()` of workflow
        if self.workflow:
            _dict['workflow'] = self.workflow.to_dict()
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of SignatureRequest from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "id": obj.get("id"),
            "status": obj.get("status") if obj.get("status") is not None else SignatureStatus.UNSPECIFIED,
            "signerEmail": obj.get("signerEmail"),
            "signerName": obj.get("signerName"),
            "role": obj.get("role"),
            "authenticationMethod": obj.get("authenticationMethod"),
            "expiresAt": obj.get("expiresAt"),
            "requiresMfa": obj.get("requiresMfa"),
            "signatureType": obj.get("signatureType"),
            "reminderSchedule": obj.get("reminderSchedule"),
            "documentId": obj.get("documentId"),
            "signatureBlocks": [SignatureBlock.from_dict(_item) for _item in obj["signatureBlocks"]] if obj.get("signatureBlocks") is not None else None,
            "workflow": SignatureWorkflow.from_dict(obj["workflow"]) if obj.get("workflow") is not None else None
        })
        return _obj


