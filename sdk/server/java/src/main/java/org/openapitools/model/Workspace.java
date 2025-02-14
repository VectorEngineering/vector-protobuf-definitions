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
import org.openapitools.model.APIKey;
import org.openapitools.model.ScrapingJob;
import org.openapitools.model.ScrapingWorkflow;
import org.openapitools.model.WebhookConfig;
import org.openapitools.model.WorkspaceType;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * Workspace
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T08:15:02.658504-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class Workspace {

  private String id;

  private String name;

  private String industry;

  private String domain;

  private Boolean gdprCompliant;

  private Boolean hipaaCompliant;

  private Boolean soc2Compliant;

  private String storageQuota;

  private String usedStorage;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime deletedAt;

  @Valid
  private List<@Valid ScrapingWorkflow> workflows = new ArrayList<>();

  private Integer jobsRunThisMonth;

  private Integer workspaceJobLimit;

  private Integer dailyJobQuota;

  private Integer activeScrapers;

  private Integer totalLeadsCollected;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime lastJobRun;

  @Valid
  private List<@Valid ScrapingJob> scrapingJobs = new ArrayList<>();

  @Valid
  private List<@Valid APIKey> apiKeys = new ArrayList<>();

  @Valid
  private List<@Valid WebhookConfig> webhooks = new ArrayList<>();

  private WorkspaceType workspaceType = WorkspaceType.UNSPECIFIED;

  private String description;

  @Valid
  private Map<String, String> metadata = new HashMap<>();

  @Valid
  private List<String> tags = new ArrayList<>();

  private Integer maxTeamMembers;

  private Integer currentTeamMembers;

  private Boolean allowGuestAccess;

  @Valid
  private List<String> allowedEmailDomains = new ArrayList<>();

  @Valid
  private List<String> exportFormats = new ArrayList<>();

  private String exportSchedule;

  @Valid
  private List<String> exportDestinations = new ArrayList<>();

  @Valid
  private Map<String, String> exportDestinationConfig = new HashMap<>();

  private Boolean compressExports;

  private String compressionFormat;

  private Boolean salesforceEnabled;

  private String salesforceConfig;

  private Boolean hubspotEnabled;

  private String hubspotConfig;

  private Boolean zapierEnabled;

  private String zapierWebhook;

  @Valid
  private List<String> activeIntegrations = new ArrayList<>();

  @Valid
  private Map<String, String> integrationSettings = new HashMap<>();

  public Workspace id(String id) {
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

  public Workspace name(String name) {
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

  public Workspace industry(String industry) {
    this.industry = industry;
    return this;
  }

  /**
   * Get industry
   * @return industry
   */
  
  @Schema(name = "industry", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("industry")
  public String getIndustry() {
    return industry;
  }

  public void setIndustry(String industry) {
    this.industry = industry;
  }

  public Workspace domain(String domain) {
    this.domain = domain;
    return this;
  }

  /**
   * Get domain
   * @return domain
   */
  
  @Schema(name = "domain", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("domain")
  public String getDomain() {
    return domain;
  }

  public void setDomain(String domain) {
    this.domain = domain;
  }

  public Workspace gdprCompliant(Boolean gdprCompliant) {
    this.gdprCompliant = gdprCompliant;
    return this;
  }

  /**
   * Get gdprCompliant
   * @return gdprCompliant
   */
  
  @Schema(name = "gdprCompliant", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("gdprCompliant")
  public Boolean getGdprCompliant() {
    return gdprCompliant;
  }

  public void setGdprCompliant(Boolean gdprCompliant) {
    this.gdprCompliant = gdprCompliant;
  }

  public Workspace hipaaCompliant(Boolean hipaaCompliant) {
    this.hipaaCompliant = hipaaCompliant;
    return this;
  }

  /**
   * Get hipaaCompliant
   * @return hipaaCompliant
   */
  
  @Schema(name = "hipaaCompliant", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("hipaaCompliant")
  public Boolean getHipaaCompliant() {
    return hipaaCompliant;
  }

  public void setHipaaCompliant(Boolean hipaaCompliant) {
    this.hipaaCompliant = hipaaCompliant;
  }

  public Workspace soc2Compliant(Boolean soc2Compliant) {
    this.soc2Compliant = soc2Compliant;
    return this;
  }

  /**
   * Get soc2Compliant
   * @return soc2Compliant
   */
  
  @Schema(name = "soc2Compliant", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("soc2Compliant")
  public Boolean getSoc2Compliant() {
    return soc2Compliant;
  }

  public void setSoc2Compliant(Boolean soc2Compliant) {
    this.soc2Compliant = soc2Compliant;
  }

  public Workspace storageQuota(String storageQuota) {
    this.storageQuota = storageQuota;
    return this;
  }

  /**
   * in bytes
   * @return storageQuota
   */
  
  @Schema(name = "storageQuota", description = "in bytes", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("storageQuota")
  public String getStorageQuota() {
    return storageQuota;
  }

  public void setStorageQuota(String storageQuota) {
    this.storageQuota = storageQuota;
  }

  public Workspace usedStorage(String usedStorage) {
    this.usedStorage = usedStorage;
    return this;
  }

  /**
   * Get usedStorage
   * @return usedStorage
   */
  
  @Schema(name = "usedStorage", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("usedStorage")
  public String getUsedStorage() {
    return usedStorage;
  }

  public void setUsedStorage(String usedStorage) {
    this.usedStorage = usedStorage;
  }

  public Workspace createdAt(OffsetDateTime createdAt) {
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

  public Workspace updatedAt(OffsetDateTime updatedAt) {
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

  public Workspace deletedAt(OffsetDateTime deletedAt) {
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

  public Workspace workflows(List<@Valid ScrapingWorkflow> workflows) {
    this.workflows = workflows;
    return this;
  }

  public Workspace addWorkflowsItem(ScrapingWorkflow workflowsItem) {
    if (this.workflows == null) {
      this.workflows = new ArrayList<>();
    }
    this.workflows.add(workflowsItem);
    return this;
  }

  /**
   * Get workflows
   * @return workflows
   */
  @Valid 
  @Schema(name = "workflows", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("workflows")
  public List<@Valid ScrapingWorkflow> getWorkflows() {
    return workflows;
  }

  public void setWorkflows(List<@Valid ScrapingWorkflow> workflows) {
    this.workflows = workflows;
  }

  public Workspace jobsRunThisMonth(Integer jobsRunThisMonth) {
    this.jobsRunThisMonth = jobsRunThisMonth;
    return this;
  }

  /**
   * Get jobsRunThisMonth
   * @return jobsRunThisMonth
   */
  
  @Schema(name = "jobsRunThisMonth", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("jobsRunThisMonth")
  public Integer getJobsRunThisMonth() {
    return jobsRunThisMonth;
  }

  public void setJobsRunThisMonth(Integer jobsRunThisMonth) {
    this.jobsRunThisMonth = jobsRunThisMonth;
  }

  public Workspace workspaceJobLimit(Integer workspaceJobLimit) {
    this.workspaceJobLimit = workspaceJobLimit;
    return this;
  }

  /**
   * Get workspaceJobLimit
   * @return workspaceJobLimit
   */
  
  @Schema(name = "workspaceJobLimit", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("workspaceJobLimit")
  public Integer getWorkspaceJobLimit() {
    return workspaceJobLimit;
  }

  public void setWorkspaceJobLimit(Integer workspaceJobLimit) {
    this.workspaceJobLimit = workspaceJobLimit;
  }

  public Workspace dailyJobQuota(Integer dailyJobQuota) {
    this.dailyJobQuota = dailyJobQuota;
    return this;
  }

  /**
   * Get dailyJobQuota
   * @return dailyJobQuota
   */
  
  @Schema(name = "dailyJobQuota", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("dailyJobQuota")
  public Integer getDailyJobQuota() {
    return dailyJobQuota;
  }

  public void setDailyJobQuota(Integer dailyJobQuota) {
    this.dailyJobQuota = dailyJobQuota;
  }

  public Workspace activeScrapers(Integer activeScrapers) {
    this.activeScrapers = activeScrapers;
    return this;
  }

  /**
   * Get activeScrapers
   * @return activeScrapers
   */
  
  @Schema(name = "activeScrapers", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("activeScrapers")
  public Integer getActiveScrapers() {
    return activeScrapers;
  }

  public void setActiveScrapers(Integer activeScrapers) {
    this.activeScrapers = activeScrapers;
  }

  public Workspace totalLeadsCollected(Integer totalLeadsCollected) {
    this.totalLeadsCollected = totalLeadsCollected;
    return this;
  }

  /**
   * Get totalLeadsCollected
   * @return totalLeadsCollected
   */
  
  @Schema(name = "totalLeadsCollected", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("totalLeadsCollected")
  public Integer getTotalLeadsCollected() {
    return totalLeadsCollected;
  }

  public void setTotalLeadsCollected(Integer totalLeadsCollected) {
    this.totalLeadsCollected = totalLeadsCollected;
  }

  public Workspace lastJobRun(OffsetDateTime lastJobRun) {
    this.lastJobRun = lastJobRun;
    return this;
  }

  /**
   * Get lastJobRun
   * @return lastJobRun
   */
  @Valid 
  @Schema(name = "lastJobRun", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("lastJobRun")
  public OffsetDateTime getLastJobRun() {
    return lastJobRun;
  }

  public void setLastJobRun(OffsetDateTime lastJobRun) {
    this.lastJobRun = lastJobRun;
  }

  public Workspace scrapingJobs(List<@Valid ScrapingJob> scrapingJobs) {
    this.scrapingJobs = scrapingJobs;
    return this;
  }

  public Workspace addScrapingJobsItem(ScrapingJob scrapingJobsItem) {
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

  public Workspace apiKeys(List<@Valid APIKey> apiKeys) {
    this.apiKeys = apiKeys;
    return this;
  }

  public Workspace addApiKeysItem(APIKey apiKeysItem) {
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
  public List<@Valid APIKey> getApiKeys() {
    return apiKeys;
  }

  public void setApiKeys(List<@Valid APIKey> apiKeys) {
    this.apiKeys = apiKeys;
  }

  public Workspace webhooks(List<@Valid WebhookConfig> webhooks) {
    this.webhooks = webhooks;
    return this;
  }

  public Workspace addWebhooksItem(WebhookConfig webhooksItem) {
    if (this.webhooks == null) {
      this.webhooks = new ArrayList<>();
    }
    this.webhooks.add(webhooksItem);
    return this;
  }

  /**
   * Get webhooks
   * @return webhooks
   */
  @Valid 
  @Schema(name = "webhooks", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("webhooks")
  public List<@Valid WebhookConfig> getWebhooks() {
    return webhooks;
  }

  public void setWebhooks(List<@Valid WebhookConfig> webhooks) {
    this.webhooks = webhooks;
  }

  public Workspace workspaceType(WorkspaceType workspaceType) {
    this.workspaceType = workspaceType;
    return this;
  }

  /**
   * Get workspaceType
   * @return workspaceType
   */
  @Valid 
  @Schema(name = "workspaceType", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("workspaceType")
  public WorkspaceType getWorkspaceType() {
    return workspaceType;
  }

  public void setWorkspaceType(WorkspaceType workspaceType) {
    this.workspaceType = workspaceType;
  }

  public Workspace description(String description) {
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

  public Workspace metadata(Map<String, String> metadata) {
    this.metadata = metadata;
    return this;
  }

  public Workspace putMetadataItem(String key, String metadataItem) {
    if (this.metadata == null) {
      this.metadata = new HashMap<>();
    }
    this.metadata.put(key, metadataItem);
    return this;
  }

  /**
   * Get metadata
   * @return metadata
   */
  
  @Schema(name = "metadata", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("metadata")
  public Map<String, String> getMetadata() {
    return metadata;
  }

  public void setMetadata(Map<String, String> metadata) {
    this.metadata = metadata;
  }

  public Workspace tags(List<String> tags) {
    this.tags = tags;
    return this;
  }

  public Workspace addTagsItem(String tagsItem) {
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

  public Workspace maxTeamMembers(Integer maxTeamMembers) {
    this.maxTeamMembers = maxTeamMembers;
    return this;
  }

  /**
   * Get maxTeamMembers
   * @return maxTeamMembers
   */
  
  @Schema(name = "maxTeamMembers", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("maxTeamMembers")
  public Integer getMaxTeamMembers() {
    return maxTeamMembers;
  }

  public void setMaxTeamMembers(Integer maxTeamMembers) {
    this.maxTeamMembers = maxTeamMembers;
  }

  public Workspace currentTeamMembers(Integer currentTeamMembers) {
    this.currentTeamMembers = currentTeamMembers;
    return this;
  }

  /**
   * Get currentTeamMembers
   * @return currentTeamMembers
   */
  
  @Schema(name = "currentTeamMembers", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("currentTeamMembers")
  public Integer getCurrentTeamMembers() {
    return currentTeamMembers;
  }

  public void setCurrentTeamMembers(Integer currentTeamMembers) {
    this.currentTeamMembers = currentTeamMembers;
  }

  public Workspace allowGuestAccess(Boolean allowGuestAccess) {
    this.allowGuestAccess = allowGuestAccess;
    return this;
  }

  /**
   * Get allowGuestAccess
   * @return allowGuestAccess
   */
  
  @Schema(name = "allowGuestAccess", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("allowGuestAccess")
  public Boolean getAllowGuestAccess() {
    return allowGuestAccess;
  }

  public void setAllowGuestAccess(Boolean allowGuestAccess) {
    this.allowGuestAccess = allowGuestAccess;
  }

  public Workspace allowedEmailDomains(List<String> allowedEmailDomains) {
    this.allowedEmailDomains = allowedEmailDomains;
    return this;
  }

  public Workspace addAllowedEmailDomainsItem(String allowedEmailDomainsItem) {
    if (this.allowedEmailDomains == null) {
      this.allowedEmailDomains = new ArrayList<>();
    }
    this.allowedEmailDomains.add(allowedEmailDomainsItem);
    return this;
  }

  /**
   * Get allowedEmailDomains
   * @return allowedEmailDomains
   */
  
  @Schema(name = "allowedEmailDomains", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("allowedEmailDomains")
  public List<String> getAllowedEmailDomains() {
    return allowedEmailDomains;
  }

  public void setAllowedEmailDomains(List<String> allowedEmailDomains) {
    this.allowedEmailDomains = allowedEmailDomains;
  }

  public Workspace exportFormats(List<String> exportFormats) {
    this.exportFormats = exportFormats;
    return this;
  }

  public Workspace addExportFormatsItem(String exportFormatsItem) {
    if (this.exportFormats == null) {
      this.exportFormats = new ArrayList<>();
    }
    this.exportFormats.add(exportFormatsItem);
    return this;
  }

  /**
   * e.g., \"csv\", \"json\", \"excel\"
   * @return exportFormats
   */
  
  @Schema(name = "exportFormats", description = "e.g., \"csv\", \"json\", \"excel\"", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("exportFormats")
  public List<String> getExportFormats() {
    return exportFormats;
  }

  public void setExportFormats(List<String> exportFormats) {
    this.exportFormats = exportFormats;
  }

  public Workspace exportSchedule(String exportSchedule) {
    this.exportSchedule = exportSchedule;
    return this;
  }

  /**
   * Get exportSchedule
   * @return exportSchedule
   */
  
  @Schema(name = "exportSchedule", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("exportSchedule")
  public String getExportSchedule() {
    return exportSchedule;
  }

  public void setExportSchedule(String exportSchedule) {
    this.exportSchedule = exportSchedule;
  }

  public Workspace exportDestinations(List<String> exportDestinations) {
    this.exportDestinations = exportDestinations;
    return this;
  }

  public Workspace addExportDestinationsItem(String exportDestinationsItem) {
    if (this.exportDestinations == null) {
      this.exportDestinations = new ArrayList<>();
    }
    this.exportDestinations.add(exportDestinationsItem);
    return this;
  }

  /**
   * Get exportDestinations
   * @return exportDestinations
   */
  
  @Schema(name = "exportDestinations", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("exportDestinations")
  public List<String> getExportDestinations() {
    return exportDestinations;
  }

  public void setExportDestinations(List<String> exportDestinations) {
    this.exportDestinations = exportDestinations;
  }

  public Workspace exportDestinationConfig(Map<String, String> exportDestinationConfig) {
    this.exportDestinationConfig = exportDestinationConfig;
    return this;
  }

  public Workspace putExportDestinationConfigItem(String key, String exportDestinationConfigItem) {
    if (this.exportDestinationConfig == null) {
      this.exportDestinationConfig = new HashMap<>();
    }
    this.exportDestinationConfig.put(key, exportDestinationConfigItem);
    return this;
  }

  /**
   * Get exportDestinationConfig
   * @return exportDestinationConfig
   */
  
  @Schema(name = "exportDestinationConfig", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("exportDestinationConfig")
  public Map<String, String> getExportDestinationConfig() {
    return exportDestinationConfig;
  }

  public void setExportDestinationConfig(Map<String, String> exportDestinationConfig) {
    this.exportDestinationConfig = exportDestinationConfig;
  }

  public Workspace compressExports(Boolean compressExports) {
    this.compressExports = compressExports;
    return this;
  }

  /**
   * Get compressExports
   * @return compressExports
   */
  
  @Schema(name = "compressExports", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("compressExports")
  public Boolean getCompressExports() {
    return compressExports;
  }

  public void setCompressExports(Boolean compressExports) {
    this.compressExports = compressExports;
  }

  public Workspace compressionFormat(String compressionFormat) {
    this.compressionFormat = compressionFormat;
    return this;
  }

  /**
   * Get compressionFormat
   * @return compressionFormat
   */
  
  @Schema(name = "compressionFormat", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("compressionFormat")
  public String getCompressionFormat() {
    return compressionFormat;
  }

  public void setCompressionFormat(String compressionFormat) {
    this.compressionFormat = compressionFormat;
  }

  public Workspace salesforceEnabled(Boolean salesforceEnabled) {
    this.salesforceEnabled = salesforceEnabled;
    return this;
  }

  /**
   * Get salesforceEnabled
   * @return salesforceEnabled
   */
  
  @Schema(name = "salesforceEnabled", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("salesforceEnabled")
  public Boolean getSalesforceEnabled() {
    return salesforceEnabled;
  }

  public void setSalesforceEnabled(Boolean salesforceEnabled) {
    this.salesforceEnabled = salesforceEnabled;
  }

  public Workspace salesforceConfig(String salesforceConfig) {
    this.salesforceConfig = salesforceConfig;
    return this;
  }

  /**
   * Get salesforceConfig
   * @return salesforceConfig
   */
  
  @Schema(name = "salesforceConfig", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("salesforceConfig")
  public String getSalesforceConfig() {
    return salesforceConfig;
  }

  public void setSalesforceConfig(String salesforceConfig) {
    this.salesforceConfig = salesforceConfig;
  }

  public Workspace hubspotEnabled(Boolean hubspotEnabled) {
    this.hubspotEnabled = hubspotEnabled;
    return this;
  }

  /**
   * Get hubspotEnabled
   * @return hubspotEnabled
   */
  
  @Schema(name = "hubspotEnabled", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("hubspotEnabled")
  public Boolean getHubspotEnabled() {
    return hubspotEnabled;
  }

  public void setHubspotEnabled(Boolean hubspotEnabled) {
    this.hubspotEnabled = hubspotEnabled;
  }

  public Workspace hubspotConfig(String hubspotConfig) {
    this.hubspotConfig = hubspotConfig;
    return this;
  }

  /**
   * Get hubspotConfig
   * @return hubspotConfig
   */
  
  @Schema(name = "hubspotConfig", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("hubspotConfig")
  public String getHubspotConfig() {
    return hubspotConfig;
  }

  public void setHubspotConfig(String hubspotConfig) {
    this.hubspotConfig = hubspotConfig;
  }

  public Workspace zapierEnabled(Boolean zapierEnabled) {
    this.zapierEnabled = zapierEnabled;
    return this;
  }

  /**
   * Get zapierEnabled
   * @return zapierEnabled
   */
  
  @Schema(name = "zapierEnabled", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("zapierEnabled")
  public Boolean getZapierEnabled() {
    return zapierEnabled;
  }

  public void setZapierEnabled(Boolean zapierEnabled) {
    this.zapierEnabled = zapierEnabled;
  }

  public Workspace zapierWebhook(String zapierWebhook) {
    this.zapierWebhook = zapierWebhook;
    return this;
  }

  /**
   * Get zapierWebhook
   * @return zapierWebhook
   */
  
  @Schema(name = "zapierWebhook", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("zapierWebhook")
  public String getZapierWebhook() {
    return zapierWebhook;
  }

  public void setZapierWebhook(String zapierWebhook) {
    this.zapierWebhook = zapierWebhook;
  }

  public Workspace activeIntegrations(List<String> activeIntegrations) {
    this.activeIntegrations = activeIntegrations;
    return this;
  }

  public Workspace addActiveIntegrationsItem(String activeIntegrationsItem) {
    if (this.activeIntegrations == null) {
      this.activeIntegrations = new ArrayList<>();
    }
    this.activeIntegrations.add(activeIntegrationsItem);
    return this;
  }

  /**
   * Get activeIntegrations
   * @return activeIntegrations
   */
  
  @Schema(name = "activeIntegrations", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("activeIntegrations")
  public List<String> getActiveIntegrations() {
    return activeIntegrations;
  }

  public void setActiveIntegrations(List<String> activeIntegrations) {
    this.activeIntegrations = activeIntegrations;
  }

  public Workspace integrationSettings(Map<String, String> integrationSettings) {
    this.integrationSettings = integrationSettings;
    return this;
  }

  public Workspace putIntegrationSettingsItem(String key, String integrationSettingsItem) {
    if (this.integrationSettings == null) {
      this.integrationSettings = new HashMap<>();
    }
    this.integrationSettings.put(key, integrationSettingsItem);
    return this;
  }

  /**
   * Get integrationSettings
   * @return integrationSettings
   */
  
  @Schema(name = "integrationSettings", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("integrationSettings")
  public Map<String, String> getIntegrationSettings() {
    return integrationSettings;
  }

  public void setIntegrationSettings(Map<String, String> integrationSettings) {
    this.integrationSettings = integrationSettings;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    Workspace workspace = (Workspace) o;
    return Objects.equals(this.id, workspace.id) &&
        Objects.equals(this.name, workspace.name) &&
        Objects.equals(this.industry, workspace.industry) &&
        Objects.equals(this.domain, workspace.domain) &&
        Objects.equals(this.gdprCompliant, workspace.gdprCompliant) &&
        Objects.equals(this.hipaaCompliant, workspace.hipaaCompliant) &&
        Objects.equals(this.soc2Compliant, workspace.soc2Compliant) &&
        Objects.equals(this.storageQuota, workspace.storageQuota) &&
        Objects.equals(this.usedStorage, workspace.usedStorage) &&
        Objects.equals(this.createdAt, workspace.createdAt) &&
        Objects.equals(this.updatedAt, workspace.updatedAt) &&
        Objects.equals(this.deletedAt, workspace.deletedAt) &&
        Objects.equals(this.workflows, workspace.workflows) &&
        Objects.equals(this.jobsRunThisMonth, workspace.jobsRunThisMonth) &&
        Objects.equals(this.workspaceJobLimit, workspace.workspaceJobLimit) &&
        Objects.equals(this.dailyJobQuota, workspace.dailyJobQuota) &&
        Objects.equals(this.activeScrapers, workspace.activeScrapers) &&
        Objects.equals(this.totalLeadsCollected, workspace.totalLeadsCollected) &&
        Objects.equals(this.lastJobRun, workspace.lastJobRun) &&
        Objects.equals(this.scrapingJobs, workspace.scrapingJobs) &&
        Objects.equals(this.apiKeys, workspace.apiKeys) &&
        Objects.equals(this.webhooks, workspace.webhooks) &&
        Objects.equals(this.workspaceType, workspace.workspaceType) &&
        Objects.equals(this.description, workspace.description) &&
        Objects.equals(this.metadata, workspace.metadata) &&
        Objects.equals(this.tags, workspace.tags) &&
        Objects.equals(this.maxTeamMembers, workspace.maxTeamMembers) &&
        Objects.equals(this.currentTeamMembers, workspace.currentTeamMembers) &&
        Objects.equals(this.allowGuestAccess, workspace.allowGuestAccess) &&
        Objects.equals(this.allowedEmailDomains, workspace.allowedEmailDomains) &&
        Objects.equals(this.exportFormats, workspace.exportFormats) &&
        Objects.equals(this.exportSchedule, workspace.exportSchedule) &&
        Objects.equals(this.exportDestinations, workspace.exportDestinations) &&
        Objects.equals(this.exportDestinationConfig, workspace.exportDestinationConfig) &&
        Objects.equals(this.compressExports, workspace.compressExports) &&
        Objects.equals(this.compressionFormat, workspace.compressionFormat) &&
        Objects.equals(this.salesforceEnabled, workspace.salesforceEnabled) &&
        Objects.equals(this.salesforceConfig, workspace.salesforceConfig) &&
        Objects.equals(this.hubspotEnabled, workspace.hubspotEnabled) &&
        Objects.equals(this.hubspotConfig, workspace.hubspotConfig) &&
        Objects.equals(this.zapierEnabled, workspace.zapierEnabled) &&
        Objects.equals(this.zapierWebhook, workspace.zapierWebhook) &&
        Objects.equals(this.activeIntegrations, workspace.activeIntegrations) &&
        Objects.equals(this.integrationSettings, workspace.integrationSettings);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, name, industry, domain, gdprCompliant, hipaaCompliant, soc2Compliant, storageQuota, usedStorage, createdAt, updatedAt, deletedAt, workflows, jobsRunThisMonth, workspaceJobLimit, dailyJobQuota, activeScrapers, totalLeadsCollected, lastJobRun, scrapingJobs, apiKeys, webhooks, workspaceType, description, metadata, tags, maxTeamMembers, currentTeamMembers, allowGuestAccess, allowedEmailDomains, exportFormats, exportSchedule, exportDestinations, exportDestinationConfig, compressExports, compressionFormat, salesforceEnabled, salesforceConfig, hubspotEnabled, hubspotConfig, zapierEnabled, zapierWebhook, activeIntegrations, integrationSettings);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class Workspace {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    name: ").append(toIndentedString(name)).append("\n");
    sb.append("    industry: ").append(toIndentedString(industry)).append("\n");
    sb.append("    domain: ").append(toIndentedString(domain)).append("\n");
    sb.append("    gdprCompliant: ").append(toIndentedString(gdprCompliant)).append("\n");
    sb.append("    hipaaCompliant: ").append(toIndentedString(hipaaCompliant)).append("\n");
    sb.append("    soc2Compliant: ").append(toIndentedString(soc2Compliant)).append("\n");
    sb.append("    storageQuota: ").append(toIndentedString(storageQuota)).append("\n");
    sb.append("    usedStorage: ").append(toIndentedString(usedStorage)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
    sb.append("    updatedAt: ").append(toIndentedString(updatedAt)).append("\n");
    sb.append("    deletedAt: ").append(toIndentedString(deletedAt)).append("\n");
    sb.append("    workflows: ").append(toIndentedString(workflows)).append("\n");
    sb.append("    jobsRunThisMonth: ").append(toIndentedString(jobsRunThisMonth)).append("\n");
    sb.append("    workspaceJobLimit: ").append(toIndentedString(workspaceJobLimit)).append("\n");
    sb.append("    dailyJobQuota: ").append(toIndentedString(dailyJobQuota)).append("\n");
    sb.append("    activeScrapers: ").append(toIndentedString(activeScrapers)).append("\n");
    sb.append("    totalLeadsCollected: ").append(toIndentedString(totalLeadsCollected)).append("\n");
    sb.append("    lastJobRun: ").append(toIndentedString(lastJobRun)).append("\n");
    sb.append("    scrapingJobs: ").append(toIndentedString(scrapingJobs)).append("\n");
    sb.append("    apiKeys: ").append(toIndentedString(apiKeys)).append("\n");
    sb.append("    webhooks: ").append(toIndentedString(webhooks)).append("\n");
    sb.append("    workspaceType: ").append(toIndentedString(workspaceType)).append("\n");
    sb.append("    description: ").append(toIndentedString(description)).append("\n");
    sb.append("    metadata: ").append(toIndentedString(metadata)).append("\n");
    sb.append("    tags: ").append(toIndentedString(tags)).append("\n");
    sb.append("    maxTeamMembers: ").append(toIndentedString(maxTeamMembers)).append("\n");
    sb.append("    currentTeamMembers: ").append(toIndentedString(currentTeamMembers)).append("\n");
    sb.append("    allowGuestAccess: ").append(toIndentedString(allowGuestAccess)).append("\n");
    sb.append("    allowedEmailDomains: ").append(toIndentedString(allowedEmailDomains)).append("\n");
    sb.append("    exportFormats: ").append(toIndentedString(exportFormats)).append("\n");
    sb.append("    exportSchedule: ").append(toIndentedString(exportSchedule)).append("\n");
    sb.append("    exportDestinations: ").append(toIndentedString(exportDestinations)).append("\n");
    sb.append("    exportDestinationConfig: ").append(toIndentedString(exportDestinationConfig)).append("\n");
    sb.append("    compressExports: ").append(toIndentedString(compressExports)).append("\n");
    sb.append("    compressionFormat: ").append(toIndentedString(compressionFormat)).append("\n");
    sb.append("    salesforceEnabled: ").append(toIndentedString(salesforceEnabled)).append("\n");
    sb.append("    salesforceConfig: ").append(toIndentedString(salesforceConfig)).append("\n");
    sb.append("    hubspotEnabled: ").append(toIndentedString(hubspotEnabled)).append("\n");
    sb.append("    hubspotConfig: ").append(toIndentedString(hubspotConfig)).append("\n");
    sb.append("    zapierEnabled: ").append(toIndentedString(zapierEnabled)).append("\n");
    sb.append("    zapierWebhook: ").append(toIndentedString(zapierWebhook)).append("\n");
    sb.append("    activeIntegrations: ").append(toIndentedString(activeIntegrations)).append("\n");
    sb.append("    integrationSettings: ").append(toIndentedString(integrationSettings)).append("\n");
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

