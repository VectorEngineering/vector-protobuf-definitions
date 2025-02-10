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
import org.openapitools.model.ComplianceCheck;
import org.openapitools.model.RiskAssessment;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * ContractIntelligence
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-09T19:46:06.918250-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ContractIntelligence {

  private String id;

  private String contractType;

  @Valid
  private Map<String, Float> riskScores = new HashMap<>();

  @Valid
  private Map<String, String> detectedClauses = new HashMap<>();

  @Valid
  private List<String> keyObligations = new ArrayList<>();

  @Valid
  private Map<String, Float> complianceScores = new HashMap<>();

  private String jurisdiction;

  @Valid
  private List<String> governingLaws = new ArrayList<>();

  @Valid
  private Map<String, String> semanticAnalysis = new HashMap<>();

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime analysisDate;

  @Valid
  private List<@Valid RiskAssessment> riskAssessments = new ArrayList<>();

  @Valid
  private List<@Valid ComplianceCheck> complianceChecks = new ArrayList<>();

  public ContractIntelligence id(String id) {
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

  public ContractIntelligence contractType(String contractType) {
    this.contractType = contractType;
    return this;
  }

  /**
   * Get contractType
   * @return contractType
   */
  
  @Schema(name = "contractType", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("contractType")
  public String getContractType() {
    return contractType;
  }

  public void setContractType(String contractType) {
    this.contractType = contractType;
  }

  public ContractIntelligence riskScores(Map<String, Float> riskScores) {
    this.riskScores = riskScores;
    return this;
  }

  public ContractIntelligence putRiskScoresItem(String key, Float riskScoresItem) {
    if (this.riskScores == null) {
      this.riskScores = new HashMap<>();
    }
    this.riskScores.put(key, riskScoresItem);
    return this;
  }

  /**
   * Get riskScores
   * @return riskScores
   */
  
  @Schema(name = "riskScores", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("riskScores")
  public Map<String, Float> getRiskScores() {
    return riskScores;
  }

  public void setRiskScores(Map<String, Float> riskScores) {
    this.riskScores = riskScores;
  }

  public ContractIntelligence detectedClauses(Map<String, String> detectedClauses) {
    this.detectedClauses = detectedClauses;
    return this;
  }

  public ContractIntelligence putDetectedClausesItem(String key, String detectedClausesItem) {
    if (this.detectedClauses == null) {
      this.detectedClauses = new HashMap<>();
    }
    this.detectedClauses.put(key, detectedClausesItem);
    return this;
  }

  /**
   * Get detectedClauses
   * @return detectedClauses
   */
  
  @Schema(name = "detectedClauses", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("detectedClauses")
  public Map<String, String> getDetectedClauses() {
    return detectedClauses;
  }

  public void setDetectedClauses(Map<String, String> detectedClauses) {
    this.detectedClauses = detectedClauses;
  }

  public ContractIntelligence keyObligations(List<String> keyObligations) {
    this.keyObligations = keyObligations;
    return this;
  }

  public ContractIntelligence addKeyObligationsItem(String keyObligationsItem) {
    if (this.keyObligations == null) {
      this.keyObligations = new ArrayList<>();
    }
    this.keyObligations.add(keyObligationsItem);
    return this;
  }

  /**
   * Get keyObligations
   * @return keyObligations
   */
  
  @Schema(name = "keyObligations", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("keyObligations")
  public List<String> getKeyObligations() {
    return keyObligations;
  }

  public void setKeyObligations(List<String> keyObligations) {
    this.keyObligations = keyObligations;
  }

  public ContractIntelligence complianceScores(Map<String, Float> complianceScores) {
    this.complianceScores = complianceScores;
    return this;
  }

  public ContractIntelligence putComplianceScoresItem(String key, Float complianceScoresItem) {
    if (this.complianceScores == null) {
      this.complianceScores = new HashMap<>();
    }
    this.complianceScores.put(key, complianceScoresItem);
    return this;
  }

  /**
   * Get complianceScores
   * @return complianceScores
   */
  
  @Schema(name = "complianceScores", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("complianceScores")
  public Map<String, Float> getComplianceScores() {
    return complianceScores;
  }

  public void setComplianceScores(Map<String, Float> complianceScores) {
    this.complianceScores = complianceScores;
  }

  public ContractIntelligence jurisdiction(String jurisdiction) {
    this.jurisdiction = jurisdiction;
    return this;
  }

  /**
   * Get jurisdiction
   * @return jurisdiction
   */
  
  @Schema(name = "jurisdiction", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("jurisdiction")
  public String getJurisdiction() {
    return jurisdiction;
  }

  public void setJurisdiction(String jurisdiction) {
    this.jurisdiction = jurisdiction;
  }

  public ContractIntelligence governingLaws(List<String> governingLaws) {
    this.governingLaws = governingLaws;
    return this;
  }

  public ContractIntelligence addGoverningLawsItem(String governingLawsItem) {
    if (this.governingLaws == null) {
      this.governingLaws = new ArrayList<>();
    }
    this.governingLaws.add(governingLawsItem);
    return this;
  }

  /**
   * Get governingLaws
   * @return governingLaws
   */
  
  @Schema(name = "governingLaws", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("governingLaws")
  public List<String> getGoverningLaws() {
    return governingLaws;
  }

  public void setGoverningLaws(List<String> governingLaws) {
    this.governingLaws = governingLaws;
  }

  public ContractIntelligence semanticAnalysis(Map<String, String> semanticAnalysis) {
    this.semanticAnalysis = semanticAnalysis;
    return this;
  }

  public ContractIntelligence putSemanticAnalysisItem(String key, String semanticAnalysisItem) {
    if (this.semanticAnalysis == null) {
      this.semanticAnalysis = new HashMap<>();
    }
    this.semanticAnalysis.put(key, semanticAnalysisItem);
    return this;
  }

  /**
   * Get semanticAnalysis
   * @return semanticAnalysis
   */
  
  @Schema(name = "semanticAnalysis", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("semanticAnalysis")
  public Map<String, String> getSemanticAnalysis() {
    return semanticAnalysis;
  }

  public void setSemanticAnalysis(Map<String, String> semanticAnalysis) {
    this.semanticAnalysis = semanticAnalysis;
  }

  public ContractIntelligence analysisDate(OffsetDateTime analysisDate) {
    this.analysisDate = analysisDate;
    return this;
  }

  /**
   * Get analysisDate
   * @return analysisDate
   */
  @Valid 
  @Schema(name = "analysisDate", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("analysisDate")
  public OffsetDateTime getAnalysisDate() {
    return analysisDate;
  }

  public void setAnalysisDate(OffsetDateTime analysisDate) {
    this.analysisDate = analysisDate;
  }

  public ContractIntelligence riskAssessments(List<@Valid RiskAssessment> riskAssessments) {
    this.riskAssessments = riskAssessments;
    return this;
  }

  public ContractIntelligence addRiskAssessmentsItem(RiskAssessment riskAssessmentsItem) {
    if (this.riskAssessments == null) {
      this.riskAssessments = new ArrayList<>();
    }
    this.riskAssessments.add(riskAssessmentsItem);
    return this;
  }

  /**
   * Get riskAssessments
   * @return riskAssessments
   */
  @Valid 
  @Schema(name = "riskAssessments", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("riskAssessments")
  public List<@Valid RiskAssessment> getRiskAssessments() {
    return riskAssessments;
  }

  public void setRiskAssessments(List<@Valid RiskAssessment> riskAssessments) {
    this.riskAssessments = riskAssessments;
  }

  public ContractIntelligence complianceChecks(List<@Valid ComplianceCheck> complianceChecks) {
    this.complianceChecks = complianceChecks;
    return this;
  }

  public ContractIntelligence addComplianceChecksItem(ComplianceCheck complianceChecksItem) {
    if (this.complianceChecks == null) {
      this.complianceChecks = new ArrayList<>();
    }
    this.complianceChecks.add(complianceChecksItem);
    return this;
  }

  /**
   * Get complianceChecks
   * @return complianceChecks
   */
  @Valid 
  @Schema(name = "complianceChecks", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("complianceChecks")
  public List<@Valid ComplianceCheck> getComplianceChecks() {
    return complianceChecks;
  }

  public void setComplianceChecks(List<@Valid ComplianceCheck> complianceChecks) {
    this.complianceChecks = complianceChecks;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    ContractIntelligence contractIntelligence = (ContractIntelligence) o;
    return Objects.equals(this.id, contractIntelligence.id) &&
        Objects.equals(this.contractType, contractIntelligence.contractType) &&
        Objects.equals(this.riskScores, contractIntelligence.riskScores) &&
        Objects.equals(this.detectedClauses, contractIntelligence.detectedClauses) &&
        Objects.equals(this.keyObligations, contractIntelligence.keyObligations) &&
        Objects.equals(this.complianceScores, contractIntelligence.complianceScores) &&
        Objects.equals(this.jurisdiction, contractIntelligence.jurisdiction) &&
        Objects.equals(this.governingLaws, contractIntelligence.governingLaws) &&
        Objects.equals(this.semanticAnalysis, contractIntelligence.semanticAnalysis) &&
        Objects.equals(this.analysisDate, contractIntelligence.analysisDate) &&
        Objects.equals(this.riskAssessments, contractIntelligence.riskAssessments) &&
        Objects.equals(this.complianceChecks, contractIntelligence.complianceChecks);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, contractType, riskScores, detectedClauses, keyObligations, complianceScores, jurisdiction, governingLaws, semanticAnalysis, analysisDate, riskAssessments, complianceChecks);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ContractIntelligence {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    contractType: ").append(toIndentedString(contractType)).append("\n");
    sb.append("    riskScores: ").append(toIndentedString(riskScores)).append("\n");
    sb.append("    detectedClauses: ").append(toIndentedString(detectedClauses)).append("\n");
    sb.append("    keyObligations: ").append(toIndentedString(keyObligations)).append("\n");
    sb.append("    complianceScores: ").append(toIndentedString(complianceScores)).append("\n");
    sb.append("    jurisdiction: ").append(toIndentedString(jurisdiction)).append("\n");
    sb.append("    governingLaws: ").append(toIndentedString(governingLaws)).append("\n");
    sb.append("    semanticAnalysis: ").append(toIndentedString(semanticAnalysis)).append("\n");
    sb.append("    analysisDate: ").append(toIndentedString(analysisDate)).append("\n");
    sb.append("    riskAssessments: ").append(toIndentedString(riskAssessments)).append("\n");
    sb.append("    complianceChecks: ").append(toIndentedString(complianceChecks)).append("\n");
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

