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
 * Gets or Sets ComplianceLevel
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-09T18:48:15.982371-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum ComplianceLevel {
  
  UNSPECIFIED("COMPLIANCE_LEVEL_UNSPECIFIED"),
  
  NONE("COMPLIANCE_LEVEL_NONE"),
  
  BASIC("COMPLIANCE_LEVEL_BASIC"),
  
  ADVANCED("COMPLIANCE_LEVEL_ADVANCED"),
  
  ENTERPRISE("COMPLIANCE_LEVEL_ENTERPRISE");

  private String value;

  ComplianceLevel(String value) {
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
  public static ComplianceLevel fromValue(String value) {
    for (ComplianceLevel b : ComplianceLevel.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

