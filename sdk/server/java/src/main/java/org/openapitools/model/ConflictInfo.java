package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * ConflictInfo
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-08T10:25:12.747579-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ConflictInfo {

  private String resourceType;

  private String identifier;

  private String conflictReason;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  private String createdBy;

  public ConflictInfo resourceType(String resourceType) {
    this.resourceType = resourceType;
    return this;
  }

  /**
   * Get resourceType
   * @return resourceType
   */
  
  @Schema(name = "resourceType", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("resourceType")
  public String getResourceType() {
    return resourceType;
  }

  public void setResourceType(String resourceType) {
    this.resourceType = resourceType;
  }

  public ConflictInfo identifier(String identifier) {
    this.identifier = identifier;
    return this;
  }

  /**
   * Get identifier
   * @return identifier
   */
  
  @Schema(name = "identifier", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("identifier")
  public String getIdentifier() {
    return identifier;
  }

  public void setIdentifier(String identifier) {
    this.identifier = identifier;
  }

  public ConflictInfo conflictReason(String conflictReason) {
    this.conflictReason = conflictReason;
    return this;
  }

  /**
   * Get conflictReason
   * @return conflictReason
   */
  
  @Schema(name = "conflictReason", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("conflictReason")
  public String getConflictReason() {
    return conflictReason;
  }

  public void setConflictReason(String conflictReason) {
    this.conflictReason = conflictReason;
  }

  public ConflictInfo createdAt(OffsetDateTime createdAt) {
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

  public ConflictInfo createdBy(String createdBy) {
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

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    ConflictInfo conflictInfo = (ConflictInfo) o;
    return Objects.equals(this.resourceType, conflictInfo.resourceType) &&
        Objects.equals(this.identifier, conflictInfo.identifier) &&
        Objects.equals(this.conflictReason, conflictInfo.conflictReason) &&
        Objects.equals(this.createdAt, conflictInfo.createdAt) &&
        Objects.equals(this.createdBy, conflictInfo.createdBy);
  }

  @Override
  public int hashCode() {
    return Objects.hash(resourceType, identifier, conflictReason, createdAt, createdBy);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ConflictInfo {\n");
    sb.append("    resourceType: ").append(toIndentedString(resourceType)).append("\n");
    sb.append("    identifier: ").append(toIndentedString(identifier)).append("\n");
    sb.append("    conflictReason: ").append(toIndentedString(conflictReason)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
    sb.append("    createdBy: ").append(toIndentedString(createdBy)).append("\n");
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

