package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import java.util.HashMap;
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
 * ChangeSet
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T12:21:38.405762-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ChangeSet {

  private String id;

  private String versionId;

  private String changeType;

  private String contentBefore;

  private String contentAfter;

  @Valid
  private Map<String, String> metadata = new HashMap<>();

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  public ChangeSet id(String id) {
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

  public ChangeSet versionId(String versionId) {
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

  public ChangeSet changeType(String changeType) {
    this.changeType = changeType;
    return this;
  }

  /**
   * Get changeType
   * @return changeType
   */
  
  @Schema(name = "changeType", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("changeType")
  public String getChangeType() {
    return changeType;
  }

  public void setChangeType(String changeType) {
    this.changeType = changeType;
  }

  public ChangeSet contentBefore(String contentBefore) {
    this.contentBefore = contentBefore;
    return this;
  }

  /**
   * Get contentBefore
   * @return contentBefore
   */
  
  @Schema(name = "contentBefore", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("contentBefore")
  public String getContentBefore() {
    return contentBefore;
  }

  public void setContentBefore(String contentBefore) {
    this.contentBefore = contentBefore;
  }

  public ChangeSet contentAfter(String contentAfter) {
    this.contentAfter = contentAfter;
    return this;
  }

  /**
   * Get contentAfter
   * @return contentAfter
   */
  
  @Schema(name = "contentAfter", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("contentAfter")
  public String getContentAfter() {
    return contentAfter;
  }

  public void setContentAfter(String contentAfter) {
    this.contentAfter = contentAfter;
  }

  public ChangeSet metadata(Map<String, String> metadata) {
    this.metadata = metadata;
    return this;
  }

  public ChangeSet putMetadataItem(String key, String metadataItem) {
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

  public ChangeSet createdAt(OffsetDateTime createdAt) {
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
    ChangeSet changeSet = (ChangeSet) o;
    return Objects.equals(this.id, changeSet.id) &&
        Objects.equals(this.versionId, changeSet.versionId) &&
        Objects.equals(this.changeType, changeSet.changeType) &&
        Objects.equals(this.contentBefore, changeSet.contentBefore) &&
        Objects.equals(this.contentAfter, changeSet.contentAfter) &&
        Objects.equals(this.metadata, changeSet.metadata) &&
        Objects.equals(this.createdAt, changeSet.createdAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, versionId, changeType, contentBefore, contentAfter, metadata, createdAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ChangeSet {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    versionId: ").append(toIndentedString(versionId)).append("\n");
    sb.append("    changeType: ").append(toIndentedString(changeType)).append("\n");
    sb.append("    contentBefore: ").append(toIndentedString(contentBefore)).append("\n");
    sb.append("    contentAfter: ").append(toIndentedString(contentAfter)).append("\n");
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

