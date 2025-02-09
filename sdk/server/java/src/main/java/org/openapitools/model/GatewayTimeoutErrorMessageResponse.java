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
 * Represents errors when the server did not receive a timely response from an upstream server
 */

@Schema(name = "GatewayTimeoutErrorMessageResponse", description = "Represents errors when the server did not receive a timely response from an upstream server")
@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-09T17:57:27.369576-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class GatewayTimeoutErrorMessageResponse {

  private Integer code;

  private String message;

  private String upstreamService;

  private ErrorResponse errorResponse;

  public GatewayTimeoutErrorMessageResponse code(Integer code) {
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

  public GatewayTimeoutErrorMessageResponse message(String message) {
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

  public GatewayTimeoutErrorMessageResponse upstreamService(String upstreamService) {
    this.upstreamService = upstreamService;
    return this;
  }

  /**
   * Get upstreamService
   * @return upstreamService
   */
  
  @Schema(name = "upstreamService", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("upstreamService")
  public String getUpstreamService() {
    return upstreamService;
  }

  public void setUpstreamService(String upstreamService) {
    this.upstreamService = upstreamService;
  }

  public GatewayTimeoutErrorMessageResponse errorResponse(ErrorResponse errorResponse) {
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
    GatewayTimeoutErrorMessageResponse gatewayTimeoutErrorMessageResponse = (GatewayTimeoutErrorMessageResponse) o;
    return Objects.equals(this.code, gatewayTimeoutErrorMessageResponse.code) &&
        Objects.equals(this.message, gatewayTimeoutErrorMessageResponse.message) &&
        Objects.equals(this.upstreamService, gatewayTimeoutErrorMessageResponse.upstreamService) &&
        Objects.equals(this.errorResponse, gatewayTimeoutErrorMessageResponse.errorResponse);
  }

  @Override
  public int hashCode() {
    return Objects.hash(code, message, upstreamService, errorResponse);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class GatewayTimeoutErrorMessageResponse {\n");
    sb.append("    code: ").append(toIndentedString(code)).append("\n");
    sb.append("    message: ").append(toIndentedString(message)).append("\n");
    sb.append("    upstreamService: ").append(toIndentedString(upstreamService)).append("\n");
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

