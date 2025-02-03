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
 * Gets or Sets PayloadFormat
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-03T06:30:06.935816-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum PayloadFormat {
  
  UNSPECIFIED("PAYLOAD_FORMAT_UNSPECIFIED"),
  
  JSON("PAYLOAD_FORMAT_JSON"),
  
  XML("PAYLOAD_FORMAT_XML"),
  
  FORM_DATA("PAYLOAD_FORMAT_FORM_DATA"),
  
  PROTOBUF("PAYLOAD_FORMAT_PROTOBUF"),
  
  YAML("PAYLOAD_FORMAT_YAML");

  private String value;

  PayloadFormat(String value) {
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
  public static PayloadFormat fromValue(String value) {
    for (PayloadFormat b : PayloadFormat.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

