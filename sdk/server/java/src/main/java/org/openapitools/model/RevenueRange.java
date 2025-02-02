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
 * Gets or Sets RevenueRange
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-02T05:34:49.009100-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum RevenueRange {
  
  UNSPECIFIED("REVENUE_RANGE_UNSPECIFIED"),
  
  UNDER_100_K("REVENUE_RANGE_UNDER_100K"),
  
  _100_K_TO_1_M("REVENUE_RANGE_100K_TO_1M"),
  
  _1_M_TO_10_M("REVENUE_RANGE_1M_TO_10M"),
  
  _10_M_TO_50_M("REVENUE_RANGE_10M_TO_50M"),
  
  OVER_50_M("REVENUE_RANGE_OVER_50M");

  private String value;

  RevenueRange(String value) {
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
  public static RevenueRange fromValue(String value) {
    for (RevenueRange b : RevenueRange.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

