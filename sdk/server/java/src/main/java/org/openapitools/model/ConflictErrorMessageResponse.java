package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import org.openapitools.model.ConflictInfo;
import org.openapitools.model.ErrorResponse;
import org.openapitools.model.ValidationErrorCode;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * Represents resource conflict errors
 */

@Schema(name = "ConflictErrorMessageResponse", description = "Represents resource conflict errors")
@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-08T09:28:11.323460-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ConflictErrorMessageResponse {

  private ValidationErrorCode code = ValidationErrorCode.NO_ERROR;

  private String message;

  private ConflictInfo conflictInfo;

  private ErrorResponse errorResponse;

  public ConflictErrorMessageResponse code(ValidationErrorCode code) {
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
  public ValidationErrorCode getCode() {
    return code;
  }

  public void setCode(ValidationErrorCode code) {
    this.code = code;
  }

  public ConflictErrorMessageResponse message(String message) {
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

  public ConflictErrorMessageResponse conflictInfo(ConflictInfo conflictInfo) {
    this.conflictInfo = conflictInfo;
    return this;
  }

  /**
   * Get conflictInfo
   * @return conflictInfo
   */
  @Valid 
  @Schema(name = "conflictInfo", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("conflictInfo")
  public ConflictInfo getConflictInfo() {
    return conflictInfo;
  }

  public void setConflictInfo(ConflictInfo conflictInfo) {
    this.conflictInfo = conflictInfo;
  }

  public ConflictErrorMessageResponse errorResponse(ErrorResponse errorResponse) {
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
    ConflictErrorMessageResponse conflictErrorMessageResponse = (ConflictErrorMessageResponse) o;
    return Objects.equals(this.code, conflictErrorMessageResponse.code) &&
        Objects.equals(this.message, conflictErrorMessageResponse.message) &&
        Objects.equals(this.conflictInfo, conflictErrorMessageResponse.conflictInfo) &&
        Objects.equals(this.errorResponse, conflictErrorMessageResponse.errorResponse);
  }

  @Override
  public int hashCode() {
    return Objects.hash(code, message, conflictInfo, errorResponse);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ConflictErrorMessageResponse {\n");
    sb.append("    code: ").append(toIndentedString(code)).append("\n");
    sb.append("    message: ").append(toIndentedString(message)).append("\n");
    sb.append("    conflictInfo: ").append(toIndentedString(conflictInfo)).append("\n");
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

