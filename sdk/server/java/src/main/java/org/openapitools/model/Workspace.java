package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.APIKey;
import org.openapitools.model.ScrapingJob;
import org.openapitools.model.ScrapingWorkflow;
import org.openapitools.model.WebhookConfig;
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

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T21:14:09.107639-05:00[America/New_York]", comments = "Generator version: 7.7.0")
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
        Objects.equals(this.webhooks, workspace.webhooks);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, name, industry, domain, gdprCompliant, hipaaCompliant, soc2Compliant, storageQuota, usedStorage, createdAt, updatedAt, deletedAt, workflows, jobsRunThisMonth, workspaceJobLimit, dailyJobQuota, activeScrapers, totalLeadsCollected, lastJobRun, scrapingJobs, apiKeys, webhooks);
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

