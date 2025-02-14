package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.Workspace1;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * ListWorkspacesResponse1
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T08:12:58.450907-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ListWorkspacesResponse1 {

  @Valid
  private List<@Valid Workspace1> workspaces = new ArrayList<>();

  private String nextPageToken;

  private Integer totalSize;

  public ListWorkspacesResponse1 workspaces(List<@Valid Workspace1> workspaces) {
    this.workspaces = workspaces;
    return this;
  }

  public ListWorkspacesResponse1 addWorkspacesItem(Workspace1 workspacesItem) {
    if (this.workspaces == null) {
      this.workspaces = new ArrayList<>();
    }
    this.workspaces.add(workspacesItem);
    return this;
  }

  /**
   * Get workspaces
   * @return workspaces
   */
  @Valid 
  @Schema(name = "workspaces", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("workspaces")
  public List<@Valid Workspace1> getWorkspaces() {
    return workspaces;
  }

  public void setWorkspaces(List<@Valid Workspace1> workspaces) {
    this.workspaces = workspaces;
  }

  public ListWorkspacesResponse1 nextPageToken(String nextPageToken) {
    this.nextPageToken = nextPageToken;
    return this;
  }

  /**
   * Get nextPageToken
   * @return nextPageToken
   */
  
  @Schema(name = "nextPageToken", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("nextPageToken")
  public String getNextPageToken() {
    return nextPageToken;
  }

  public void setNextPageToken(String nextPageToken) {
    this.nextPageToken = nextPageToken;
  }

  public ListWorkspacesResponse1 totalSize(Integer totalSize) {
    this.totalSize = totalSize;
    return this;
  }

  /**
   * Get totalSize
   * @return totalSize
   */
  
  @Schema(name = "totalSize", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("totalSize")
  public Integer getTotalSize() {
    return totalSize;
  }

  public void setTotalSize(Integer totalSize) {
    this.totalSize = totalSize;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    ListWorkspacesResponse1 listWorkspacesResponse1 = (ListWorkspacesResponse1) o;
    return Objects.equals(this.workspaces, listWorkspacesResponse1.workspaces) &&
        Objects.equals(this.nextPageToken, listWorkspacesResponse1.nextPageToken) &&
        Objects.equals(this.totalSize, listWorkspacesResponse1.totalSize);
  }

  @Override
  public int hashCode() {
    return Objects.hash(workspaces, nextPageToken, totalSize);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ListWorkspacesResponse1 {\n");
    sb.append("    workspaces: ").append(toIndentedString(workspaces)).append("\n");
    sb.append("    nextPageToken: ").append(toIndentedString(nextPageToken)).append("\n");
    sb.append("    totalSize: ").append(toIndentedString(totalSize)).append("\n");
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

