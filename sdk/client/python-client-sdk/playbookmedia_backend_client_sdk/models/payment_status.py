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


class PaymentStatus(str, Enum):
    """
    PaymentStatus
    """

    """
    allowed enum values
    """
    PAYMENT_STATUS_UNSPECIFIED = 'PAYMENT_STATUS_UNSPECIFIED'
    PAYMENT_STATUS_PAID = 'PAYMENT_STATUS_PAID'
    PAYMENT_STATUS_PAST_DUE = 'PAYMENT_STATUS_PAST_DUE'
    PAYMENT_STATUS_FAILED = 'PAYMENT_STATUS_FAILED'
    PAYMENT_STATUS_CANCELED = 'PAYMENT_STATUS_CANCELED'

    @classmethod
    def from_json(cls, json_str: str) -> Self:
        """Create an instance of PaymentStatus from a JSON string"""
        return cls(json.loads(json_str))


