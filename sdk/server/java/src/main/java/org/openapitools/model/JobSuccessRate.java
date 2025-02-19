package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * JobSuccessRate
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-19T11:32:59.900685-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class JobSuccessRate {

  private String workflowId;

  private Float successRate;

  private Integer totalRuns;

  public JobSuccessRate workflowId(String workflowId) {
    this.workflowId = workflowId;
    return this;
  }

  /**
   * Get workflowId
   * @return workflowId
   */
  
  @Schema(name = "workflowId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("workflowId")
  public String getWorkflowId() {
    return workflowId;
  }

  public void setWorkflowId(String workflowId) {
    this.workflowId = workflowId;
  }

  public JobSuccessRate successRate(Float successRate) {
    this.successRate = successRate;
    return this;
  }

  /**
   * Get successRate
   * @return successRate
   */
  
  @Schema(name = "successRate", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("successRate")
  public Float getSuccessRate() {
    return successRate;
  }

  public void setSuccessRate(Float successRate) {
    this.successRate = successRate;
  }

  public JobSuccessRate totalRuns(Integer totalRuns) {
    this.totalRuns = totalRuns;
    return this;
  }

  /**
   * Get totalRuns
   * @return totalRuns
   */
  
  @Schema(name = "totalRuns", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("totalRuns")
  public Integer getTotalRuns() {
    return totalRuns;
  }

  public void setTotalRuns(Integer totalRuns) {
    this.totalRuns = totalRuns;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    JobSuccessRate jobSuccessRate = (JobSuccessRate) o;
    return Objects.equals(this.workflowId, jobSuccessRate.workflowId) &&
        Objects.equals(this.successRate, jobSuccessRate.successRate) &&
        Objects.equals(this.totalRuns, jobSuccessRate.totalRuns);
  }

  @Override
  public int hashCode() {
    return Objects.hash(workflowId, successRate, totalRuns);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class JobSuccessRate {\n");
    sb.append("    workflowId: ").append(toIndentedString(workflowId)).append("\n");
    sb.append("    successRate: ").append(toIndentedString(successRate)).append("\n");
    sb.append("    totalRuns: ").append(toIndentedString(totalRuns)).append("\n");
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

