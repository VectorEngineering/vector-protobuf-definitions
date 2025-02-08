package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.DocumentTemplate;
import org.openapitools.model.FolderMetadata;
import org.openapitools.model.MarketplaceApp;
import org.openapitools.model.WorkspaceActivity;
import org.openapitools.model.WorkspaceCompliance;
import org.openapitools.model.WorkspaceSharing;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * Workspace1
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-08T09:23:51.243070-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class Workspace1 {

  private String id;

  private String name;

  private String uniqueIdentifier;

  private String s3BucketName;

  private String s3FolderPath;

  private String storageQuota;

  private String usedStorage;

  private Boolean allowPublicSharing;

  private Boolean requireApproval;

  private Boolean gdprCompliant;

  private Boolean hipaaCompliant;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime deletedAt;

  @Valid
  private List<@Valid FolderMetadata> folders = new ArrayList<>();

  @Valid
  private List<@Valid DocumentTemplate> templates = new ArrayList<>();

  @Valid
  private List<@Valid WorkspaceSharing> sharing = new ArrayList<>();

  @Valid
  private List<@Valid WorkspaceActivity> activities = new ArrayList<>();

  @Valid
  private List<@Valid WorkspaceCompliance> compliance = new ArrayList<>();

  @Valid
  private List<@Valid MarketplaceApp> installedApps = new ArrayList<>();

  public Workspace1 id(String id) {
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

  public Workspace1 name(String name) {
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

  public Workspace1 uniqueIdentifier(String uniqueIdentifier) {
    this.uniqueIdentifier = uniqueIdentifier;
    return this;
  }

  /**
   * Get uniqueIdentifier
   * @return uniqueIdentifier
   */
  
  @Schema(name = "uniqueIdentifier", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("uniqueIdentifier")
  public String getUniqueIdentifier() {
    return uniqueIdentifier;
  }

  public void setUniqueIdentifier(String uniqueIdentifier) {
    this.uniqueIdentifier = uniqueIdentifier;
  }

  public Workspace1 s3BucketName(String s3BucketName) {
    this.s3BucketName = s3BucketName;
    return this;
  }

  /**
   * Get s3BucketName
   * @return s3BucketName
   */
  
  @Schema(name = "s3BucketName", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("s3BucketName")
  public String getS3BucketName() {
    return s3BucketName;
  }

  public void setS3BucketName(String s3BucketName) {
    this.s3BucketName = s3BucketName;
  }

  public Workspace1 s3FolderPath(String s3FolderPath) {
    this.s3FolderPath = s3FolderPath;
    return this;
  }

  /**
   * Get s3FolderPath
   * @return s3FolderPath
   */
  
  @Schema(name = "s3FolderPath", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("s3FolderPath")
  public String getS3FolderPath() {
    return s3FolderPath;
  }

  public void setS3FolderPath(String s3FolderPath) {
    this.s3FolderPath = s3FolderPath;
  }

  public Workspace1 storageQuota(String storageQuota) {
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

  public Workspace1 usedStorage(String usedStorage) {
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

  public Workspace1 allowPublicSharing(Boolean allowPublicSharing) {
    this.allowPublicSharing = allowPublicSharing;
    return this;
  }

  /**
   * Get allowPublicSharing
   * @return allowPublicSharing
   */
  
  @Schema(name = "allowPublicSharing", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("allowPublicSharing")
  public Boolean getAllowPublicSharing() {
    return allowPublicSharing;
  }

  public void setAllowPublicSharing(Boolean allowPublicSharing) {
    this.allowPublicSharing = allowPublicSharing;
  }

  public Workspace1 requireApproval(Boolean requireApproval) {
    this.requireApproval = requireApproval;
    return this;
  }

  /**
   * Get requireApproval
   * @return requireApproval
   */
  
  @Schema(name = "requireApproval", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("requireApproval")
  public Boolean getRequireApproval() {
    return requireApproval;
  }

  public void setRequireApproval(Boolean requireApproval) {
    this.requireApproval = requireApproval;
  }

  public Workspace1 gdprCompliant(Boolean gdprCompliant) {
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

  public Workspace1 hipaaCompliant(Boolean hipaaCompliant) {
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

  public Workspace1 createdAt(OffsetDateTime createdAt) {
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

  public Workspace1 updatedAt(OffsetDateTime updatedAt) {
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

  public Workspace1 deletedAt(OffsetDateTime deletedAt) {
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

  public Workspace1 folders(List<@Valid FolderMetadata> folders) {
    this.folders = folders;
    return this;
  }

  public Workspace1 addFoldersItem(FolderMetadata foldersItem) {
    if (this.folders == null) {
      this.folders = new ArrayList<>();
    }
    this.folders.add(foldersItem);
    return this;
  }

  /**
   * Get folders
   * @return folders
   */
  @Valid 
  @Schema(name = "folders", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("folders")
  public List<@Valid FolderMetadata> getFolders() {
    return folders;
  }

  public void setFolders(List<@Valid FolderMetadata> folders) {
    this.folders = folders;
  }

  public Workspace1 templates(List<@Valid DocumentTemplate> templates) {
    this.templates = templates;
    return this;
  }

  public Workspace1 addTemplatesItem(DocumentTemplate templatesItem) {
    if (this.templates == null) {
      this.templates = new ArrayList<>();
    }
    this.templates.add(templatesItem);
    return this;
  }

  /**
   * Get templates
   * @return templates
   */
  @Valid 
  @Schema(name = "templates", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("templates")
  public List<@Valid DocumentTemplate> getTemplates() {
    return templates;
  }

  public void setTemplates(List<@Valid DocumentTemplate> templates) {
    this.templates = templates;
  }

  public Workspace1 sharing(List<@Valid WorkspaceSharing> sharing) {
    this.sharing = sharing;
    return this;
  }

  public Workspace1 addSharingItem(WorkspaceSharing sharingItem) {
    if (this.sharing == null) {
      this.sharing = new ArrayList<>();
    }
    this.sharing.add(sharingItem);
    return this;
  }

  /**
   * Get sharing
   * @return sharing
   */
  @Valid 
  @Schema(name = "sharing", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("sharing")
  public List<@Valid WorkspaceSharing> getSharing() {
    return sharing;
  }

  public void setSharing(List<@Valid WorkspaceSharing> sharing) {
    this.sharing = sharing;
  }

  public Workspace1 activities(List<@Valid WorkspaceActivity> activities) {
    this.activities = activities;
    return this;
  }

  public Workspace1 addActivitiesItem(WorkspaceActivity activitiesItem) {
    if (this.activities == null) {
      this.activities = new ArrayList<>();
    }
    this.activities.add(activitiesItem);
    return this;
  }

  /**
   * Get activities
   * @return activities
   */
  @Valid 
  @Schema(name = "activities", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("activities")
  public List<@Valid WorkspaceActivity> getActivities() {
    return activities;
  }

  public void setActivities(List<@Valid WorkspaceActivity> activities) {
    this.activities = activities;
  }

  public Workspace1 compliance(List<@Valid WorkspaceCompliance> compliance) {
    this.compliance = compliance;
    return this;
  }

  public Workspace1 addComplianceItem(WorkspaceCompliance complianceItem) {
    if (this.compliance == null) {
      this.compliance = new ArrayList<>();
    }
    this.compliance.add(complianceItem);
    return this;
  }

  /**
   * Get compliance
   * @return compliance
   */
  @Valid 
  @Schema(name = "compliance", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("compliance")
  public List<@Valid WorkspaceCompliance> getCompliance() {
    return compliance;
  }

  public void setCompliance(List<@Valid WorkspaceCompliance> compliance) {
    this.compliance = compliance;
  }

  public Workspace1 installedApps(List<@Valid MarketplaceApp> installedApps) {
    this.installedApps = installedApps;
    return this;
  }

  public Workspace1 addInstalledAppsItem(MarketplaceApp installedAppsItem) {
    if (this.installedApps == null) {
      this.installedApps = new ArrayList<>();
    }
    this.installedApps.add(installedAppsItem);
    return this;
  }

  /**
   * Get installedApps
   * @return installedApps
   */
  @Valid 
  @Schema(name = "installedApps", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("installedApps")
  public List<@Valid MarketplaceApp> getInstalledApps() {
    return installedApps;
  }

  public void setInstalledApps(List<@Valid MarketplaceApp> installedApps) {
    this.installedApps = installedApps;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    Workspace1 workspace1 = (Workspace1) o;
    return Objects.equals(this.id, workspace1.id) &&
        Objects.equals(this.name, workspace1.name) &&
        Objects.equals(this.uniqueIdentifier, workspace1.uniqueIdentifier) &&
        Objects.equals(this.s3BucketName, workspace1.s3BucketName) &&
        Objects.equals(this.s3FolderPath, workspace1.s3FolderPath) &&
        Objects.equals(this.storageQuota, workspace1.storageQuota) &&
        Objects.equals(this.usedStorage, workspace1.usedStorage) &&
        Objects.equals(this.allowPublicSharing, workspace1.allowPublicSharing) &&
        Objects.equals(this.requireApproval, workspace1.requireApproval) &&
        Objects.equals(this.gdprCompliant, workspace1.gdprCompliant) &&
        Objects.equals(this.hipaaCompliant, workspace1.hipaaCompliant) &&
        Objects.equals(this.createdAt, workspace1.createdAt) &&
        Objects.equals(this.updatedAt, workspace1.updatedAt) &&
        Objects.equals(this.deletedAt, workspace1.deletedAt) &&
        Objects.equals(this.folders, workspace1.folders) &&
        Objects.equals(this.templates, workspace1.templates) &&
        Objects.equals(this.sharing, workspace1.sharing) &&
        Objects.equals(this.activities, workspace1.activities) &&
        Objects.equals(this.compliance, workspace1.compliance) &&
        Objects.equals(this.installedApps, workspace1.installedApps);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, name, uniqueIdentifier, s3BucketName, s3FolderPath, storageQuota, usedStorage, allowPublicSharing, requireApproval, gdprCompliant, hipaaCompliant, createdAt, updatedAt, deletedAt, folders, templates, sharing, activities, compliance, installedApps);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class Workspace1 {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    name: ").append(toIndentedString(name)).append("\n");
    sb.append("    uniqueIdentifier: ").append(toIndentedString(uniqueIdentifier)).append("\n");
    sb.append("    s3BucketName: ").append(toIndentedString(s3BucketName)).append("\n");
    sb.append("    s3FolderPath: ").append(toIndentedString(s3FolderPath)).append("\n");
    sb.append("    storageQuota: ").append(toIndentedString(storageQuota)).append("\n");
    sb.append("    usedStorage: ").append(toIndentedString(usedStorage)).append("\n");
    sb.append("    allowPublicSharing: ").append(toIndentedString(allowPublicSharing)).append("\n");
    sb.append("    requireApproval: ").append(toIndentedString(requireApproval)).append("\n");
    sb.append("    gdprCompliant: ").append(toIndentedString(gdprCompliant)).append("\n");
    sb.append("    hipaaCompliant: ").append(toIndentedString(hipaaCompliant)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
    sb.append("    updatedAt: ").append(toIndentedString(updatedAt)).append("\n");
    sb.append("    deletedAt: ").append(toIndentedString(deletedAt)).append("\n");
    sb.append("    folders: ").append(toIndentedString(folders)).append("\n");
    sb.append("    templates: ").append(toIndentedString(templates)).append("\n");
    sb.append("    sharing: ").append(toIndentedString(sharing)).append("\n");
    sb.append("    activities: ").append(toIndentedString(activities)).append("\n");
    sb.append("    compliance: ").append(toIndentedString(compliance)).append("\n");
    sb.append("    installedApps: ").append(toIndentedString(installedApps)).append("\n");
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

