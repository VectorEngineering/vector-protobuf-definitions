package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * OperationDetails
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T07:41:41.360877-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class OperationDetails {

  private String operationId;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime startTime;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime endTime;

  @Valid
  private List<String> stackTrace = new ArrayList<>();

  @Valid
  private Map<String, String> parameters = new HashMap<>();

  public OperationDetails operationId(String operationId) {
    this.operationId = operationId;
    return this;
  }

  /**
   * Get operationId
   * @return operationId
   */
  
  @Schema(name = "operationId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("operationId")
  public String getOperationId() {
    return operationId;
  }

  public void setOperationId(String operationId) {
    this.operationId = operationId;
  }

  public OperationDetails startTime(OffsetDateTime startTime) {
    this.startTime = startTime;
    return this;
  }

  /**
   * Get startTime
   * @return startTime
   */
  @Valid 
  @Schema(name = "startTime", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("startTime")
  public OffsetDateTime getStartTime() {
    return startTime;
  }

  public void setStartTime(OffsetDateTime startTime) {
    this.startTime = startTime;
  }

  public OperationDetails endTime(OffsetDateTime endTime) {
    this.endTime = endTime;
    return this;
  }

  /**
   * Get endTime
   * @return endTime
   */
  @Valid 
  @Schema(name = "endTime", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("endTime")
  public OffsetDateTime getEndTime() {
    return endTime;
  }

  public void setEndTime(OffsetDateTime endTime) {
    this.endTime = endTime;
  }

  public OperationDetails stackTrace(List<String> stackTrace) {
    this.stackTrace = stackTrace;
    return this;
  }

  public OperationDetails addStackTraceItem(String stackTraceItem) {
    if (this.stackTrace == null) {
      this.stackTrace = new ArrayList<>();
    }
    this.stackTrace.add(stackTraceItem);
    return this;
  }

  /**
   * Get stackTrace
   * @return stackTrace
   */
  
  @Schema(name = "stackTrace", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("stackTrace")
  public List<String> getStackTrace() {
    return stackTrace;
  }

  public void setStackTrace(List<String> stackTrace) {
    this.stackTrace = stackTrace;
  }

  public OperationDetails parameters(Map<String, String> parameters) {
    this.parameters = parameters;
    return this;
  }

  public OperationDetails putParametersItem(String key, String parametersItem) {
    if (this.parameters == null) {
      this.parameters = new HashMap<>();
    }
    this.parameters.put(key, parametersItem);
    return this;
  }

  /**
   * Get parameters
   * @return parameters
   */
  
  @Schema(name = "parameters", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("parameters")
  public Map<String, String> getParameters() {
    return parameters;
  }

  public void setParameters(Map<String, String> parameters) {
    this.parameters = parameters;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    OperationDetails operationDetails = (OperationDetails) o;
    return Objects.equals(this.operationId, operationDetails.operationId) &&
        Objects.equals(this.startTime, operationDetails.startTime) &&
        Objects.equals(this.endTime, operationDetails.endTime) &&
        Objects.equals(this.stackTrace, operationDetails.stackTrace) &&
        Objects.equals(this.parameters, operationDetails.parameters);
  }

  @Override
  public int hashCode() {
    return Objects.hash(operationId, startTime, endTime, stackTrace, parameters);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class OperationDetails {\n");
    sb.append("    operationId: ").append(toIndentedString(operationId)).append("\n");
    sb.append("    startTime: ").append(toIndentedString(startTime)).append("\n");
    sb.append("    endTime: ").append(toIndentedString(endTime)).append("\n");
    sb.append("    stackTrace: ").append(toIndentedString(stackTrace)).append("\n");
    sb.append("    parameters: ").append(toIndentedString(parameters)).append("\n");
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

