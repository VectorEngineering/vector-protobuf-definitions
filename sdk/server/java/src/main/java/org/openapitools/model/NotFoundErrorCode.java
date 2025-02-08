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
 * - NO_NOT_FOUND_ERROR: Default value as per proto3 convention  - UNDEFINED_ENDPOINT: Endpoint errors  Requested endpoint doesn't exist  - UNIMPLEMENTED: Endpoint not implemented  - STORE_ID_NOT_FOUND: Resource errors  Requested store ID doesn't exist  - USER_NOT_FOUND: Requested user doesn't exist  - RESOURCE_NOT_FOUND: Generic resource not found  - TENANT_NOT_FOUND: Requested tenant doesn't exist
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-08T10:26:51.224811-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum NotFoundErrorCode {
  
  NO_NOT_FOUND_ERROR("NO_NOT_FOUND_ERROR"),
  
  UNDEFINED_ENDPOINT("UNDEFINED_ENDPOINT"),
  
  UNIMPLEMENTED("UNIMPLEMENTED"),
  
  STORE_ID_NOT_FOUND("STORE_ID_NOT_FOUND"),
  
  USER_NOT_FOUND("USER_NOT_FOUND"),
  
  RESOURCE_NOT_FOUND("RESOURCE_NOT_FOUND"),
  
  TENANT_NOT_FOUND("TENANT_NOT_FOUND");

  private String value;

  NotFoundErrorCode(String value) {
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
  public static NotFoundErrorCode fromValue(String value) {
    for (NotFoundErrorCode b : NotFoundErrorCode.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

