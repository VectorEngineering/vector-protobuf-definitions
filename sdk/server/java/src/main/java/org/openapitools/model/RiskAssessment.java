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
 * RiskAssessment
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-09T13:53:28.529590-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class RiskAssessment {

  private String id;

  private String intelligenceId;

  private String riskType;

  private Float riskScore;

  private String assessmentDetails;

  @Valid
  private List<String> mitigationSuggestions = new ArrayList<>();

  private String assessorId;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  public RiskAssessment id(String id) {
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

  public RiskAssessment intelligenceId(String intelligenceId) {
    this.intelligenceId = intelligenceId;
    return this;
  }

  /**
   * Get intelligenceId
   * @return intelligenceId
   */
  
  @Schema(name = "intelligenceId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("intelligenceId")
  public String getIntelligenceId() {
    return intelligenceId;
  }

  public void setIntelligenceId(String intelligenceId) {
    this.intelligenceId = intelligenceId;
  }

  public RiskAssessment riskType(String riskType) {
    this.riskType = riskType;
    return this;
  }

  /**
   * Get riskType
   * @return riskType
   */
  
  @Schema(name = "riskType", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("riskType")
  public String getRiskType() {
    return riskType;
  }

  public void setRiskType(String riskType) {
    this.riskType = riskType;
  }

  public RiskAssessment riskScore(Float riskScore) {
    this.riskScore = riskScore;
    return this;
  }

  /**
   * Get riskScore
   * @return riskScore
   */
  
  @Schema(name = "riskScore", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("riskScore")
  public Float getRiskScore() {
    return riskScore;
  }

  public void setRiskScore(Float riskScore) {
    this.riskScore = riskScore;
  }

  public RiskAssessment assessmentDetails(String assessmentDetails) {
    this.assessmentDetails = assessmentDetails;
    return this;
  }

  /**
   * Get assessmentDetails
   * @return assessmentDetails
   */
  
  @Schema(name = "assessmentDetails", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("assessmentDetails")
  public String getAssessmentDetails() {
    return assessmentDetails;
  }

  public void setAssessmentDetails(String assessmentDetails) {
    this.assessmentDetails = assessmentDetails;
  }

  public RiskAssessment mitigationSuggestions(List<String> mitigationSuggestions) {
    this.mitigationSuggestions = mitigationSuggestions;
    return this;
  }

  public RiskAssessment addMitigationSuggestionsItem(String mitigationSuggestionsItem) {
    if (this.mitigationSuggestions == null) {
      this.mitigationSuggestions = new ArrayList<>();
    }
    this.mitigationSuggestions.add(mitigationSuggestionsItem);
    return this;
  }

  /**
   * Get mitigationSuggestions
   * @return mitigationSuggestions
   */
  
  @Schema(name = "mitigationSuggestions", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("mitigationSuggestions")
  public List<String> getMitigationSuggestions() {
    return mitigationSuggestions;
  }

  public void setMitigationSuggestions(List<String> mitigationSuggestions) {
    this.mitigationSuggestions = mitigationSuggestions;
  }

  public RiskAssessment assessorId(String assessorId) {
    this.assessorId = assessorId;
    return this;
  }

  /**
   * Get assessorId
   * @return assessorId
   */
  
  @Schema(name = "assessorId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("assessorId")
  public String getAssessorId() {
    return assessorId;
  }

  public void setAssessorId(String assessorId) {
    this.assessorId = assessorId;
  }

  public RiskAssessment createdAt(OffsetDateTime createdAt) {
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
    RiskAssessment riskAssessment = (RiskAssessment) o;
    return Objects.equals(this.id, riskAssessment.id) &&
        Objects.equals(this.intelligenceId, riskAssessment.intelligenceId) &&
        Objects.equals(this.riskType, riskAssessment.riskType) &&
        Objects.equals(this.riskScore, riskAssessment.riskScore) &&
        Objects.equals(this.assessmentDetails, riskAssessment.assessmentDetails) &&
        Objects.equals(this.mitigationSuggestions, riskAssessment.mitigationSuggestions) &&
        Objects.equals(this.assessorId, riskAssessment.assessorId) &&
        Objects.equals(this.createdAt, riskAssessment.createdAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, intelligenceId, riskType, riskScore, assessmentDetails, mitigationSuggestions, assessorId, createdAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class RiskAssessment {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    intelligenceId: ").append(toIndentedString(intelligenceId)).append("\n");
    sb.append("    riskType: ").append(toIndentedString(riskType)).append("\n");
    sb.append("    riskScore: ").append(toIndentedString(riskScore)).append("\n");
    sb.append("    assessmentDetails: ").append(toIndentedString(assessmentDetails)).append("\n");
    sb.append("    mitigationSuggestions: ").append(toIndentedString(mitigationSuggestions)).append("\n");
    sb.append("    assessorId: ").append(toIndentedString(assessorId)).append("\n");
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

