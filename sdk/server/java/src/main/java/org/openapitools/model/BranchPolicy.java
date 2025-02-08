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
 * BranchPolicy
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-08T09:37:38.060433-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class BranchPolicy {

  private String id;

  private String branchId;

  @Valid
  private List<String> requiredApprovers = new ArrayList<>();

  private Integer minimumApprovals;

  private Boolean enforceLinearHistory;

  private Boolean allowForcePush;

  @Valid
  private List<String> protectedPaths = new ArrayList<>();

  @Valid
  private Map<String, String> mergeRules = new HashMap<>();

  @Valid
  private List<String> automatedChecks = new ArrayList<>();

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  public BranchPolicy id(String id) {
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

  public BranchPolicy branchId(String branchId) {
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

  public BranchPolicy requiredApprovers(List<String> requiredApprovers) {
    this.requiredApprovers = requiredApprovers;
    return this;
  }

  public BranchPolicy addRequiredApproversItem(String requiredApproversItem) {
    if (this.requiredApprovers == null) {
      this.requiredApprovers = new ArrayList<>();
    }
    this.requiredApprovers.add(requiredApproversItem);
    return this;
  }

  /**
   * Get requiredApprovers
   * @return requiredApprovers
   */
  
  @Schema(name = "requiredApprovers", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("requiredApprovers")
  public List<String> getRequiredApprovers() {
    return requiredApprovers;
  }

  public void setRequiredApprovers(List<String> requiredApprovers) {
    this.requiredApprovers = requiredApprovers;
  }

  public BranchPolicy minimumApprovals(Integer minimumApprovals) {
    this.minimumApprovals = minimumApprovals;
    return this;
  }

  /**
   * Get minimumApprovals
   * @return minimumApprovals
   */
  
  @Schema(name = "minimumApprovals", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("minimumApprovals")
  public Integer getMinimumApprovals() {
    return minimumApprovals;
  }

  public void setMinimumApprovals(Integer minimumApprovals) {
    this.minimumApprovals = minimumApprovals;
  }

  public BranchPolicy enforceLinearHistory(Boolean enforceLinearHistory) {
    this.enforceLinearHistory = enforceLinearHistory;
    return this;
  }

  /**
   * Get enforceLinearHistory
   * @return enforceLinearHistory
   */
  
  @Schema(name = "enforceLinearHistory", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("enforceLinearHistory")
  public Boolean getEnforceLinearHistory() {
    return enforceLinearHistory;
  }

  public void setEnforceLinearHistory(Boolean enforceLinearHistory) {
    this.enforceLinearHistory = enforceLinearHistory;
  }

  public BranchPolicy allowForcePush(Boolean allowForcePush) {
    this.allowForcePush = allowForcePush;
    return this;
  }

  /**
   * Get allowForcePush
   * @return allowForcePush
   */
  
  @Schema(name = "allowForcePush", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("allowForcePush")
  public Boolean getAllowForcePush() {
    return allowForcePush;
  }

  public void setAllowForcePush(Boolean allowForcePush) {
    this.allowForcePush = allowForcePush;
  }

  public BranchPolicy protectedPaths(List<String> protectedPaths) {
    this.protectedPaths = protectedPaths;
    return this;
  }

  public BranchPolicy addProtectedPathsItem(String protectedPathsItem) {
    if (this.protectedPaths == null) {
      this.protectedPaths = new ArrayList<>();
    }
    this.protectedPaths.add(protectedPathsItem);
    return this;
  }

  /**
   * Get protectedPaths
   * @return protectedPaths
   */
  
  @Schema(name = "protectedPaths", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("protectedPaths")
  public List<String> getProtectedPaths() {
    return protectedPaths;
  }

  public void setProtectedPaths(List<String> protectedPaths) {
    this.protectedPaths = protectedPaths;
  }

  public BranchPolicy mergeRules(Map<String, String> mergeRules) {
    this.mergeRules = mergeRules;
    return this;
  }

  public BranchPolicy putMergeRulesItem(String key, String mergeRulesItem) {
    if (this.mergeRules == null) {
      this.mergeRules = new HashMap<>();
    }
    this.mergeRules.put(key, mergeRulesItem);
    return this;
  }

  /**
   * Get mergeRules
   * @return mergeRules
   */
  
  @Schema(name = "mergeRules", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("mergeRules")
  public Map<String, String> getMergeRules() {
    return mergeRules;
  }

  public void setMergeRules(Map<String, String> mergeRules) {
    this.mergeRules = mergeRules;
  }

  public BranchPolicy automatedChecks(List<String> automatedChecks) {
    this.automatedChecks = automatedChecks;
    return this;
  }

  public BranchPolicy addAutomatedChecksItem(String automatedChecksItem) {
    if (this.automatedChecks == null) {
      this.automatedChecks = new ArrayList<>();
    }
    this.automatedChecks.add(automatedChecksItem);
    return this;
  }

  /**
   * Get automatedChecks
   * @return automatedChecks
   */
  
  @Schema(name = "automatedChecks", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("automatedChecks")
  public List<String> getAutomatedChecks() {
    return automatedChecks;
  }

  public void setAutomatedChecks(List<String> automatedChecks) {
    this.automatedChecks = automatedChecks;
  }

  public BranchPolicy createdAt(OffsetDateTime createdAt) {
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

  public BranchPolicy updatedAt(OffsetDateTime updatedAt) {
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
    BranchPolicy branchPolicy = (BranchPolicy) o;
    return Objects.equals(this.id, branchPolicy.id) &&
        Objects.equals(this.branchId, branchPolicy.branchId) &&
        Objects.equals(this.requiredApprovers, branchPolicy.requiredApprovers) &&
        Objects.equals(this.minimumApprovals, branchPolicy.minimumApprovals) &&
        Objects.equals(this.enforceLinearHistory, branchPolicy.enforceLinearHistory) &&
        Objects.equals(this.allowForcePush, branchPolicy.allowForcePush) &&
        Objects.equals(this.protectedPaths, branchPolicy.protectedPaths) &&
        Objects.equals(this.mergeRules, branchPolicy.mergeRules) &&
        Objects.equals(this.automatedChecks, branchPolicy.automatedChecks) &&
        Objects.equals(this.createdAt, branchPolicy.createdAt) &&
        Objects.equals(this.updatedAt, branchPolicy.updatedAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, branchId, requiredApprovers, minimumApprovals, enforceLinearHistory, allowForcePush, protectedPaths, mergeRules, automatedChecks, createdAt, updatedAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class BranchPolicy {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    branchId: ").append(toIndentedString(branchId)).append("\n");
    sb.append("    requiredApprovers: ").append(toIndentedString(requiredApprovers)).append("\n");
    sb.append("    minimumApprovals: ").append(toIndentedString(minimumApprovals)).append("\n");
    sb.append("    enforceLinearHistory: ").append(toIndentedString(enforceLinearHistory)).append("\n");
    sb.append("    allowForcePush: ").append(toIndentedString(allowForcePush)).append("\n");
    sb.append("    protectedPaths: ").append(toIndentedString(protectedPaths)).append("\n");
    sb.append("    mergeRules: ").append(toIndentedString(mergeRules)).append("\n");
    sb.append("    automatedChecks: ").append(toIndentedString(automatedChecks)).append("\n");
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

