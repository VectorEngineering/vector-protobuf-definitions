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
 * Gets or Sets AppCategory
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T08:12:58.450907-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum AppCategory {
  
  UNSPECIFIED("APP_CATEGORY_UNSPECIFIED"),
  
  CONTRACT_AUTOMATION("APP_CATEGORY_CONTRACT_AUTOMATION"),
  
  CONTRACT_ANALYSIS("APP_CATEGORY_CONTRACT_ANALYSIS"),
  
  INTEGRATION("APP_CATEGORY_INTEGRATION"),
  
  AI_POWERED("APP_CATEGORY_AI_POWERED"),
  
  WORKFLOW("APP_CATEGORY_WORKFLOW"),
  
  COMPLIANCE("APP_CATEGORY_COMPLIANCE");

  private String value;

  AppCategory(String value) {
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
  public static AppCategory fromValue(String value) {
    for (AppCategory b : AppCategory.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

