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
 * APIKey represents an authentication token for accessing the API. It includes features for rate limiting, permissions, and usage tracking.  Key features: - Unique key identification - Scope-based access control - Rate limiting - Usage tracking - Expiration management - Multi-tenant support  Database considerations: - Uses GORM for ORM mapping - Includes indexes for efficient querying - Supports soft deletes  Usage example: &#x60;&#x60;&#x60;go apiKey :&#x3D; &amp;APIKey{     Name: \&quot;Production API Key\&quot;,     Scopes: []string{\&quot;leads:read\&quot;, \&quot;leads:write\&quot;},     ExpiresAt: timestamppb.New(time.Now().AddDate(1, 0, 0)), } &#x60;&#x60;&#x60;
 */

@Schema(name = "APIKey", description = "APIKey represents an authentication token for accessing the API. It includes features for rate limiting, permissions, and usage tracking.  Key features: - Unique key identification - Scope-based access control - Rate limiting - Usage tracking - Expiration management - Multi-tenant support  Database considerations: - Uses GORM for ORM mapping - Includes indexes for efficient querying - Supports soft deletes  Usage example: ```go apiKey := &APIKey{     Name: \"Production API Key\",     Scopes: []string{\"leads:read\", \"leads:write\"},     ExpiresAt: timestamppb.New(time.Now().AddDate(1, 0, 0)), } ```")
@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-08T10:15:33.789499-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class APIKey {

  private String id;

  private String name;

  private String keyHash;

  private String keyPrefix;

  private String orgId;

  private String tenantId;

  @Valid
  private List<String> scopes = new ArrayList<>();

  @Valid
  private List<String> allowedIps = new ArrayList<>();

  @Valid
  private List<String> allowedDomains = new ArrayList<>();

  @Valid
  private List<String> allowedEnvironments = new ArrayList<>();

  private Boolean isTestKey;

  private Integer requestsPerSecond;

  private Integer requestsPerDay;

  private Integer concurrentRequests;

  private String monthlyRequestQuota;

  private Float costPerRequest;

  private String billingTier;

  private String totalRequests;

  private String totalErrors;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime lastUsedAt;

  private Float averageResponseTime;

  private byte[] endpointUsageJson;

  private byte[] errorRatesJson;

  private byte[] recentErrors;

  private Integer successfulRequestsCount;

  private Float successRate;

  private V1Status status = V1Status.UNSPECIFIED;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime expiresAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime deletedAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime lastRotatedAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime lastSecurityReviewAt;

  private Boolean requiresClientSecret;

  private String clientSecretHash;

  private Boolean enforceHttps;

  private Boolean enforceSigning;

  @Valid
  private List<String> allowedSignatureAlgorithms = new ArrayList<>();

  private Boolean enforceMutualTls;

  private String clientCertificateHash;

  private Boolean requireRequestSigning;

  private String description;

  private byte[] metadataJson;

  @Valid
  private List<String> tags = new ArrayList<>();

  private String apiVersion;

  @Valid
  private List<String> supportedFeatures = new ArrayList<>();

  private String documentationUrl;

  private String supportContact;

  private Boolean logAllRequests;

  private String lastRotationReason;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime lastRotationDate;

  private Integer rotationFrequencyDays;

  @Valid
  private List<String> complianceStandards = new ArrayList<>();

  private Boolean requiresAuditLogging;

  private String dataResidency;

  @Valid
  private List<String> approvedIntegrations = new ArrayList<>();

  @Valid
  private List<String> alertEmails = new ArrayList<>();

  private String webhookUrl;

  private Boolean alertOnQuotaThreshold;

  private Float quotaAlertThreshold;

  private Boolean alertOnErrorSpike;

  private Float errorAlertThreshold;

  @Valid
  private List<String> monitoringIntegrations = new ArrayList<>();

  private Boolean encrypted;

  private String dataClassification;

  public APIKey id(String id) {
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

  public APIKey name(String name) {
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

  public APIKey keyHash(String keyHash) {
    this.keyHash = keyHash;
    return this;
  }

  /**
   * Get keyHash
   * @return keyHash
   */
  
  @Schema(name = "keyHash", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("keyHash")
  public String getKeyHash() {
    return keyHash;
  }

  public void setKeyHash(String keyHash) {
    this.keyHash = keyHash;
  }

  public APIKey keyPrefix(String keyPrefix) {
    this.keyPrefix = keyPrefix;
    return this;
  }

  /**
   * Get keyPrefix
   * @return keyPrefix
   */
  
  @Schema(name = "keyPrefix", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("keyPrefix")
  public String getKeyPrefix() {
    return keyPrefix;
  }

  public void setKeyPrefix(String keyPrefix) {
    this.keyPrefix = keyPrefix;
  }

  public APIKey orgId(String orgId) {
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

  public APIKey tenantId(String tenantId) {
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

  public APIKey scopes(List<String> scopes) {
    this.scopes = scopes;
    return this;
  }

  public APIKey addScopesItem(String scopesItem) {
    if (this.scopes == null) {
      this.scopes = new ArrayList<>();
    }
    this.scopes.add(scopesItem);
    return this;
  }

  /**
   * Get scopes
   * @return scopes
   */
  
  @Schema(name = "scopes", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("scopes")
  public List<String> getScopes() {
    return scopes;
  }

  public void setScopes(List<String> scopes) {
    this.scopes = scopes;
  }

  public APIKey allowedIps(List<String> allowedIps) {
    this.allowedIps = allowedIps;
    return this;
  }

  public APIKey addAllowedIpsItem(String allowedIpsItem) {
    if (this.allowedIps == null) {
      this.allowedIps = new ArrayList<>();
    }
    this.allowedIps.add(allowedIpsItem);
    return this;
  }

  /**
   * Get allowedIps
   * @return allowedIps
   */
  
  @Schema(name = "allowedIps", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("allowedIps")
  public List<String> getAllowedIps() {
    return allowedIps;
  }

  public void setAllowedIps(List<String> allowedIps) {
    this.allowedIps = allowedIps;
  }

  public APIKey allowedDomains(List<String> allowedDomains) {
    this.allowedDomains = allowedDomains;
    return this;
  }

  public APIKey addAllowedDomainsItem(String allowedDomainsItem) {
    if (this.allowedDomains == null) {
      this.allowedDomains = new ArrayList<>();
    }
    this.allowedDomains.add(allowedDomainsItem);
    return this;
  }

  /**
   * Get allowedDomains
   * @return allowedDomains
   */
  
  @Schema(name = "allowedDomains", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("allowedDomains")
  public List<String> getAllowedDomains() {
    return allowedDomains;
  }

  public void setAllowedDomains(List<String> allowedDomains) {
    this.allowedDomains = allowedDomains;
  }

  public APIKey allowedEnvironments(List<String> allowedEnvironments) {
    this.allowedEnvironments = allowedEnvironments;
    return this;
  }

  public APIKey addAllowedEnvironmentsItem(String allowedEnvironmentsItem) {
    if (this.allowedEnvironments == null) {
      this.allowedEnvironments = new ArrayList<>();
    }
    this.allowedEnvironments.add(allowedEnvironmentsItem);
    return this;
  }

  /**
   * Get allowedEnvironments
   * @return allowedEnvironments
   */
  
  @Schema(name = "allowedEnvironments", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("allowedEnvironments")
  public List<String> getAllowedEnvironments() {
    return allowedEnvironments;
  }

  public void setAllowedEnvironments(List<String> allowedEnvironments) {
    this.allowedEnvironments = allowedEnvironments;
  }

  public APIKey isTestKey(Boolean isTestKey) {
    this.isTestKey = isTestKey;
    return this;
  }

  /**
   * Get isTestKey
   * @return isTestKey
   */
  
  @Schema(name = "isTestKey", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("isTestKey")
  public Boolean getIsTestKey() {
    return isTestKey;
  }

  public void setIsTestKey(Boolean isTestKey) {
    this.isTestKey = isTestKey;
  }

  public APIKey requestsPerSecond(Integer requestsPerSecond) {
    this.requestsPerSecond = requestsPerSecond;
    return this;
  }

  /**
   * Get requestsPerSecond
   * @return requestsPerSecond
   */
  
  @Schema(name = "requestsPerSecond", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("requestsPerSecond")
  public Integer getRequestsPerSecond() {
    return requestsPerSecond;
  }

  public void setRequestsPerSecond(Integer requestsPerSecond) {
    this.requestsPerSecond = requestsPerSecond;
  }

  public APIKey requestsPerDay(Integer requestsPerDay) {
    this.requestsPerDay = requestsPerDay;
    return this;
  }

  /**
   * Get requestsPerDay
   * @return requestsPerDay
   */
  
  @Schema(name = "requestsPerDay", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("requestsPerDay")
  public Integer getRequestsPerDay() {
    return requestsPerDay;
  }

  public void setRequestsPerDay(Integer requestsPerDay) {
    this.requestsPerDay = requestsPerDay;
  }

  public APIKey concurrentRequests(Integer concurrentRequests) {
    this.concurrentRequests = concurrentRequests;
    return this;
  }

  /**
   * Get concurrentRequests
   * @return concurrentRequests
   */
  
  @Schema(name = "concurrentRequests", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("concurrentRequests")
  public Integer getConcurrentRequests() {
    return concurrentRequests;
  }

  public void setConcurrentRequests(Integer concurrentRequests) {
    this.concurrentRequests = concurrentRequests;
  }

  public APIKey monthlyRequestQuota(String monthlyRequestQuota) {
    this.monthlyRequestQuota = monthlyRequestQuota;
    return this;
  }

  /**
   * Get monthlyRequestQuota
   * @return monthlyRequestQuota
   */
  
  @Schema(name = "monthlyRequestQuota", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("monthlyRequestQuota")
  public String getMonthlyRequestQuota() {
    return monthlyRequestQuota;
  }

  public void setMonthlyRequestQuota(String monthlyRequestQuota) {
    this.monthlyRequestQuota = monthlyRequestQuota;
  }

  public APIKey costPerRequest(Float costPerRequest) {
    this.costPerRequest = costPerRequest;
    return this;
  }

  /**
   * Get costPerRequest
   * @return costPerRequest
   */
  
  @Schema(name = "costPerRequest", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("costPerRequest")
  public Float getCostPerRequest() {
    return costPerRequest;
  }

  public void setCostPerRequest(Float costPerRequest) {
    this.costPerRequest = costPerRequest;
  }

  public APIKey billingTier(String billingTier) {
    this.billingTier = billingTier;
    return this;
  }

  /**
   * Get billingTier
   * @return billingTier
   */
  
  @Schema(name = "billingTier", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("billingTier")
  public String getBillingTier() {
    return billingTier;
  }

  public void setBillingTier(String billingTier) {
    this.billingTier = billingTier;
  }

  public APIKey totalRequests(String totalRequests) {
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

  public APIKey totalErrors(String totalErrors) {
    this.totalErrors = totalErrors;
    return this;
  }

  /**
   * Get totalErrors
   * @return totalErrors
   */
  
  @Schema(name = "totalErrors", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("totalErrors")
  public String getTotalErrors() {
    return totalErrors;
  }

  public void setTotalErrors(String totalErrors) {
    this.totalErrors = totalErrors;
  }

  public APIKey lastUsedAt(OffsetDateTime lastUsedAt) {
    this.lastUsedAt = lastUsedAt;
    return this;
  }

  /**
   * Get lastUsedAt
   * @return lastUsedAt
   */
  @Valid 
  @Schema(name = "lastUsedAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("lastUsedAt")
  public OffsetDateTime getLastUsedAt() {
    return lastUsedAt;
  }

  public void setLastUsedAt(OffsetDateTime lastUsedAt) {
    this.lastUsedAt = lastUsedAt;
  }

  public APIKey averageResponseTime(Float averageResponseTime) {
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

  public APIKey endpointUsageJson(byte[] endpointUsageJson) {
    this.endpointUsageJson = endpointUsageJson;
    return this;
  }

  /**
   * Tracks usage per endpoint
   * @return endpointUsageJson
   */
  
  @Schema(name = "endpointUsageJson", description = "Tracks usage per endpoint", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("endpointUsageJson")
  public byte[] getEndpointUsageJson() {
    return endpointUsageJson;
  }

  public void setEndpointUsageJson(byte[] endpointUsageJson) {
    this.endpointUsageJson = endpointUsageJson;
  }

  public APIKey errorRatesJson(byte[] errorRatesJson) {
    this.errorRatesJson = errorRatesJson;
    return this;
  }

  /**
   * Get errorRatesJson
   * @return errorRatesJson
   */
  
  @Schema(name = "errorRatesJson", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("errorRatesJson")
  public byte[] getErrorRatesJson() {
    return errorRatesJson;
  }

  public void setErrorRatesJson(byte[] errorRatesJson) {
    this.errorRatesJson = errorRatesJson;
  }

  public APIKey recentErrors(byte[] recentErrors) {
    this.recentErrors = recentErrors;
    return this;
  }

  /**
   * Get recentErrors
   * @return recentErrors
   */
  
  @Schema(name = "recentErrors", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("recentErrors")
  public byte[] getRecentErrors() {
    return recentErrors;
  }

  public void setRecentErrors(byte[] recentErrors) {
    this.recentErrors = recentErrors;
  }

  public APIKey successfulRequestsCount(Integer successfulRequestsCount) {
    this.successfulRequestsCount = successfulRequestsCount;
    return this;
  }

  /**
   * Get successfulRequestsCount
   * @return successfulRequestsCount
   */
  
  @Schema(name = "successfulRequestsCount", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("successfulRequestsCount")
  public Integer getSuccessfulRequestsCount() {
    return successfulRequestsCount;
  }

  public void setSuccessfulRequestsCount(Integer successfulRequestsCount) {
    this.successfulRequestsCount = successfulRequestsCount;
  }

  public APIKey successRate(Float successRate) {
    this.successRate = successRate;
    return this;
  }

  /**
   * Get successRate
   * @return successRate
   */
  
  @Schema(name = "successRate", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("successRate")
  public Float getSuccessRate() {
    return successRate;
  }

  public void setSuccessRate(Float successRate) {
    this.successRate = successRate;
  }

  public APIKey status(V1Status status) {
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

  public APIKey createdAt(OffsetDateTime createdAt) {
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

  public APIKey updatedAt(OffsetDateTime updatedAt) {
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

  public APIKey expiresAt(OffsetDateTime expiresAt) {
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

  public APIKey deletedAt(OffsetDateTime deletedAt) {
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

  public APIKey lastRotatedAt(OffsetDateTime lastRotatedAt) {
    this.lastRotatedAt = lastRotatedAt;
    return this;
  }

  /**
   * Get lastRotatedAt
   * @return lastRotatedAt
   */
  @Valid 
  @Schema(name = "lastRotatedAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("lastRotatedAt")
  public OffsetDateTime getLastRotatedAt() {
    return lastRotatedAt;
  }

  public void setLastRotatedAt(OffsetDateTime lastRotatedAt) {
    this.lastRotatedAt = lastRotatedAt;
  }

  public APIKey lastSecurityReviewAt(OffsetDateTime lastSecurityReviewAt) {
    this.lastSecurityReviewAt = lastSecurityReviewAt;
    return this;
  }

  /**
   * Get lastSecurityReviewAt
   * @return lastSecurityReviewAt
   */
  @Valid 
  @Schema(name = "lastSecurityReviewAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("lastSecurityReviewAt")
  public OffsetDateTime getLastSecurityReviewAt() {
    return lastSecurityReviewAt;
  }

  public void setLastSecurityReviewAt(OffsetDateTime lastSecurityReviewAt) {
    this.lastSecurityReviewAt = lastSecurityReviewAt;
  }

  public APIKey requiresClientSecret(Boolean requiresClientSecret) {
    this.requiresClientSecret = requiresClientSecret;
    return this;
  }

  /**
   * Get requiresClientSecret
   * @return requiresClientSecret
   */
  
  @Schema(name = "requiresClientSecret", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("requiresClientSecret")
  public Boolean getRequiresClientSecret() {
    return requiresClientSecret;
  }

  public void setRequiresClientSecret(Boolean requiresClientSecret) {
    this.requiresClientSecret = requiresClientSecret;
  }

  public APIKey clientSecretHash(String clientSecretHash) {
    this.clientSecretHash = clientSecretHash;
    return this;
  }

  /**
   * Get clientSecretHash
   * @return clientSecretHash
   */
  
  @Schema(name = "clientSecretHash", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("clientSecretHash")
  public String getClientSecretHash() {
    return clientSecretHash;
  }

  public void setClientSecretHash(String clientSecretHash) {
    this.clientSecretHash = clientSecretHash;
  }

  public APIKey enforceHttps(Boolean enforceHttps) {
    this.enforceHttps = enforceHttps;
    return this;
  }

  /**
   * Get enforceHttps
   * @return enforceHttps
   */
  
  @Schema(name = "enforceHttps", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("enforceHttps")
  public Boolean getEnforceHttps() {
    return enforceHttps;
  }

  public void setEnforceHttps(Boolean enforceHttps) {
    this.enforceHttps = enforceHttps;
  }

  public APIKey enforceSigning(Boolean enforceSigning) {
    this.enforceSigning = enforceSigning;
    return this;
  }

  /**
   * Get enforceSigning
   * @return enforceSigning
   */
  
  @Schema(name = "enforceSigning", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("enforceSigning")
  public Boolean getEnforceSigning() {
    return enforceSigning;
  }

  public void setEnforceSigning(Boolean enforceSigning) {
    this.enforceSigning = enforceSigning;
  }

  public APIKey allowedSignatureAlgorithms(List<String> allowedSignatureAlgorithms) {
    this.allowedSignatureAlgorithms = allowedSignatureAlgorithms;
    return this;
  }

  public APIKey addAllowedSignatureAlgorithmsItem(String allowedSignatureAlgorithmsItem) {
    if (this.allowedSignatureAlgorithms == null) {
      this.allowedSignatureAlgorithms = new ArrayList<>();
    }
    this.allowedSignatureAlgorithms.add(allowedSignatureAlgorithmsItem);
    return this;
  }

  /**
   * Get allowedSignatureAlgorithms
   * @return allowedSignatureAlgorithms
   */
  
  @Schema(name = "allowedSignatureAlgorithms", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("allowedSignatureAlgorithms")
  public List<String> getAllowedSignatureAlgorithms() {
    return allowedSignatureAlgorithms;
  }

  public void setAllowedSignatureAlgorithms(List<String> allowedSignatureAlgorithms) {
    this.allowedSignatureAlgorithms = allowedSignatureAlgorithms;
  }

  public APIKey enforceMutualTls(Boolean enforceMutualTls) {
    this.enforceMutualTls = enforceMutualTls;
    return this;
  }

  /**
   * Get enforceMutualTls
   * @return enforceMutualTls
   */
  
  @Schema(name = "enforceMutualTls", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("enforceMutualTls")
  public Boolean getEnforceMutualTls() {
    return enforceMutualTls;
  }

  public void setEnforceMutualTls(Boolean enforceMutualTls) {
    this.enforceMutualTls = enforceMutualTls;
  }

  public APIKey clientCertificateHash(String clientCertificateHash) {
    this.clientCertificateHash = clientCertificateHash;
    return this;
  }

  /**
   * Get clientCertificateHash
   * @return clientCertificateHash
   */
  
  @Schema(name = "clientCertificateHash", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("clientCertificateHash")
  public String getClientCertificateHash() {
    return clientCertificateHash;
  }

  public void setClientCertificateHash(String clientCertificateHash) {
    this.clientCertificateHash = clientCertificateHash;
  }

  public APIKey requireRequestSigning(Boolean requireRequestSigning) {
    this.requireRequestSigning = requireRequestSigning;
    return this;
  }

  /**
   * Get requireRequestSigning
   * @return requireRequestSigning
   */
  
  @Schema(name = "requireRequestSigning", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("requireRequestSigning")
  public Boolean getRequireRequestSigning() {
    return requireRequestSigning;
  }

  public void setRequireRequestSigning(Boolean requireRequestSigning) {
    this.requireRequestSigning = requireRequestSigning;
  }

  public APIKey description(String description) {
    this.description = description;
    return this;
  }

  /**
   * Detailed description of key usage
   * @return description
   */
  
  @Schema(name = "description", description = "Detailed description of key usage", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("description")
  public String getDescription() {
    return description;
  }

  public void setDescription(String description) {
    this.description = description;
  }

  public APIKey metadataJson(byte[] metadataJson) {
    this.metadataJson = metadataJson;
    return this;
  }

  /**
   * Get metadataJson
   * @return metadataJson
   */
  
  @Schema(name = "metadataJson", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("metadataJson")
  public byte[] getMetadataJson() {
    return metadataJson;
  }

  public void setMetadataJson(byte[] metadataJson) {
    this.metadataJson = metadataJson;
  }

  public APIKey tags(List<String> tags) {
    this.tags = tags;
    return this;
  }

  public APIKey addTagsItem(String tagsItem) {
    if (this.tags == null) {
      this.tags = new ArrayList<>();
    }
    this.tags.add(tagsItem);
    return this;
  }

  /**
   * Get tags
   * @return tags
   */
  
  @Schema(name = "tags", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("tags")
  public List<String> getTags() {
    return tags;
  }

  public void setTags(List<String> tags) {
    this.tags = tags;
  }

  public APIKey apiVersion(String apiVersion) {
    this.apiVersion = apiVersion;
    return this;
  }

  /**
   * Get apiVersion
   * @return apiVersion
   */
  
  @Schema(name = "apiVersion", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("apiVersion")
  public String getApiVersion() {
    return apiVersion;
  }

  public void setApiVersion(String apiVersion) {
    this.apiVersion = apiVersion;
  }

  public APIKey supportedFeatures(List<String> supportedFeatures) {
    this.supportedFeatures = supportedFeatures;
    return this;
  }

  public APIKey addSupportedFeaturesItem(String supportedFeaturesItem) {
    if (this.supportedFeatures == null) {
      this.supportedFeatures = new ArrayList<>();
    }
    this.supportedFeatures.add(supportedFeaturesItem);
    return this;
  }

  /**
   * Get supportedFeatures
   * @return supportedFeatures
   */
  
  @Schema(name = "supportedFeatures", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("supportedFeatures")
  public List<String> getSupportedFeatures() {
    return supportedFeatures;
  }

  public void setSupportedFeatures(List<String> supportedFeatures) {
    this.supportedFeatures = supportedFeatures;
  }

  public APIKey documentationUrl(String documentationUrl) {
    this.documentationUrl = documentationUrl;
    return this;
  }

  /**
   * Get documentationUrl
   * @return documentationUrl
   */
  
  @Schema(name = "documentationUrl", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("documentationUrl")
  public String getDocumentationUrl() {
    return documentationUrl;
  }

  public void setDocumentationUrl(String documentationUrl) {
    this.documentationUrl = documentationUrl;
  }

  public APIKey supportContact(String supportContact) {
    this.supportContact = supportContact;
    return this;
  }

  /**
   * Get supportContact
   * @return supportContact
   */
  
  @Schema(name = "supportContact", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("supportContact")
  public String getSupportContact() {
    return supportContact;
  }

  public void setSupportContact(String supportContact) {
    this.supportContact = supportContact;
  }

  public APIKey logAllRequests(Boolean logAllRequests) {
    this.logAllRequests = logAllRequests;
    return this;
  }

  /**
   * Get logAllRequests
   * @return logAllRequests
   */
  
  @Schema(name = "logAllRequests", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("logAllRequests")
  public Boolean getLogAllRequests() {
    return logAllRequests;
  }

  public void setLogAllRequests(Boolean logAllRequests) {
    this.logAllRequests = logAllRequests;
  }

  public APIKey lastRotationReason(String lastRotationReason) {
    this.lastRotationReason = lastRotationReason;
    return this;
  }

  /**
   * Get lastRotationReason
   * @return lastRotationReason
   */
  
  @Schema(name = "lastRotationReason", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("lastRotationReason")
  public String getLastRotationReason() {
    return lastRotationReason;
  }

  public void setLastRotationReason(String lastRotationReason) {
    this.lastRotationReason = lastRotationReason;
  }

  public APIKey lastRotationDate(OffsetDateTime lastRotationDate) {
    this.lastRotationDate = lastRotationDate;
    return this;
  }

  /**
   * Get lastRotationDate
   * @return lastRotationDate
   */
  @Valid 
  @Schema(name = "lastRotationDate", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("lastRotationDate")
  public OffsetDateTime getLastRotationDate() {
    return lastRotationDate;
  }

  public void setLastRotationDate(OffsetDateTime lastRotationDate) {
    this.lastRotationDate = lastRotationDate;
  }

  public APIKey rotationFrequencyDays(Integer rotationFrequencyDays) {
    this.rotationFrequencyDays = rotationFrequencyDays;
    return this;
  }

  /**
   * Get rotationFrequencyDays
   * @return rotationFrequencyDays
   */
  
  @Schema(name = "rotationFrequencyDays", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("rotationFrequencyDays")
  public Integer getRotationFrequencyDays() {
    return rotationFrequencyDays;
  }

  public void setRotationFrequencyDays(Integer rotationFrequencyDays) {
    this.rotationFrequencyDays = rotationFrequencyDays;
  }

  public APIKey complianceStandards(List<String> complianceStandards) {
    this.complianceStandards = complianceStandards;
    return this;
  }

  public APIKey addComplianceStandardsItem(String complianceStandardsItem) {
    if (this.complianceStandards == null) {
      this.complianceStandards = new ArrayList<>();
    }
    this.complianceStandards.add(complianceStandardsItem);
    return this;
  }

  /**
   * Get complianceStandards
   * @return complianceStandards
   */
  
  @Schema(name = "complianceStandards", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("complianceStandards")
  public List<String> getComplianceStandards() {
    return complianceStandards;
  }

  public void setComplianceStandards(List<String> complianceStandards) {
    this.complianceStandards = complianceStandards;
  }

  public APIKey requiresAuditLogging(Boolean requiresAuditLogging) {
    this.requiresAuditLogging = requiresAuditLogging;
    return this;
  }

  /**
   * Get requiresAuditLogging
   * @return requiresAuditLogging
   */
  
  @Schema(name = "requiresAuditLogging", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("requiresAuditLogging")
  public Boolean getRequiresAuditLogging() {
    return requiresAuditLogging;
  }

  public void setRequiresAuditLogging(Boolean requiresAuditLogging) {
    this.requiresAuditLogging = requiresAuditLogging;
  }

  public APIKey dataResidency(String dataResidency) {
    this.dataResidency = dataResidency;
    return this;
  }

  /**
   * Get dataResidency
   * @return dataResidency
   */
  
  @Schema(name = "dataResidency", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("dataResidency")
  public String getDataResidency() {
    return dataResidency;
  }

  public void setDataResidency(String dataResidency) {
    this.dataResidency = dataResidency;
  }

  public APIKey approvedIntegrations(List<String> approvedIntegrations) {
    this.approvedIntegrations = approvedIntegrations;
    return this;
  }

  public APIKey addApprovedIntegrationsItem(String approvedIntegrationsItem) {
    if (this.approvedIntegrations == null) {
      this.approvedIntegrations = new ArrayList<>();
    }
    this.approvedIntegrations.add(approvedIntegrationsItem);
    return this;
  }

  /**
   * Get approvedIntegrations
   * @return approvedIntegrations
   */
  
  @Schema(name = "approvedIntegrations", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("approvedIntegrations")
  public List<String> getApprovedIntegrations() {
    return approvedIntegrations;
  }

  public void setApprovedIntegrations(List<String> approvedIntegrations) {
    this.approvedIntegrations = approvedIntegrations;
  }

  public APIKey alertEmails(List<String> alertEmails) {
    this.alertEmails = alertEmails;
    return this;
  }

  public APIKey addAlertEmailsItem(String alertEmailsItem) {
    if (this.alertEmails == null) {
      this.alertEmails = new ArrayList<>();
    }
    this.alertEmails.add(alertEmailsItem);
    return this;
  }

  /**
   * Get alertEmails
   * @return alertEmails
   */
  
  @Schema(name = "alertEmails", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("alertEmails")
  public List<String> getAlertEmails() {
    return alertEmails;
  }

  public void setAlertEmails(List<String> alertEmails) {
    this.alertEmails = alertEmails;
  }

  public APIKey webhookUrl(String webhookUrl) {
    this.webhookUrl = webhookUrl;
    return this;
  }

  /**
   * Get webhookUrl
   * @return webhookUrl
   */
  
  @Schema(name = "webhookUrl", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("webhookUrl")
  public String getWebhookUrl() {
    return webhookUrl;
  }

  public void setWebhookUrl(String webhookUrl) {
    this.webhookUrl = webhookUrl;
  }

  public APIKey alertOnQuotaThreshold(Boolean alertOnQuotaThreshold) {
    this.alertOnQuotaThreshold = alertOnQuotaThreshold;
    return this;
  }

  /**
   * Get alertOnQuotaThreshold
   * @return alertOnQuotaThreshold
   */
  
  @Schema(name = "alertOnQuotaThreshold", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("alertOnQuotaThreshold")
  public Boolean getAlertOnQuotaThreshold() {
    return alertOnQuotaThreshold;
  }

  public void setAlertOnQuotaThreshold(Boolean alertOnQuotaThreshold) {
    this.alertOnQuotaThreshold = alertOnQuotaThreshold;
  }

  public APIKey quotaAlertThreshold(Float quotaAlertThreshold) {
    this.quotaAlertThreshold = quotaAlertThreshold;
    return this;
  }

  /**
   * Get quotaAlertThreshold
   * @return quotaAlertThreshold
   */
  
  @Schema(name = "quotaAlertThreshold", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("quotaAlertThreshold")
  public Float getQuotaAlertThreshold() {
    return quotaAlertThreshold;
  }

  public void setQuotaAlertThreshold(Float quotaAlertThreshold) {
    this.quotaAlertThreshold = quotaAlertThreshold;
  }

  public APIKey alertOnErrorSpike(Boolean alertOnErrorSpike) {
    this.alertOnErrorSpike = alertOnErrorSpike;
    return this;
  }

  /**
   * Get alertOnErrorSpike
   * @return alertOnErrorSpike
   */
  
  @Schema(name = "alertOnErrorSpike", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("alertOnErrorSpike")
  public Boolean getAlertOnErrorSpike() {
    return alertOnErrorSpike;
  }

  public void setAlertOnErrorSpike(Boolean alertOnErrorSpike) {
    this.alertOnErrorSpike = alertOnErrorSpike;
  }

  public APIKey errorAlertThreshold(Float errorAlertThreshold) {
    this.errorAlertThreshold = errorAlertThreshold;
    return this;
  }

  /**
   * Get errorAlertThreshold
   * @return errorAlertThreshold
   */
  
  @Schema(name = "errorAlertThreshold", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("errorAlertThreshold")
  public Float getErrorAlertThreshold() {
    return errorAlertThreshold;
  }

  public void setErrorAlertThreshold(Float errorAlertThreshold) {
    this.errorAlertThreshold = errorAlertThreshold;
  }

  public APIKey monitoringIntegrations(List<String> monitoringIntegrations) {
    this.monitoringIntegrations = monitoringIntegrations;
    return this;
  }

  public APIKey addMonitoringIntegrationsItem(String monitoringIntegrationsItem) {
    if (this.monitoringIntegrations == null) {
      this.monitoringIntegrations = new ArrayList<>();
    }
    this.monitoringIntegrations.add(monitoringIntegrationsItem);
    return this;
  }

  /**
   * Get monitoringIntegrations
   * @return monitoringIntegrations
   */
  
  @Schema(name = "monitoringIntegrations", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("monitoringIntegrations")
  public List<String> getMonitoringIntegrations() {
    return monitoringIntegrations;
  }

  public void setMonitoringIntegrations(List<String> monitoringIntegrations) {
    this.monitoringIntegrations = monitoringIntegrations;
  }

  public APIKey encrypted(Boolean encrypted) {
    this.encrypted = encrypted;
    return this;
  }

  /**
   * Get encrypted
   * @return encrypted
   */
  
  @Schema(name = "encrypted", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("encrypted")
  public Boolean getEncrypted() {
    return encrypted;
  }

  public void setEncrypted(Boolean encrypted) {
    this.encrypted = encrypted;
  }

  public APIKey dataClassification(String dataClassification) {
    this.dataClassification = dataClassification;
    return this;
  }

  /**
   * Get dataClassification
   * @return dataClassification
   */
  
  @Schema(name = "dataClassification", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("dataClassification")
  public String getDataClassification() {
    return dataClassification;
  }

  public void setDataClassification(String dataClassification) {
    this.dataClassification = dataClassification;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    APIKey apIKey = (APIKey) o;
    return Objects.equals(this.id, apIKey.id) &&
        Objects.equals(this.name, apIKey.name) &&
        Objects.equals(this.keyHash, apIKey.keyHash) &&
        Objects.equals(this.keyPrefix, apIKey.keyPrefix) &&
        Objects.equals(this.orgId, apIKey.orgId) &&
        Objects.equals(this.tenantId, apIKey.tenantId) &&
        Objects.equals(this.scopes, apIKey.scopes) &&
        Objects.equals(this.allowedIps, apIKey.allowedIps) &&
        Objects.equals(this.allowedDomains, apIKey.allowedDomains) &&
        Objects.equals(this.allowedEnvironments, apIKey.allowedEnvironments) &&
        Objects.equals(this.isTestKey, apIKey.isTestKey) &&
        Objects.equals(this.requestsPerSecond, apIKey.requestsPerSecond) &&
        Objects.equals(this.requestsPerDay, apIKey.requestsPerDay) &&
        Objects.equals(this.concurrentRequests, apIKey.concurrentRequests) &&
        Objects.equals(this.monthlyRequestQuota, apIKey.monthlyRequestQuota) &&
        Objects.equals(this.costPerRequest, apIKey.costPerRequest) &&
        Objects.equals(this.billingTier, apIKey.billingTier) &&
        Objects.equals(this.totalRequests, apIKey.totalRequests) &&
        Objects.equals(this.totalErrors, apIKey.totalErrors) &&
        Objects.equals(this.lastUsedAt, apIKey.lastUsedAt) &&
        Objects.equals(this.averageResponseTime, apIKey.averageResponseTime) &&
        Arrays.equals(this.endpointUsageJson, apIKey.endpointUsageJson) &&
        Arrays.equals(this.errorRatesJson, apIKey.errorRatesJson) &&
        Arrays.equals(this.recentErrors, apIKey.recentErrors) &&
        Objects.equals(this.successfulRequestsCount, apIKey.successfulRequestsCount) &&
        Objects.equals(this.successRate, apIKey.successRate) &&
        Objects.equals(this.status, apIKey.status) &&
        Objects.equals(this.createdAt, apIKey.createdAt) &&
        Objects.equals(this.updatedAt, apIKey.updatedAt) &&
        Objects.equals(this.expiresAt, apIKey.expiresAt) &&
        Objects.equals(this.deletedAt, apIKey.deletedAt) &&
        Objects.equals(this.lastRotatedAt, apIKey.lastRotatedAt) &&
        Objects.equals(this.lastSecurityReviewAt, apIKey.lastSecurityReviewAt) &&
        Objects.equals(this.requiresClientSecret, apIKey.requiresClientSecret) &&
        Objects.equals(this.clientSecretHash, apIKey.clientSecretHash) &&
        Objects.equals(this.enforceHttps, apIKey.enforceHttps) &&
        Objects.equals(this.enforceSigning, apIKey.enforceSigning) &&
        Objects.equals(this.allowedSignatureAlgorithms, apIKey.allowedSignatureAlgorithms) &&
        Objects.equals(this.enforceMutualTls, apIKey.enforceMutualTls) &&
        Objects.equals(this.clientCertificateHash, apIKey.clientCertificateHash) &&
        Objects.equals(this.requireRequestSigning, apIKey.requireRequestSigning) &&
        Objects.equals(this.description, apIKey.description) &&
        Arrays.equals(this.metadataJson, apIKey.metadataJson) &&
        Objects.equals(this.tags, apIKey.tags) &&
        Objects.equals(this.apiVersion, apIKey.apiVersion) &&
        Objects.equals(this.supportedFeatures, apIKey.supportedFeatures) &&
        Objects.equals(this.documentationUrl, apIKey.documentationUrl) &&
        Objects.equals(this.supportContact, apIKey.supportContact) &&
        Objects.equals(this.logAllRequests, apIKey.logAllRequests) &&
        Objects.equals(this.lastRotationReason, apIKey.lastRotationReason) &&
        Objects.equals(this.lastRotationDate, apIKey.lastRotationDate) &&
        Objects.equals(this.rotationFrequencyDays, apIKey.rotationFrequencyDays) &&
        Objects.equals(this.complianceStandards, apIKey.complianceStandards) &&
        Objects.equals(this.requiresAuditLogging, apIKey.requiresAuditLogging) &&
        Objects.equals(this.dataResidency, apIKey.dataResidency) &&
        Objects.equals(this.approvedIntegrations, apIKey.approvedIntegrations) &&
        Objects.equals(this.alertEmails, apIKey.alertEmails) &&
        Objects.equals(this.webhookUrl, apIKey.webhookUrl) &&
        Objects.equals(this.alertOnQuotaThreshold, apIKey.alertOnQuotaThreshold) &&
        Objects.equals(this.quotaAlertThreshold, apIKey.quotaAlertThreshold) &&
        Objects.equals(this.alertOnErrorSpike, apIKey.alertOnErrorSpike) &&
        Objects.equals(this.errorAlertThreshold, apIKey.errorAlertThreshold) &&
        Objects.equals(this.monitoringIntegrations, apIKey.monitoringIntegrations) &&
        Objects.equals(this.encrypted, apIKey.encrypted) &&
        Objects.equals(this.dataClassification, apIKey.dataClassification);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, name, keyHash, keyPrefix, orgId, tenantId, scopes, allowedIps, allowedDomains, allowedEnvironments, isTestKey, requestsPerSecond, requestsPerDay, concurrentRequests, monthlyRequestQuota, costPerRequest, billingTier, totalRequests, totalErrors, lastUsedAt, averageResponseTime, Arrays.hashCode(endpointUsageJson), Arrays.hashCode(errorRatesJson), Arrays.hashCode(recentErrors), successfulRequestsCount, successRate, status, createdAt, updatedAt, expiresAt, deletedAt, lastRotatedAt, lastSecurityReviewAt, requiresClientSecret, clientSecretHash, enforceHttps, enforceSigning, allowedSignatureAlgorithms, enforceMutualTls, clientCertificateHash, requireRequestSigning, description, Arrays.hashCode(metadataJson), tags, apiVersion, supportedFeatures, documentationUrl, supportContact, logAllRequests, lastRotationReason, lastRotationDate, rotationFrequencyDays, complianceStandards, requiresAuditLogging, dataResidency, approvedIntegrations, alertEmails, webhookUrl, alertOnQuotaThreshold, quotaAlertThreshold, alertOnErrorSpike, errorAlertThreshold, monitoringIntegrations, encrypted, dataClassification);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class APIKey {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    name: ").append(toIndentedString(name)).append("\n");
    sb.append("    keyHash: ").append(toIndentedString(keyHash)).append("\n");
    sb.append("    keyPrefix: ").append(toIndentedString(keyPrefix)).append("\n");
    sb.append("    orgId: ").append(toIndentedString(orgId)).append("\n");
    sb.append("    tenantId: ").append(toIndentedString(tenantId)).append("\n");
    sb.append("    scopes: ").append(toIndentedString(scopes)).append("\n");
    sb.append("    allowedIps: ").append(toIndentedString(allowedIps)).append("\n");
    sb.append("    allowedDomains: ").append(toIndentedString(allowedDomains)).append("\n");
    sb.append("    allowedEnvironments: ").append(toIndentedString(allowedEnvironments)).append("\n");
    sb.append("    isTestKey: ").append(toIndentedString(isTestKey)).append("\n");
    sb.append("    requestsPerSecond: ").append(toIndentedString(requestsPerSecond)).append("\n");
    sb.append("    requestsPerDay: ").append(toIndentedString(requestsPerDay)).append("\n");
    sb.append("    concurrentRequests: ").append(toIndentedString(concurrentRequests)).append("\n");
    sb.append("    monthlyRequestQuota: ").append(toIndentedString(monthlyRequestQuota)).append("\n");
    sb.append("    costPerRequest: ").append(toIndentedString(costPerRequest)).append("\n");
    sb.append("    billingTier: ").append(toIndentedString(billingTier)).append("\n");
    sb.append("    totalRequests: ").append(toIndentedString(totalRequests)).append("\n");
    sb.append("    totalErrors: ").append(toIndentedString(totalErrors)).append("\n");
    sb.append("    lastUsedAt: ").append(toIndentedString(lastUsedAt)).append("\n");
    sb.append("    averageResponseTime: ").append(toIndentedString(averageResponseTime)).append("\n");
    sb.append("    endpointUsageJson: ").append(toIndentedString(endpointUsageJson)).append("\n");
    sb.append("    errorRatesJson: ").append(toIndentedString(errorRatesJson)).append("\n");
    sb.append("    recentErrors: ").append(toIndentedString(recentErrors)).append("\n");
    sb.append("    successfulRequestsCount: ").append(toIndentedString(successfulRequestsCount)).append("\n");
    sb.append("    successRate: ").append(toIndentedString(successRate)).append("\n");
    sb.append("    status: ").append(toIndentedString(status)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
    sb.append("    updatedAt: ").append(toIndentedString(updatedAt)).append("\n");
    sb.append("    expiresAt: ").append(toIndentedString(expiresAt)).append("\n");
    sb.append("    deletedAt: ").append(toIndentedString(deletedAt)).append("\n");
    sb.append("    lastRotatedAt: ").append(toIndentedString(lastRotatedAt)).append("\n");
    sb.append("    lastSecurityReviewAt: ").append(toIndentedString(lastSecurityReviewAt)).append("\n");
    sb.append("    requiresClientSecret: ").append(toIndentedString(requiresClientSecret)).append("\n");
    sb.append("    clientSecretHash: ").append(toIndentedString(clientSecretHash)).append("\n");
    sb.append("    enforceHttps: ").append(toIndentedString(enforceHttps)).append("\n");
    sb.append("    enforceSigning: ").append(toIndentedString(enforceSigning)).append("\n");
    sb.append("    allowedSignatureAlgorithms: ").append(toIndentedString(allowedSignatureAlgorithms)).append("\n");
    sb.append("    enforceMutualTls: ").append(toIndentedString(enforceMutualTls)).append("\n");
    sb.append("    clientCertificateHash: ").append(toIndentedString(clientCertificateHash)).append("\n");
    sb.append("    requireRequestSigning: ").append(toIndentedString(requireRequestSigning)).append("\n");
    sb.append("    description: ").append(toIndentedString(description)).append("\n");
    sb.append("    metadataJson: ").append(toIndentedString(metadataJson)).append("\n");
    sb.append("    tags: ").append(toIndentedString(tags)).append("\n");
    sb.append("    apiVersion: ").append(toIndentedString(apiVersion)).append("\n");
    sb.append("    supportedFeatures: ").append(toIndentedString(supportedFeatures)).append("\n");
    sb.append("    documentationUrl: ").append(toIndentedString(documentationUrl)).append("\n");
    sb.append("    supportContact: ").append(toIndentedString(supportContact)).append("\n");
    sb.append("    logAllRequests: ").append(toIndentedString(logAllRequests)).append("\n");
    sb.append("    lastRotationReason: ").append(toIndentedString(lastRotationReason)).append("\n");
    sb.append("    lastRotationDate: ").append(toIndentedString(lastRotationDate)).append("\n");
    sb.append("    rotationFrequencyDays: ").append(toIndentedString(rotationFrequencyDays)).append("\n");
    sb.append("    complianceStandards: ").append(toIndentedString(complianceStandards)).append("\n");
    sb.append("    requiresAuditLogging: ").append(toIndentedString(requiresAuditLogging)).append("\n");
    sb.append("    dataResidency: ").append(toIndentedString(dataResidency)).append("\n");
    sb.append("    approvedIntegrations: ").append(toIndentedString(approvedIntegrations)).append("\n");
    sb.append("    alertEmails: ").append(toIndentedString(alertEmails)).append("\n");
    sb.append("    webhookUrl: ").append(toIndentedString(webhookUrl)).append("\n");
    sb.append("    alertOnQuotaThreshold: ").append(toIndentedString(alertOnQuotaThreshold)).append("\n");
    sb.append("    quotaAlertThreshold: ").append(toIndentedString(quotaAlertThreshold)).append("\n");
    sb.append("    alertOnErrorSpike: ").append(toIndentedString(alertOnErrorSpike)).append("\n");
    sb.append("    errorAlertThreshold: ").append(toIndentedString(errorAlertThreshold)).append("\n");
    sb.append("    monitoringIntegrations: ").append(toIndentedString(monitoringIntegrations)).append("\n");
    sb.append("    encrypted: ").append(toIndentedString(encrypted)).append("\n");
    sb.append("    dataClassification: ").append(toIndentedString(dataClassification)).append("\n");
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

