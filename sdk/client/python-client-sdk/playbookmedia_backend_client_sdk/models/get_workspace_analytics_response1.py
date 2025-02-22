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

from pydantic import BaseModel, ConfigDict, Field
from typing import Any, ClassVar, Dict, List, Optional
from playbookmedia_backend_client_sdk.models.activity_metrics import ActivityMetrics
from playbookmedia_backend_client_sdk.models.compliance_metrics import ComplianceMetrics
from playbookmedia_backend_client_sdk.models.user_activity import UserActivity
from playbookmedia_backend_client_sdk.models.workspace_activity import WorkspaceActivity
from typing import Optional, Set
from typing_extensions import Self

class GetWorkspaceAnalyticsResponse1(BaseModel):
    """
    GetWorkspaceAnalyticsResponse1
    """ # noqa: E501
    activity: Optional[ActivityMetrics] = None
    user_activities: Optional[List[UserActivity]] = Field(default=None, alias="userActivities")
    compliance: Optional[ComplianceMetrics] = None
    recent_activities: Optional[List[WorkspaceActivity]] = Field(default=None, alias="recentActivities")
    __properties: ClassVar[List[str]] = ["activity", "userActivities", "compliance", "recentActivities"]

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
        """Create an instance of GetWorkspaceAnalyticsResponse1 from a JSON string"""
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
        # override the default output from pydantic by calling `to_dict()` of activity
        if self.activity:
            _dict['activity'] = self.activity.to_dict()
        # override the default output from pydantic by calling `to_dict()` of each item in user_activities (list)
        _items = []
        if self.user_activities:
            for _item in self.user_activities:
                if _item:
                    _items.append(_item.to_dict())
            _dict['userActivities'] = _items
        # override the default output from pydantic by calling `to_dict()` of compliance
        if self.compliance:
            _dict['compliance'] = self.compliance.to_dict()
        # override the default output from pydantic by calling `to_dict()` of each item in recent_activities (list)
        _items = []
        if self.recent_activities:
            for _item in self.recent_activities:
                if _item:
                    _items.append(_item.to_dict())
            _dict['recentActivities'] = _items
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of GetWorkspaceAnalyticsResponse1 from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "activity": ActivityMetrics.from_dict(obj["activity"]) if obj.get("activity") is not None else None,
            "userActivities": [UserActivity.from_dict(_item) for _item in obj["userActivities"]] if obj.get("userActivities") is not None else None,
            "compliance": ComplianceMetrics.from_dict(obj["compliance"]) if obj.get("compliance") is not None else None,
            "recentActivities": [WorkspaceActivity.from_dict(_item) for _item in obj["recentActivities"]] if obj.get("recentActivities") is not None else None
        })
        return _obj


