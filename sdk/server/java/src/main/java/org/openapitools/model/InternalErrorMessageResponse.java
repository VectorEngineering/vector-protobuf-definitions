package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import org.openapitools.model.ErrorResponse;
import org.openapitools.model.InternalErrorCode;
import org.openapitools.model.OperationDetails;
import org.openapitools.model.ResourceUtilization;
import org.openapitools.model.ServiceStatus;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * Represents internal server and system-level errors
 */

@Schema(name = "InternalErrorMessageResponse", description = "Represents internal server and system-level errors")
@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-09T22:48:58.302614-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class InternalErrorMessageResponse {

  private InternalErrorCode code = InternalErrorCode.NO_INTERNAL_ERROR;

  private String message;

  private String incidentId;

  private ServiceStatus serviceStatus;

  private ResourceUtilization resourceUtilization;

  private OperationDetails operationDetails;

  private ErrorResponse errorResponse;

  public InternalErrorMessageResponse code(InternalErrorCode code) {
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

  public InternalErrorMessageResponse message(String message) {
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

  public InternalErrorMessageResponse incidentId(String incidentId) {
    this.incidentId = incidentId;
    return this;
  }

  /**
   * Get incidentId
   * @return incidentId
   */
  
  @Schema(name = "incidentId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("incidentId")
  public String getIncidentId() {
    return incidentId;
  }

  public void setIncidentId(String incidentId) {
    this.incidentId = incidentId;
  }

  public InternalErrorMessageResponse serviceStatus(ServiceStatus serviceStatus) {
    this.serviceStatus = serviceStatus;
    return this;
  }

  /**
   * Get serviceStatus
   * @return serviceStatus
   */
  @Valid 
  @Schema(name = "serviceStatus", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("serviceStatus")
  public ServiceStatus getServiceStatus() {
    return serviceStatus;
  }

  public void setServiceStatus(ServiceStatus serviceStatus) {
    this.serviceStatus = serviceStatus;
  }

  public InternalErrorMessageResponse resourceUtilization(ResourceUtilization resourceUtilization) {
    this.resourceUtilization = resourceUtilization;
    return this;
  }

  /**
   * Get resourceUtilization
   * @return resourceUtilization
   */
  @Valid 
  @Schema(name = "resourceUtilization", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("resourceUtilization")
  public ResourceUtilization getResourceUtilization() {
    return resourceUtilization;
  }

  public void setResourceUtilization(ResourceUtilization resourceUtilization) {
    this.resourceUtilization = resourceUtilization;
  }

  public InternalErrorMessageResponse operationDetails(OperationDetails operationDetails) {
    this.operationDetails = operationDetails;
    return this;
  }

  /**
   * Get operationDetails
   * @return operationDetails
   */
  @Valid 
  @Schema(name = "operationDetails", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("operationDetails")
  public OperationDetails getOperationDetails() {
    return operationDetails;
  }

  public void setOperationDetails(OperationDetails operationDetails) {
    this.operationDetails = operationDetails;
  }

  public InternalErrorMessageResponse errorResponse(ErrorResponse errorResponse) {
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
    InternalErrorMessageResponse internalErrorMessageResponse = (InternalErrorMessageResponse) o;
    return Objects.equals(this.code, internalErrorMessageResponse.code) &&
        Objects.equals(this.message, internalErrorMessageResponse.message) &&
        Objects.equals(this.incidentId, internalErrorMessageResponse.incidentId) &&
        Objects.equals(this.serviceStatus, internalErrorMessageResponse.serviceStatus) &&
        Objects.equals(this.resourceUtilization, internalErrorMessageResponse.resourceUtilization) &&
        Objects.equals(this.operationDetails, internalErrorMessageResponse.operationDetails) &&
        Objects.equals(this.errorResponse, internalErrorMessageResponse.errorResponse);
  }

  @Override
  public int hashCode() {
    return Objects.hash(code, message, incidentId, serviceStatus, resourceUtilization, operationDetails, errorResponse);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class InternalErrorMessageResponse {\n");
    sb.append("    code: ").append(toIndentedString(code)).append("\n");
    sb.append("    message: ").append(toIndentedString(message)).append("\n");
    sb.append("    incidentId: ").append(toIndentedString(incidentId)).append("\n");
    sb.append("    serviceStatus: ").append(toIndentedString(serviceStatus)).append("\n");
    sb.append("    resourceUtilization: ").append(toIndentedString(resourceUtilization)).append("\n");
    sb.append("    operationDetails: ").append(toIndentedString(operationDetails)).append("\n");
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

