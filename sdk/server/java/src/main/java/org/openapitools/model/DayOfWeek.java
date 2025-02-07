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
 * Gets or Sets DayOfWeek
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T14:39:19.806517-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum DayOfWeek {
  
  UNSPECIFIED("DAY_OF_WEEK_UNSPECIFIED"),
  
  MONDAY("DAY_OF_WEEK_MONDAY"),
  
  TUESDAY("DAY_OF_WEEK_TUESDAY"),
  
  WEDNESDAY("DAY_OF_WEEK_WEDNESDAY"),
  
  THURSDAY("DAY_OF_WEEK_THURSDAY"),
  
  FRIDAY("DAY_OF_WEEK_FRIDAY"),
  
  SATURDAY("DAY_OF_WEEK_SATURDAY"),
  
  SUNDAY("DAY_OF_WEEK_SUNDAY");

  private String value;

  DayOfWeek(String value) {
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
  public static DayOfWeek fromValue(String value) {
    for (DayOfWeek b : DayOfWeek.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

