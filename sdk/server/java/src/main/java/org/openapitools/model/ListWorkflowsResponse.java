package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.ScrapingWorkflow;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * ListWorkflowsResponse
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T12:22:33.767757-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ListWorkflowsResponse {

  @Valid
  private List<@Valid ScrapingWorkflow> workflows = new ArrayList<>();

  private Integer nextPageNumber;

  public ListWorkflowsResponse workflows(List<@Valid ScrapingWorkflow> workflows) {
    this.workflows = workflows;
    return this;
  }

  public ListWorkflowsResponse addWorkflowsItem(ScrapingWorkflow workflowsItem) {
    if (this.workflows == null) {
      this.workflows = new ArrayList<>();
    }
    this.workflows.add(workflowsItem);
    return this;
  }

  /**
   * Get workflows
   * @return workflows
   */
  @Valid 
  @Schema(name = "workflows", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("workflows")
  public List<@Valid ScrapingWorkflow> getWorkflows() {
    return workflows;
  }

  public void setWorkflows(List<@Valid ScrapingWorkflow> workflows) {
    this.workflows = workflows;
  }

  public ListWorkflowsResponse nextPageNumber(Integer nextPageNumber) {
    this.nextPageNumber = nextPageNumber;
    return this;
  }

  /**
   * Get nextPageNumber
   * @return nextPageNumber
   */
  
  @Schema(name = "nextPageNumber", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("nextPageNumber")
  public Integer getNextPageNumber() {
    return nextPageNumber;
  }

  public void setNextPageNumber(Integer nextPageNumber) {
    this.nextPageNumber = nextPageNumber;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    ListWorkflowsResponse listWorkflowsResponse = (ListWorkflowsResponse) o;
    return Objects.equals(this.workflows, listWorkflowsResponse.workflows) &&
        Objects.equals(this.nextPageNumber, listWorkflowsResponse.nextPageNumber);
  }

  @Override
  public int hashCode() {
    return Objects.hash(workflows, nextPageNumber);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ListWorkflowsResponse {\n");
    sb.append("    workflows: ").append(toIndentedString(workflows)).append("\n");
    sb.append("    nextPageNumber: ").append(toIndentedString(nextPageNumber)).append("\n");
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

