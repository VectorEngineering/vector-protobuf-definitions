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
 * Gets or Sets TemplateType
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-09T18:48:15.982371-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum TemplateType {
  
  UNSPECIFIED("TEMPLATE_TYPE_UNSPECIFIED"),
  
  STANDARD("TEMPLATE_TYPE_STANDARD"),
  
  SMART("TEMPLATE_TYPE_SMART"),
  
  ADAPTIVE("TEMPLATE_TYPE_ADAPTIVE"),
  
  AI_GENERATED("TEMPLATE_TYPE_AI_GENERATED");

  private String value;

  TemplateType(String value) {
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
  public static TemplateType fromValue(String value) {
    for (TemplateType b : TemplateType.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

