package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.Workspace;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * ListWorkspacesResponse
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T08:10:47.537779-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ListWorkspacesResponse {

  @Valid
  private List<@Valid Workspace> workspaces = new ArrayList<>();

  private Integer nextPageNumber;

  public ListWorkspacesResponse workspaces(List<@Valid Workspace> workspaces) {
    this.workspaces = workspaces;
    return this;
  }

  public ListWorkspacesResponse addWorkspacesItem(Workspace workspacesItem) {
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
  public List<@Valid Workspace> getWorkspaces() {
    return workspaces;
  }

  public void setWorkspaces(List<@Valid Workspace> workspaces) {
    this.workspaces = workspaces;
  }

  public ListWorkspacesResponse nextPageNumber(Integer nextPageNumber) {
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
    ListWorkspacesResponse listWorkspacesResponse = (ListWorkspacesResponse) o;
    return Objects.equals(this.workspaces, listWorkspacesResponse.workspaces) &&
        Objects.equals(this.nextPageNumber, listWorkspacesResponse.nextPageNumber);
  }

  @Override
  public int hashCode() {
    return Objects.hash(workspaces, nextPageNumber);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ListWorkspacesResponse {\n");
    sb.append("    workspaces: ").append(toIndentedString(workspaces)).append("\n");
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

