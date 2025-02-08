package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.ComplianceScore;
import org.openapitools.model.ComplianceViolation;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * GetWorkspaceComplianceReportResponse
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-08T09:28:11.323460-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class GetWorkspaceComplianceReportResponse {

  private Float overallScore;

  private String complianceStatus;

  @Valid
  private List<@Valid ComplianceViolation> violations = new ArrayList<>();

  @Valid
  private List<@Valid ComplianceScore> categoryScores = new ArrayList<>();

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime reportGeneratedAt;

  private String certificationId;

  public GetWorkspaceComplianceReportResponse overallScore(Float overallScore) {
    this.overallScore = overallScore;
    return this;
  }

  /**
   * Get overallScore
   * @return overallScore
   */
  
  @Schema(name = "overallScore", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("overallScore")
  public Float getOverallScore() {
    return overallScore;
  }

  public void setOverallScore(Float overallScore) {
    this.overallScore = overallScore;
  }

  public GetWorkspaceComplianceReportResponse complianceStatus(String complianceStatus) {
    this.complianceStatus = complianceStatus;
    return this;
  }

  /**
   * Get complianceStatus
   * @return complianceStatus
   */
  
  @Schema(name = "complianceStatus", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("complianceStatus")
  public String getComplianceStatus() {
    return complianceStatus;
  }

  public void setComplianceStatus(String complianceStatus) {
    this.complianceStatus = complianceStatus;
  }

  public GetWorkspaceComplianceReportResponse violations(List<@Valid ComplianceViolation> violations) {
    this.violations = violations;
    return this;
  }

  public GetWorkspaceComplianceReportResponse addViolationsItem(ComplianceViolation violationsItem) {
    if (this.violations == null) {
      this.violations = new ArrayList<>();
    }
    this.violations.add(violationsItem);
    return this;
  }

  /**
   * Get violations
   * @return violations
   */
  @Valid 
  @Schema(name = "violations", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("violations")
  public List<@Valid ComplianceViolation> getViolations() {
    return violations;
  }

  public void setViolations(List<@Valid ComplianceViolation> violations) {
    this.violations = violations;
  }

  public GetWorkspaceComplianceReportResponse categoryScores(List<@Valid ComplianceScore> categoryScores) {
    this.categoryScores = categoryScores;
    return this;
  }

  public GetWorkspaceComplianceReportResponse addCategoryScoresItem(ComplianceScore categoryScoresItem) {
    if (this.categoryScores == null) {
      this.categoryScores = new ArrayList<>();
    }
    this.categoryScores.add(categoryScoresItem);
    return this;
  }

  /**
   * Get categoryScores
   * @return categoryScores
   */
  @Valid 
  @Schema(name = "categoryScores", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("categoryScores")
  public List<@Valid ComplianceScore> getCategoryScores() {
    return categoryScores;
  }

  public void setCategoryScores(List<@Valid ComplianceScore> categoryScores) {
    this.categoryScores = categoryScores;
  }

  public GetWorkspaceComplianceReportResponse reportGeneratedAt(OffsetDateTime reportGeneratedAt) {
    this.reportGeneratedAt = reportGeneratedAt;
    return this;
  }

  /**
   * Get reportGeneratedAt
   * @return reportGeneratedAt
   */
  @Valid 
  @Schema(name = "reportGeneratedAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("reportGeneratedAt")
  public OffsetDateTime getReportGeneratedAt() {
    return reportGeneratedAt;
  }

  public void setReportGeneratedAt(OffsetDateTime reportGeneratedAt) {
    this.reportGeneratedAt = reportGeneratedAt;
  }

  public GetWorkspaceComplianceReportResponse certificationId(String certificationId) {
    this.certificationId = certificationId;
    return this;
  }

  /**
   * Get certificationId
   * @return certificationId
   */
  
  @Schema(name = "certificationId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("certificationId")
  public String getCertificationId() {
    return certificationId;
  }

  public void setCertificationId(String certificationId) {
    this.certificationId = certificationId;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    GetWorkspaceComplianceReportResponse getWorkspaceComplianceReportResponse = (GetWorkspaceComplianceReportResponse) o;
    return Objects.equals(this.overallScore, getWorkspaceComplianceReportResponse.overallScore) &&
        Objects.equals(this.complianceStatus, getWorkspaceComplianceReportResponse.complianceStatus) &&
        Objects.equals(this.violations, getWorkspaceComplianceReportResponse.violations) &&
        Objects.equals(this.categoryScores, getWorkspaceComplianceReportResponse.categoryScores) &&
        Objects.equals(this.reportGeneratedAt, getWorkspaceComplianceReportResponse.reportGeneratedAt) &&
        Objects.equals(this.certificationId, getWorkspaceComplianceReportResponse.certificationId);
  }

  @Override
  public int hashCode() {
    return Objects.hash(overallScore, complianceStatus, violations, categoryScores, reportGeneratedAt, certificationId);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class GetWorkspaceComplianceReportResponse {\n");
    sb.append("    overallScore: ").append(toIndentedString(overallScore)).append("\n");
    sb.append("    complianceStatus: ").append(toIndentedString(complianceStatus)).append("\n");
    sb.append("    violations: ").append(toIndentedString(violations)).append("\n");
    sb.append("    categoryScores: ").append(toIndentedString(categoryScores)).append("\n");
    sb.append("    reportGeneratedAt: ").append(toIndentedString(reportGeneratedAt)).append("\n");
    sb.append("    certificationId: ").append(toIndentedString(certificationId)).append("\n");
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

