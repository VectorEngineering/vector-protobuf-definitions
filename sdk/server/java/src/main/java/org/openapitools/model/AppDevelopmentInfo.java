package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * AppDevelopmentInfo
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-13T17:57:23.762503-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class AppDevelopmentInfo {

  private String id;

  private String appId;

  @Valid
  private List<String> supportedLanguages = new ArrayList<>();

  @Valid
  private List<String> sdkFeatures = new ArrayList<>();

  @Valid
  private Map<String, Float> testCoverage = new HashMap<>();

  @Valid
  private List<String> securityScans = new ArrayList<>();

  @Valid
  private Map<String, String> performanceMetrics = new HashMap<>();

  private String developmentStatus;

  @Valid
  private List<String> knownIssues = new ArrayList<>();

  private String roadmapUrl;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime lastSecurityAudit;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  public AppDevelopmentInfo id(String id) {
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

  public AppDevelopmentInfo appId(String appId) {
    this.appId = appId;
    return this;
  }

  /**
   * Get appId
   * @return appId
   */
  
  @Schema(name = "appId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("appId")
  public String getAppId() {
    return appId;
  }

  public void setAppId(String appId) {
    this.appId = appId;
  }

  public AppDevelopmentInfo supportedLanguages(List<String> supportedLanguages) {
    this.supportedLanguages = supportedLanguages;
    return this;
  }

  public AppDevelopmentInfo addSupportedLanguagesItem(String supportedLanguagesItem) {
    if (this.supportedLanguages == null) {
      this.supportedLanguages = new ArrayList<>();
    }
    this.supportedLanguages.add(supportedLanguagesItem);
    return this;
  }

  /**
   * Get supportedLanguages
   * @return supportedLanguages
   */
  
  @Schema(name = "supportedLanguages", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("supportedLanguages")
  public List<String> getSupportedLanguages() {
    return supportedLanguages;
  }

  public void setSupportedLanguages(List<String> supportedLanguages) {
    this.supportedLanguages = supportedLanguages;
  }

  public AppDevelopmentInfo sdkFeatures(List<String> sdkFeatures) {
    this.sdkFeatures = sdkFeatures;
    return this;
  }

  public AppDevelopmentInfo addSdkFeaturesItem(String sdkFeaturesItem) {
    if (this.sdkFeatures == null) {
      this.sdkFeatures = new ArrayList<>();
    }
    this.sdkFeatures.add(sdkFeaturesItem);
    return this;
  }

  /**
   * Get sdkFeatures
   * @return sdkFeatures
   */
  
  @Schema(name = "sdkFeatures", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("sdkFeatures")
  public List<String> getSdkFeatures() {
    return sdkFeatures;
  }

  public void setSdkFeatures(List<String> sdkFeatures) {
    this.sdkFeatures = sdkFeatures;
  }

  public AppDevelopmentInfo testCoverage(Map<String, Float> testCoverage) {
    this.testCoverage = testCoverage;
    return this;
  }

  public AppDevelopmentInfo putTestCoverageItem(String key, Float testCoverageItem) {
    if (this.testCoverage == null) {
      this.testCoverage = new HashMap<>();
    }
    this.testCoverage.put(key, testCoverageItem);
    return this;
  }

  /**
   * Get testCoverage
   * @return testCoverage
   */
  
  @Schema(name = "testCoverage", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("testCoverage")
  public Map<String, Float> getTestCoverage() {
    return testCoverage;
  }

  public void setTestCoverage(Map<String, Float> testCoverage) {
    this.testCoverage = testCoverage;
  }

  public AppDevelopmentInfo securityScans(List<String> securityScans) {
    this.securityScans = securityScans;
    return this;
  }

  public AppDevelopmentInfo addSecurityScansItem(String securityScansItem) {
    if (this.securityScans == null) {
      this.securityScans = new ArrayList<>();
    }
    this.securityScans.add(securityScansItem);
    return this;
  }

  /**
   * Get securityScans
   * @return securityScans
   */
  
  @Schema(name = "securityScans", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("securityScans")
  public List<String> getSecurityScans() {
    return securityScans;
  }

  public void setSecurityScans(List<String> securityScans) {
    this.securityScans = securityScans;
  }

  public AppDevelopmentInfo performanceMetrics(Map<String, String> performanceMetrics) {
    this.performanceMetrics = performanceMetrics;
    return this;
  }

  public AppDevelopmentInfo putPerformanceMetricsItem(String key, String performanceMetricsItem) {
    if (this.performanceMetrics == null) {
      this.performanceMetrics = new HashMap<>();
    }
    this.performanceMetrics.put(key, performanceMetricsItem);
    return this;
  }

  /**
   * Get performanceMetrics
   * @return performanceMetrics
   */
  
  @Schema(name = "performanceMetrics", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("performanceMetrics")
  public Map<String, String> getPerformanceMetrics() {
    return performanceMetrics;
  }

  public void setPerformanceMetrics(Map<String, String> performanceMetrics) {
    this.performanceMetrics = performanceMetrics;
  }

  public AppDevelopmentInfo developmentStatus(String developmentStatus) {
    this.developmentStatus = developmentStatus;
    return this;
  }

  /**
   * Get developmentStatus
   * @return developmentStatus
   */
  
  @Schema(name = "developmentStatus", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("developmentStatus")
  public String getDevelopmentStatus() {
    return developmentStatus;
  }

  public void setDevelopmentStatus(String developmentStatus) {
    this.developmentStatus = developmentStatus;
  }

  public AppDevelopmentInfo knownIssues(List<String> knownIssues) {
    this.knownIssues = knownIssues;
    return this;
  }

  public AppDevelopmentInfo addKnownIssuesItem(String knownIssuesItem) {
    if (this.knownIssues == null) {
      this.knownIssues = new ArrayList<>();
    }
    this.knownIssues.add(knownIssuesItem);
    return this;
  }

  /**
   * Get knownIssues
   * @return knownIssues
   */
  
  @Schema(name = "knownIssues", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("knownIssues")
  public List<String> getKnownIssues() {
    return knownIssues;
  }

  public void setKnownIssues(List<String> knownIssues) {
    this.knownIssues = knownIssues;
  }

  public AppDevelopmentInfo roadmapUrl(String roadmapUrl) {
    this.roadmapUrl = roadmapUrl;
    return this;
  }

  /**
   * Get roadmapUrl
   * @return roadmapUrl
   */
  
  @Schema(name = "roadmapUrl", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("roadmapUrl")
  public String getRoadmapUrl() {
    return roadmapUrl;
  }

  public void setRoadmapUrl(String roadmapUrl) {
    this.roadmapUrl = roadmapUrl;
  }

  public AppDevelopmentInfo lastSecurityAudit(OffsetDateTime lastSecurityAudit) {
    this.lastSecurityAudit = lastSecurityAudit;
    return this;
  }

  /**
   * Get lastSecurityAudit
   * @return lastSecurityAudit
   */
  @Valid 
  @Schema(name = "lastSecurityAudit", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("lastSecurityAudit")
  public OffsetDateTime getLastSecurityAudit() {
    return lastSecurityAudit;
  }

  public void setLastSecurityAudit(OffsetDateTime lastSecurityAudit) {
    this.lastSecurityAudit = lastSecurityAudit;
  }

  public AppDevelopmentInfo createdAt(OffsetDateTime createdAt) {
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

  public AppDevelopmentInfo updatedAt(OffsetDateTime updatedAt) {
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

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    AppDevelopmentInfo appDevelopmentInfo = (AppDevelopmentInfo) o;
    return Objects.equals(this.id, appDevelopmentInfo.id) &&
        Objects.equals(this.appId, appDevelopmentInfo.appId) &&
        Objects.equals(this.supportedLanguages, appDevelopmentInfo.supportedLanguages) &&
        Objects.equals(this.sdkFeatures, appDevelopmentInfo.sdkFeatures) &&
        Objects.equals(this.testCoverage, appDevelopmentInfo.testCoverage) &&
        Objects.equals(this.securityScans, appDevelopmentInfo.securityScans) &&
        Objects.equals(this.performanceMetrics, appDevelopmentInfo.performanceMetrics) &&
        Objects.equals(this.developmentStatus, appDevelopmentInfo.developmentStatus) &&
        Objects.equals(this.knownIssues, appDevelopmentInfo.knownIssues) &&
        Objects.equals(this.roadmapUrl, appDevelopmentInfo.roadmapUrl) &&
        Objects.equals(this.lastSecurityAudit, appDevelopmentInfo.lastSecurityAudit) &&
        Objects.equals(this.createdAt, appDevelopmentInfo.createdAt) &&
        Objects.equals(this.updatedAt, appDevelopmentInfo.updatedAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, appId, supportedLanguages, sdkFeatures, testCoverage, securityScans, performanceMetrics, developmentStatus, knownIssues, roadmapUrl, lastSecurityAudit, createdAt, updatedAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class AppDevelopmentInfo {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    appId: ").append(toIndentedString(appId)).append("\n");
    sb.append("    supportedLanguages: ").append(toIndentedString(supportedLanguages)).append("\n");
    sb.append("    sdkFeatures: ").append(toIndentedString(sdkFeatures)).append("\n");
    sb.append("    testCoverage: ").append(toIndentedString(testCoverage)).append("\n");
    sb.append("    securityScans: ").append(toIndentedString(securityScans)).append("\n");
    sb.append("    performanceMetrics: ").append(toIndentedString(performanceMetrics)).append("\n");
    sb.append("    developmentStatus: ").append(toIndentedString(developmentStatus)).append("\n");
    sb.append("    knownIssues: ").append(toIndentedString(knownIssues)).append("\n");
    sb.append("    roadmapUrl: ").append(toIndentedString(roadmapUrl)).append("\n");
    sb.append("    lastSecurityAudit: ").append(toIndentedString(lastSecurityAudit)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
    sb.append("    updatedAt: ").append(toIndentedString(updatedAt)).append("\n");
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

