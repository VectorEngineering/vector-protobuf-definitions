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
 * ContextualSummary
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-19T11:32:59.900685-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ContextualSummary {

  private String id;

  private String versionId;

  private String summaryType;

  private String content;

  private String targetAudience;

  @Valid
  private List<String> keyPoints = new ArrayList<>();

  @Valid
  private Map<String, String> metadata = new HashMap<>();

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  public ContextualSummary id(String id) {
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

  public ContextualSummary versionId(String versionId) {
    this.versionId = versionId;
    return this;
  }

  /**
   * Get versionId
   * @return versionId
   */
  
  @Schema(name = "versionId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("versionId")
  public String getVersionId() {
    return versionId;
  }

  public void setVersionId(String versionId) {
    this.versionId = versionId;
  }

  public ContextualSummary summaryType(String summaryType) {
    this.summaryType = summaryType;
    return this;
  }

  /**
   * Get summaryType
   * @return summaryType
   */
  
  @Schema(name = "summaryType", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("summaryType")
  public String getSummaryType() {
    return summaryType;
  }

  public void setSummaryType(String summaryType) {
    this.summaryType = summaryType;
  }

  public ContextualSummary content(String content) {
    this.content = content;
    return this;
  }

  /**
   * Get content
   * @return content
   */
  
  @Schema(name = "content", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("content")
  public String getContent() {
    return content;
  }

  public void setContent(String content) {
    this.content = content;
  }

  public ContextualSummary targetAudience(String targetAudience) {
    this.targetAudience = targetAudience;
    return this;
  }

  /**
   * Get targetAudience
   * @return targetAudience
   */
  
  @Schema(name = "targetAudience", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("targetAudience")
  public String getTargetAudience() {
    return targetAudience;
  }

  public void setTargetAudience(String targetAudience) {
    this.targetAudience = targetAudience;
  }

  public ContextualSummary keyPoints(List<String> keyPoints) {
    this.keyPoints = keyPoints;
    return this;
  }

  public ContextualSummary addKeyPointsItem(String keyPointsItem) {
    if (this.keyPoints == null) {
      this.keyPoints = new ArrayList<>();
    }
    this.keyPoints.add(keyPointsItem);
    return this;
  }

  /**
   * Get keyPoints
   * @return keyPoints
   */
  
  @Schema(name = "keyPoints", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("keyPoints")
  public List<String> getKeyPoints() {
    return keyPoints;
  }

  public void setKeyPoints(List<String> keyPoints) {
    this.keyPoints = keyPoints;
  }

  public ContextualSummary metadata(Map<String, String> metadata) {
    this.metadata = metadata;
    return this;
  }

  public ContextualSummary putMetadataItem(String key, String metadataItem) {
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

  public ContextualSummary createdAt(OffsetDateTime createdAt) {
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

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    ContextualSummary contextualSummary = (ContextualSummary) o;
    return Objects.equals(this.id, contextualSummary.id) &&
        Objects.equals(this.versionId, contextualSummary.versionId) &&
        Objects.equals(this.summaryType, contextualSummary.summaryType) &&
        Objects.equals(this.content, contextualSummary.content) &&
        Objects.equals(this.targetAudience, contextualSummary.targetAudience) &&
        Objects.equals(this.keyPoints, contextualSummary.keyPoints) &&
        Objects.equals(this.metadata, contextualSummary.metadata) &&
        Objects.equals(this.createdAt, contextualSummary.createdAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, versionId, summaryType, content, targetAudience, keyPoints, metadata, createdAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ContextualSummary {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    versionId: ").append(toIndentedString(versionId)).append("\n");
    sb.append("    summaryType: ").append(toIndentedString(summaryType)).append("\n");
    sb.append("    content: ").append(toIndentedString(content)).append("\n");
    sb.append("    targetAudience: ").append(toIndentedString(targetAudience)).append("\n");
    sb.append("    keyPoints: ").append(toIndentedString(keyPoints)).append("\n");
    sb.append("    metadata: ").append(toIndentedString(metadata)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
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

