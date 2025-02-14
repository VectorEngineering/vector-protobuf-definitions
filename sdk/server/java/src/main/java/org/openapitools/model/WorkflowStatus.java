package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonValue;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

/**
 * WorkflowStatus represents the current state of a scraping workflow. This enum tracks the lifecycle of automated, recurring scraping configurations.  State transitions: 1. DRAFT -> ACTIVE/ARCHIVED 2. ACTIVE -> PAUSED/FAILED/COMPLETED/ARCHIVED 3. PAUSED -> ACTIVE/ARCHIVED 4. FAILED -> ACTIVE/ARCHIVED   - WORKFLOW_STATUS_UNSPECIFIED: Default state, should not be used explicitly  - WORKFLOW_STATUS_DRAFT: Initial draft state before activation  - WORKFLOW_STATUS_ACTIVE: Workflow is actively running on schedule  - WORKFLOW_STATUS_PAUSED: Workflow is temporarily suspended  - WORKFLOW_STATUS_FAILED: Workflow encountered an error  - WORKFLOW_STATUS_COMPLETED: Workflow completed all scheduled runs  - WORKFLOW_STATUS_ARCHIVED: Workflow is archived (soft-deleted)  - WORKFLOW_STATUS_PENDING_APPROVAL: Workflow is pending approval  - WORKFLOW_STATUS_VALIDATING: Workflow is being validated  - WORKFLOW_STATUS_QUOTA_EXCEEDED: Workflow exceeded resource quotas  - WORKFLOW_STATUS_WARNING: Workflow is running but with warnings
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T00:28:49.378974-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum WorkflowStatus {
  
  UNSPECIFIED("WORKFLOW_STATUS_UNSPECIFIED"),
  
  DRAFT("WORKFLOW_STATUS_DRAFT"),
  
  ACTIVE("WORKFLOW_STATUS_ACTIVE"),
  
  PAUSED("WORKFLOW_STATUS_PAUSED"),
  
  FAILED("WORKFLOW_STATUS_FAILED"),
  
  COMPLETED("WORKFLOW_STATUS_COMPLETED"),
  
  ARCHIVED("WORKFLOW_STATUS_ARCHIVED"),
  
  PENDING_APPROVAL("WORKFLOW_STATUS_PENDING_APPROVAL"),
  
  VALIDATING("WORKFLOW_STATUS_VALIDATING"),
  
  QUOTA_EXCEEDED("WORKFLOW_STATUS_QUOTA_EXCEEDED"),
  
  WARNING("WORKFLOW_STATUS_WARNING");

  private String value;

  WorkflowStatus(String value) {
    this.value = value;
  }

  @JsonValue
  public String getValue() {
    return value;
  }

  @Override
  public String toString() {
    return String.valueOf(value);
  }

  @JsonCreator
  public static WorkflowStatus fromValue(String value) {
    for (WorkflowStatus b : WorkflowStatus.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

