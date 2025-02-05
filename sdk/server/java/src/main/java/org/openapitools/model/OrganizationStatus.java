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
 * Gets or Sets Organization.Status
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-05T07:11:09.755832-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum OrganizationStatus {
  
  UNSPECIFIED("STATUS_UNSPECIFIED"),
  
  ACTIVE("STATUS_ACTIVE"),
  
  SUSPENDED("STATUS_SUSPENDED"),
  
  PENDING_VERIFICATION("STATUS_PENDING_VERIFICATION");

  private String value;

  OrganizationStatus(String value) {
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
  public static OrganizationStatus fromValue(String value) {
    for (OrganizationStatus b : OrganizationStatus.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

