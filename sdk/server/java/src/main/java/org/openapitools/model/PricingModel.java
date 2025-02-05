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
 * Gets or Sets PricingModel
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-05T14:06:38.155571-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum PricingModel {
  
  UNSPECIFIED("PRICING_MODEL_UNSPECIFIED"),
  
  FREE("PRICING_MODEL_FREE"),
  
  SUBSCRIPTION("PRICING_MODEL_SUBSCRIPTION"),
  
  USAGE_BASED("PRICING_MODEL_USAGE_BASED"),
  
  ENTERPRISE("PRICING_MODEL_ENTERPRISE");

  private String value;

  PricingModel(String value) {
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
  public static PricingModel fromValue(String value) {
    for (PricingModel b : PricingModel.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

