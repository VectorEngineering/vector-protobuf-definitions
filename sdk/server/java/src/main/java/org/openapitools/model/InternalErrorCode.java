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
 * - INTERNAL_ERROR: General internal errors  - RESOURCE_EXHAUSTED: Resource exhaustion errors  - CANCELLED: Operation errors  - UNAVAILABLE: Availability errors  - DEPENDENCY_FAILURE: New internal error codes  Failure in external dependency  - CONFIGURATION_ERROR: Service configuration error  - SECURITY_BREACH_DETECTED: Potential security breach detected
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T15:17:20.852245-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum InternalErrorCode {
  
  NO_INTERNAL_ERROR("NO_INTERNAL_ERROR"),
  
  INTERNAL_ERROR("INTERNAL_ERROR"),
  
  DATA_CORRUPTION("DATA_CORRUPTION"),
  
  DATABASE_ERROR("DATABASE_ERROR"),
  
  CACHE_ERROR("CACHE_ERROR"),
  
  RESOURCE_EXHAUSTED("RESOURCE_EXHAUSTED"),
  
  QUOTA_EXCEEDED("QUOTA_EXCEEDED"),
  
  RATE_LIMIT_EXCEEDED("RATE_LIMIT_EXCEEDED"),
  
  CANCELLED("CANCELLED"),
  
  DEADLINE_EXCEEDED("DEADLINE_EXCEEDED"),
  
  ALREADY_EXISTS("ALREADY_EXISTS"),
  
  FAILED_PRECONDITION("FAILED_PRECONDITION"),
  
  ABORTED("ABORTED"),
  
  OUT_OF_RANGE("OUT_OF_RANGE"),
  
  UNAVAILABLE("UNAVAILABLE"),
  
  DATA_LOSS("DATA_LOSS"),
  
  SERVICE_DEGRADED("SERVICE_DEGRADED"),
  
  DEPENDENCY_FAILURE("DEPENDENCY_FAILURE"),
  
  CONFIGURATION_ERROR("CONFIGURATION_ERROR"),
  
  SECURITY_BREACH_DETECTED("SECURITY_BREACH_DETECTED");

  private String value;

  InternalErrorCode(String value) {
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
  public static InternalErrorCode fromValue(String value) {
    for (InternalErrorCode b : InternalErrorCode.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

