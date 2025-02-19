package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * GetAccountUsageResponse
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-19T11:32:59.900685-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class GetAccountUsageResponse {

  private Integer totalJobsRun;

  private Integer monthlyJobLimit;

  private Integer remainingJobs;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime resetTime;

  public GetAccountUsageResponse totalJobsRun(Integer totalJobsRun) {
    this.totalJobsRun = totalJobsRun;
    return this;
  }

  /**
   * Get totalJobsRun
   * @return totalJobsRun
   */
  
  @Schema(name = "totalJobsRun", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("totalJobsRun")
  public Integer getTotalJobsRun() {
    return totalJobsRun;
  }

  public void setTotalJobsRun(Integer totalJobsRun) {
    this.totalJobsRun = totalJobsRun;
  }

  public GetAccountUsageResponse monthlyJobLimit(Integer monthlyJobLimit) {
    this.monthlyJobLimit = monthlyJobLimit;
    return this;
  }

  /**
   * Get monthlyJobLimit
   * @return monthlyJobLimit
   */
  
  @Schema(name = "monthlyJobLimit", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("monthlyJobLimit")
  public Integer getMonthlyJobLimit() {
    return monthlyJobLimit;
  }

  public void setMonthlyJobLimit(Integer monthlyJobLimit) {
    this.monthlyJobLimit = monthlyJobLimit;
  }

  public GetAccountUsageResponse remainingJobs(Integer remainingJobs) {
    this.remainingJobs = remainingJobs;
    return this;
  }

  /**
   * Get remainingJobs
   * @return remainingJobs
   */
  
  @Schema(name = "remainingJobs", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("remainingJobs")
  public Integer getRemainingJobs() {
    return remainingJobs;
  }

  public void setRemainingJobs(Integer remainingJobs) {
    this.remainingJobs = remainingJobs;
  }

  public GetAccountUsageResponse resetTime(OffsetDateTime resetTime) {
    this.resetTime = resetTime;
    return this;
  }

  /**
   * Get resetTime
   * @return resetTime
   */
  @Valid 
  @Schema(name = "resetTime", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("resetTime")
  public OffsetDateTime getResetTime() {
    return resetTime;
  }

  public void setResetTime(OffsetDateTime resetTime) {
    this.resetTime = resetTime;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    GetAccountUsageResponse getAccountUsageResponse = (GetAccountUsageResponse) o;
    return Objects.equals(this.totalJobsRun, getAccountUsageResponse.totalJobsRun) &&
        Objects.equals(this.monthlyJobLimit, getAccountUsageResponse.monthlyJobLimit) &&
        Objects.equals(this.remainingJobs, getAccountUsageResponse.remainingJobs) &&
        Objects.equals(this.resetTime, getAccountUsageResponse.resetTime);
  }

  @Override
  public int hashCode() {
    return Objects.hash(totalJobsRun, monthlyJobLimit, remainingJobs, resetTime);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class GetAccountUsageResponse {\n");
    sb.append("    totalJobsRun: ").append(toIndentedString(totalJobsRun)).append("\n");
    sb.append("    monthlyJobLimit: ").append(toIndentedString(monthlyJobLimit)).append("\n");
    sb.append("    remainingJobs: ").append(toIndentedString(remainingJobs)).append("\n");
    sb.append("    resetTime: ").append(toIndentedString(resetTime)).append("\n");
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

