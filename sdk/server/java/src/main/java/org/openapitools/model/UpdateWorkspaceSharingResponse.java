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
 * UpdateWorkspaceSharingResponse
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T22:05:34.423190-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class UpdateWorkspaceSharingResponse {

  private WorkspaceSharing sharing;

  public UpdateWorkspaceSharingResponse sharing(WorkspaceSharing sharing) {
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
    UpdateWorkspaceSharingResponse updateWorkspaceSharingResponse = (UpdateWorkspaceSharingResponse) o;
    return Objects.equals(this.sharing, updateWorkspaceSharingResponse.sharing);
  }

  @Override
  public int hashCode() {
    return Objects.hash(sharing);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class UpdateWorkspaceSharingResponse {\n");
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

