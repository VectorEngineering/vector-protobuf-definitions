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
 * MergeRequest
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-08T10:12:03.775072-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class MergeRequest {

  private String id;

  private String sourceBranchId;

  private String targetBranchId;

  private String title;

  private String description;

  private String authorId;

  @Valid
  private List<String> reviewers = new ArrayList<>();

  private String status;

  @Valid
  private List<String> labels = new ArrayList<>();

  private Boolean hasConflicts;

  @Valid
  private List<String> automatedCheckResults = new ArrayList<>();

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  public MergeRequest id(String id) {
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

  public MergeRequest sourceBranchId(String sourceBranchId) {
    this.sourceBranchId = sourceBranchId;
    return this;
  }

  /**
   * Get sourceBranchId
   * @return sourceBranchId
   */
  
  @Schema(name = "sourceBranchId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("sourceBranchId")
  public String getSourceBranchId() {
    return sourceBranchId;
  }

  public void setSourceBranchId(String sourceBranchId) {
    this.sourceBranchId = sourceBranchId;
  }

  public MergeRequest targetBranchId(String targetBranchId) {
    this.targetBranchId = targetBranchId;
    return this;
  }

  /**
   * Get targetBranchId
   * @return targetBranchId
   */
  
  @Schema(name = "targetBranchId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("targetBranchId")
  public String getTargetBranchId() {
    return targetBranchId;
  }

  public void setTargetBranchId(String targetBranchId) {
    this.targetBranchId = targetBranchId;
  }

  public MergeRequest title(String title) {
    this.title = title;
    return this;
  }

  /**
   * Get title
   * @return title
   */
  
  @Schema(name = "title", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("title")
  public String getTitle() {
    return title;
  }

  public void setTitle(String title) {
    this.title = title;
  }

  public MergeRequest description(String description) {
    this.description = description;
    return this;
  }

  /**
   * Get description
   * @return description
   */
  
  @Schema(name = "description", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("description")
  public String getDescription() {
    return description;
  }

  public void setDescription(String description) {
    this.description = description;
  }

  public MergeRequest authorId(String authorId) {
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

  public MergeRequest reviewers(List<String> reviewers) {
    this.reviewers = reviewers;
    return this;
  }

  public MergeRequest addReviewersItem(String reviewersItem) {
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

  public MergeRequest status(String status) {
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

  public MergeRequest labels(List<String> labels) {
    this.labels = labels;
    return this;
  }

  public MergeRequest addLabelsItem(String labelsItem) {
    if (this.labels == null) {
      this.labels = new ArrayList<>();
    }
    this.labels.add(labelsItem);
    return this;
  }

  /**
   * Get labels
   * @return labels
   */
  
  @Schema(name = "labels", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("labels")
  public List<String> getLabels() {
    return labels;
  }

  public void setLabels(List<String> labels) {
    this.labels = labels;
  }

  public MergeRequest hasConflicts(Boolean hasConflicts) {
    this.hasConflicts = hasConflicts;
    return this;
  }

  /**
   * Get hasConflicts
   * @return hasConflicts
   */
  
  @Schema(name = "hasConflicts", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("hasConflicts")
  public Boolean getHasConflicts() {
    return hasConflicts;
  }

  public void setHasConflicts(Boolean hasConflicts) {
    this.hasConflicts = hasConflicts;
  }

  public MergeRequest automatedCheckResults(List<String> automatedCheckResults) {
    this.automatedCheckResults = automatedCheckResults;
    return this;
  }

  public MergeRequest addAutomatedCheckResultsItem(String automatedCheckResultsItem) {
    if (this.automatedCheckResults == null) {
      this.automatedCheckResults = new ArrayList<>();
    }
    this.automatedCheckResults.add(automatedCheckResultsItem);
    return this;
  }

  /**
   * Get automatedCheckResults
   * @return automatedCheckResults
   */
  
  @Schema(name = "automatedCheckResults", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("automatedCheckResults")
  public List<String> getAutomatedCheckResults() {
    return automatedCheckResults;
  }

  public void setAutomatedCheckResults(List<String> automatedCheckResults) {
    this.automatedCheckResults = automatedCheckResults;
  }

  public MergeRequest createdAt(OffsetDateTime createdAt) {
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

  public MergeRequest updatedAt(OffsetDateTime updatedAt) {
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
    MergeRequest mergeRequest = (MergeRequest) o;
    return Objects.equals(this.id, mergeRequest.id) &&
        Objects.equals(this.sourceBranchId, mergeRequest.sourceBranchId) &&
        Objects.equals(this.targetBranchId, mergeRequest.targetBranchId) &&
        Objects.equals(this.title, mergeRequest.title) &&
        Objects.equals(this.description, mergeRequest.description) &&
        Objects.equals(this.authorId, mergeRequest.authorId) &&
        Objects.equals(this.reviewers, mergeRequest.reviewers) &&
        Objects.equals(this.status, mergeRequest.status) &&
        Objects.equals(this.labels, mergeRequest.labels) &&
        Objects.equals(this.hasConflicts, mergeRequest.hasConflicts) &&
        Objects.equals(this.automatedCheckResults, mergeRequest.automatedCheckResults) &&
        Objects.equals(this.createdAt, mergeRequest.createdAt) &&
        Objects.equals(this.updatedAt, mergeRequest.updatedAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, sourceBranchId, targetBranchId, title, description, authorId, reviewers, status, labels, hasConflicts, automatedCheckResults, createdAt, updatedAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class MergeRequest {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    sourceBranchId: ").append(toIndentedString(sourceBranchId)).append("\n");
    sb.append("    targetBranchId: ").append(toIndentedString(targetBranchId)).append("\n");
    sb.append("    title: ").append(toIndentedString(title)).append("\n");
    sb.append("    description: ").append(toIndentedString(description)).append("\n");
    sb.append("    authorId: ").append(toIndentedString(authorId)).append("\n");
    sb.append("    reviewers: ").append(toIndentedString(reviewers)).append("\n");
    sb.append("    status: ").append(toIndentedString(status)).append("\n");
    sb.append("    labels: ").append(toIndentedString(labels)).append("\n");
    sb.append("    hasConflicts: ").append(toIndentedString(hasConflicts)).append("\n");
    sb.append("    automatedCheckResults: ").append(toIndentedString(automatedCheckResults)).append("\n");
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

