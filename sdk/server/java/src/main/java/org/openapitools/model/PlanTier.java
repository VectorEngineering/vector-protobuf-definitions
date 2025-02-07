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
 * Gets or Sets PlanTier
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T14:39:19.806517-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum PlanTier {
  
  UNSPECIFIED("PLAN_TIER_UNSPECIFIED"),
  
  FREE("PLAN_TIER_FREE"),
  
  STARTER("PLAN_TIER_STARTER"),
  
  PROFESSIONAL("PLAN_TIER_PROFESSIONAL"),
  
  ENTERPRISE("PLAN_TIER_ENTERPRISE");

  private String value;

  PlanTier(String value) {
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
  public static PlanTier fromValue(String value) {
    for (PlanTier b : PlanTier.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

