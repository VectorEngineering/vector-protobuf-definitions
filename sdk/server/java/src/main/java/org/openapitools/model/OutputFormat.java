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
 * Gets or Sets OutputFormat
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-01-29T12:50:06.029660-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum OutputFormat {
  
  UNSPECIFIED("OUTPUT_FORMAT_UNSPECIFIED"),
  
  JSON("OUTPUT_FORMAT_JSON"),
  
  CSV("OUTPUT_FORMAT_CSV"),
  
  BIGQUERY("OUTPUT_FORMAT_BIGQUERY"),
  
  POSTGRES("OUTPUT_FORMAT_POSTGRES");

  private String value;

  OutputFormat(String value) {
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
  public static OutputFormat fromValue(String value) {
    for (OutputFormat b : OutputFormat.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

