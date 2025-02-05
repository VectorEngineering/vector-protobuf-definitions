package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.ScrapingJob;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * ListScrapingJobsResponse
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-05T07:44:51.137714-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ListScrapingJobsResponse {

  @Valid
  private List<@Valid ScrapingJob> jobs = new ArrayList<>();

  public ListScrapingJobsResponse jobs(List<@Valid ScrapingJob> jobs) {
    this.jobs = jobs;
    return this;
  }

  public ListScrapingJobsResponse addJobsItem(ScrapingJob jobsItem) {
    if (this.jobs == null) {
      this.jobs = new ArrayList<>();
    }
    this.jobs.add(jobsItem);
    return this;
  }

  /**
   * Get jobs
   * @return jobs
   */
  @Valid 
  @Schema(name = "jobs", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("jobs")
  public List<@Valid ScrapingJob> getJobs() {
    return jobs;
  }

  public void setJobs(List<@Valid ScrapingJob> jobs) {
    this.jobs = jobs;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    ListScrapingJobsResponse listScrapingJobsResponse = (ListScrapingJobsResponse) o;
    return Objects.equals(this.jobs, listScrapingJobsResponse.jobs);
  }

  @Override
  public int hashCode() {
    return Objects.hash(jobs);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ListScrapingJobsResponse {\n");
    sb.append("    jobs: ").append(toIndentedString(jobs)).append("\n");
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

