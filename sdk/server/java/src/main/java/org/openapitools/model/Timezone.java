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
 * Gets or Sets Timezone
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-13T18:55:20.876030-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum Timezone {
  
  UNSPECIFIED("TIMEZONE_UNSPECIFIED"),
  
  UTC("TIMEZONE_UTC"),
  
  EST("TIMEZONE_EST"),
  
  CST("TIMEZONE_CST"),
  
  MST("TIMEZONE_MST"),
  
  PST("TIMEZONE_PST"),
  
  GMT("TIMEZONE_GMT"),
  
  CET("TIMEZONE_CET"),
  
  IST("TIMEZONE_IST"),
  
  JST("TIMEZONE_JST"),
  
  AEST("TIMEZONE_AEST");

  private String value;

  Timezone(String value) {
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
  public static Timezone fromValue(String value) {
    for (Timezone b : Timezone.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

