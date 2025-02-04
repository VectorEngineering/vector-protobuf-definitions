package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * BranchMerge
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-04T01:38:18.342811-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class BranchMerge {

  private String id;

  private String branchId;

  private String sourceBranch;

  private String targetBranch;

  private String mergeStatus;

  private String mergeStrategy;

  @Valid
  private List<String> conflicts = new ArrayList<>();

  private String mergerId;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime completedAt;

  public BranchMerge id(String id) {
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

  public BranchMerge branchId(String branchId) {
    this.branchId = branchId;
    return this;
  }

  /**
   * Get branchId
   * @return branchId
   */
  
  @Schema(name = "branchId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("branchId")
  public String getBranchId() {
    return branchId;
  }

  public void setBranchId(String branchId) {
    this.branchId = branchId;
  }

  public BranchMerge sourceBranch(String sourceBranch) {
    this.sourceBranch = sourceBranch;
    return this;
  }

  /**
   * Get sourceBranch
   * @return sourceBranch
   */
  
  @Schema(name = "sourceBranch", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("sourceBranch")
  public String getSourceBranch() {
    return sourceBranch;
  }

  public void setSourceBranch(String sourceBranch) {
    this.sourceBranch = sourceBranch;
  }

  public BranchMerge targetBranch(String targetBranch) {
    this.targetBranch = targetBranch;
    return this;
  }

  /**
   * Get targetBranch
   * @return targetBranch
   */
  
  @Schema(name = "targetBranch", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("targetBranch")
  public String getTargetBranch() {
    return targetBranch;
  }

  public void setTargetBranch(String targetBranch) {
    this.targetBranch = targetBranch;
  }

  public BranchMerge mergeStatus(String mergeStatus) {
    this.mergeStatus = mergeStatus;
    return this;
  }

  /**
   * Get mergeStatus
   * @return mergeStatus
   */
  
  @Schema(name = "mergeStatus", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("mergeStatus")
  public String getMergeStatus() {
    return mergeStatus;
  }

  public void setMergeStatus(String mergeStatus) {
    this.mergeStatus = mergeStatus;
  }

  public BranchMerge mergeStrategy(String mergeStrategy) {
    this.mergeStrategy = mergeStrategy;
    return this;
  }

  /**
   * Get mergeStrategy
   * @return mergeStrategy
   */
  
  @Schema(name = "mergeStrategy", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("mergeStrategy")
  public String getMergeStrategy() {
    return mergeStrategy;
  }

  public void setMergeStrategy(String mergeStrategy) {
    this.mergeStrategy = mergeStrategy;
  }

  public BranchMerge conflicts(List<String> conflicts) {
    this.conflicts = conflicts;
    return this;
  }

  public BranchMerge addConflictsItem(String conflictsItem) {
    if (this.conflicts == null) {
      this.conflicts = new ArrayList<>();
    }
    this.conflicts.add(conflictsItem);
    return this;
  }

  /**
   * Get conflicts
   * @return conflicts
   */
  
  @Schema(name = "conflicts", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("conflicts")
  public List<String> getConflicts() {
    return conflicts;
  }

  public void setConflicts(List<String> conflicts) {
    this.conflicts = conflicts;
  }

  public BranchMerge mergerId(String mergerId) {
    this.mergerId = mergerId;
    return this;
  }

  /**
   * Get mergerId
   * @return mergerId
   */
  
  @Schema(name = "mergerId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("mergerId")
  public String getMergerId() {
    return mergerId;
  }

  public void setMergerId(String mergerId) {
    this.mergerId = mergerId;
  }

  public BranchMerge createdAt(OffsetDateTime createdAt) {
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

  public BranchMerge completedAt(OffsetDateTime completedAt) {
    this.completedAt = completedAt;
    return this;
  }

  /**
   * Get completedAt
   * @return completedAt
   */
  @Valid 
  @Schema(name = "completedAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("completedAt")
  public OffsetDateTime getCompletedAt() {
    return completedAt;
  }

  public void setCompletedAt(OffsetDateTime completedAt) {
    this.completedAt = completedAt;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    BranchMerge branchMerge = (BranchMerge) o;
    return Objects.equals(this.id, branchMerge.id) &&
        Objects.equals(this.branchId, branchMerge.branchId) &&
        Objects.equals(this.sourceBranch, branchMerge.sourceBranch) &&
        Objects.equals(this.targetBranch, branchMerge.targetBranch) &&
        Objects.equals(this.mergeStatus, branchMerge.mergeStatus) &&
        Objects.equals(this.mergeStrategy, branchMerge.mergeStrategy) &&
        Objects.equals(this.conflicts, branchMerge.conflicts) &&
        Objects.equals(this.mergerId, branchMerge.mergerId) &&
        Objects.equals(this.createdAt, branchMerge.createdAt) &&
        Objects.equals(this.completedAt, branchMerge.completedAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, branchId, sourceBranch, targetBranch, mergeStatus, mergeStrategy, conflicts, mergerId, createdAt, completedAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class BranchMerge {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    branchId: ").append(toIndentedString(branchId)).append("\n");
    sb.append("    sourceBranch: ").append(toIndentedString(sourceBranch)).append("\n");
    sb.append("    targetBranch: ").append(toIndentedString(targetBranch)).append("\n");
    sb.append("    mergeStatus: ").append(toIndentedString(mergeStatus)).append("\n");
    sb.append("    mergeStrategy: ").append(toIndentedString(mergeStrategy)).append("\n");
    sb.append("    conflicts: ").append(toIndentedString(conflicts)).append("\n");
    sb.append("    mergerId: ").append(toIndentedString(mergerId)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
    sb.append("    completedAt: ").append(toIndentedString(completedAt)).append("\n");
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

