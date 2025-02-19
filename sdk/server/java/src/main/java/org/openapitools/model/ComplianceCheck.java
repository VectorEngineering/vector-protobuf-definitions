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
 * ComplianceCheck
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-19T01:49:19.350068-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ComplianceCheck {

  private String id;

  private String intelligenceId;

  private String complianceStandard;

  private String checkResult;

  @Valid
  private List<String> violations = new ArrayList<>();

  private String remediationSteps;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime checkDate;

  public ComplianceCheck id(String id) {
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

  public ComplianceCheck intelligenceId(String intelligenceId) {
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

  public ComplianceCheck complianceStandard(String complianceStandard) {
    this.complianceStandard = complianceStandard;
    return this;
  }

  /**
   * Get complianceStandard
   * @return complianceStandard
   */
  
  @Schema(name = "complianceStandard", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("complianceStandard")
  public String getComplianceStandard() {
    return complianceStandard;
  }

  public void setComplianceStandard(String complianceStandard) {
    this.complianceStandard = complianceStandard;
  }

  public ComplianceCheck checkResult(String checkResult) {
    this.checkResult = checkResult;
    return this;
  }

  /**
   * Get checkResult
   * @return checkResult
   */
  
  @Schema(name = "checkResult", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("checkResult")
  public String getCheckResult() {
    return checkResult;
  }

  public void setCheckResult(String checkResult) {
    this.checkResult = checkResult;
  }

  public ComplianceCheck violations(List<String> violations) {
    this.violations = violations;
    return this;
  }

  public ComplianceCheck addViolationsItem(String violationsItem) {
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
  
  @Schema(name = "violations", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("violations")
  public List<String> getViolations() {
    return violations;
  }

  public void setViolations(List<String> violations) {
    this.violations = violations;
  }

  public ComplianceCheck remediationSteps(String remediationSteps) {
    this.remediationSteps = remediationSteps;
    return this;
  }

  /**
   * Get remediationSteps
   * @return remediationSteps
   */
  
  @Schema(name = "remediationSteps", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("remediationSteps")
  public String getRemediationSteps() {
    return remediationSteps;
  }

  public void setRemediationSteps(String remediationSteps) {
    this.remediationSteps = remediationSteps;
  }

  public ComplianceCheck checkDate(OffsetDateTime checkDate) {
    this.checkDate = checkDate;
    return this;
  }

  /**
   * Get checkDate
   * @return checkDate
   */
  @Valid 
  @Schema(name = "checkDate", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("checkDate")
  public OffsetDateTime getCheckDate() {
    return checkDate;
  }

  public void setCheckDate(OffsetDateTime checkDate) {
    this.checkDate = checkDate;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    ComplianceCheck complianceCheck = (ComplianceCheck) o;
    return Objects.equals(this.id, complianceCheck.id) &&
        Objects.equals(this.intelligenceId, complianceCheck.intelligenceId) &&
        Objects.equals(this.complianceStandard, complianceCheck.complianceStandard) &&
        Objects.equals(this.checkResult, complianceCheck.checkResult) &&
        Objects.equals(this.violations, complianceCheck.violations) &&
        Objects.equals(this.remediationSteps, complianceCheck.remediationSteps) &&
        Objects.equals(this.checkDate, complianceCheck.checkDate);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, intelligenceId, complianceStandard, checkResult, violations, remediationSteps, checkDate);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ComplianceCheck {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    intelligenceId: ").append(toIndentedString(intelligenceId)).append("\n");
    sb.append("    complianceStandard: ").append(toIndentedString(complianceStandard)).append("\n");
    sb.append("    checkResult: ").append(toIndentedString(checkResult)).append("\n");
    sb.append("    violations: ").append(toIndentedString(violations)).append("\n");
    sb.append("    remediationSteps: ").append(toIndentedString(remediationSteps)).append("\n");
    sb.append("    checkDate: ").append(toIndentedString(checkDate)).append("\n");
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

