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
 * Represents errors when HTTP method is not allowed on the endpoint
 */

@Schema(name = "MethodNotAllowedErrorMessageResponse", description = "Represents errors when HTTP method is not allowed on the endpoint")
@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T15:17:20.852245-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class MethodNotAllowedErrorMessageResponse {

  private Integer code;

  private String message;

  @Valid
  private List<String> allowedMethods = new ArrayList<>();

  private ErrorResponse errorResponse;

  public MethodNotAllowedErrorMessageResponse code(Integer code) {
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

  public MethodNotAllowedErrorMessageResponse message(String message) {
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

  public MethodNotAllowedErrorMessageResponse allowedMethods(List<String> allowedMethods) {
    this.allowedMethods = allowedMethods;
    return this;
  }

  public MethodNotAllowedErrorMessageResponse addAllowedMethodsItem(String allowedMethodsItem) {
    if (this.allowedMethods == null) {
      this.allowedMethods = new ArrayList<>();
    }
    this.allowedMethods.add(allowedMethodsItem);
    return this;
  }

  /**
   * Get allowedMethods
   * @return allowedMethods
   */
  
  @Schema(name = "allowedMethods", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("allowedMethods")
  public List<String> getAllowedMethods() {
    return allowedMethods;
  }

  public void setAllowedMethods(List<String> allowedMethods) {
    this.allowedMethods = allowedMethods;
  }

  public MethodNotAllowedErrorMessageResponse errorResponse(ErrorResponse errorResponse) {
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
    MethodNotAllowedErrorMessageResponse methodNotAllowedErrorMessageResponse = (MethodNotAllowedErrorMessageResponse) o;
    return Objects.equals(this.code, methodNotAllowedErrorMessageResponse.code) &&
        Objects.equals(this.message, methodNotAllowedErrorMessageResponse.message) &&
        Objects.equals(this.allowedMethods, methodNotAllowedErrorMessageResponse.allowedMethods) &&
        Objects.equals(this.errorResponse, methodNotAllowedErrorMessageResponse.errorResponse);
  }

  @Override
  public int hashCode() {
    return Objects.hash(code, message, allowedMethods, errorResponse);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class MethodNotAllowedErrorMessageResponse {\n");
    sb.append("    code: ").append(toIndentedString(code)).append("\n");
    sb.append("    message: ").append(toIndentedString(message)).append("\n");
    sb.append("    allowedMethods: ").append(toIndentedString(allowedMethods)).append("\n");
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

