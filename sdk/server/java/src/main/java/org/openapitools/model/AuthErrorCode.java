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
 * - AUTH_FAILED_INVALID_BEARER_TOKEN: Authentication errors  - SESSION_EXPIRED: Session errors  - UNAUTHENTICATED: Other authentication errors  - ACCOUNT_LOCKED: New authentication error codes  Account is locked due to too many failed login attempts  - ACCOUNT_DISABLED: Account has been disabled by admin  - PASSWORD_EXPIRED: Password has expired and must be changed  - PASSWORD_RESET_REQUIRED: Password reset is required  - UNRECOGNIZED_DEVICE: Login attempt from an unrecognized device
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-19T01:49:19.350068-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum AuthErrorCode {
  
  NO_AUTH_ERROR("NO_AUTH_ERROR"),
  
  AUTH_FAILED_INVALID_BEARER_TOKEN("AUTH_FAILED_INVALID_BEARER_TOKEN"),
  
  AUTH_FAILED_INVALID_SUBJECT("AUTH_FAILED_INVALID_SUBJECT"),
  
  AUTH_FAILED_INVALID_AUDIENCE("AUTH_FAILED_INVALID_AUDIENCE"),
  
  AUTH_FAILED_INVALID_ISSUER("AUTH_FAILED_INVALID_ISSUER"),
  
  BEARER_TOKEN_MISSING("BEARER_TOKEN_MISSING"),
  
  TOKEN_EXPIRED("TOKEN_EXPIRED"),
  
  TOKEN_NOT_ACTIVE("TOKEN_NOT_ACTIVE"),
  
  TOKEN_REVOKED("TOKEN_REVOKED"),
  
  INVALID_CLAIMS("INVALID_CLAIMS"),
  
  MISSING_REQUIRED_CLAIMS("MISSING_REQUIRED_CLAIMS"),
  
  INVALID_SCOPE("INVALID_SCOPE"),
  
  INVALID_PERMISSIONS("INVALID_PERMISSIONS"),
  
  SESSION_EXPIRED("SESSION_EXPIRED"),
  
  SESSION_INVALID("SESSION_INVALID"),
  
  SESSION_REVOKED("SESSION_REVOKED"),
  
  UNAUTHENTICATED("UNAUTHENTICATED"),
  
  MULTI_FACTOR_REQUIRED("MULTI_FACTOR_REQUIRED"),
  
  MULTI_FACTOR_FAILED("MULTI_FACTOR_FAILED"),
  
  ACCOUNT_LOCKED("ACCOUNT_LOCKED"),
  
  ACCOUNT_DISABLED("ACCOUNT_DISABLED"),
  
  PASSWORD_EXPIRED("PASSWORD_EXPIRED"),
  
  PASSWORD_RESET_REQUIRED("PASSWORD_RESET_REQUIRED"),
  
  UNRECOGNIZED_DEVICE("UNRECOGNIZED_DEVICE");

  private String value;

  AuthErrorCode(String value) {
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
  public static AuthErrorCode fromValue(String value) {
    for (AuthErrorCode b : AuthErrorCode.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

