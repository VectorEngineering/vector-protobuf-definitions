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
 * ExplanationBlock
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T15:22:25.234549-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ExplanationBlock {

  private String id;

  private String templateId;

  private String sectionIdentifier;

  private String explanationText;

  private String complexityLevel;

  @Valid
  private List<String> relatedTerms = new ArrayList<>();

  @Valid
  private List<String> legalReferences = new ArrayList<>();

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  public ExplanationBlock id(String id) {
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

  public ExplanationBlock templateId(String templateId) {
    this.templateId = templateId;
    return this;
  }

  /**
   * Get templateId
   * @return templateId
   */
  
  @Schema(name = "templateId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("templateId")
  public String getTemplateId() {
    return templateId;
  }

  public void setTemplateId(String templateId) {
    this.templateId = templateId;
  }

  public ExplanationBlock sectionIdentifier(String sectionIdentifier) {
    this.sectionIdentifier = sectionIdentifier;
    return this;
  }

  /**
   * Get sectionIdentifier
   * @return sectionIdentifier
   */
  
  @Schema(name = "sectionIdentifier", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("sectionIdentifier")
  public String getSectionIdentifier() {
    return sectionIdentifier;
  }

  public void setSectionIdentifier(String sectionIdentifier) {
    this.sectionIdentifier = sectionIdentifier;
  }

  public ExplanationBlock explanationText(String explanationText) {
    this.explanationText = explanationText;
    return this;
  }

  /**
   * Get explanationText
   * @return explanationText
   */
  
  @Schema(name = "explanationText", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("explanationText")
  public String getExplanationText() {
    return explanationText;
  }

  public void setExplanationText(String explanationText) {
    this.explanationText = explanationText;
  }

  public ExplanationBlock complexityLevel(String complexityLevel) {
    this.complexityLevel = complexityLevel;
    return this;
  }

  /**
   * Get complexityLevel
   * @return complexityLevel
   */
  
  @Schema(name = "complexityLevel", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("complexityLevel")
  public String getComplexityLevel() {
    return complexityLevel;
  }

  public void setComplexityLevel(String complexityLevel) {
    this.complexityLevel = complexityLevel;
  }

  public ExplanationBlock relatedTerms(List<String> relatedTerms) {
    this.relatedTerms = relatedTerms;
    return this;
  }

  public ExplanationBlock addRelatedTermsItem(String relatedTermsItem) {
    if (this.relatedTerms == null) {
      this.relatedTerms = new ArrayList<>();
    }
    this.relatedTerms.add(relatedTermsItem);
    return this;
  }

  /**
   * Get relatedTerms
   * @return relatedTerms
   */
  
  @Schema(name = "relatedTerms", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("relatedTerms")
  public List<String> getRelatedTerms() {
    return relatedTerms;
  }

  public void setRelatedTerms(List<String> relatedTerms) {
    this.relatedTerms = relatedTerms;
  }

  public ExplanationBlock legalReferences(List<String> legalReferences) {
    this.legalReferences = legalReferences;
    return this;
  }

  public ExplanationBlock addLegalReferencesItem(String legalReferencesItem) {
    if (this.legalReferences == null) {
      this.legalReferences = new ArrayList<>();
    }
    this.legalReferences.add(legalReferencesItem);
    return this;
  }

  /**
   * Get legalReferences
   * @return legalReferences
   */
  
  @Schema(name = "legalReferences", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("legalReferences")
  public List<String> getLegalReferences() {
    return legalReferences;
  }

  public void setLegalReferences(List<String> legalReferences) {
    this.legalReferences = legalReferences;
  }

  public ExplanationBlock createdAt(OffsetDateTime createdAt) {
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

  public ExplanationBlock updatedAt(OffsetDateTime updatedAt) {
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
    ExplanationBlock explanationBlock = (ExplanationBlock) o;
    return Objects.equals(this.id, explanationBlock.id) &&
        Objects.equals(this.templateId, explanationBlock.templateId) &&
        Objects.equals(this.sectionIdentifier, explanationBlock.sectionIdentifier) &&
        Objects.equals(this.explanationText, explanationBlock.explanationText) &&
        Objects.equals(this.complexityLevel, explanationBlock.complexityLevel) &&
        Objects.equals(this.relatedTerms, explanationBlock.relatedTerms) &&
        Objects.equals(this.legalReferences, explanationBlock.legalReferences) &&
        Objects.equals(this.createdAt, explanationBlock.createdAt) &&
        Objects.equals(this.updatedAt, explanationBlock.updatedAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, templateId, sectionIdentifier, explanationText, complexityLevel, relatedTerms, legalReferences, createdAt, updatedAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ExplanationBlock {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    templateId: ").append(toIndentedString(templateId)).append("\n");
    sb.append("    sectionIdentifier: ").append(toIndentedString(sectionIdentifier)).append("\n");
    sb.append("    explanationText: ").append(toIndentedString(explanationText)).append("\n");
    sb.append("    complexityLevel: ").append(toIndentedString(complexityLevel)).append("\n");
    sb.append("    relatedTerms: ").append(toIndentedString(relatedTerms)).append("\n");
    sb.append("    legalReferences: ").append(toIndentedString(legalReferences)).append("\n");
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

