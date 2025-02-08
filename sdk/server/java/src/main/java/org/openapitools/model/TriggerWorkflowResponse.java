package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import org.openapitools.model.BackgroundJobStatus;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * TriggerWorkflowResponse
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-08T10:26:51.224811-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class TriggerWorkflowResponse {

  private String jobId;

  private BackgroundJobStatus status = BackgroundJobStatus.UNSPECIFIED;

  public TriggerWorkflowResponse jobId(String jobId) {
    this.jobId = jobId;
    return this;
  }

  /**
   * Get jobId
   * @return jobId
   */
  
  @Schema(name = "jobId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("jobId")
  public String getJobId() {
    return jobId;
  }

  public void setJobId(String jobId) {
    this.jobId = jobId;
  }

  public TriggerWorkflowResponse status(BackgroundJobStatus status) {
    this.status = status;
    return this;
  }

  /**
   * Get status
   * @return status
   */
  @Valid 
  @Schema(name = "status", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("status")
  public BackgroundJobStatus getStatus() {
    return status;
  }

  public void setStatus(BackgroundJobStatus status) {
    this.status = status;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    TriggerWorkflowResponse triggerWorkflowResponse = (TriggerWorkflowResponse) o;
    return Objects.equals(this.jobId, triggerWorkflowResponse.jobId) &&
        Objects.equals(this.status, triggerWorkflowResponse.status);
  }

  @Override
  public int hashCode() {
    return Objects.hash(jobId, status);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class TriggerWorkflowResponse {\n");
    sb.append("    jobId: ").append(toIndentedString(jobId)).append("\n");
    sb.append("    status: ").append(toIndentedString(status)).append("\n");
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

