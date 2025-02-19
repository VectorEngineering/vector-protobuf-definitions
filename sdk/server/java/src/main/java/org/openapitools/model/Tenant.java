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
import org.openapitools.model.Account;
import org.openapitools.model.TenantAPIKey;
import org.openapitools.model.V1Status;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * Key features: - Resource isolation - Independent configuration - Usage tracking - Service subscriptions
 */

@Schema(name = "Tenant", description = "Key features: - Resource isolation - Independent configuration - Usage tracking - Service subscriptions")
@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-19T01:49:19.350068-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class Tenant {

  private String id;

  private String name;

  private String displayName;

  private String description;

  private String apiBaseUrl;

  @Valid
  private Map<String, String> environmentVariables = new HashMap<>();

  @Valid
  private List<String> allowedOrigins = new ArrayList<>();

  private String storageQuota;

  private String monthlyRequestLimit;

  private Integer maxConcurrentJobs;

  private Boolean enableCaching;

  private Boolean enableRateLimiting;

  private Boolean enableRequestLogging;

  @Valid
  private List<@Valid Account> accounts = new ArrayList<>();

  @Valid
  private List<@Valid TenantAPIKey> apiKeys = new ArrayList<>();

  private String totalRequests;

  private String totalStorageUsed;

  private Float averageResponseTime;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime deletedAt;

  private V1Status status = V1Status.UNSPECIFIED;

  public Tenant id(String id) {
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

  public Tenant name(String name) {
    this.name = name;
    return this;
  }

  /**
   * Get name
   * @return name
   */
  
  @Schema(name = "name", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("name")
  public String getName() {
    return name;
  }

  public void setName(String name) {
    this.name = name;
  }

  public Tenant displayName(String displayName) {
    this.displayName = displayName;
    return this;
  }

  /**
   * Get displayName
   * @return displayName
   */
  
  @Schema(name = "displayName", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("displayName")
  public String getDisplayName() {
    return displayName;
  }

  public void setDisplayName(String displayName) {
    this.displayName = displayName;
  }

  public Tenant description(String description) {
    this.description = description;
    return this;
  }

  /**
   * Get description
   * @return description
   */
  
  @Schema(name = "description", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("description")
  public String getDescription() {
    return description;
  }

  public void setDescription(String description) {
    this.description = description;
  }

  public Tenant apiBaseUrl(String apiBaseUrl) {
    this.apiBaseUrl = apiBaseUrl;
    return this;
  }

  /**
   * Get apiBaseUrl
   * @return apiBaseUrl
   */
  
  @Schema(name = "apiBaseUrl", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("apiBaseUrl")
  public String getApiBaseUrl() {
    return apiBaseUrl;
  }

  public void setApiBaseUrl(String apiBaseUrl) {
    this.apiBaseUrl = apiBaseUrl;
  }

  public Tenant environmentVariables(Map<String, String> environmentVariables) {
    this.environmentVariables = environmentVariables;
    return this;
  }

  public Tenant putEnvironmentVariablesItem(String key, String environmentVariablesItem) {
    if (this.environmentVariables == null) {
      this.environmentVariables = new HashMap<>();
    }
    this.environmentVariables.put(key, environmentVariablesItem);
    return this;
  }

  /**
   * Get environmentVariables
   * @return environmentVariables
   */
  
  @Schema(name = "environmentVariables", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("environmentVariables")
  public Map<String, String> getEnvironmentVariables() {
    return environmentVariables;
  }

  public void setEnvironmentVariables(Map<String, String> environmentVariables) {
    this.environmentVariables = environmentVariables;
  }

  public Tenant allowedOrigins(List<String> allowedOrigins) {
    this.allowedOrigins = allowedOrigins;
    return this;
  }

  public Tenant addAllowedOriginsItem(String allowedOriginsItem) {
    if (this.allowedOrigins == null) {
      this.allowedOrigins = new ArrayList<>();
    }
    this.allowedOrigins.add(allowedOriginsItem);
    return this;
  }

  /**
   * Get allowedOrigins
   * @return allowedOrigins
   */
  
  @Schema(name = "allowedOrigins", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("allowedOrigins")
  public List<String> getAllowedOrigins() {
    return allowedOrigins;
  }

  public void setAllowedOrigins(List<String> allowedOrigins) {
    this.allowedOrigins = allowedOrigins;
  }

  public Tenant storageQuota(String storageQuota) {
    this.storageQuota = storageQuota;
    return this;
  }

  /**
   * Get storageQuota
   * @return storageQuota
   */
  
  @Schema(name = "storageQuota", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("storageQuota")
  public String getStorageQuota() {
    return storageQuota;
  }

  public void setStorageQuota(String storageQuota) {
    this.storageQuota = storageQuota;
  }

  public Tenant monthlyRequestLimit(String monthlyRequestLimit) {
    this.monthlyRequestLimit = monthlyRequestLimit;
    return this;
  }

  /**
   * Get monthlyRequestLimit
   * @return monthlyRequestLimit
   */
  
  @Schema(name = "monthlyRequestLimit", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("monthlyRequestLimit")
  public String getMonthlyRequestLimit() {
    return monthlyRequestLimit;
  }

  public void setMonthlyRequestLimit(String monthlyRequestLimit) {
    this.monthlyRequestLimit = monthlyRequestLimit;
  }

  public Tenant maxConcurrentJobs(Integer maxConcurrentJobs) {
    this.maxConcurrentJobs = maxConcurrentJobs;
    return this;
  }

  /**
   * Get maxConcurrentJobs
   * @return maxConcurrentJobs
   */
  
  @Schema(name = "maxConcurrentJobs", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("maxConcurrentJobs")
  public Integer getMaxConcurrentJobs() {
    return maxConcurrentJobs;
  }

  public void setMaxConcurrentJobs(Integer maxConcurrentJobs) {
    this.maxConcurrentJobs = maxConcurrentJobs;
  }

  public Tenant enableCaching(Boolean enableCaching) {
    this.enableCaching = enableCaching;
    return this;
  }

  /**
   * Get enableCaching
   * @return enableCaching
   */
  
  @Schema(name = "enableCaching", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("enableCaching")
  public Boolean getEnableCaching() {
    return enableCaching;
  }

  public void setEnableCaching(Boolean enableCaching) {
    this.enableCaching = enableCaching;
  }

  public Tenant enableRateLimiting(Boolean enableRateLimiting) {
    this.enableRateLimiting = enableRateLimiting;
    return this;
  }

  /**
   * Get enableRateLimiting
   * @return enableRateLimiting
   */
  
  @Schema(name = "enableRateLimiting", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("enableRateLimiting")
  public Boolean getEnableRateLimiting() {
    return enableRateLimiting;
  }

  public void setEnableRateLimiting(Boolean enableRateLimiting) {
    this.enableRateLimiting = enableRateLimiting;
  }

  public Tenant enableRequestLogging(Boolean enableRequestLogging) {
    this.enableRequestLogging = enableRequestLogging;
    return this;
  }

  /**
   * Get enableRequestLogging
   * @return enableRequestLogging
   */
  
  @Schema(name = "enableRequestLogging", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("enableRequestLogging")
  public Boolean getEnableRequestLogging() {
    return enableRequestLogging;
  }

  public void setEnableRequestLogging(Boolean enableRequestLogging) {
    this.enableRequestLogging = enableRequestLogging;
  }

  public Tenant accounts(List<@Valid Account> accounts) {
    this.accounts = accounts;
    return this;
  }

  public Tenant addAccountsItem(Account accountsItem) {
    if (this.accounts == null) {
      this.accounts = new ArrayList<>();
    }
    this.accounts.add(accountsItem);
    return this;
  }

  /**
   * Get accounts
   * @return accounts
   */
  @Valid 
  @Schema(name = "accounts", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("accounts")
  public List<@Valid Account> getAccounts() {
    return accounts;
  }

  public void setAccounts(List<@Valid Account> accounts) {
    this.accounts = accounts;
  }

  public Tenant apiKeys(List<@Valid TenantAPIKey> apiKeys) {
    this.apiKeys = apiKeys;
    return this;
  }

  public Tenant addApiKeysItem(TenantAPIKey apiKeysItem) {
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
  @Valid 
  @Schema(name = "apiKeys", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("apiKeys")
  public List<@Valid TenantAPIKey> getApiKeys() {
    return apiKeys;
  }

  public void setApiKeys(List<@Valid TenantAPIKey> apiKeys) {
    this.apiKeys = apiKeys;
  }

  public Tenant totalRequests(String totalRequests) {
    this.totalRequests = totalRequests;
    return this;
  }

  /**
   * Get totalRequests
   * @return totalRequests
   */
  
  @Schema(name = "totalRequests", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("totalRequests")
  public String getTotalRequests() {
    return totalRequests;
  }

  public void setTotalRequests(String totalRequests) {
    this.totalRequests = totalRequests;
  }

  public Tenant totalStorageUsed(String totalStorageUsed) {
    this.totalStorageUsed = totalStorageUsed;
    return this;
  }

  /**
   * Get totalStorageUsed
   * @return totalStorageUsed
   */
  
  @Schema(name = "totalStorageUsed", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("totalStorageUsed")
  public String getTotalStorageUsed() {
    return totalStorageUsed;
  }

  public void setTotalStorageUsed(String totalStorageUsed) {
    this.totalStorageUsed = totalStorageUsed;
  }

  public Tenant averageResponseTime(Float averageResponseTime) {
    this.averageResponseTime = averageResponseTime;
    return this;
  }

  /**
   * Get averageResponseTime
   * @return averageResponseTime
   */
  
  @Schema(name = "averageResponseTime", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("averageResponseTime")
  public Float getAverageResponseTime() {
    return averageResponseTime;
  }

  public void setAverageResponseTime(Float averageResponseTime) {
    this.averageResponseTime = averageResponseTime;
  }

  public Tenant createdAt(OffsetDateTime createdAt) {
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

  public Tenant updatedAt(OffsetDateTime updatedAt) {
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

  public Tenant deletedAt(OffsetDateTime deletedAt) {
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

  public Tenant status(V1Status status) {
    this.status = status;
    return this;
  }

  /**
   * Get status
   * @return status
   */
  @Valid 
  @Schema(name = "status", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("status")
  public V1Status getStatus() {
    return status;
  }

  public void setStatus(V1Status status) {
    this.status = status;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    Tenant tenant = (Tenant) o;
    return Objects.equals(this.id, tenant.id) &&
        Objects.equals(this.name, tenant.name) &&
        Objects.equals(this.displayName, tenant.displayName) &&
        Objects.equals(this.description, tenant.description) &&
        Objects.equals(this.apiBaseUrl, tenant.apiBaseUrl) &&
        Objects.equals(this.environmentVariables, tenant.environmentVariables) &&
        Objects.equals(this.allowedOrigins, tenant.allowedOrigins) &&
        Objects.equals(this.storageQuota, tenant.storageQuota) &&
        Objects.equals(this.monthlyRequestLimit, tenant.monthlyRequestLimit) &&
        Objects.equals(this.maxConcurrentJobs, tenant.maxConcurrentJobs) &&
        Objects.equals(this.enableCaching, tenant.enableCaching) &&
        Objects.equals(this.enableRateLimiting, tenant.enableRateLimiting) &&
        Objects.equals(this.enableRequestLogging, tenant.enableRequestLogging) &&
        Objects.equals(this.accounts, tenant.accounts) &&
        Objects.equals(this.apiKeys, tenant.apiKeys) &&
        Objects.equals(this.totalRequests, tenant.totalRequests) &&
        Objects.equals(this.totalStorageUsed, tenant.totalStorageUsed) &&
        Objects.equals(this.averageResponseTime, tenant.averageResponseTime) &&
        Objects.equals(this.createdAt, tenant.createdAt) &&
        Objects.equals(this.updatedAt, tenant.updatedAt) &&
        Objects.equals(this.deletedAt, tenant.deletedAt) &&
        Objects.equals(this.status, tenant.status);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, name, displayName, description, apiBaseUrl, environmentVariables, allowedOrigins, storageQuota, monthlyRequestLimit, maxConcurrentJobs, enableCaching, enableRateLimiting, enableRequestLogging, accounts, apiKeys, totalRequests, totalStorageUsed, averageResponseTime, createdAt, updatedAt, deletedAt, status);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class Tenant {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    name: ").append(toIndentedString(name)).append("\n");
    sb.append("    displayName: ").append(toIndentedString(displayName)).append("\n");
    sb.append("    description: ").append(toIndentedString(description)).append("\n");
    sb.append("    apiBaseUrl: ").append(toIndentedString(apiBaseUrl)).append("\n");
    sb.append("    environmentVariables: ").append(toIndentedString(environmentVariables)).append("\n");
    sb.append("    allowedOrigins: ").append(toIndentedString(allowedOrigins)).append("\n");
    sb.append("    storageQuota: ").append(toIndentedString(storageQuota)).append("\n");
    sb.append("    monthlyRequestLimit: ").append(toIndentedString(monthlyRequestLimit)).append("\n");
    sb.append("    maxConcurrentJobs: ").append(toIndentedString(maxConcurrentJobs)).append("\n");
    sb.append("    enableCaching: ").append(toIndentedString(enableCaching)).append("\n");
    sb.append("    enableRateLimiting: ").append(toIndentedString(enableRateLimiting)).append("\n");
    sb.append("    enableRequestLogging: ").append(toIndentedString(enableRequestLogging)).append("\n");
    sb.append("    accounts: ").append(toIndentedString(accounts)).append("\n");
    sb.append("    apiKeys: ").append(toIndentedString(apiKeys)).append("\n");
    sb.append("    totalRequests: ").append(toIndentedString(totalRequests)).append("\n");
    sb.append("    totalStorageUsed: ").append(toIndentedString(totalStorageUsed)).append("\n");
    sb.append("    averageResponseTime: ").append(toIndentedString(averageResponseTime)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
    sb.append("    updatedAt: ").append(toIndentedString(updatedAt)).append("\n");
    sb.append("    deletedAt: ").append(toIndentedString(deletedAt)).append("\n");
    sb.append("    status: ").append(toIndentedString(status)).append("\n");
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

