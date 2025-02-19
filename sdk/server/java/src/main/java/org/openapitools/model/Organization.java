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
import org.openapitools.model.BillingPlan;
import org.openapitools.model.Subscription;
import org.openapitools.model.Tenant;
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
 * Key features: - Multi-tenant support - Hierarchical structure - Enterprise features - Billing management  Use cases: - Company creates multiple apps/tenants - Enterprise manages business units - Usage tracking across tenants
 */

@Schema(name = "Organization", description = "Key features: - Multi-tenant support - Hierarchical structure - Enterprise features - Billing management  Use cases: - Company creates multiple apps/tenants - Enterprise manages business units - Usage tracking across tenants")
@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-19T02:14:00.124734-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class Organization {

  private String id;

  private String name;

  private String displayName;

  private String website;

  private String description;

  private String billingEmail;

  private String technicalEmail;

  private String phone;

  private String address;

  private Boolean ssoEnabled;

  private String ssoProvider;

  private String ssoDomain;

  @Valid
  private List<String> allowedDomains = new ArrayList<>();

  private Boolean enforce2fa;

  private BillingPlan billingPlan = BillingPlan.UNSPECIFIED;

  private String billingCurrency;

  private Boolean autoBilling;

  private String taxId;

  @Valid
  private List<String> complianceFrameworks = new ArrayList<>();

  private Boolean dataProcessingAgreement;

  private String dataRegion;

  private Integer maxTenants;

  private String totalStorageLimit;

  private Integer maxApiKeys;

  private Integer maxUsers;

  @Valid
  private List<@Valid Tenant> tenants = new ArrayList<>();

  private Subscription subscriptions;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime deletedAt;

  private String createdBy;

  private V1Status status = V1Status.UNSPECIFIED;

  public Organization id(String id) {
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

  public Organization name(String name) {
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

  public Organization displayName(String displayName) {
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

  public Organization website(String website) {
    this.website = website;
    return this;
  }

  /**
   * Get website
   * @return website
   */
  
  @Schema(name = "website", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("website")
  public String getWebsite() {
    return website;
  }

  public void setWebsite(String website) {
    this.website = website;
  }

  public Organization description(String description) {
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

  public Organization billingEmail(String billingEmail) {
    this.billingEmail = billingEmail;
    return this;
  }

  /**
   * Get billingEmail
   * @return billingEmail
   */
  
  @Schema(name = "billingEmail", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("billingEmail")
  public String getBillingEmail() {
    return billingEmail;
  }

  public void setBillingEmail(String billingEmail) {
    this.billingEmail = billingEmail;
  }

  public Organization technicalEmail(String technicalEmail) {
    this.technicalEmail = technicalEmail;
    return this;
  }

  /**
   * Get technicalEmail
   * @return technicalEmail
   */
  
  @Schema(name = "technicalEmail", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("technicalEmail")
  public String getTechnicalEmail() {
    return technicalEmail;
  }

  public void setTechnicalEmail(String technicalEmail) {
    this.technicalEmail = technicalEmail;
  }

  public Organization phone(String phone) {
    this.phone = phone;
    return this;
  }

  /**
   * Get phone
   * @return phone
   */
  
  @Schema(name = "phone", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("phone")
  public String getPhone() {
    return phone;
  }

  public void setPhone(String phone) {
    this.phone = phone;
  }

  public Organization address(String address) {
    this.address = address;
    return this;
  }

  /**
   * Get address
   * @return address
   */
  
  @Schema(name = "address", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("address")
  public String getAddress() {
    return address;
  }

  public void setAddress(String address) {
    this.address = address;
  }

  public Organization ssoEnabled(Boolean ssoEnabled) {
    this.ssoEnabled = ssoEnabled;
    return this;
  }

  /**
   * Get ssoEnabled
   * @return ssoEnabled
   */
  
  @Schema(name = "ssoEnabled", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("ssoEnabled")
  public Boolean getSsoEnabled() {
    return ssoEnabled;
  }

  public void setSsoEnabled(Boolean ssoEnabled) {
    this.ssoEnabled = ssoEnabled;
  }

  public Organization ssoProvider(String ssoProvider) {
    this.ssoProvider = ssoProvider;
    return this;
  }

  /**
   * Get ssoProvider
   * @return ssoProvider
   */
  
  @Schema(name = "ssoProvider", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("ssoProvider")
  public String getSsoProvider() {
    return ssoProvider;
  }

  public void setSsoProvider(String ssoProvider) {
    this.ssoProvider = ssoProvider;
  }

  public Organization ssoDomain(String ssoDomain) {
    this.ssoDomain = ssoDomain;
    return this;
  }

  /**
   * Get ssoDomain
   * @return ssoDomain
   */
  
  @Schema(name = "ssoDomain", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("ssoDomain")
  public String getSsoDomain() {
    return ssoDomain;
  }

  public void setSsoDomain(String ssoDomain) {
    this.ssoDomain = ssoDomain;
  }

  public Organization allowedDomains(List<String> allowedDomains) {
    this.allowedDomains = allowedDomains;
    return this;
  }

  public Organization addAllowedDomainsItem(String allowedDomainsItem) {
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

  public Organization enforce2fa(Boolean enforce2fa) {
    this.enforce2fa = enforce2fa;
    return this;
  }

  /**
   * Get enforce2fa
   * @return enforce2fa
   */
  
  @Schema(name = "enforce2fa", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("enforce2fa")
  public Boolean getEnforce2fa() {
    return enforce2fa;
  }

  public void setEnforce2fa(Boolean enforce2fa) {
    this.enforce2fa = enforce2fa;
  }

  public Organization billingPlan(BillingPlan billingPlan) {
    this.billingPlan = billingPlan;
    return this;
  }

  /**
   * Get billingPlan
   * @return billingPlan
   */
  @Valid 
  @Schema(name = "billingPlan", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("billingPlan")
  public BillingPlan getBillingPlan() {
    return billingPlan;
  }

  public void setBillingPlan(BillingPlan billingPlan) {
    this.billingPlan = billingPlan;
  }

  public Organization billingCurrency(String billingCurrency) {
    this.billingCurrency = billingCurrency;
    return this;
  }

  /**
   * Get billingCurrency
   * @return billingCurrency
   */
  
  @Schema(name = "billingCurrency", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("billingCurrency")
  public String getBillingCurrency() {
    return billingCurrency;
  }

  public void setBillingCurrency(String billingCurrency) {
    this.billingCurrency = billingCurrency;
  }

  public Organization autoBilling(Boolean autoBilling) {
    this.autoBilling = autoBilling;
    return this;
  }

  /**
   * Get autoBilling
   * @return autoBilling
   */
  
  @Schema(name = "autoBilling", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("autoBilling")
  public Boolean getAutoBilling() {
    return autoBilling;
  }

  public void setAutoBilling(Boolean autoBilling) {
    this.autoBilling = autoBilling;
  }

  public Organization taxId(String taxId) {
    this.taxId = taxId;
    return this;
  }

  /**
   * Get taxId
   * @return taxId
   */
  
  @Schema(name = "taxId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("taxId")
  public String getTaxId() {
    return taxId;
  }

  public void setTaxId(String taxId) {
    this.taxId = taxId;
  }

  public Organization complianceFrameworks(List<String> complianceFrameworks) {
    this.complianceFrameworks = complianceFrameworks;
    return this;
  }

  public Organization addComplianceFrameworksItem(String complianceFrameworksItem) {
    if (this.complianceFrameworks == null) {
      this.complianceFrameworks = new ArrayList<>();
    }
    this.complianceFrameworks.add(complianceFrameworksItem);
    return this;
  }

  /**
   * SOC2, GDPR, etc
   * @return complianceFrameworks
   */
  
  @Schema(name = "complianceFrameworks", description = "SOC2, GDPR, etc", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("complianceFrameworks")
  public List<String> getComplianceFrameworks() {
    return complianceFrameworks;
  }

  public void setComplianceFrameworks(List<String> complianceFrameworks) {
    this.complianceFrameworks = complianceFrameworks;
  }

  public Organization dataProcessingAgreement(Boolean dataProcessingAgreement) {
    this.dataProcessingAgreement = dataProcessingAgreement;
    return this;
  }

  /**
   * Get dataProcessingAgreement
   * @return dataProcessingAgreement
   */
  
  @Schema(name = "dataProcessingAgreement", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("dataProcessingAgreement")
  public Boolean getDataProcessingAgreement() {
    return dataProcessingAgreement;
  }

  public void setDataProcessingAgreement(Boolean dataProcessingAgreement) {
    this.dataProcessingAgreement = dataProcessingAgreement;
  }

  public Organization dataRegion(String dataRegion) {
    this.dataRegion = dataRegion;
    return this;
  }

  /**
   * Get dataRegion
   * @return dataRegion
   */
  
  @Schema(name = "dataRegion", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("dataRegion")
  public String getDataRegion() {
    return dataRegion;
  }

  public void setDataRegion(String dataRegion) {
    this.dataRegion = dataRegion;
  }

  public Organization maxTenants(Integer maxTenants) {
    this.maxTenants = maxTenants;
    return this;
  }

  /**
   * Get maxTenants
   * @return maxTenants
   */
  
  @Schema(name = "maxTenants", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("maxTenants")
  public Integer getMaxTenants() {
    return maxTenants;
  }

  public void setMaxTenants(Integer maxTenants) {
    this.maxTenants = maxTenants;
  }

  public Organization totalStorageLimit(String totalStorageLimit) {
    this.totalStorageLimit = totalStorageLimit;
    return this;
  }

  /**
   * Get totalStorageLimit
   * @return totalStorageLimit
   */
  
  @Schema(name = "totalStorageLimit", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("totalStorageLimit")
  public String getTotalStorageLimit() {
    return totalStorageLimit;
  }

  public void setTotalStorageLimit(String totalStorageLimit) {
    this.totalStorageLimit = totalStorageLimit;
  }

  public Organization maxApiKeys(Integer maxApiKeys) {
    this.maxApiKeys = maxApiKeys;
    return this;
  }

  /**
   * Get maxApiKeys
   * @return maxApiKeys
   */
  
  @Schema(name = "maxApiKeys", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("maxApiKeys")
  public Integer getMaxApiKeys() {
    return maxApiKeys;
  }

  public void setMaxApiKeys(Integer maxApiKeys) {
    this.maxApiKeys = maxApiKeys;
  }

  public Organization maxUsers(Integer maxUsers) {
    this.maxUsers = maxUsers;
    return this;
  }

  /**
   * Get maxUsers
   * @return maxUsers
   */
  
  @Schema(name = "maxUsers", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("maxUsers")
  public Integer getMaxUsers() {
    return maxUsers;
  }

  public void setMaxUsers(Integer maxUsers) {
    this.maxUsers = maxUsers;
  }

  public Organization tenants(List<@Valid Tenant> tenants) {
    this.tenants = tenants;
    return this;
  }

  public Organization addTenantsItem(Tenant tenantsItem) {
    if (this.tenants == null) {
      this.tenants = new ArrayList<>();
    }
    this.tenants.add(tenantsItem);
    return this;
  }

  /**
   * Get tenants
   * @return tenants
   */
  @Valid 
  @Schema(name = "tenants", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("tenants")
  public List<@Valid Tenant> getTenants() {
    return tenants;
  }

  public void setTenants(List<@Valid Tenant> tenants) {
    this.tenants = tenants;
  }

  public Organization subscriptions(Subscription subscriptions) {
    this.subscriptions = subscriptions;
    return this;
  }

  /**
   * Get subscriptions
   * @return subscriptions
   */
  @Valid 
  @Schema(name = "subscriptions", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("subscriptions")
  public Subscription getSubscriptions() {
    return subscriptions;
  }

  public void setSubscriptions(Subscription subscriptions) {
    this.subscriptions = subscriptions;
  }

  public Organization createdAt(OffsetDateTime createdAt) {
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

  public Organization updatedAt(OffsetDateTime updatedAt) {
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

  public Organization deletedAt(OffsetDateTime deletedAt) {
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

  public Organization createdBy(String createdBy) {
    this.createdBy = createdBy;
    return this;
  }

  /**
   * Get createdBy
   * @return createdBy
   */
  
  @Schema(name = "createdBy", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("createdBy")
  public String getCreatedBy() {
    return createdBy;
  }

  public void setCreatedBy(String createdBy) {
    this.createdBy = createdBy;
  }

  public Organization status(V1Status status) {
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
    Organization organization = (Organization) o;
    return Objects.equals(this.id, organization.id) &&
        Objects.equals(this.name, organization.name) &&
        Objects.equals(this.displayName, organization.displayName) &&
        Objects.equals(this.website, organization.website) &&
        Objects.equals(this.description, organization.description) &&
        Objects.equals(this.billingEmail, organization.billingEmail) &&
        Objects.equals(this.technicalEmail, organization.technicalEmail) &&
        Objects.equals(this.phone, organization.phone) &&
        Objects.equals(this.address, organization.address) &&
        Objects.equals(this.ssoEnabled, organization.ssoEnabled) &&
        Objects.equals(this.ssoProvider, organization.ssoProvider) &&
        Objects.equals(this.ssoDomain, organization.ssoDomain) &&
        Objects.equals(this.allowedDomains, organization.allowedDomains) &&
        Objects.equals(this.enforce2fa, organization.enforce2fa) &&
        Objects.equals(this.billingPlan, organization.billingPlan) &&
        Objects.equals(this.billingCurrency, organization.billingCurrency) &&
        Objects.equals(this.autoBilling, organization.autoBilling) &&
        Objects.equals(this.taxId, organization.taxId) &&
        Objects.equals(this.complianceFrameworks, organization.complianceFrameworks) &&
        Objects.equals(this.dataProcessingAgreement, organization.dataProcessingAgreement) &&
        Objects.equals(this.dataRegion, organization.dataRegion) &&
        Objects.equals(this.maxTenants, organization.maxTenants) &&
        Objects.equals(this.totalStorageLimit, organization.totalStorageLimit) &&
        Objects.equals(this.maxApiKeys, organization.maxApiKeys) &&
        Objects.equals(this.maxUsers, organization.maxUsers) &&
        Objects.equals(this.tenants, organization.tenants) &&
        Objects.equals(this.subscriptions, organization.subscriptions) &&
        Objects.equals(this.createdAt, organization.createdAt) &&
        Objects.equals(this.updatedAt, organization.updatedAt) &&
        Objects.equals(this.deletedAt, organization.deletedAt) &&
        Objects.equals(this.createdBy, organization.createdBy) &&
        Objects.equals(this.status, organization.status);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, name, displayName, website, description, billingEmail, technicalEmail, phone, address, ssoEnabled, ssoProvider, ssoDomain, allowedDomains, enforce2fa, billingPlan, billingCurrency, autoBilling, taxId, complianceFrameworks, dataProcessingAgreement, dataRegion, maxTenants, totalStorageLimit, maxApiKeys, maxUsers, tenants, subscriptions, createdAt, updatedAt, deletedAt, createdBy, status);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class Organization {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    name: ").append(toIndentedString(name)).append("\n");
    sb.append("    displayName: ").append(toIndentedString(displayName)).append("\n");
    sb.append("    website: ").append(toIndentedString(website)).append("\n");
    sb.append("    description: ").append(toIndentedString(description)).append("\n");
    sb.append("    billingEmail: ").append(toIndentedString(billingEmail)).append("\n");
    sb.append("    technicalEmail: ").append(toIndentedString(technicalEmail)).append("\n");
    sb.append("    phone: ").append(toIndentedString(phone)).append("\n");
    sb.append("    address: ").append(toIndentedString(address)).append("\n");
    sb.append("    ssoEnabled: ").append(toIndentedString(ssoEnabled)).append("\n");
    sb.append("    ssoProvider: ").append(toIndentedString(ssoProvider)).append("\n");
    sb.append("    ssoDomain: ").append(toIndentedString(ssoDomain)).append("\n");
    sb.append("    allowedDomains: ").append(toIndentedString(allowedDomains)).append("\n");
    sb.append("    enforce2fa: ").append(toIndentedString(enforce2fa)).append("\n");
    sb.append("    billingPlan: ").append(toIndentedString(billingPlan)).append("\n");
    sb.append("    billingCurrency: ").append(toIndentedString(billingCurrency)).append("\n");
    sb.append("    autoBilling: ").append(toIndentedString(autoBilling)).append("\n");
    sb.append("    taxId: ").append(toIndentedString(taxId)).append("\n");
    sb.append("    complianceFrameworks: ").append(toIndentedString(complianceFrameworks)).append("\n");
    sb.append("    dataProcessingAgreement: ").append(toIndentedString(dataProcessingAgreement)).append("\n");
    sb.append("    dataRegion: ").append(toIndentedString(dataRegion)).append("\n");
    sb.append("    maxTenants: ").append(toIndentedString(maxTenants)).append("\n");
    sb.append("    totalStorageLimit: ").append(toIndentedString(totalStorageLimit)).append("\n");
    sb.append("    maxApiKeys: ").append(toIndentedString(maxApiKeys)).append("\n");
    sb.append("    maxUsers: ").append(toIndentedString(maxUsers)).append("\n");
    sb.append("    tenants: ").append(toIndentedString(tenants)).append("\n");
    sb.append("    subscriptions: ").append(toIndentedString(subscriptions)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
    sb.append("    updatedAt: ").append(toIndentedString(updatedAt)).append("\n");
    sb.append("    deletedAt: ").append(toIndentedString(deletedAt)).append("\n");
    sb.append("    createdBy: ").append(toIndentedString(createdBy)).append("\n");
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

