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
 * - VALIDATION_ERROR: General validation errors  - AUTHORIZATION_MODEL_NOT_FOUND: Resource not found errors  - INVALID_CONTINUATION_TOKEN: Input errors  - DUPLICATE_ENTRY: Conflict errors  - INVALID_SCHEMA_VERSION: Schema errors  - INVALID_FORMAT: New validation error codes  General invalid format  - INVALID_EMAIL_FORMAT: Email format is invalid  - INVALID_PASSWORD_STRENGTH: Password does not meet strength requirements  - FIELD_TOO_SHORT: Field value is too short  - FIELD_TOO_LONG: Field value is too long  - UNSUPPORTED_MEDIA_TYPE: Media type is not supported
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-09T19:46:06.918250-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum ValidationErrorCode {
  
  NO_ERROR("NO_ERROR"),
  
  VALIDATION_ERROR("VALIDATION_ERROR"),
  
  INVALID_WRITE_INPUT("INVALID_WRITE_INPUT"),
  
  INVALID_CHECK_INPUT("INVALID_CHECK_INPUT"),
  
  INVALID_EXPAND_INPUT("INVALID_EXPAND_INPUT"),
  
  INVALID_OBJECT_FORMAT("INVALID_OBJECT_FORMAT"),
  
  INVALID_USER("INVALID_USER"),
  
  INVALID_TUPLE("INVALID_TUPLE"),
  
  AUTHORIZATION_MODEL_NOT_FOUND("AUTHORIZATION_MODEL_NOT_FOUND"),
  
  TYPE_NOT_FOUND("TYPE_NOT_FOUND"),
  
  RELATION_NOT_FOUND("RELATION_NOT_FOUND"),
  
  STORE_ID_INVALID_LENGTH("STORE_ID_INVALID_LENGTH"),
  
  ID_TOO_LONG("ID_TOO_LONG"),
  
  INVALID_CONTINUATION_TOKEN("INVALID_CONTINUATION_TOKEN"),
  
  PAGE_SIZE_INVALID("PAGE_SIZE_INVALID"),
  
  PARAM_MISSING_VALUE("PARAM_MISSING_VALUE"),
  
  EXCEEDED_ENTITY_LIMIT("EXCEEDED_ENTITY_LIMIT"),
  
  DUPLICATE_ENTRY("DUPLICATE_ENTRY"),
  
  CANNOT_ALLOW_DUPLICATE_TUPLES("CANNOT_ALLOW_DUPLICATE_TUPLES"),
  
  CANNOT_ALLOW_DUPLICATE_TYPES("CANNOT_ALLOW_DUPLICATE_TYPES"),
  
  INVALID_SCHEMA_VERSION("INVALID_SCHEMA_VERSION"),
  
  INVALID_TYPE_PATTERN("INVALID_TYPE_PATTERN"),
  
  INVALID_RELATION_PATTERN("INVALID_RELATION_PATTERN"),
  
  INVALID_OBJECT_PATTERN("INVALID_OBJECT_PATTERN"),
  
  INVALID_FORMAT("INVALID_FORMAT"),
  
  INVALID_EMAIL_FORMAT("INVALID_EMAIL_FORMAT"),
  
  INVALID_PASSWORD_STRENGTH("INVALID_PASSWORD_STRENGTH"),
  
  FIELD_TOO_SHORT("FIELD_TOO_SHORT"),
  
  FIELD_TOO_LONG("FIELD_TOO_LONG"),
  
  UNSUPPORTED_MEDIA_TYPE("UNSUPPORTED_MEDIA_TYPE");

  private String value;

  ValidationErrorCode(String value) {
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
  public static ValidationErrorCode fromValue(String value) {
    for (ValidationErrorCode b : ValidationErrorCode.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

