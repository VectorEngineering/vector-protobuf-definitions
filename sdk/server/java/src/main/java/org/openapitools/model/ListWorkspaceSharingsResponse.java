package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.WorkspaceSharing;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * ListWorkspaceSharingsResponse
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-19T01:49:19.350068-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ListWorkspaceSharingsResponse {

  @Valid
  private List<@Valid WorkspaceSharing> sharings = new ArrayList<>();

  private String nextPageToken;

  private Integer totalSize;

  public ListWorkspaceSharingsResponse sharings(List<@Valid WorkspaceSharing> sharings) {
    this.sharings = sharings;
    return this;
  }

  public ListWorkspaceSharingsResponse addSharingsItem(WorkspaceSharing sharingsItem) {
    if (this.sharings == null) {
      this.sharings = new ArrayList<>();
    }
    this.sharings.add(sharingsItem);
    return this;
  }

  /**
   * Get sharings
   * @return sharings
   */
  @Valid 
  @Schema(name = "sharings", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("sharings")
  public List<@Valid WorkspaceSharing> getSharings() {
    return sharings;
  }

  public void setSharings(List<@Valid WorkspaceSharing> sharings) {
    this.sharings = sharings;
  }

  public ListWorkspaceSharingsResponse nextPageToken(String nextPageToken) {
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

  public ListWorkspaceSharingsResponse totalSize(Integer totalSize) {
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
    ListWorkspaceSharingsResponse listWorkspaceSharingsResponse = (ListWorkspaceSharingsResponse) o;
    return Objects.equals(this.sharings, listWorkspaceSharingsResponse.sharings) &&
        Objects.equals(this.nextPageToken, listWorkspaceSharingsResponse.nextPageToken) &&
        Objects.equals(this.totalSize, listWorkspaceSharingsResponse.totalSize);
  }

  @Override
  public int hashCode() {
    return Objects.hash(sharings, nextPageToken, totalSize);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ListWorkspaceSharingsResponse {\n");
    sb.append("    sharings: ").append(toIndentedString(sharings)).append("\n");
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

