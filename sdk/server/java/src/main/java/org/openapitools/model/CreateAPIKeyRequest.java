package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * CreateAPIKeyRequest
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-05T14:03:51.683325-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class CreateAPIKeyRequest {

  private String organizationId;

  private String workspaceId;

  private String tenantId;

  private String accountId;

  private String name;

  private String description;

  @Valid
  private List<String> scopes = new ArrayList<>();

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime expiresAt;

  private Integer maxUses;

  @Valid
  private List<String> allowedIps = new ArrayList<>();

  private Integer rateLimit;

  private Boolean enforceSigning;

  @Valid
  private List<String> allowedSignatureAlgorithms = new ArrayList<>();

  private Boolean enforceMutualTls;

  @Valid
  private List<String> alertEmails = new ArrayList<>();

  private Boolean alertOnQuotaThreshold;

  private Float quotaAlertThreshold;

  public CreateAPIKeyRequest organizationId(String organizationId) {
    this.organizationId = organizationId;
    return this;
  }

  /**
   * Get organizationId
   * @return organizationId
   */
  
  @Schema(name = "organizationId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("organizationId")
  public String getOrganizationId() {
    return organizationId;
  }

  public void setOrganizationId(String organizationId) {
    this.organizationId = organizationId;
  }

  public CreateAPIKeyRequest workspaceId(String workspaceId) {
    this.workspaceId = workspaceId;
    return this;
  }

  /**
   * Get workspaceId
   * @return workspaceId
   */
  
  @Schema(name = "workspaceId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("workspaceId")
  public String getWorkspaceId() {
    return workspaceId;
  }

  public void setWorkspaceId(String workspaceId) {
    this.workspaceId = workspaceId;
  }

  public CreateAPIKeyRequest tenantId(String tenantId) {
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

  public CreateAPIKeyRequest accountId(String accountId) {
    this.accountId = accountId;
    return this;
  }

  /**
   * Get accountId
   * @return accountId
   */
  
  @Schema(name = "accountId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("accountId")
  public String getAccountId() {
    return accountId;
  }

  public void setAccountId(String accountId) {
    this.accountId = accountId;
  }

  public CreateAPIKeyRequest name(String name) {
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

  public CreateAPIKeyRequest description(String description) {
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

  public CreateAPIKeyRequest scopes(List<String> scopes) {
    this.scopes = scopes;
    return this;
  }

  public CreateAPIKeyRequest addScopesItem(String scopesItem) {
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

  public CreateAPIKeyRequest expiresAt(OffsetDateTime expiresAt) {
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

  public CreateAPIKeyRequest maxUses(Integer maxUses) {
    this.maxUses = maxUses;
    return this;
  }

  /**
   * Get maxUses
   * @return maxUses
   */
  
  @Schema(name = "maxUses", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("maxUses")
  public Integer getMaxUses() {
    return maxUses;
  }

  public void setMaxUses(Integer maxUses) {
    this.maxUses = maxUses;
  }

  public CreateAPIKeyRequest allowedIps(List<String> allowedIps) {
    this.allowedIps = allowedIps;
    return this;
  }

  public CreateAPIKeyRequest addAllowedIpsItem(String allowedIpsItem) {
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

  public CreateAPIKeyRequest rateLimit(Integer rateLimit) {
    this.rateLimit = rateLimit;
    return this;
  }

  /**
   * Get rateLimit
   * @return rateLimit
   */
  
  @Schema(name = "rateLimit", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("rateLimit")
  public Integer getRateLimit() {
    return rateLimit;
  }

  public void setRateLimit(Integer rateLimit) {
    this.rateLimit = rateLimit;
  }

  public CreateAPIKeyRequest enforceSigning(Boolean enforceSigning) {
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

  public CreateAPIKeyRequest allowedSignatureAlgorithms(List<String> allowedSignatureAlgorithms) {
    this.allowedSignatureAlgorithms = allowedSignatureAlgorithms;
    return this;
  }

  public CreateAPIKeyRequest addAllowedSignatureAlgorithmsItem(String allowedSignatureAlgorithmsItem) {
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

  public CreateAPIKeyRequest enforceMutualTls(Boolean enforceMutualTls) {
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

  public CreateAPIKeyRequest alertEmails(List<String> alertEmails) {
    this.alertEmails = alertEmails;
    return this;
  }

  public CreateAPIKeyRequest addAlertEmailsItem(String alertEmailsItem) {
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

  public CreateAPIKeyRequest alertOnQuotaThreshold(Boolean alertOnQuotaThreshold) {
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

  public CreateAPIKeyRequest quotaAlertThreshold(Float quotaAlertThreshold) {
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

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    CreateAPIKeyRequest createAPIKeyRequest = (CreateAPIKeyRequest) o;
    return Objects.equals(this.organizationId, createAPIKeyRequest.organizationId) &&
        Objects.equals(this.workspaceId, createAPIKeyRequest.workspaceId) &&
        Objects.equals(this.tenantId, createAPIKeyRequest.tenantId) &&
        Objects.equals(this.accountId, createAPIKeyRequest.accountId) &&
        Objects.equals(this.name, createAPIKeyRequest.name) &&
        Objects.equals(this.description, createAPIKeyRequest.description) &&
        Objects.equals(this.scopes, createAPIKeyRequest.scopes) &&
        Objects.equals(this.expiresAt, createAPIKeyRequest.expiresAt) &&
        Objects.equals(this.maxUses, createAPIKeyRequest.maxUses) &&
        Objects.equals(this.allowedIps, createAPIKeyRequest.allowedIps) &&
        Objects.equals(this.rateLimit, createAPIKeyRequest.rateLimit) &&
        Objects.equals(this.enforceSigning, createAPIKeyRequest.enforceSigning) &&
        Objects.equals(this.allowedSignatureAlgorithms, createAPIKeyRequest.allowedSignatureAlgorithms) &&
        Objects.equals(this.enforceMutualTls, createAPIKeyRequest.enforceMutualTls) &&
        Objects.equals(this.alertEmails, createAPIKeyRequest.alertEmails) &&
        Objects.equals(this.alertOnQuotaThreshold, createAPIKeyRequest.alertOnQuotaThreshold) &&
        Objects.equals(this.quotaAlertThreshold, createAPIKeyRequest.quotaAlertThreshold);
  }

  @Override
  public int hashCode() {
    return Objects.hash(organizationId, workspaceId, tenantId, accountId, name, description, scopes, expiresAt, maxUses, allowedIps, rateLimit, enforceSigning, allowedSignatureAlgorithms, enforceMutualTls, alertEmails, alertOnQuotaThreshold, quotaAlertThreshold);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class CreateAPIKeyRequest {\n");
    sb.append("    organizationId: ").append(toIndentedString(organizationId)).append("\n");
    sb.append("    workspaceId: ").append(toIndentedString(workspaceId)).append("\n");
    sb.append("    tenantId: ").append(toIndentedString(tenantId)).append("\n");
    sb.append("    accountId: ").append(toIndentedString(accountId)).append("\n");
    sb.append("    name: ").append(toIndentedString(name)).append("\n");
    sb.append("    description: ").append(toIndentedString(description)).append("\n");
    sb.append("    scopes: ").append(toIndentedString(scopes)).append("\n");
    sb.append("    expiresAt: ").append(toIndentedString(expiresAt)).append("\n");
    sb.append("    maxUses: ").append(toIndentedString(maxUses)).append("\n");
    sb.append("    allowedIps: ").append(toIndentedString(allowedIps)).append("\n");
    sb.append("    rateLimit: ").append(toIndentedString(rateLimit)).append("\n");
    sb.append("    enforceSigning: ").append(toIndentedString(enforceSigning)).append("\n");
    sb.append("    allowedSignatureAlgorithms: ").append(toIndentedString(allowedSignatureAlgorithms)).append("\n");
    sb.append("    enforceMutualTls: ").append(toIndentedString(enforceMutualTls)).append("\n");
    sb.append("    alertEmails: ").append(toIndentedString(alertEmails)).append("\n");
    sb.append("    alertOnQuotaThreshold: ").append(toIndentedString(alertOnQuotaThreshold)).append("\n");
    sb.append("    quotaAlertThreshold: ").append(toIndentedString(quotaAlertThreshold)).append("\n");
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

