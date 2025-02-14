package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import java.time.OffsetDateTime;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import org.openapitools.model.ComplianceLevel;
import org.openapitools.model.DataProfile;
import org.openapitools.model.Workspace1;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * Account1
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T12:22:33.767757-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class Account1 {

  private String id;

  private String auth0UserId;

  private String email;

  private String baseDirectory;

  private String bucketName;

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

  @Valid
  private List<String> apiKeys = new ArrayList<>();

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime deletedAt;

  @Valid
  private List<@Valid Workspace1> workspaces = new ArrayList<>();

  @Valid
  private List<@Valid DataProfile> dataProfiles = new ArrayList<>();

  public Account1 id(String id) {
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

  public Account1 auth0UserId(String auth0UserId) {
    this.auth0UserId = auth0UserId;
    return this;
  }

  /**
   * Get auth0UserId
   * @return auth0UserId
   */
  
  @Schema(name = "auth0UserId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("auth0UserId")
  public String getAuth0UserId() {
    return auth0UserId;
  }

  public void setAuth0UserId(String auth0UserId) {
    this.auth0UserId = auth0UserId;
  }

  public Account1 email(String email) {
    this.email = email;
    return this;
  }

  /**
   * Get email
   * @return email
   */
  
  @Schema(name = "email", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("email")
  public String getEmail() {
    return email;
  }

  public void setEmail(String email) {
    this.email = email;
  }

  public Account1 baseDirectory(String baseDirectory) {
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

  public Account1 bucketName(String bucketName) {
    this.bucketName = bucketName;
    return this;
  }

  /**
   * Get bucketName
   * @return bucketName
   */
  
  @Schema(name = "bucketName", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("bucketName")
  public String getBucketName() {
    return bucketName;
  }

  public void setBucketName(String bucketName) {
    this.bucketName = bucketName;
  }

  public Account1 region(String region) {
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

  public Account1 orgId(String orgId) {
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

  public Account1 tenantId(String tenantId) {
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

  public Account1 roles(List<String> roles) {
    this.roles = roles;
    return this;
  }

  public Account1 addRolesItem(String rolesItem) {
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

  public Account1 permissions(List<String> permissions) {
    this.permissions = permissions;
    return this;
  }

  public Account1 addPermissionsItem(String permissionsItem) {
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

  public Account1 mfaEnabled(Boolean mfaEnabled) {
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

  public Account1 complianceLevel(ComplianceLevel complianceLevel) {
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

  public Account1 preferences(Map<String, String> preferences) {
    this.preferences = preferences;
    return this;
  }

  public Account1 putPreferencesItem(String key, String preferencesItem) {
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

  public Account1 apiKeys(List<String> apiKeys) {
    this.apiKeys = apiKeys;
    return this;
  }

  public Account1 addApiKeysItem(String apiKeysItem) {
    if (this.apiKeys == null) {
      this.apiKeys = new ArrayList<>();
    }
    this.apiKeys.add(apiKeysItem);
    return this;
  }

  /**
   * Get apiKeys
   * @return apiKeys
   */
  
  @Schema(name = "apiKeys", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("apiKeys")
  public List<String> getApiKeys() {
    return apiKeys;
  }

  public void setApiKeys(List<String> apiKeys) {
    this.apiKeys = apiKeys;
  }

  public Account1 createdAt(OffsetDateTime createdAt) {
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

  public Account1 updatedAt(OffsetDateTime updatedAt) {
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

  public Account1 deletedAt(OffsetDateTime deletedAt) {
    this.deletedAt = deletedAt;
    return this;
  }

  /**
   * Get deletedAt
   * @return deletedAt
   */
  @Valid 
  @Schema(name = "deletedAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("deletedAt")
  public OffsetDateTime getDeletedAt() {
    return deletedAt;
  }

  public void setDeletedAt(OffsetDateTime deletedAt) {
    this.deletedAt = deletedAt;
  }

  public Account1 workspaces(List<@Valid Workspace1> workspaces) {
    this.workspaces = workspaces;
    return this;
  }

  public Account1 addWorkspacesItem(Workspace1 workspacesItem) {
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

  public Account1 dataProfiles(List<@Valid DataProfile> dataProfiles) {
    this.dataProfiles = dataProfiles;
    return this;
  }

  public Account1 addDataProfilesItem(DataProfile dataProfilesItem) {
    if (this.dataProfiles == null) {
      this.dataProfiles = new ArrayList<>();
    }
    this.dataProfiles.add(dataProfilesItem);
    return this;
  }

  /**
   * Get dataProfiles
   * @return dataProfiles
   */
  @Valid 
  @Schema(name = "dataProfiles", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("dataProfiles")
  public List<@Valid DataProfile> getDataProfiles() {
    return dataProfiles;
  }

  public void setDataProfiles(List<@Valid DataProfile> dataProfiles) {
    this.dataProfiles = dataProfiles;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    Account1 account1 = (Account1) o;
    return Objects.equals(this.id, account1.id) &&
        Objects.equals(this.auth0UserId, account1.auth0UserId) &&
        Objects.equals(this.email, account1.email) &&
        Objects.equals(this.baseDirectory, account1.baseDirectory) &&
        Objects.equals(this.bucketName, account1.bucketName) &&
        Objects.equals(this.region, account1.region) &&
        Objects.equals(this.orgId, account1.orgId) &&
        Objects.equals(this.tenantId, account1.tenantId) &&
        Objects.equals(this.roles, account1.roles) &&
        Objects.equals(this.permissions, account1.permissions) &&
        Objects.equals(this.mfaEnabled, account1.mfaEnabled) &&
        Objects.equals(this.complianceLevel, account1.complianceLevel) &&
        Objects.equals(this.preferences, account1.preferences) &&
        Objects.equals(this.apiKeys, account1.apiKeys) &&
        Objects.equals(this.createdAt, account1.createdAt) &&
        Objects.equals(this.updatedAt, account1.updatedAt) &&
        Objects.equals(this.deletedAt, account1.deletedAt) &&
        Objects.equals(this.workspaces, account1.workspaces) &&
        Objects.equals(this.dataProfiles, account1.dataProfiles);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, auth0UserId, email, baseDirectory, bucketName, region, orgId, tenantId, roles, permissions, mfaEnabled, complianceLevel, preferences, apiKeys, createdAt, updatedAt, deletedAt, workspaces, dataProfiles);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class Account1 {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    auth0UserId: ").append(toIndentedString(auth0UserId)).append("\n");
    sb.append("    email: ").append(toIndentedString(email)).append("\n");
    sb.append("    baseDirectory: ").append(toIndentedString(baseDirectory)).append("\n");
    sb.append("    bucketName: ").append(toIndentedString(bucketName)).append("\n");
    sb.append("    region: ").append(toIndentedString(region)).append("\n");
    sb.append("    orgId: ").append(toIndentedString(orgId)).append("\n");
    sb.append("    tenantId: ").append(toIndentedString(tenantId)).append("\n");
    sb.append("    roles: ").append(toIndentedString(roles)).append("\n");
    sb.append("    permissions: ").append(toIndentedString(permissions)).append("\n");
    sb.append("    mfaEnabled: ").append(toIndentedString(mfaEnabled)).append("\n");
    sb.append("    complianceLevel: ").append(toIndentedString(complianceLevel)).append("\n");
    sb.append("    preferences: ").append(toIndentedString(preferences)).append("\n");
    sb.append("    apiKeys: ").append(toIndentedString(apiKeys)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
    sb.append("    updatedAt: ").append(toIndentedString(updatedAt)).append("\n");
    sb.append("    deletedAt: ").append(toIndentedString(deletedAt)).append("\n");
    sb.append("    workspaces: ").append(toIndentedString(workspaces)).append("\n");
    sb.append("    dataProfiles: ").append(toIndentedString(dataProfiles)).append("\n");
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

