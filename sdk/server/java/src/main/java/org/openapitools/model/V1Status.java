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
 * Gets or Sets v1.Status
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T14:04:25.138120-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum V1Status {
  
  UNSPECIFIED("STATUS_UNSPECIFIED"),
  
  ACTIVE("STATUS_ACTIVE"),
  
  SUSPENDED("STATUS_SUSPENDED"),
  
  PENDING_VERIFICATION("STATUS_PENDING_VERIFICATION"),
  
  REVOKED("STATUS_REVOKED"),
  
  EXPIRED("STATUS_EXPIRED"),
  
  RATE_LIMITED("STATUS_RATE_LIMITED"),
  
  PENDING_REVIEW("STATUS_PENDING_REVIEW"),
  
  DEPRECATED("STATUS_DEPRECATED"),
  
  MAINTENANCE("STATUS_MAINTENANCE");

  private String value;

  V1Status(String value) {
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
  public static V1Status fromValue(String value) {
    for (V1Status b : V1Status.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

