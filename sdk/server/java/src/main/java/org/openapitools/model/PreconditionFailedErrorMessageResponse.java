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
 * Represents errors when a precondition provided in the request fails
 */

@Schema(name = "PreconditionFailedErrorMessageResponse", description = "Represents errors when a precondition provided in the request fails")
@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-05T06:54:24.429429-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class PreconditionFailedErrorMessageResponse {

  private Integer code;

  private String message;

  @Valid
  private List<String> failedConditions = new ArrayList<>();

  private ErrorResponse errorResponse;

  public PreconditionFailedErrorMessageResponse code(Integer code) {
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

  public PreconditionFailedErrorMessageResponse message(String message) {
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

  public PreconditionFailedErrorMessageResponse failedConditions(List<String> failedConditions) {
    this.failedConditions = failedConditions;
    return this;
  }

  public PreconditionFailedErrorMessageResponse addFailedConditionsItem(String failedConditionsItem) {
    if (this.failedConditions == null) {
      this.failedConditions = new ArrayList<>();
    }
    this.failedConditions.add(failedConditionsItem);
    return this;
  }

  /**
   * Get failedConditions
   * @return failedConditions
   */
  
  @Schema(name = "failedConditions", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("failedConditions")
  public List<String> getFailedConditions() {
    return failedConditions;
  }

  public void setFailedConditions(List<String> failedConditions) {
    this.failedConditions = failedConditions;
  }

  public PreconditionFailedErrorMessageResponse errorResponse(ErrorResponse errorResponse) {
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
    PreconditionFailedErrorMessageResponse preconditionFailedErrorMessageResponse = (PreconditionFailedErrorMessageResponse) o;
    return Objects.equals(this.code, preconditionFailedErrorMessageResponse.code) &&
        Objects.equals(this.message, preconditionFailedErrorMessageResponse.message) &&
        Objects.equals(this.failedConditions, preconditionFailedErrorMessageResponse.failedConditions) &&
        Objects.equals(this.errorResponse, preconditionFailedErrorMessageResponse.errorResponse);
  }

  @Override
  public int hashCode() {
    return Objects.hash(code, message, failedConditions, errorResponse);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class PreconditionFailedErrorMessageResponse {\n");
    sb.append("    code: ").append(toIndentedString(code)).append("\n");
    sb.append("    message: ").append(toIndentedString(message)).append("\n");
    sb.append("    failedConditions: ").append(toIndentedString(failedConditions)).append("\n");
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

