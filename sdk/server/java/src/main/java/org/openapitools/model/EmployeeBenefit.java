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
 * Gets or Sets EmployeeBenefit
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-08T09:39:48.398562-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum EmployeeBenefit {
  
  UNSPECIFIED("EMPLOYEE_BENEFIT_UNSPECIFIED"),
  
  HEALTH_INSURANCE("EMPLOYEE_BENEFIT_HEALTH_INSURANCE"),
  
  RETIREMENT_PLAN("EMPLOYEE_BENEFIT_RETIREMENT_PLAN"),
  
  PAID_TIME_OFF("EMPLOYEE_BENEFIT_PAID_TIME_OFF"),
  
  REMOTE_WORK("EMPLOYEE_BENEFIT_REMOTE_WORK");

  private String value;

  EmployeeBenefit(String value) {
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
  public static EmployeeBenefit fromValue(String value) {
    for (EmployeeBenefit b : EmployeeBenefit.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

