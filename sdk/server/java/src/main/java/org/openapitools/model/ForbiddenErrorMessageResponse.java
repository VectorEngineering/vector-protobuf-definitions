package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.ErrorResponse;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * Indicates that the server understood the request but refuses to authorize it
 */

@Schema(name = "ForbiddenErrorMessageResponse", description = "Indicates that the server understood the request but refuses to authorize it")
@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T17:07:07.052157-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ForbiddenErrorMessageResponse {

  private Integer code;

  private String message;

  private String reason;

  @Valid
  private List<String> requiredPermissions = new ArrayList<>();

  private ErrorResponse errorResponse;

  public ForbiddenErrorMessageResponse code(Integer code) {
    this.code = code;
    return this;
  }

  /**
   * Get code
   * @return code
   */
  
  @Schema(name = "code", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("code")
  public Integer getCode() {
    return code;
  }

  public void setCode(Integer code) {
    this.code = code;
  }

  public ForbiddenErrorMessageResponse message(String message) {
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

  public ForbiddenErrorMessageResponse reason(String reason) {
    this.reason = reason;
    return this;
  }

  /**
   * Get reason
   * @return reason
   */
  
  @Schema(name = "reason", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("reason")
  public String getReason() {
    return reason;
  }

  public void setReason(String reason) {
    this.reason = reason;
  }

  public ForbiddenErrorMessageResponse requiredPermissions(List<String> requiredPermissions) {
    this.requiredPermissions = requiredPermissions;
    return this;
  }

  public ForbiddenErrorMessageResponse addRequiredPermissionsItem(String requiredPermissionsItem) {
    if (this.requiredPermissions == null) {
      this.requiredPermissions = new ArrayList<>();
    }
    this.requiredPermissions.add(requiredPermissionsItem);
    return this;
  }

  /**
   * Get requiredPermissions
   * @return requiredPermissions
   */
  
  @Schema(name = "requiredPermissions", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("requiredPermissions")
  public List<String> getRequiredPermissions() {
    return requiredPermissions;
  }

  public void setRequiredPermissions(List<String> requiredPermissions) {
    this.requiredPermissions = requiredPermissions;
  }

  public ForbiddenErrorMessageResponse errorResponse(ErrorResponse errorResponse) {
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
    ForbiddenErrorMessageResponse forbiddenErrorMessageResponse = (ForbiddenErrorMessageResponse) o;
    return Objects.equals(this.code, forbiddenErrorMessageResponse.code) &&
        Objects.equals(this.message, forbiddenErrorMessageResponse.message) &&
        Objects.equals(this.reason, forbiddenErrorMessageResponse.reason) &&
        Objects.equals(this.requiredPermissions, forbiddenErrorMessageResponse.requiredPermissions) &&
        Objects.equals(this.errorResponse, forbiddenErrorMessageResponse.errorResponse);
  }

  @Override
  public int hashCode() {
    return Objects.hash(code, message, reason, requiredPermissions, errorResponse);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ForbiddenErrorMessageResponse {\n");
    sb.append("    code: ").append(toIndentedString(code)).append("\n");
    sb.append("    message: ").append(toIndentedString(message)).append("\n");
    sb.append("    reason: ").append(toIndentedString(reason)).append("\n");
    sb.append("    requiredPermissions: ").append(toIndentedString(requiredPermissions)).append("\n");
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

