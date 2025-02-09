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
 * Gets or Sets SignatureStatus
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-09T18:48:15.982371-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum SignatureStatus {
  
  UNSPECIFIED("SIGNATURE_STATUS_UNSPECIFIED"),
  
  PENDING("SIGNATURE_STATUS_PENDING"),
  
  SIGNED("SIGNATURE_STATUS_SIGNED"),
  
  REJECTED("SIGNATURE_STATUS_REJECTED"),
  
  EXPIRED("SIGNATURE_STATUS_EXPIRED"),
  
  REVOKED("SIGNATURE_STATUS_REVOKED");

  private String value;

  SignatureStatus(String value) {
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
  public static SignatureStatus fromValue(String value) {
    for (SignatureStatus b : SignatureStatus.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

