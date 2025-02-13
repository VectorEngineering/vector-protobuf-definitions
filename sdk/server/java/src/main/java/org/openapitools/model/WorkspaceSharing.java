package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * WorkspaceSharing
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-13T18:55:20.876030-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class WorkspaceSharing {

  private String id;

  private String sharedWithEmail;

  private String permissionLevel;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime expiresAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  public WorkspaceSharing id(String id) {
    this.id = id;
    return this;
  }

  /**
   * Get id
   * @return id
   */
  
  @Schema(name = "id", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("id")
  public String getId() {
    return id;
  }

  public void setId(String id) {
    this.id = id;
  }

  public WorkspaceSharing sharedWithEmail(String sharedWithEmail) {
    this.sharedWithEmail = sharedWithEmail;
    return this;
  }

  /**
   * Get sharedWithEmail
   * @return sharedWithEmail
   */
  
  @Schema(name = "sharedWithEmail", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("sharedWithEmail")
  public String getSharedWithEmail() {
    return sharedWithEmail;
  }

  public void setSharedWithEmail(String sharedWithEmail) {
    this.sharedWithEmail = sharedWithEmail;
  }

  public WorkspaceSharing permissionLevel(String permissionLevel) {
    this.permissionLevel = permissionLevel;
    return this;
  }

  /**
   * Get permissionLevel
   * @return permissionLevel
   */
  
  @Schema(name = "permissionLevel", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("permissionLevel")
  public String getPermissionLevel() {
    return permissionLevel;
  }

  public void setPermissionLevel(String permissionLevel) {
    this.permissionLevel = permissionLevel;
  }

  public WorkspaceSharing expiresAt(OffsetDateTime expiresAt) {
    this.expiresAt = expiresAt;
    return this;
  }

  /**
   * Get expiresAt
   * @return expiresAt
   */
  @Valid 
  @Schema(name = "expiresAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("expiresAt")
  public OffsetDateTime getExpiresAt() {
    return expiresAt;
  }

  public void setExpiresAt(OffsetDateTime expiresAt) {
    this.expiresAt = expiresAt;
  }

  public WorkspaceSharing createdAt(OffsetDateTime createdAt) {
    this.createdAt = createdAt;
    return this;
  }

  /**
   * Get createdAt
   * @return createdAt
   */
  @Valid 
  @Schema(name = "createdAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("createdAt")
  public OffsetDateTime getCreatedAt() {
    return createdAt;
  }

  public void setCreatedAt(OffsetDateTime createdAt) {
    this.createdAt = createdAt;
  }

  public WorkspaceSharing updatedAt(OffsetDateTime updatedAt) {
    this.updatedAt = updatedAt;
    return this;
  }

  /**
   * Get updatedAt
   * @return updatedAt
   */
  @Valid 
  @Schema(name = "updatedAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("updatedAt")
  public OffsetDateTime getUpdatedAt() {
    return updatedAt;
  }

  public void setUpdatedAt(OffsetDateTime updatedAt) {
    this.updatedAt = updatedAt;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    WorkspaceSharing workspaceSharing = (WorkspaceSharing) o;
    return Objects.equals(this.id, workspaceSharing.id) &&
        Objects.equals(this.sharedWithEmail, workspaceSharing.sharedWithEmail) &&
        Objects.equals(this.permissionLevel, workspaceSharing.permissionLevel) &&
        Objects.equals(this.expiresAt, workspaceSharing.expiresAt) &&
        Objects.equals(this.createdAt, workspaceSharing.createdAt) &&
        Objects.equals(this.updatedAt, workspaceSharing.updatedAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, sharedWithEmail, permissionLevel, expiresAt, createdAt, updatedAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class WorkspaceSharing {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    sharedWithEmail: ").append(toIndentedString(sharedWithEmail)).append("\n");
    sb.append("    permissionLevel: ").append(toIndentedString(permissionLevel)).append("\n");
    sb.append("    expiresAt: ").append(toIndentedString(expiresAt)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
    sb.append("    updatedAt: ").append(toIndentedString(updatedAt)).append("\n");
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

