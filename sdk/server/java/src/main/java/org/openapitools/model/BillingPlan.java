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
 * Gets or Sets BillingPlan
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-19T11:32:59.900685-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum BillingPlan {
  
  UNSPECIFIED("BILLING_PLAN_UNSPECIFIED"),
  
  STARTUP("BILLING_PLAN_STARTUP"),
  
  BUSINESS("BILLING_PLAN_BUSINESS"),
  
  ENTERPRISE("BILLING_PLAN_ENTERPRISE");

  private String value;

  BillingPlan(String value) {
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
  public static BillingPlan fromValue(String value) {
    for (BillingPlan b : BillingPlan.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

