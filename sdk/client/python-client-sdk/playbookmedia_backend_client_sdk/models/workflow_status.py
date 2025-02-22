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


class WorkflowStatus(str, Enum):
    """
    WorkflowStatus represents the current state of a scraping workflow. This enum tracks the lifecycle of automated, recurring scraping configurations.  State transitions: 1. DRAFT -> ACTIVE/ARCHIVED 2. ACTIVE -> PAUSED/FAILED/COMPLETED/ARCHIVED 3. PAUSED -> ACTIVE/ARCHIVED 4. FAILED -> ACTIVE/ARCHIVED   - WORKFLOW_STATUS_UNSPECIFIED: Default state, should not be used explicitly  - WORKFLOW_STATUS_DRAFT: Initial draft state before activation  - WORKFLOW_STATUS_ACTIVE: Workflow is actively running on schedule  - WORKFLOW_STATUS_PAUSED: Workflow is temporarily suspended  - WORKFLOW_STATUS_FAILED: Workflow encountered an error  - WORKFLOW_STATUS_COMPLETED: Workflow completed all scheduled runs  - WORKFLOW_STATUS_ARCHIVED: Workflow is archived (soft-deleted)  - WORKFLOW_STATUS_PENDING_APPROVAL: Workflow is pending approval  - WORKFLOW_STATUS_VALIDATING: Workflow is being validated  - WORKFLOW_STATUS_QUOTA_EXCEEDED: Workflow exceeded resource quotas  - WORKFLOW_STATUS_WARNING: Workflow is running but with warnings
    """

    """
    allowed enum values
    """
    WORKFLOW_STATUS_UNSPECIFIED = 'WORKFLOW_STATUS_UNSPECIFIED'
    WORKFLOW_STATUS_DRAFT = 'WORKFLOW_STATUS_DRAFT'
    WORKFLOW_STATUS_ACTIVE = 'WORKFLOW_STATUS_ACTIVE'
    WORKFLOW_STATUS_PAUSED = 'WORKFLOW_STATUS_PAUSED'
    WORKFLOW_STATUS_FAILED = 'WORKFLOW_STATUS_FAILED'
    WORKFLOW_STATUS_COMPLETED = 'WORKFLOW_STATUS_COMPLETED'
    WORKFLOW_STATUS_ARCHIVED = 'WORKFLOW_STATUS_ARCHIVED'
    WORKFLOW_STATUS_PENDING_APPROVAL = 'WORKFLOW_STATUS_PENDING_APPROVAL'
    WORKFLOW_STATUS_VALIDATING = 'WORKFLOW_STATUS_VALIDATING'
    WORKFLOW_STATUS_QUOTA_EXCEEDED = 'WORKFLOW_STATUS_QUOTA_EXCEEDED'
    WORKFLOW_STATUS_WARNING = 'WORKFLOW_STATUS_WARNING'

    @classmethod
    def from_json(cls, json_str: str) -> Self:
        """Create an instance of WorkflowStatus from a JSON string"""
        return cls(json.loads(json_str))


