package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import org.openapitools.model.WorkspaceSharing;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * UpdateWorkspaceSharingRequest
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T12:22:33.767757-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class UpdateWorkspaceSharingRequest {

  private WorkspaceSharing sharing;

  public UpdateWorkspaceSharingRequest sharing(WorkspaceSharing sharing) {
    this.sharing = sharing;
    return this;
  }

  /**
   * Get sharing
   * @return sharing
   */
  @Valid 
  @Schema(name = "sharing", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("sharing")
  public WorkspaceSharing getSharing() {
    return sharing;
  }

  public void setSharing(WorkspaceSharing sharing) {
    this.sharing = sharing;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    UpdateWorkspaceSharingRequest updateWorkspaceSharingRequest = (UpdateWorkspaceSharingRequest) o;
    return Objects.equals(this.sharing, updateWorkspaceSharingRequest.sharing);
  }

  @Override
  public int hashCode() {
    return Objects.hash(sharing);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class UpdateWorkspaceSharingRequest {\n");
    sb.append("    sharing: ").append(toIndentedString(sharing)).append("\n");
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

