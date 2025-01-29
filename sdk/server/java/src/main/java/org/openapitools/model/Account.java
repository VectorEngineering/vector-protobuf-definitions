package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import java.time.OffsetDateTime;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.AccountSettings;
import org.openapitools.model.AccountStatus;
import org.openapitools.model.ScrapingJob;
import org.openapitools.model.Workspace;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * Account represents a user account within the workspace service. It serves as the top-level container for all user-specific workspaces and settings.  Key features: - Unique identification via Auth0 - Organization and tenant context - Audit timestamps - Associated scraping jobs  Database considerations: - Uses GORM for ORM mapping - Includes indexes for efficient querying - Supports soft deletes via deleted_at  Usage example: &#x60;&#x60;&#x60;go account :&#x3D; &amp;Account{     AuthPlatformUserId: \&quot;auth0|123\&quot;,     OrgId: \&quot;org_123\&quot;,     TenantId: \&quot;tenant_456\&quot;, } &#x60;&#x60;&#x60;
 */

@Schema(name = "Account", description = "Account represents a user account within the workspace service. It serves as the top-level container for all user-specific workspaces and settings.  Key features: - Unique identification via Auth0 - Organization and tenant context - Audit timestamps - Associated scraping jobs  Database considerations: - Uses GORM for ORM mapping - Includes indexes for efficient querying - Supports soft deletes via deleted_at  Usage example: ```go account := &Account{     AuthPlatformUserId: \"auth0|123\",     OrgId: \"org_123\",     TenantId: \"tenant_456\", } ```")
@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-01-29T12:50:06.029660-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class Account {

  private String id;

  private String authPlatformUserId;

  private String orgId;

  private String tenantId;

  private String email;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime lastModifiedAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime deletedAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @Valid
  private List<@Valid ScrapingJob> scrapingJobs = new ArrayList<>();

  private AccountStatus accountStatus = AccountStatus.UNSPECIFIED;

  @Valid
  private List<String> roles = new ArrayList<>();

  @Valid
  private List<String> permissions = new ArrayList<>();

  private Boolean mfaEnabled;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime lastLoginAt;

  private String timezone;

  private Integer totalJobsRun;

  private Integer monthlyJobLimit;

  private Integer concurrentJobLimit;

  @Valid
  private List<@Valid Workspace> workspaces = new ArrayList<>();

  private AccountSettings settings;

  public Account id(String id) {
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

  public Account authPlatformUserId(String authPlatformUserId) {
    this.authPlatformUserId = authPlatformUserId;
    return this;
  }

  /**
   * Get authPlatformUserId
   * @return authPlatformUserId
   */
  
  @Schema(name = "authPlatformUserId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("authPlatformUserId")
  public String getAuthPlatformUserId() {
    return authPlatformUserId;
  }

  public void setAuthPlatformUserId(String authPlatformUserId) {
    this.authPlatformUserId = authPlatformUserId;
  }

  public Account orgId(String orgId) {
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

  public Account tenantId(String tenantId) {
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

  public Account email(String email) {
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

  public Account lastModifiedAt(OffsetDateTime lastModifiedAt) {
    this.lastModifiedAt = lastModifiedAt;
    return this;
  }

  /**
   * Get lastModifiedAt
   * @return lastModifiedAt
   */
  @Valid 
  @Schema(name = "lastModifiedAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("lastModifiedAt")
  public OffsetDateTime getLastModifiedAt() {
    return lastModifiedAt;
  }

  public void setLastModifiedAt(OffsetDateTime lastModifiedAt) {
    this.lastModifiedAt = lastModifiedAt;
  }

  public Account deletedAt(OffsetDateTime deletedAt) {
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

  public Account createdAt(OffsetDateTime createdAt) {
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

  public Account scrapingJobs(List<@Valid ScrapingJob> scrapingJobs) {
    this.scrapingJobs = scrapingJobs;
    return this;
  }

  public Account addScrapingJobsItem(ScrapingJob scrapingJobsItem) {
    if (this.scrapingJobs == null) {
      this.scrapingJobs = new ArrayList<>();
    }
    this.scrapingJobs.add(scrapingJobsItem);
    return this;
  }

  /**
   * Get scrapingJobs
   * @return scrapingJobs
   */
  @Valid 
  @Schema(name = "scrapingJobs", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("scrapingJobs")
  public List<@Valid ScrapingJob> getScrapingJobs() {
    return scrapingJobs;
  }

  public void setScrapingJobs(List<@Valid ScrapingJob> scrapingJobs) {
    this.scrapingJobs = scrapingJobs;
  }

  public Account accountStatus(AccountStatus accountStatus) {
    this.accountStatus = accountStatus;
    return this;
  }

  /**
   * Get accountStatus
   * @return accountStatus
   */
  @Valid 
  @Schema(name = "accountStatus", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("accountStatus")
  public AccountStatus getAccountStatus() {
    return accountStatus;
  }

  public void setAccountStatus(AccountStatus accountStatus) {
    this.accountStatus = accountStatus;
  }

  public Account roles(List<String> roles) {
    this.roles = roles;
    return this;
  }

  public Account addRolesItem(String rolesItem) {
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

  public Account permissions(List<String> permissions) {
    this.permissions = permissions;
    return this;
  }

  public Account addPermissionsItem(String permissionsItem) {
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

  public Account mfaEnabled(Boolean mfaEnabled) {
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

  public Account lastLoginAt(OffsetDateTime lastLoginAt) {
    this.lastLoginAt = lastLoginAt;
    return this;
  }

  /**
   * Get lastLoginAt
   * @return lastLoginAt
   */
  @Valid 
  @Schema(name = "lastLoginAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("lastLoginAt")
  public OffsetDateTime getLastLoginAt() {
    return lastLoginAt;
  }

  public void setLastLoginAt(OffsetDateTime lastLoginAt) {
    this.lastLoginAt = lastLoginAt;
  }

  public Account timezone(String timezone) {
    this.timezone = timezone;
    return this;
  }

  /**
   * Get timezone
   * @return timezone
   */
  
  @Schema(name = "timezone", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("timezone")
  public String getTimezone() {
    return timezone;
  }

  public void setTimezone(String timezone) {
    this.timezone = timezone;
  }

  public Account totalJobsRun(Integer totalJobsRun) {
    this.totalJobsRun = totalJobsRun;
    return this;
  }

  /**
   * Get totalJobsRun
   * @return totalJobsRun
   */
  
  @Schema(name = "totalJobsRun", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("totalJobsRun")
  public Integer getTotalJobsRun() {
    return totalJobsRun;
  }

  public void setTotalJobsRun(Integer totalJobsRun) {
    this.totalJobsRun = totalJobsRun;
  }

  public Account monthlyJobLimit(Integer monthlyJobLimit) {
    this.monthlyJobLimit = monthlyJobLimit;
    return this;
  }

  /**
   * Get monthlyJobLimit
   * @return monthlyJobLimit
   */
  
  @Schema(name = "monthlyJobLimit", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("monthlyJobLimit")
  public Integer getMonthlyJobLimit() {
    return monthlyJobLimit;
  }

  public void setMonthlyJobLimit(Integer monthlyJobLimit) {
    this.monthlyJobLimit = monthlyJobLimit;
  }

  public Account concurrentJobLimit(Integer concurrentJobLimit) {
    this.concurrentJobLimit = concurrentJobLimit;
    return this;
  }

  /**
   * Get concurrentJobLimit
   * @return concurrentJobLimit
   */
  
  @Schema(name = "concurrentJobLimit", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("concurrentJobLimit")
  public Integer getConcurrentJobLimit() {
    return concurrentJobLimit;
  }

  public void setConcurrentJobLimit(Integer concurrentJobLimit) {
    this.concurrentJobLimit = concurrentJobLimit;
  }

  public Account workspaces(List<@Valid Workspace> workspaces) {
    this.workspaces = workspaces;
    return this;
  }

  public Account addWorkspacesItem(Workspace workspacesItem) {
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

  public Account settings(AccountSettings settings) {
    this.settings = settings;
    return this;
  }

  /**
   * Get settings
   * @return settings
   */
  @Valid 
  @Schema(name = "settings", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("settings")
  public AccountSettings getSettings() {
    return settings;
  }

  public void setSettings(AccountSettings settings) {
    this.settings = settings;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    Account account = (Account) o;
    return Objects.equals(this.id, account.id) &&
        Objects.equals(this.authPlatformUserId, account.authPlatformUserId) &&
        Objects.equals(this.orgId, account.orgId) &&
        Objects.equals(this.tenantId, account.tenantId) &&
        Objects.equals(this.email, account.email) &&
        Objects.equals(this.lastModifiedAt, account.lastModifiedAt) &&
        Objects.equals(this.deletedAt, account.deletedAt) &&
        Objects.equals(this.createdAt, account.createdAt) &&
        Objects.equals(this.scrapingJobs, account.scrapingJobs) &&
        Objects.equals(this.accountStatus, account.accountStatus) &&
        Objects.equals(this.roles, account.roles) &&
        Objects.equals(this.permissions, account.permissions) &&
        Objects.equals(this.mfaEnabled, account.mfaEnabled) &&
        Objects.equals(this.lastLoginAt, account.lastLoginAt) &&
        Objects.equals(this.timezone, account.timezone) &&
        Objects.equals(this.totalJobsRun, account.totalJobsRun) &&
        Objects.equals(this.monthlyJobLimit, account.monthlyJobLimit) &&
        Objects.equals(this.concurrentJobLimit, account.concurrentJobLimit) &&
        Objects.equals(this.workspaces, account.workspaces) &&
        Objects.equals(this.settings, account.settings);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, authPlatformUserId, orgId, tenantId, email, lastModifiedAt, deletedAt, createdAt, scrapingJobs, accountStatus, roles, permissions, mfaEnabled, lastLoginAt, timezone, totalJobsRun, monthlyJobLimit, concurrentJobLimit, workspaces, settings);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class Account {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    authPlatformUserId: ").append(toIndentedString(authPlatformUserId)).append("\n");
    sb.append("    orgId: ").append(toIndentedString(orgId)).append("\n");
    sb.append("    tenantId: ").append(toIndentedString(tenantId)).append("\n");
    sb.append("    email: ").append(toIndentedString(email)).append("\n");
    sb.append("    lastModifiedAt: ").append(toIndentedString(lastModifiedAt)).append("\n");
    sb.append("    deletedAt: ").append(toIndentedString(deletedAt)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
    sb.append("    scrapingJobs: ").append(toIndentedString(scrapingJobs)).append("\n");
    sb.append("    accountStatus: ").append(toIndentedString(accountStatus)).append("\n");
    sb.append("    roles: ").append(toIndentedString(roles)).append("\n");
    sb.append("    permissions: ").append(toIndentedString(permissions)).append("\n");
    sb.append("    mfaEnabled: ").append(toIndentedString(mfaEnabled)).append("\n");
    sb.append("    lastLoginAt: ").append(toIndentedString(lastLoginAt)).append("\n");
    sb.append("    timezone: ").append(toIndentedString(timezone)).append("\n");
    sb.append("    totalJobsRun: ").append(toIndentedString(totalJobsRun)).append("\n");
    sb.append("    monthlyJobLimit: ").append(toIndentedString(monthlyJobLimit)).append("\n");
    sb.append("    concurrentJobLimit: ").append(toIndentedString(concurrentJobLimit)).append("\n");
    sb.append("    workspaces: ").append(toIndentedString(workspaces)).append("\n");
    sb.append("    settings: ").append(toIndentedString(settings)).append("\n");
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

