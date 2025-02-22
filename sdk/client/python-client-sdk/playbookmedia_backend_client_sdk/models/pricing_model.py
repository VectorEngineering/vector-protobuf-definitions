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
import json
from enum import Enum
from typing_extensions import Self


class PricingModel(str, Enum):
    """
    PricingModel
    """

    """
    allowed enum values
    """
    PRICING_MODEL_UNSPECIFIED = 'PRICING_MODEL_UNSPECIFIED'
    PRICING_MODEL_FREE = 'PRICING_MODEL_FREE'
    PRICING_MODEL_SUBSCRIPTION = 'PRICING_MODEL_SUBSCRIPTION'
    PRICING_MODEL_USAGE_BASED = 'PRICING_MODEL_USAGE_BASED'
    PRICING_MODEL_ENTERPRISE = 'PRICING_MODEL_ENTERPRISE'

    @classmethod
    def from_json(cls, json_str: str) -> Self:
        """Create an instance of PricingModel from a JSON string"""
        return cls(json.loads(json_str))


