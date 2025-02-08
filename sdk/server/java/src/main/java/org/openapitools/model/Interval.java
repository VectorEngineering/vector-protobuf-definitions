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
 * Gets or Sets Interval
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-08T10:26:51.224811-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum Interval {
  
  UNSPECIFIED("INTERVAL_UNSPECIFIED"),
  
  MONTHLY("INTERVAL_MONTHLY"),
  
  YEARLY("INTERVAL_YEARLY");

  private String value;

  Interval(String value) {
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
  public static Interval fromValue(String value) {
    for (Interval b : Interval.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

