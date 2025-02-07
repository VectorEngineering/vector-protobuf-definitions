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
 * Gets or Sets Role
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T15:17:20.852245-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum Role {
  
  UNSPECIFIED("ROLE_UNSPECIFIED"),
  
  ADMIN("ROLE_ADMIN"),
  
  USER("ROLE_USER"),
  
  VIEWER("ROLE_VIEWER"),
  
  MANAGER("ROLE_MANAGER");

  private String value;

  Role(String value) {
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
  public static Role fromValue(String value) {
    for (Role b : Role.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

