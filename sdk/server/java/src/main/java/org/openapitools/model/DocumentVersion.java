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
import org.openapitools.model.ChangeSet;
import org.openapitools.model.ContextualSummary;
import org.openapitools.model.DocumentBranch;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * DocumentVersion
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-04T01:38:18.342811-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class DocumentVersion {

  private String id;

  private String versionHash;

  private String parentHash;

  private String commitMessage;

  private String authorId;

  private String branchName;

  @Valid
  private Map<String, String> metadata = new HashMap<>();

  @Valid
  private List<String> tags = new ArrayList<>();

  private Boolean isApproved;

  private String approvalChain;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @Valid
  private List<@Valid DocumentBranch> branches = new ArrayList<>();

  @Valid
  private List<@Valid ChangeSet> changes = new ArrayList<>();

  @Valid
  private List<@Valid ContextualSummary> summaries = new ArrayList<>();

  public DocumentVersion id(String id) {
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

  public DocumentVersion versionHash(String versionHash) {
    this.versionHash = versionHash;
    return this;
  }

  /**
   * Get versionHash
   * @return versionHash
   */
  
  @Schema(name = "versionHash", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("versionHash")
  public String getVersionHash() {
    return versionHash;
  }

  public void setVersionHash(String versionHash) {
    this.versionHash = versionHash;
  }

  public DocumentVersion parentHash(String parentHash) {
    this.parentHash = parentHash;
    return this;
  }

  /**
   * Get parentHash
   * @return parentHash
   */
  
  @Schema(name = "parentHash", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("parentHash")
  public String getParentHash() {
    return parentHash;
  }

  public void setParentHash(String parentHash) {
    this.parentHash = parentHash;
  }

  public DocumentVersion commitMessage(String commitMessage) {
    this.commitMessage = commitMessage;
    return this;
  }

  /**
   * Get commitMessage
   * @return commitMessage
   */
  
  @Schema(name = "commitMessage", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("commitMessage")
  public String getCommitMessage() {
    return commitMessage;
  }

  public void setCommitMessage(String commitMessage) {
    this.commitMessage = commitMessage;
  }

  public DocumentVersion authorId(String authorId) {
    this.authorId = authorId;
    return this;
  }

  /**
   * Get authorId
   * @return authorId
   */
  
  @Schema(name = "authorId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("authorId")
  public String getAuthorId() {
    return authorId;
  }

  public void setAuthorId(String authorId) {
    this.authorId = authorId;
  }

  public DocumentVersion branchName(String branchName) {
    this.branchName = branchName;
    return this;
  }

  /**
   * Get branchName
   * @return branchName
   */
  
  @Schema(name = "branchName", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("branchName")
  public String getBranchName() {
    return branchName;
  }

  public void setBranchName(String branchName) {
    this.branchName = branchName;
  }

  public DocumentVersion metadata(Map<String, String> metadata) {
    this.metadata = metadata;
    return this;
  }

  public DocumentVersion putMetadataItem(String key, String metadataItem) {
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

  public DocumentVersion tags(List<String> tags) {
    this.tags = tags;
    return this;
  }

  public DocumentVersion addTagsItem(String tagsItem) {
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

  public DocumentVersion isApproved(Boolean isApproved) {
    this.isApproved = isApproved;
    return this;
  }

  /**
   * Get isApproved
   * @return isApproved
   */
  
  @Schema(name = "isApproved", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("isApproved")
  public Boolean getIsApproved() {
    return isApproved;
  }

  public void setIsApproved(Boolean isApproved) {
    this.isApproved = isApproved;
  }

  public DocumentVersion approvalChain(String approvalChain) {
    this.approvalChain = approvalChain;
    return this;
  }

  /**
   * Get approvalChain
   * @return approvalChain
   */
  
  @Schema(name = "approvalChain", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("approvalChain")
  public String getApprovalChain() {
    return approvalChain;
  }

  public void setApprovalChain(String approvalChain) {
    this.approvalChain = approvalChain;
  }

  public DocumentVersion createdAt(OffsetDateTime createdAt) {
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

  public DocumentVersion branches(List<@Valid DocumentBranch> branches) {
    this.branches = branches;
    return this;
  }

  public DocumentVersion addBranchesItem(DocumentBranch branchesItem) {
    if (this.branches == null) {
      this.branches = new ArrayList<>();
    }
    this.branches.add(branchesItem);
    return this;
  }

  /**
   * Get branches
   * @return branches
   */
  @Valid 
  @Schema(name = "branches", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("branches")
  public List<@Valid DocumentBranch> getBranches() {
    return branches;
  }

  public void setBranches(List<@Valid DocumentBranch> branches) {
    this.branches = branches;
  }

  public DocumentVersion changes(List<@Valid ChangeSet> changes) {
    this.changes = changes;
    return this;
  }

  public DocumentVersion addChangesItem(ChangeSet changesItem) {
    if (this.changes == null) {
      this.changes = new ArrayList<>();
    }
    this.changes.add(changesItem);
    return this;
  }

  /**
   * Get changes
   * @return changes
   */
  @Valid 
  @Schema(name = "changes", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("changes")
  public List<@Valid ChangeSet> getChanges() {
    return changes;
  }

  public void setChanges(List<@Valid ChangeSet> changes) {
    this.changes = changes;
  }

  public DocumentVersion summaries(List<@Valid ContextualSummary> summaries) {
    this.summaries = summaries;
    return this;
  }

  public DocumentVersion addSummariesItem(ContextualSummary summariesItem) {
    if (this.summaries == null) {
      this.summaries = new ArrayList<>();
    }
    this.summaries.add(summariesItem);
    return this;
  }

  /**
   * Get summaries
   * @return summaries
   */
  @Valid 
  @Schema(name = "summaries", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("summaries")
  public List<@Valid ContextualSummary> getSummaries() {
    return summaries;
  }

  public void setSummaries(List<@Valid ContextualSummary> summaries) {
    this.summaries = summaries;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    DocumentVersion documentVersion = (DocumentVersion) o;
    return Objects.equals(this.id, documentVersion.id) &&
        Objects.equals(this.versionHash, documentVersion.versionHash) &&
        Objects.equals(this.parentHash, documentVersion.parentHash) &&
        Objects.equals(this.commitMessage, documentVersion.commitMessage) &&
        Objects.equals(this.authorId, documentVersion.authorId) &&
        Objects.equals(this.branchName, documentVersion.branchName) &&
        Objects.equals(this.metadata, documentVersion.metadata) &&
        Objects.equals(this.tags, documentVersion.tags) &&
        Objects.equals(this.isApproved, documentVersion.isApproved) &&
        Objects.equals(this.approvalChain, documentVersion.approvalChain) &&
        Objects.equals(this.createdAt, documentVersion.createdAt) &&
        Objects.equals(this.branches, documentVersion.branches) &&
        Objects.equals(this.changes, documentVersion.changes) &&
        Objects.equals(this.summaries, documentVersion.summaries);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, versionHash, parentHash, commitMessage, authorId, branchName, metadata, tags, isApproved, approvalChain, createdAt, branches, changes, summaries);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class DocumentVersion {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    versionHash: ").append(toIndentedString(versionHash)).append("\n");
    sb.append("    parentHash: ").append(toIndentedString(parentHash)).append("\n");
    sb.append("    commitMessage: ").append(toIndentedString(commitMessage)).append("\n");
    sb.append("    authorId: ").append(toIndentedString(authorId)).append("\n");
    sb.append("    branchName: ").append(toIndentedString(branchName)).append("\n");
    sb.append("    metadata: ").append(toIndentedString(metadata)).append("\n");
    sb.append("    tags: ").append(toIndentedString(tags)).append("\n");
    sb.append("    isApproved: ").append(toIndentedString(isApproved)).append("\n");
    sb.append("    approvalChain: ").append(toIndentedString(approvalChain)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
    sb.append("    branches: ").append(toIndentedString(branches)).append("\n");
    sb.append("    changes: ").append(toIndentedString(changes)).append("\n");
    sb.append("    summaries: ").append(toIndentedString(summaries)).append("\n");
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

