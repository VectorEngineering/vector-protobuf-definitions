package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import org.openapitools.model.Workspace1;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * GetWorkspaceResponse1
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-09T12:56:25.550372-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class GetWorkspaceResponse1 {

  private Workspace1 workspace;

  public GetWorkspaceResponse1 workspace(Workspace1 workspace) {
    this.workspace = workspace;
    return this;
  }

  /**
   * Get workspace
   * @return workspace
   */
  @Valid 
  @Schema(name = "workspace", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("workspace")
  public Workspace1 getWorkspace() {
    return workspace;
  }

  public void setWorkspace(Workspace1 workspace) {
    this.workspace = workspace;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    GetWorkspaceResponse1 getWorkspaceResponse1 = (GetWorkspaceResponse1) o;
    return Objects.equals(this.workspace, getWorkspaceResponse1.workspace);
  }

  @Override
  public int hashCode() {
    return Objects.hash(workspace);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class GetWorkspaceResponse1 {\n");
    sb.append("    workspace: ").append(toIndentedString(workspace)).append("\n");
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

