package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.ErrorResponse;
import org.openapitools.model.FieldError;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * Represents errors when the server understands the content type but cannot process the instructions
 */

@Schema(name = "UnprocessableEntityErrorMessageResponse", description = "Represents errors when the server understands the content type but cannot process the instructions")
@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-05T06:54:24.429429-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class UnprocessableEntityErrorMessageResponse {

  private Integer code;

  private String message;

  @Valid
  private List<@Valid FieldError> errors = new ArrayList<>();

  private ErrorResponse errorResponse;

  public UnprocessableEntityErrorMessageResponse code(Integer code) {
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

  public UnprocessableEntityErrorMessageResponse message(String message) {
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

  public UnprocessableEntityErrorMessageResponse errors(List<@Valid FieldError> errors) {
    this.errors = errors;
    return this;
  }

  public UnprocessableEntityErrorMessageResponse addErrorsItem(FieldError errorsItem) {
    if (this.errors == null) {
      this.errors = new ArrayList<>();
    }
    this.errors.add(errorsItem);
    return this;
  }

  /**
   * Get errors
   * @return errors
   */
  @Valid 
  @Schema(name = "errors", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("errors")
  public List<@Valid FieldError> getErrors() {
    return errors;
  }

  public void setErrors(List<@Valid FieldError> errors) {
    this.errors = errors;
  }

  public UnprocessableEntityErrorMessageResponse errorResponse(ErrorResponse errorResponse) {
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
    UnprocessableEntityErrorMessageResponse unprocessableEntityErrorMessageResponse = (UnprocessableEntityErrorMessageResponse) o;
    return Objects.equals(this.code, unprocessableEntityErrorMessageResponse.code) &&
        Objects.equals(this.message, unprocessableEntityErrorMessageResponse.message) &&
        Objects.equals(this.errors, unprocessableEntityErrorMessageResponse.errors) &&
        Objects.equals(this.errorResponse, unprocessableEntityErrorMessageResponse.errorResponse);
  }

  @Override
  public int hashCode() {
    return Objects.hash(code, message, errors, errorResponse);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class UnprocessableEntityErrorMessageResponse {\n");
    sb.append("    code: ").append(toIndentedString(code)).append("\n");
    sb.append("    message: ").append(toIndentedString(message)).append("\n");
    sb.append("    errors: ").append(toIndentedString(errors)).append("\n");
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

