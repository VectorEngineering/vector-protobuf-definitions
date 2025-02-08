package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import org.openapitools.model.AuthContext;
import org.openapitools.model.AuthErrorCode;
import org.openapitools.model.ErrorResponse;
import org.openapitools.model.MFAInfo;
import org.openapitools.model.SessionInfo;
import org.openapitools.model.TokenInfo;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * Represents authentication and authorization failures
 */

@Schema(name = "AuthenticationErrorMessageResponse", description = "Represents authentication and authorization failures")
@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-08T10:15:33.789499-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class AuthenticationErrorMessageResponse {

  private AuthErrorCode code = AuthErrorCode.NO_AUTH_ERROR;

  private String message;

  private TokenInfo tokenInfo;

  private AuthContext authContext;

  private SessionInfo sessionInfo;

  private MFAInfo mfaInfo;

  private ErrorResponse errorResponse;

  public AuthenticationErrorMessageResponse code(AuthErrorCode code) {
    this.code = code;
    return this;
  }

  /**
   * Get code
   * @return code
   */
  @Valid 
  @Schema(name = "code", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("code")
  public AuthErrorCode getCode() {
    return code;
  }

  public void setCode(AuthErrorCode code) {
    this.code = code;
  }

  public AuthenticationErrorMessageResponse message(String message) {
    this.message = message;
    return this;
  }

  /**
   * Get message
   * @return message
   */
  
  @Schema(name = "message", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("message")
  public String getMessage() {
    return message;
  }

  public void setMessage(String message) {
    this.message = message;
  }

  public AuthenticationErrorMessageResponse tokenInfo(TokenInfo tokenInfo) {
    this.tokenInfo = tokenInfo;
    return this;
  }

  /**
   * Get tokenInfo
   * @return tokenInfo
   */
  @Valid 
  @Schema(name = "tokenInfo", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("tokenInfo")
  public TokenInfo getTokenInfo() {
    return tokenInfo;
  }

  public void setTokenInfo(TokenInfo tokenInfo) {
    this.tokenInfo = tokenInfo;
  }

  public AuthenticationErrorMessageResponse authContext(AuthContext authContext) {
    this.authContext = authContext;
    return this;
  }

  /**
   * Get authContext
   * @return authContext
   */
  @Valid 
  @Schema(name = "authContext", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("authContext")
  public AuthContext getAuthContext() {
    return authContext;
  }

  public void setAuthContext(AuthContext authContext) {
    this.authContext = authContext;
  }

  public AuthenticationErrorMessageResponse sessionInfo(SessionInfo sessionInfo) {
    this.sessionInfo = sessionInfo;
    return this;
  }

  /**
   * Get sessionInfo
   * @return sessionInfo
   */
  @Valid 
  @Schema(name = "sessionInfo", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("sessionInfo")
  public SessionInfo getSessionInfo() {
    return sessionInfo;
  }

  public void setSessionInfo(SessionInfo sessionInfo) {
    this.sessionInfo = sessionInfo;
  }

  public AuthenticationErrorMessageResponse mfaInfo(MFAInfo mfaInfo) {
    this.mfaInfo = mfaInfo;
    return this;
  }

  /**
   * Get mfaInfo
   * @return mfaInfo
   */
  @Valid 
  @Schema(name = "mfaInfo", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("mfaInfo")
  public MFAInfo getMfaInfo() {
    return mfaInfo;
  }

  public void setMfaInfo(MFAInfo mfaInfo) {
    this.mfaInfo = mfaInfo;
  }

  public AuthenticationErrorMessageResponse errorResponse(ErrorResponse errorResponse) {
    this.errorResponse = errorResponse;
    return this;
  }

  /**
   * Get errorResponse
   * @return errorResponse
   */
  @Valid 
  @Schema(name = "errorResponse", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("errorResponse")
  public ErrorResponse getErrorResponse() {
    return errorResponse;
  }

  public void setErrorResponse(ErrorResponse errorResponse) {
    this.errorResponse = errorResponse;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    AuthenticationErrorMessageResponse authenticationErrorMessageResponse = (AuthenticationErrorMessageResponse) o;
    return Objects.equals(this.code, authenticationErrorMessageResponse.code) &&
        Objects.equals(this.message, authenticationErrorMessageResponse.message) &&
        Objects.equals(this.tokenInfo, authenticationErrorMessageResponse.tokenInfo) &&
        Objects.equals(this.authContext, authenticationErrorMessageResponse.authContext) &&
        Objects.equals(this.sessionInfo, authenticationErrorMessageResponse.sessionInfo) &&
        Objects.equals(this.mfaInfo, authenticationErrorMessageResponse.mfaInfo) &&
        Objects.equals(this.errorResponse, authenticationErrorMessageResponse.errorResponse);
  }

  @Override
  public int hashCode() {
    return Objects.hash(code, message, tokenInfo, authContext, sessionInfo, mfaInfo, errorResponse);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class AuthenticationErrorMessageResponse {\n");
    sb.append("    code: ").append(toIndentedString(code)).append("\n");
    sb.append("    message: ").append(toIndentedString(message)).append("\n");
    sb.append("    tokenInfo: ").append(toIndentedString(tokenInfo)).append("\n");
    sb.append("    authContext: ").append(toIndentedString(authContext)).append("\n");
    sb.append("    sessionInfo: ").append(toIndentedString(sessionInfo)).append("\n");
    sb.append("    mfaInfo: ").append(toIndentedString(mfaInfo)).append("\n");
    sb.append("    errorResponse: ").append(toIndentedString(errorResponse)).append("\n");
    sb.append("}");
    return sb.toString();
  }

  /**
   * Convert the given object to string with each line indented by 4 spaces
   * (except the first line).
   */
  private String toIndentedString(Object o) {
    if (o == null) {
      return "null";
    }
    return o.toString().replace("\n", "\n    ");
  }
}

