package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * AuthContext
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-08T10:15:33.789499-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class AuthContext {

  @Valid
  private List<String> requiredScopes = new ArrayList<>();

  @Valid
  private List<String> providedScopes = new ArrayList<>();

  @Valid
  private List<String> missingPermissions = new ArrayList<>();

  private String tenantId;

  public AuthContext requiredScopes(List<String> requiredScopes) {
    this.requiredScopes = requiredScopes;
    return this;
  }

  public AuthContext addRequiredScopesItem(String requiredScopesItem) {
    if (this.requiredScopes == null) {
      this.requiredScopes = new ArrayList<>();
    }
    this.requiredScopes.add(requiredScopesItem);
    return this;
  }

  /**
   * Get requiredScopes
   * @return requiredScopes
   */
  
  @Schema(name = "requiredScopes", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("requiredScopes")
  public List<String> getRequiredScopes() {
    return requiredScopes;
  }

  public void setRequiredScopes(List<String> requiredScopes) {
    this.requiredScopes = requiredScopes;
  }

  public AuthContext providedScopes(List<String> providedScopes) {
    this.providedScopes = providedScopes;
    return this;
  }

  public AuthContext addProvidedScopesItem(String providedScopesItem) {
    if (this.providedScopes == null) {
      this.providedScopes = new ArrayList<>();
    }
    this.providedScopes.add(providedScopesItem);
    return this;
  }

  /**
   * Get providedScopes
   * @return providedScopes
   */
  
  @Schema(name = "providedScopes", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("providedScopes")
  public List<String> getProvidedScopes() {
    return providedScopes;
  }

  public void setProvidedScopes(List<String> providedScopes) {
    this.providedScopes = providedScopes;
  }

  public AuthContext missingPermissions(List<String> missingPermissions) {
    this.missingPermissions = missingPermissions;
    return this;
  }

  public AuthContext addMissingPermissionsItem(String missingPermissionsItem) {
    if (this.missingPermissions == null) {
      this.missingPermissions = new ArrayList<>();
    }
    this.missingPermissions.add(missingPermissionsItem);
    return this;
  }

  /**
   * Get missingPermissions
   * @return missingPermissions
   */
  
  @Schema(name = "missingPermissions", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("missingPermissions")
  public List<String> getMissingPermissions() {
    return missingPermissions;
  }

  public void setMissingPermissions(List<String> missingPermissions) {
    this.missingPermissions = missingPermissions;
  }

  public AuthContext tenantId(String tenantId) {
    this.tenantId = tenantId;
    return this;
  }

  /**
   * Get tenantId
   * @return tenantId
   */
  
  @Schema(name = "tenantId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("tenantId")
  public String getTenantId() {
    return tenantId;
  }

  public void setTenantId(String tenantId) {
    this.tenantId = tenantId;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    AuthContext authContext = (AuthContext) o;
    return Objects.equals(this.requiredScopes, authContext.requiredScopes) &&
        Objects.equals(this.providedScopes, authContext.providedScopes) &&
        Objects.equals(this.missingPermissions, authContext.missingPermissions) &&
        Objects.equals(this.tenantId, authContext.tenantId);
  }

  @Override
  public int hashCode() {
    return Objects.hash(requiredScopes, providedScopes, missingPermissions, tenantId);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class AuthContext {\n");
    sb.append("    requiredScopes: ").append(toIndentedString(requiredScopes)).append("\n");
    sb.append("    providedScopes: ").append(toIndentedString(providedScopes)).append("\n");
    sb.append("    missingPermissions: ").append(toIndentedString(missingPermissions)).append("\n");
    sb.append("    tenantId: ").append(toIndentedString(tenantId)).append("\n");
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

