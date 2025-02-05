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
 * Gets or Sets TriggerEvent
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-05T14:03:51.683325-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum TriggerEvent {
  
  UNSPECIFIED("TRIGGER_EVENT_UNSPECIFIED"),
  
  JOB_STARTED("TRIGGER_EVENT_JOB_STARTED"),
  
  JOB_COMPLETED("TRIGGER_EVENT_JOB_COMPLETED"),
  
  JOB_FAILED("TRIGGER_EVENT_JOB_FAILED"),
  
  LEAD_FOUND("TRIGGER_EVENT_LEAD_FOUND"),
  
  QUOTA_EXCEEDED("TRIGGER_EVENT_QUOTA_EXCEEDED"),
  
  ERROR_THRESHOLD_REACHED("TRIGGER_EVENT_ERROR_THRESHOLD_REACHED"),
  
  RATE_LIMIT_REACHED("TRIGGER_EVENT_RATE_LIMIT_REACHED"),
  
  DATA_VALIDATION_FAILED("TRIGGER_EVENT_DATA_VALIDATION_FAILED"),
  
  NEW_PROXY_NEEDED("TRIGGER_EVENT_NEW_PROXY_NEEDED"),
  
  SCHEDULED_MAINTENANCE("TRIGGER_EVENT_SCHEDULED_MAINTENANCE");

  private String value;

  TriggerEvent(String value) {
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
  public static TriggerEvent fromValue(String value) {
    for (TriggerEvent b : TriggerEvent.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

