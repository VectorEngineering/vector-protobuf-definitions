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
import org.openapitools.model.BranchMerge;
import org.openapitools.model.BranchPolicy;
import org.openapitools.model.MergeRequest;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * DocumentBranch
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T15:17:20.852245-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class DocumentBranch {

  private String id;

  private String name;

  private String baseVersionHash;

  private String currentHeadHash;

  private String purpose;

  private String status;

  @Valid
  private List<String> reviewers = new ArrayList<>();

  @Valid
  private Map<String, String> branchMetadata = new HashMap<>();

  private Boolean isLocked;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime lastUpdated;

  @Valid
  private List<@Valid BranchMerge> merges = new ArrayList<>();

  private BranchPolicy policy;

  @Valid
  private List<@Valid MergeRequest> mergeRequests = new ArrayList<>();

  public DocumentBranch id(String id) {
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

  public DocumentBranch name(String name) {
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

  public DocumentBranch baseVersionHash(String baseVersionHash) {
    this.baseVersionHash = baseVersionHash;
    return this;
  }

  /**
   * Get baseVersionHash
   * @return baseVersionHash
   */
  
  @Schema(name = "baseVersionHash", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("baseVersionHash")
  public String getBaseVersionHash() {
    return baseVersionHash;
  }

  public void setBaseVersionHash(String baseVersionHash) {
    this.baseVersionHash = baseVersionHash;
  }

  public DocumentBranch currentHeadHash(String currentHeadHash) {
    this.currentHeadHash = currentHeadHash;
    return this;
  }

  /**
   * Get currentHeadHash
   * @return currentHeadHash
   */
  
  @Schema(name = "currentHeadHash", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("currentHeadHash")
  public String getCurrentHeadHash() {
    return currentHeadHash;
  }

  public void setCurrentHeadHash(String currentHeadHash) {
    this.currentHeadHash = currentHeadHash;
  }

  public DocumentBranch purpose(String purpose) {
    this.purpose = purpose;
    return this;
  }

  /**
   * Get purpose
   * @return purpose
   */
  
  @Schema(name = "purpose", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("purpose")
  public String getPurpose() {
    return purpose;
  }

  public void setPurpose(String purpose) {
    this.purpose = purpose;
  }

  public DocumentBranch status(String status) {
    this.status = status;
    return this;
  }

  /**
   * Get status
   * @return status
   */
  
  @Schema(name = "status", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("status")
  public String getStatus() {
    return status;
  }

  public void setStatus(String status) {
    this.status = status;
  }

  public DocumentBranch reviewers(List<String> reviewers) {
    this.reviewers = reviewers;
    return this;
  }

  public DocumentBranch addReviewersItem(String reviewersItem) {
    if (this.reviewers == null) {
      this.reviewers = new ArrayList<>();
    }
    this.reviewers.add(reviewersItem);
    return this;
  }

  /**
   * Get reviewers
   * @return reviewers
   */
  
  @Schema(name = "reviewers", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("reviewers")
  public List<String> getReviewers() {
    return reviewers;
  }

  public void setReviewers(List<String> reviewers) {
    this.reviewers = reviewers;
  }

  public DocumentBranch branchMetadata(Map<String, String> branchMetadata) {
    this.branchMetadata = branchMetadata;
    return this;
  }

  public DocumentBranch putBranchMetadataItem(String key, String branchMetadataItem) {
    if (this.branchMetadata == null) {
      this.branchMetadata = new HashMap<>();
    }
    this.branchMetadata.put(key, branchMetadataItem);
    return this;
  }

  /**
   * Get branchMetadata
   * @return branchMetadata
   */
  
  @Schema(name = "branchMetadata", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("branchMetadata")
  public Map<String, String> getBranchMetadata() {
    return branchMetadata;
  }

  public void setBranchMetadata(Map<String, String> branchMetadata) {
    this.branchMetadata = branchMetadata;
  }

  public DocumentBranch isLocked(Boolean isLocked) {
    this.isLocked = isLocked;
    return this;
  }

  /**
   * Get isLocked
   * @return isLocked
   */
  
  @Schema(name = "isLocked", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("isLocked")
  public Boolean getIsLocked() {
    return isLocked;
  }

  public void setIsLocked(Boolean isLocked) {
    this.isLocked = isLocked;
  }

  public DocumentBranch lastUpdated(OffsetDateTime lastUpdated) {
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

  public DocumentBranch merges(List<@Valid BranchMerge> merges) {
    this.merges = merges;
    return this;
  }

  public DocumentBranch addMergesItem(BranchMerge mergesItem) {
    if (this.merges == null) {
      this.merges = new ArrayList<>();
    }
    this.merges.add(mergesItem);
    return this;
  }

  /**
   * Get merges
   * @return merges
   */
  @Valid 
  @Schema(name = "merges", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("merges")
  public List<@Valid BranchMerge> getMerges() {
    return merges;
  }

  public void setMerges(List<@Valid BranchMerge> merges) {
    this.merges = merges;
  }

  public DocumentBranch policy(BranchPolicy policy) {
    this.policy = policy;
    return this;
  }

  /**
   * Get policy
   * @return policy
   */
  @Valid 
  @Schema(name = "policy", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("policy")
  public BranchPolicy getPolicy() {
    return policy;
  }

  public void setPolicy(BranchPolicy policy) {
    this.policy = policy;
  }

  public DocumentBranch mergeRequests(List<@Valid MergeRequest> mergeRequests) {
    this.mergeRequests = mergeRequests;
    return this;
  }

  public DocumentBranch addMergeRequestsItem(MergeRequest mergeRequestsItem) {
    if (this.mergeRequests == null) {
      this.mergeRequests = new ArrayList<>();
    }
    this.mergeRequests.add(mergeRequestsItem);
    return this;
  }

  /**
   * Get mergeRequests
   * @return mergeRequests
   */
  @Valid 
  @Schema(name = "mergeRequests", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("mergeRequests")
  public List<@Valid MergeRequest> getMergeRequests() {
    return mergeRequests;
  }

  public void setMergeRequests(List<@Valid MergeRequest> mergeRequests) {
    this.mergeRequests = mergeRequests;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    DocumentBranch documentBranch = (DocumentBranch) o;
    return Objects.equals(this.id, documentBranch.id) &&
        Objects.equals(this.name, documentBranch.name) &&
        Objects.equals(this.baseVersionHash, documentBranch.baseVersionHash) &&
        Objects.equals(this.currentHeadHash, documentBranch.currentHeadHash) &&
        Objects.equals(this.purpose, documentBranch.purpose) &&
        Objects.equals(this.status, documentBranch.status) &&
        Objects.equals(this.reviewers, documentBranch.reviewers) &&
        Objects.equals(this.branchMetadata, documentBranch.branchMetadata) &&
        Objects.equals(this.isLocked, documentBranch.isLocked) &&
        Objects.equals(this.lastUpdated, documentBranch.lastUpdated) &&
        Objects.equals(this.merges, documentBranch.merges) &&
        Objects.equals(this.policy, documentBranch.policy) &&
        Objects.equals(this.mergeRequests, documentBranch.mergeRequests);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, name, baseVersionHash, currentHeadHash, purpose, status, reviewers, branchMetadata, isLocked, lastUpdated, merges, policy, mergeRequests);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class DocumentBranch {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    name: ").append(toIndentedString(name)).append("\n");
    sb.append("    baseVersionHash: ").append(toIndentedString(baseVersionHash)).append("\n");
    sb.append("    currentHeadHash: ").append(toIndentedString(currentHeadHash)).append("\n");
    sb.append("    purpose: ").append(toIndentedString(purpose)).append("\n");
    sb.append("    status: ").append(toIndentedString(status)).append("\n");
    sb.append("    reviewers: ").append(toIndentedString(reviewers)).append("\n");
    sb.append("    branchMetadata: ").append(toIndentedString(branchMetadata)).append("\n");
    sb.append("    isLocked: ").append(toIndentedString(isLocked)).append("\n");
    sb.append("    lastUpdated: ").append(toIndentedString(lastUpdated)).append("\n");
    sb.append("    merges: ").append(toIndentedString(merges)).append("\n");
    sb.append("    policy: ").append(toIndentedString(policy)).append("\n");
    sb.append("    mergeRequests: ").append(toIndentedString(mergeRequests)).append("\n");
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

