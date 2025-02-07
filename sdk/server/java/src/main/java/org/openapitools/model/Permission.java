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
 * Gets or Sets Permission
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T14:39:19.806517-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum Permission {
  
  UNSPECIFIED("PERMISSION_UNSPECIFIED"),
  
  READ("PERMISSION_READ"),
  
  WRITE("PERMISSION_WRITE"),
  
  DELETE("PERMISSION_DELETE"),
  
  MANAGE_USERS("PERMISSION_MANAGE_USERS"),
  
  MANAGE_BILLING("PERMISSION_MANAGE_BILLING"),
  
  VIEW_ANALYTICS("PERMISSION_VIEW_ANALYTICS"),
  
  MANAGE_WORKFLOWS("PERMISSION_MANAGE_WORKFLOWS");

  private String value;

  Permission(String value) {
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
  public static Permission fromValue(String value) {
    for (Permission b : Permission.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

