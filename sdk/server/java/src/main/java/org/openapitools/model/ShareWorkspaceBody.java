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
 * ShareWorkspaceBody
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-03T10:37:17.791439-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ShareWorkspaceBody {

  private String sharedWithEmail;

  private String permissionLevel;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime expiresAt;

  public ShareWorkspaceBody sharedWithEmail(String sharedWithEmail) {
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

  public ShareWorkspaceBody permissionLevel(String permissionLevel) {
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

  public ShareWorkspaceBody expiresAt(OffsetDateTime expiresAt) {
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

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    ShareWorkspaceBody shareWorkspaceBody = (ShareWorkspaceBody) o;
    return Objects.equals(this.sharedWithEmail, shareWorkspaceBody.sharedWithEmail) &&
        Objects.equals(this.permissionLevel, shareWorkspaceBody.permissionLevel) &&
        Objects.equals(this.expiresAt, shareWorkspaceBody.expiresAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(sharedWithEmail, permissionLevel, expiresAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ShareWorkspaceBody {\n");
    sb.append("    sharedWithEmail: ").append(toIndentedString(sharedWithEmail)).append("\n");
    sb.append("    permissionLevel: ").append(toIndentedString(permissionLevel)).append("\n");
    sb.append("    expiresAt: ").append(toIndentedString(expiresAt)).append("\n");
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

