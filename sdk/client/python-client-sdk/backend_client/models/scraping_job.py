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
from pydantic import BaseModel, ConfigDict, Field, StrictBool, StrictBytes, StrictInt, StrictStr
from typing import Any, ClassVar, Dict, List, Optional, Union
from backend_client.models.background_job_status import BackgroundJobStatus
from typing import Optional, Set
from typing_extensions import Self

class ScrapingJob(BaseModel):
    """
    ScrapingJob represents a Google Maps scraping task. This message defines both the configuration and current state of a scraping operation.  Key components: - Basic metadata (id, name, timestamps) - Job status tracking - Search configuration parameters - Geographic settings - Performance options - Multi-tenant context  Database considerations: - Stored in \"scraping_jobs\" table - Uses GORM for ORM mapping - Includes foreign key to Account - Supports soft deletes  Usage example: ```go job := &ScrapingJob{     Name: \"Athens Cafes\",     Status: BackgroundJobStatus_BACKGROUND_JOB_STATUS_QUEUED,     Keywords: []string{\"cafe\", \"coffee\"},     Lang: \"el\",     Zoom: 15,     FastMode: true,     MaxTime: 3600, } ```
    """ # noqa: E501
    id: Optional[StrictStr] = None
    name: Optional[StrictStr] = None
    created_at: Optional[datetime] = Field(default=None, alias="createdAt")
    status: Optional[BackgroundJobStatus] = BackgroundJobStatus.UNSPECIFIED
    keywords: Optional[List[StrictStr]] = None
    lang: Optional[StrictStr] = None
    zoom: Optional[StrictInt] = None
    lat: Optional[StrictStr] = None
    lon: Optional[StrictStr] = None
    fast_mode: Optional[StrictBool] = Field(default=None, alias="fastMode")
    radius: Optional[StrictInt] = None
    depth: Optional[StrictInt] = None
    email: Optional[StrictBool] = None
    max_time: Optional[StrictInt] = Field(default=None, alias="maxTime")
    proxies: Optional[List[StrictStr]] = None
    updated_at: Optional[datetime] = Field(default=None, alias="updatedAt")
    deleted_at: Optional[datetime] = Field(default=None, alias="deletedAt")
    payload_type: Optional[StrictStr] = Field(default=None, alias="payloadType")
    priority: Optional[StrictInt] = None
    payload: Optional[Union[StrictBytes, StrictStr]] = None
    __properties: ClassVar[List[str]] = ["id", "name", "createdAt", "status", "keywords", "lang", "zoom", "lat", "lon", "fastMode", "radius", "depth", "email", "maxTime", "proxies", "updatedAt", "deletedAt", "payloadType", "priority", "payload"]

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
        """Create an instance of ScrapingJob from a JSON string"""
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
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of ScrapingJob from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "id": obj.get("id"),
            "name": obj.get("name"),
            "createdAt": obj.get("createdAt"),
            "status": obj.get("status") if obj.get("status") is not None else BackgroundJobStatus.UNSPECIFIED,
            "keywords": obj.get("keywords"),
            "lang": obj.get("lang"),
            "zoom": obj.get("zoom"),
            "lat": obj.get("lat"),
            "lon": obj.get("lon"),
            "fastMode": obj.get("fastMode"),
            "radius": obj.get("radius"),
            "depth": obj.get("depth"),
            "email": obj.get("email"),
            "maxTime": obj.get("maxTime"),
            "proxies": obj.get("proxies"),
            "updatedAt": obj.get("updatedAt"),
            "deletedAt": obj.get("deletedAt"),
            "payloadType": obj.get("payloadType"),
            "priority": obj.get("priority"),
            "payload": obj.get("payload")
        })
        return _obj


