package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.StorageBreakdown;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * GetWorkspaceStorageStatsResponse
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-03T20:00:13.934981-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class GetWorkspaceStorageStatsResponse {

  private String totalStorageUsed;

  private String storageQuota;

  private Float usagePercentage;

  @Valid
  private List<@Valid StorageBreakdown> storageByType = new ArrayList<>();

  private Integer totalFiles;

  private Integer totalFolders;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime lastUpdated;

  public GetWorkspaceStorageStatsResponse totalStorageUsed(String totalStorageUsed) {
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

  public GetWorkspaceStorageStatsResponse storageQuota(String storageQuota) {
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

  public GetWorkspaceStorageStatsResponse usagePercentage(Float usagePercentage) {
    this.usagePercentage = usagePercentage;
    return this;
  }

  /**
   * Get usagePercentage
   * @return usagePercentage
   */
  
  @Schema(name = "usagePercentage", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("usagePercentage")
  public Float getUsagePercentage() {
    return usagePercentage;
  }

  public void setUsagePercentage(Float usagePercentage) {
    this.usagePercentage = usagePercentage;
  }

  public GetWorkspaceStorageStatsResponse storageByType(List<@Valid StorageBreakdown> storageByType) {
    this.storageByType = storageByType;
    return this;
  }

  public GetWorkspaceStorageStatsResponse addStorageByTypeItem(StorageBreakdown storageByTypeItem) {
    if (this.storageByType == null) {
      this.storageByType = new ArrayList<>();
    }
    this.storageByType.add(storageByTypeItem);
    return this;
  }

  /**
   * Get storageByType
   * @return storageByType
   */
  @Valid 
  @Schema(name = "storageByType", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("storageByType")
  public List<@Valid StorageBreakdown> getStorageByType() {
    return storageByType;
  }

  public void setStorageByType(List<@Valid StorageBreakdown> storageByType) {
    this.storageByType = storageByType;
  }

  public GetWorkspaceStorageStatsResponse totalFiles(Integer totalFiles) {
    this.totalFiles = totalFiles;
    return this;
  }

  /**
   * Get totalFiles
   * @return totalFiles
   */
  
  @Schema(name = "totalFiles", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("totalFiles")
  public Integer getTotalFiles() {
    return totalFiles;
  }

  public void setTotalFiles(Integer totalFiles) {
    this.totalFiles = totalFiles;
  }

  public GetWorkspaceStorageStatsResponse totalFolders(Integer totalFolders) {
    this.totalFolders = totalFolders;
    return this;
  }

  /**
   * Get totalFolders
   * @return totalFolders
   */
  
  @Schema(name = "totalFolders", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("totalFolders")
  public Integer getTotalFolders() {
    return totalFolders;
  }

  public void setTotalFolders(Integer totalFolders) {
    this.totalFolders = totalFolders;
  }

  public GetWorkspaceStorageStatsResponse lastUpdated(OffsetDateTime lastUpdated) {
    this.lastUpdated = lastUpdated;
    return this;
  }

  /**
   * Get lastUpdated
   * @return lastUpdated
   */
  @Valid 
  @Schema(name = "lastUpdated", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("lastUpdated")
  public OffsetDateTime getLastUpdated() {
    return lastUpdated;
  }

  public void setLastUpdated(OffsetDateTime lastUpdated) {
    this.lastUpdated = lastUpdated;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    GetWorkspaceStorageStatsResponse getWorkspaceStorageStatsResponse = (GetWorkspaceStorageStatsResponse) o;
    return Objects.equals(this.totalStorageUsed, getWorkspaceStorageStatsResponse.totalStorageUsed) &&
        Objects.equals(this.storageQuota, getWorkspaceStorageStatsResponse.storageQuota) &&
        Objects.equals(this.usagePercentage, getWorkspaceStorageStatsResponse.usagePercentage) &&
        Objects.equals(this.storageByType, getWorkspaceStorageStatsResponse.storageByType) &&
        Objects.equals(this.totalFiles, getWorkspaceStorageStatsResponse.totalFiles) &&
        Objects.equals(this.totalFolders, getWorkspaceStorageStatsResponse.totalFolders) &&
        Objects.equals(this.lastUpdated, getWorkspaceStorageStatsResponse.lastUpdated);
  }

  @Override
  public int hashCode() {
    return Objects.hash(totalStorageUsed, storageQuota, usagePercentage, storageByType, totalFiles, totalFolders, lastUpdated);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class GetWorkspaceStorageStatsResponse {\n");
    sb.append("    totalStorageUsed: ").append(toIndentedString(totalStorageUsed)).append("\n");
    sb.append("    storageQuota: ").append(toIndentedString(storageQuota)).append("\n");
    sb.append("    usagePercentage: ").append(toIndentedString(usagePercentage)).append("\n");
    sb.append("    storageByType: ").append(toIndentedString(storageByType)).append("\n");
    sb.append("    totalFiles: ").append(toIndentedString(totalFiles)).append("\n");
    sb.append("    totalFolders: ").append(toIndentedString(totalFolders)).append("\n");
    sb.append("    lastUpdated: ").append(toIndentedString(lastUpdated)).append("\n");
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

