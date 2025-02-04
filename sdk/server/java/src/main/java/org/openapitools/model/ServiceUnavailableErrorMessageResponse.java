package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import org.openapitools.model.AvailabilityInfo;
import org.openapitools.model.ErrorResponse;
import org.openapitools.model.InternalErrorCode;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * Represents service availability errors
 */

@Schema(name = "ServiceUnavailableErrorMessageResponse", description = "Represents service availability errors")
@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-03T20:00:13.934981-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ServiceUnavailableErrorMessageResponse {

  private InternalErrorCode code = InternalErrorCode.NO_INTERNAL_ERROR;

  private String message;

  private AvailabilityInfo availabilityInfo;

  private ErrorResponse errorResponse;

  public ServiceUnavailableErrorMessageResponse code(InternalErrorCode code) {
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
  public InternalErrorCode getCode() {
    return code;
  }

  public void setCode(InternalErrorCode code) {
    this.code = code;
  }

  public ServiceUnavailableErrorMessageResponse message(String message) {
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

  public ServiceUnavailableErrorMessageResponse availabilityInfo(AvailabilityInfo availabilityInfo) {
    this.availabilityInfo = availabilityInfo;
    return this;
  }

  /**
   * Get availabilityInfo
   * @return availabilityInfo
   */
  @Valid 
  @Schema(name = "availabilityInfo", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("availabilityInfo")
  public AvailabilityInfo getAvailabilityInfo() {
    return availabilityInfo;
  }

  public void setAvailabilityInfo(AvailabilityInfo availabilityInfo) {
    this.availabilityInfo = availabilityInfo;
  }

  public ServiceUnavailableErrorMessageResponse errorResponse(ErrorResponse errorResponse) {
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
    ServiceUnavailableErrorMessageResponse serviceUnavailableErrorMessageResponse = (ServiceUnavailableErrorMessageResponse) o;
    return Objects.equals(this.code, serviceUnavailableErrorMessageResponse.code) &&
        Objects.equals(this.message, serviceUnavailableErrorMessageResponse.message) &&
        Objects.equals(this.availabilityInfo, serviceUnavailableErrorMessageResponse.availabilityInfo) &&
        Objects.equals(this.errorResponse, serviceUnavailableErrorMessageResponse.errorResponse);
  }

  @Override
  public int hashCode() {
    return Objects.hash(code, message, availabilityInfo, errorResponse);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ServiceUnavailableErrorMessageResponse {\n");
    sb.append("    code: ").append(toIndentedString(code)).append("\n");
    sb.append("    message: ").append(toIndentedString(message)).append("\n");
    sb.append("    availabilityInfo: ").append(toIndentedString(availabilityInfo)).append("\n");
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

