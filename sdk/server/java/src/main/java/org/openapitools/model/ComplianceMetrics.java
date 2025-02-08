package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * ComplianceMetrics
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-08T10:12:03.775072-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ComplianceMetrics {

  private Float gdprComplianceScore;

  private Float hipaaComplianceScore;

  private Integer pendingApprovals;

  private Integer complianceViolations;

  public ComplianceMetrics gdprComplianceScore(Float gdprComplianceScore) {
    this.gdprComplianceScore = gdprComplianceScore;
    return this;
  }

  /**
   * Get gdprComplianceScore
   * @return gdprComplianceScore
   */
  
  @Schema(name = "gdprComplianceScore", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("gdprComplianceScore")
  public Float getGdprComplianceScore() {
    return gdprComplianceScore;
  }

  public void setGdprComplianceScore(Float gdprComplianceScore) {
    this.gdprComplianceScore = gdprComplianceScore;
  }

  public ComplianceMetrics hipaaComplianceScore(Float hipaaComplianceScore) {
    this.hipaaComplianceScore = hipaaComplianceScore;
    return this;
  }

  /**
   * Get hipaaComplianceScore
   * @return hipaaComplianceScore
   */
  
  @Schema(name = "hipaaComplianceScore", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("hipaaComplianceScore")
  public Float getHipaaComplianceScore() {
    return hipaaComplianceScore;
  }

  public void setHipaaComplianceScore(Float hipaaComplianceScore) {
    this.hipaaComplianceScore = hipaaComplianceScore;
  }

  public ComplianceMetrics pendingApprovals(Integer pendingApprovals) {
    this.pendingApprovals = pendingApprovals;
    return this;
  }

  /**
   * Get pendingApprovals
   * @return pendingApprovals
   */
  
  @Schema(name = "pendingApprovals", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("pendingApprovals")
  public Integer getPendingApprovals() {
    return pendingApprovals;
  }

  public void setPendingApprovals(Integer pendingApprovals) {
    this.pendingApprovals = pendingApprovals;
  }

  public ComplianceMetrics complianceViolations(Integer complianceViolations) {
    this.complianceViolations = complianceViolations;
    return this;
  }

  /**
   * Get complianceViolations
   * @return complianceViolations
   */
  
  @Schema(name = "complianceViolations", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("complianceViolations")
  public Integer getComplianceViolations() {
    return complianceViolations;
  }

  public void setComplianceViolations(Integer complianceViolations) {
    this.complianceViolations = complianceViolations;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    ComplianceMetrics complianceMetrics = (ComplianceMetrics) o;
    return Objects.equals(this.gdprComplianceScore, complianceMetrics.gdprComplianceScore) &&
        Objects.equals(this.hipaaComplianceScore, complianceMetrics.hipaaComplianceScore) &&
        Objects.equals(this.pendingApprovals, complianceMetrics.pendingApprovals) &&
        Objects.equals(this.complianceViolations, complianceMetrics.complianceViolations);
  }

  @Override
  public int hashCode() {
    return Objects.hash(gdprComplianceScore, hipaaComplianceScore, pendingApprovals, complianceViolations);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ComplianceMetrics {\n");
    sb.append("    gdprComplianceScore: ").append(toIndentedString(gdprComplianceScore)).append("\n");
    sb.append("    hipaaComplianceScore: ").append(toIndentedString(hipaaComplianceScore)).append("\n");
    sb.append("    pendingApprovals: ").append(toIndentedString(pendingApprovals)).append("\n");
    sb.append("    complianceViolations: ").append(toIndentedString(complianceViolations)).append("\n");
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

