package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import org.openapitools.model.ComplianceLevel;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * CreateAccountRequest
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T22:05:34.423190-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class CreateAccountRequest {

  private String auth0UserId;

  private String email;

  private String baseDirectory;

  private String region;

  private String orgId;

  private String tenantId;

  @Valid
  private List<String> roles = new ArrayList<>();

  @Valid
  private List<String> permissions = new ArrayList<>();

  private Boolean mfaEnabled;

  private ComplianceLevel complianceLevel = ComplianceLevel.UNSPECIFIED;

  @Valid
  private Map<String, String> preferences = new HashMap<>();

  public CreateAccountRequest() {
    super();
  }

  /**
   * Constructor with only required parameters
   */
  public CreateAccountRequest(String auth0UserId, String email) {
    this.auth0UserId = auth0UserId;
    this.email = email;
  }

  public CreateAccountRequest auth0UserId(String auth0UserId) {
    this.auth0UserId = auth0UserId;
    return this;
  }

  /**
   * Get auth0UserId
   * @return auth0UserId
   */
  @NotNull 
  @Schema(name = "auth0UserId", requiredMode = Schema.RequiredMode.REQUIRED)
  @JsonProperty("auth0UserId")
  public String getAuth0UserId() {
    return auth0UserId;
  }

  public void setAuth0UserId(String auth0UserId) {
    this.auth0UserId = auth0UserId;
  }

  public CreateAccountRequest email(String email) {
    this.email = email;
    return this;
  }

  /**
   * Get email
   * @return email
   */
  @NotNull 
  @Schema(name = "email", requiredMode = Schema.RequiredMode.REQUIRED)
  @JsonProperty("email")
  public String getEmail() {
    return email;
  }

  public void setEmail(String email) {
    this.email = email;
  }

  public CreateAccountRequest baseDirectory(String baseDirectory) {
    this.baseDirectory = baseDirectory;
    return this;
  }

  /**
   * Get baseDirectory
   * @return baseDirectory
   */
  
  @Schema(name = "baseDirectory", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("baseDirectory")
  public String getBaseDirectory() {
    return baseDirectory;
  }

  public void setBaseDirectory(String baseDirectory) {
    this.baseDirectory = baseDirectory;
  }

  public CreateAccountRequest region(String region) {
    this.region = region;
    return this;
  }

  /**
   * Get region
   * @return region
   */
  
  @Schema(name = "region", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("region")
  public String getRegion() {
    return region;
  }

  public void setRegion(String region) {
    this.region = region;
  }

  public CreateAccountRequest orgId(String orgId) {
    this.orgId = orgId;
    return this;
  }

  /**
   * Get orgId
   * @return orgId
   */
  
  @Schema(name = "orgId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("orgId")
  public String getOrgId() {
    return orgId;
  }

  public void setOrgId(String orgId) {
    this.orgId = orgId;
  }

  public CreateAccountRequest tenantId(String tenantId) {
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

  public CreateAccountRequest roles(List<String> roles) {
    this.roles = roles;
    return this;
  }

  public CreateAccountRequest addRolesItem(String rolesItem) {
    if (this.roles == null) {
      this.roles = new ArrayList<>();
    }
    this.roles.add(rolesItem);
    return this;
  }

  /**
   * Get roles
   * @return roles
   */
  
  @Schema(name = "roles", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("roles")
  public List<String> getRoles() {
    return roles;
  }

  public void setRoles(List<String> roles) {
    this.roles = roles;
  }

  public CreateAccountRequest permissions(List<String> permissions) {
    this.permissions = permissions;
    return this;
  }

  public CreateAccountRequest addPermissionsItem(String permissionsItem) {
    if (this.permissions == null) {
      this.permissions = new ArrayList<>();
    }
    this.permissions.add(permissionsItem);
    return this;
  }

  /**
   * Get permissions
   * @return permissions
   */
  
  @Schema(name = "permissions", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("permissions")
  public List<String> getPermissions() {
    return permissions;
  }

  public void setPermissions(List<String> permissions) {
    this.permissions = permissions;
  }

  public CreateAccountRequest mfaEnabled(Boolean mfaEnabled) {
    this.mfaEnabled = mfaEnabled;
    return this;
  }

  /**
   * Get mfaEnabled
   * @return mfaEnabled
   */
  
  @Schema(name = "mfaEnabled", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("mfaEnabled")
  public Boolean getMfaEnabled() {
    return mfaEnabled;
  }

  public void setMfaEnabled(Boolean mfaEnabled) {
    this.mfaEnabled = mfaEnabled;
  }

  public CreateAccountRequest complianceLevel(ComplianceLevel complianceLevel) {
    this.complianceLevel = complianceLevel;
    return this;
  }

  /**
   * Get complianceLevel
   * @return complianceLevel
   */
  @Valid 
  @Schema(name = "complianceLevel", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("complianceLevel")
  public ComplianceLevel getComplianceLevel() {
    return complianceLevel;
  }

  public void setComplianceLevel(ComplianceLevel complianceLevel) {
    this.complianceLevel = complianceLevel;
  }

  public CreateAccountRequest preferences(Map<String, String> preferences) {
    this.preferences = preferences;
    return this;
  }

  public CreateAccountRequest putPreferencesItem(String key, String preferencesItem) {
    if (this.preferences == null) {
      this.preferences = new HashMap<>();
    }
    this.preferences.put(key, preferencesItem);
    return this;
  }

  /**
   * Get preferences
   * @return preferences
   */
  
  @Schema(name = "preferences", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("preferences")
  public Map<String, String> getPreferences() {
    return preferences;
  }

  public void setPreferences(Map<String, String> preferences) {
    this.preferences = preferences;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    CreateAccountRequest createAccountRequest = (CreateAccountRequest) o;
    return Objects.equals(this.auth0UserId, createAccountRequest.auth0UserId) &&
        Objects.equals(this.email, createAccountRequest.email) &&
        Objects.equals(this.baseDirectory, createAccountRequest.baseDirectory) &&
        Objects.equals(this.region, createAccountRequest.region) &&
        Objects.equals(this.orgId, createAccountRequest.orgId) &&
        Objects.equals(this.tenantId, createAccountRequest.tenantId) &&
        Objects.equals(this.roles, createAccountRequest.roles) &&
        Objects.equals(this.permissions, createAccountRequest.permissions) &&
        Objects.equals(this.mfaEnabled, createAccountRequest.mfaEnabled) &&
        Objects.equals(this.complianceLevel, createAccountRequest.complianceLevel) &&
        Objects.equals(this.preferences, createAccountRequest.preferences);
  }

  @Override
  public int hashCode() {
    return Objects.hash(auth0UserId, email, baseDirectory, region, orgId, tenantId, roles, permissions, mfaEnabled, complianceLevel, preferences);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class CreateAccountRequest {\n");
    sb.append("    auth0UserId: ").append(toIndentedString(auth0UserId)).append("\n");
    sb.append("    email: ").append(toIndentedString(email)).append("\n");
    sb.append("    baseDirectory: ").append(toIndentedString(baseDirectory)).append("\n");
    sb.append("    region: ").append(toIndentedString(region)).append("\n");
    sb.append("    orgId: ").append(toIndentedString(orgId)).append("\n");
    sb.append("    tenantId: ").append(toIndentedString(tenantId)).append("\n");
    sb.append("    roles: ").append(toIndentedString(roles)).append("\n");
    sb.append("    permissions: ").append(toIndentedString(permissions)).append("\n");
    sb.append("    mfaEnabled: ").append(toIndentedString(mfaEnabled)).append("\n");
    sb.append("    complianceLevel: ").append(toIndentedString(complianceLevel)).append("\n");
    sb.append("    preferences: ").append(toIndentedString(preferences)).append("\n");
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

