package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import org.openapitools.model.ErrorResponse;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * Indicates that the resource requested is no longer available and will not be available again
 */

@Schema(name = "GoneErrorMessageResponse", description = "Indicates that the resource requested is no longer available and will not be available again")
@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-03T06:30:06.935816-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class GoneErrorMessageResponse {

  private Integer code;

  private String message;

  private String resource;

  private ErrorResponse errorResponse;

  public GoneErrorMessageResponse code(Integer code) {
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

  public GoneErrorMessageResponse message(String message) {
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

  public GoneErrorMessageResponse resource(String resource) {
    this.resource = resource;
    return this;
  }

  /**
   * Get resource
   * @return resource
   */
  
  @Schema(name = "resource", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("resource")
  public String getResource() {
    return resource;
  }

  public void setResource(String resource) {
    this.resource = resource;
  }

  public GoneErrorMessageResponse errorResponse(ErrorResponse errorResponse) {
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
    GoneErrorMessageResponse goneErrorMessageResponse = (GoneErrorMessageResponse) o;
    return Objects.equals(this.code, goneErrorMessageResponse.code) &&
        Objects.equals(this.message, goneErrorMessageResponse.message) &&
        Objects.equals(this.resource, goneErrorMessageResponse.resource) &&
        Objects.equals(this.errorResponse, goneErrorMessageResponse.errorResponse);
  }

  @Override
  public int hashCode() {
    return Objects.hash(code, message, resource, errorResponse);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class GoneErrorMessageResponse {\n");
    sb.append("    code: ").append(toIndentedString(code)).append("\n");
    sb.append("    message: ").append(toIndentedString(message)).append("\n");
    sb.append("    resource: ").append(toIndentedString(resource)).append("\n");
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

