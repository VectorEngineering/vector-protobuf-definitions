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
 * Gets or Sets APIKey.Status
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-03T10:37:17.791439-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum APIKeyStatus {
  
  UNSPECIFIED("STATUS_UNSPECIFIED"),
  
  ACTIVE("STATUS_ACTIVE"),
  
  REVOKED("STATUS_REVOKED"),
  
  EXPIRED("STATUS_EXPIRED"),
  
  RATE_LIMITED("STATUS_RATE_LIMITED"),
  
  SUSPENDED("STATUS_SUSPENDED"),
  
  PENDING_REVIEW("STATUS_PENDING_REVIEW"),
  
  DEPRECATED("STATUS_DEPRECATED");

  private String value;

  APIKeyStatus(String value) {
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
  public static APIKeyStatus fromValue(String value) {
    for (APIKeyStatus b : APIKeyStatus.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

