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


class RevenueRange(str, Enum):
    """
    RevenueRange
    """

    """
    allowed enum values
    """
    REVENUE_RANGE_UNSPECIFIED = 'REVENUE_RANGE_UNSPECIFIED'
    REVENUE_RANGE_UNDER_100_K = 'REVENUE_RANGE_UNDER_100K'
    REVENUE_RANGE_100_K_TO_1_M = 'REVENUE_RANGE_100K_TO_1M'
    REVENUE_RANGE_1_M_TO_10_M = 'REVENUE_RANGE_1M_TO_10M'
    REVENUE_RANGE_10_M_TO_50_M = 'REVENUE_RANGE_10M_TO_50M'
    REVENUE_RANGE_OVER_50_M = 'REVENUE_RANGE_OVER_50M'

    @classmethod
    def from_json(cls, json_str: str) -> Self:
        """Create an instance of RevenueRange from a JSON string"""
        return cls(json.loads(json_str))


