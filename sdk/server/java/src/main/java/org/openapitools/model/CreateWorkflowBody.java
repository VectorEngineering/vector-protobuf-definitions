package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import org.openapitools.model.ScrapingWorkflow;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * CreateWorkflowBody
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-01-29T12:14:40.717638-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class CreateWorkflowBody {

  private ScrapingWorkflow workflow;

  public CreateWorkflowBody workflow(ScrapingWorkflow workflow) {
    this.workflow = workflow;
    return this;
  }

  /**
   * Get workflow
   * @return workflow
   */
  @Valid 
  @Schema(name = "workflow", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("workflow")
  public ScrapingWorkflow getWorkflow() {
    return workflow;
  }

  public void setWorkflow(ScrapingWorkflow workflow) {
    this.workflow = workflow;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    CreateWorkflowBody createWorkflowBody = (CreateWorkflowBody) o;
    return Objects.equals(this.workflow, createWorkflowBody.workflow);
  }

  @Override
  public int hashCode() {
    return Objects.hash(workflow);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class CreateWorkflowBody {\n");
    sb.append("    workflow: ").append(toIndentedString(workflow)).append("\n");
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

