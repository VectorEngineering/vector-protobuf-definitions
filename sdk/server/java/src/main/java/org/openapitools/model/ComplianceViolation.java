package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * ComplianceViolation
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-03T15:12:04.029446-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ComplianceViolation {

  private String ruleId;

  private String severity;

  private String description;

  @Valid
  private List<String> affectedResources = new ArrayList<>();

  private String remediationSteps;

  public ComplianceViolation ruleId(String ruleId) {
    this.ruleId = ruleId;
    return this;
  }

  /**
   * Get ruleId
   * @return ruleId
   */
  
  @Schema(name = "ruleId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("ruleId")
  public String getRuleId() {
    return ruleId;
  }

  public void setRuleId(String ruleId) {
    this.ruleId = ruleId;
  }

  public ComplianceViolation severity(String severity) {
    this.severity = severity;
    return this;
  }

  /**
   * Get severity
   * @return severity
   */
  
  @Schema(name = "severity", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("severity")
  public String getSeverity() {
    return severity;
  }

  public void setSeverity(String severity) {
    this.severity = severity;
  }

  public ComplianceViolation description(String description) {
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

  public ComplianceViolation affectedResources(List<String> affectedResources) {
    this.affectedResources = affectedResources;
    return this;
  }

  public ComplianceViolation addAffectedResourcesItem(String affectedResourcesItem) {
    if (this.affectedResources == null) {
      this.affectedResources = new ArrayList<>();
    }
    this.affectedResources.add(affectedResourcesItem);
    return this;
  }

  /**
   * Get affectedResources
   * @return affectedResources
   */
  
  @Schema(name = "affectedResources", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("affectedResources")
  public List<String> getAffectedResources() {
    return affectedResources;
  }

  public void setAffectedResources(List<String> affectedResources) {
    this.affectedResources = affectedResources;
  }

  public ComplianceViolation remediationSteps(String remediationSteps) {
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

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    ComplianceViolation complianceViolation = (ComplianceViolation) o;
    return Objects.equals(this.ruleId, complianceViolation.ruleId) &&
        Objects.equals(this.severity, complianceViolation.severity) &&
        Objects.equals(this.description, complianceViolation.description) &&
        Objects.equals(this.affectedResources, complianceViolation.affectedResources) &&
        Objects.equals(this.remediationSteps, complianceViolation.remediationSteps);
  }

  @Override
  public int hashCode() {
    return Objects.hash(ruleId, severity, description, affectedResources, remediationSteps);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ComplianceViolation {\n");
    sb.append("    ruleId: ").append(toIndentedString(ruleId)).append("\n");
    sb.append("    severity: ").append(toIndentedString(severity)).append("\n");
    sb.append("    description: ").append(toIndentedString(description)).append("\n");
    sb.append("    affectedResources: ").append(toIndentedString(affectedResources)).append("\n");
    sb.append("    remediationSteps: ").append(toIndentedString(remediationSteps)).append("\n");
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

