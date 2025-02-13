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
 * Gets or Sets DocumentStatus
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-13T18:55:20.876030-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum DocumentStatus {
  
  UNSPECIFIED("DOCUMENT_STATUS_UNSPECIFIED"),
  
  DRAFT("DOCUMENT_STATUS_DRAFT"),
  
  IN_REVIEW("DOCUMENT_STATUS_IN_REVIEW"),
  
  APPROVED("DOCUMENT_STATUS_APPROVED"),
  
  REJECTED("DOCUMENT_STATUS_REJECTED"),
  
  EXPIRED("DOCUMENT_STATUS_EXPIRED"),
  
  ARCHIVED("DOCUMENT_STATUS_ARCHIVED");

  private String value;

  DocumentStatus(String value) {
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
  public static DocumentStatus fromValue(String value) {
    for (DocumentStatus b : DocumentStatus.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

