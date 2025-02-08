package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * ActivityMetrics
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-08T10:15:33.789499-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ActivityMetrics {

  private Integer totalFiles;

  private Integer totalFolders;

  private Integer activeUsers;

  private String storageUsed;

  private Float storageUsagePercentage;

  public ActivityMetrics totalFiles(Integer totalFiles) {
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

  public ActivityMetrics totalFolders(Integer totalFolders) {
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

  public ActivityMetrics activeUsers(Integer activeUsers) {
    this.activeUsers = activeUsers;
    return this;
  }

  /**
   * Get activeUsers
   * @return activeUsers
   */
  
  @Schema(name = "activeUsers", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("activeUsers")
  public Integer getActiveUsers() {
    return activeUsers;
  }

  public void setActiveUsers(Integer activeUsers) {
    this.activeUsers = activeUsers;
  }

  public ActivityMetrics storageUsed(String storageUsed) {
    this.storageUsed = storageUsed;
    return this;
  }

  /**
   * Get storageUsed
   * @return storageUsed
   */
  
  @Schema(name = "storageUsed", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("storageUsed")
  public String getStorageUsed() {
    return storageUsed;
  }

  public void setStorageUsed(String storageUsed) {
    this.storageUsed = storageUsed;
  }

  public ActivityMetrics storageUsagePercentage(Float storageUsagePercentage) {
    this.storageUsagePercentage = storageUsagePercentage;
    return this;
  }

  /**
   * Get storageUsagePercentage
   * @return storageUsagePercentage
   */
  
  @Schema(name = "storageUsagePercentage", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("storageUsagePercentage")
  public Float getStorageUsagePercentage() {
    return storageUsagePercentage;
  }

  public void setStorageUsagePercentage(Float storageUsagePercentage) {
    this.storageUsagePercentage = storageUsagePercentage;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    ActivityMetrics activityMetrics = (ActivityMetrics) o;
    return Objects.equals(this.totalFiles, activityMetrics.totalFiles) &&
        Objects.equals(this.totalFolders, activityMetrics.totalFolders) &&
        Objects.equals(this.activeUsers, activityMetrics.activeUsers) &&
        Objects.equals(this.storageUsed, activityMetrics.storageUsed) &&
        Objects.equals(this.storageUsagePercentage, activityMetrics.storageUsagePercentage);
  }

  @Override
  public int hashCode() {
    return Objects.hash(totalFiles, totalFolders, activeUsers, storageUsed, storageUsagePercentage);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ActivityMetrics {\n");
    sb.append("    totalFiles: ").append(toIndentedString(totalFiles)).append("\n");
    sb.append("    totalFolders: ").append(toIndentedString(totalFolders)).append("\n");
    sb.append("    activeUsers: ").append(toIndentedString(activeUsers)).append("\n");
    sb.append("    storageUsed: ").append(toIndentedString(storageUsed)).append("\n");
    sb.append("    storageUsagePercentage: ").append(toIndentedString(storageUsagePercentage)).append("\n");
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

