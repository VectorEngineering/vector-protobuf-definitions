package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.JobSuccessRate;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * GetWorkspaceAnalyticsResponse
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-05T07:11:09.755832-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class GetWorkspaceAnalyticsResponse {

  private Integer totalLeads;

  private Integer activeWorkflows;

  private Integer jobsLast30Days;

  @Valid
  private List<@Valid JobSuccessRate> successRates = new ArrayList<>();

  public GetWorkspaceAnalyticsResponse totalLeads(Integer totalLeads) {
    this.totalLeads = totalLeads;
    return this;
  }

  /**
   * Get totalLeads
   * @return totalLeads
   */
  
  @Schema(name = "totalLeads", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("totalLeads")
  public Integer getTotalLeads() {
    return totalLeads;
  }

  public void setTotalLeads(Integer totalLeads) {
    this.totalLeads = totalLeads;
  }

  public GetWorkspaceAnalyticsResponse activeWorkflows(Integer activeWorkflows) {
    this.activeWorkflows = activeWorkflows;
    return this;
  }

  /**
   * Get activeWorkflows
   * @return activeWorkflows
   */
  
  @Schema(name = "activeWorkflows", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("activeWorkflows")
  public Integer getActiveWorkflows() {
    return activeWorkflows;
  }

  public void setActiveWorkflows(Integer activeWorkflows) {
    this.activeWorkflows = activeWorkflows;
  }

  public GetWorkspaceAnalyticsResponse jobsLast30Days(Integer jobsLast30Days) {
    this.jobsLast30Days = jobsLast30Days;
    return this;
  }

  /**
   * Get jobsLast30Days
   * @return jobsLast30Days
   */
  
  @Schema(name = "jobsLast30Days", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("jobsLast30Days")
  public Integer getJobsLast30Days() {
    return jobsLast30Days;
  }

  public void setJobsLast30Days(Integer jobsLast30Days) {
    this.jobsLast30Days = jobsLast30Days;
  }

  public GetWorkspaceAnalyticsResponse successRates(List<@Valid JobSuccessRate> successRates) {
    this.successRates = successRates;
    return this;
  }

  public GetWorkspaceAnalyticsResponse addSuccessRatesItem(JobSuccessRate successRatesItem) {
    if (this.successRates == null) {
      this.successRates = new ArrayList<>();
    }
    this.successRates.add(successRatesItem);
    return this;
  }

  /**
   * Get successRates
   * @return successRates
   */
  @Valid 
  @Schema(name = "successRates", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("successRates")
  public List<@Valid JobSuccessRate> getSuccessRates() {
    return successRates;
  }

  public void setSuccessRates(List<@Valid JobSuccessRate> successRates) {
    this.successRates = successRates;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    GetWorkspaceAnalyticsResponse getWorkspaceAnalyticsResponse = (GetWorkspaceAnalyticsResponse) o;
    return Objects.equals(this.totalLeads, getWorkspaceAnalyticsResponse.totalLeads) &&
        Objects.equals(this.activeWorkflows, getWorkspaceAnalyticsResponse.activeWorkflows) &&
        Objects.equals(this.jobsLast30Days, getWorkspaceAnalyticsResponse.jobsLast30Days) &&
        Objects.equals(this.successRates, getWorkspaceAnalyticsResponse.successRates);
  }

  @Override
  public int hashCode() {
    return Objects.hash(totalLeads, activeWorkflows, jobsLast30Days, successRates);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class GetWorkspaceAnalyticsResponse {\n");
    sb.append("    totalLeads: ").append(toIndentedString(totalLeads)).append("\n");
    sb.append("    activeWorkflows: ").append(toIndentedString(activeWorkflows)).append("\n");
    sb.append("    jobsLast30Days: ").append(toIndentedString(jobsLast30Days)).append("\n");
    sb.append("    successRates: ").append(toIndentedString(successRates)).append("\n");
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

