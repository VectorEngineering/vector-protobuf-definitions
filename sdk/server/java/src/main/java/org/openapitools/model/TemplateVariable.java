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
 * TemplateVariable
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-18T23:01:10.850512-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class TemplateVariable {

  private String id;

  private String name;

  private String description;

  private String variableType;

  private String defaultValue;

  private Boolean isRequired;

  private String validationRules;

  private String dataSource;

  private String aiExtractionRules;

  @Valid
  private List<String> alternatives = new ArrayList<>();

  private String templateId;

  public TemplateVariable id(String id) {
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

  public TemplateVariable name(String name) {
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

  public TemplateVariable description(String description) {
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

  public TemplateVariable variableType(String variableType) {
    this.variableType = variableType;
    return this;
  }

  /**
   * Get variableType
   * @return variableType
   */
  
  @Schema(name = "variableType", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("variableType")
  public String getVariableType() {
    return variableType;
  }

  public void setVariableType(String variableType) {
    this.variableType = variableType;
  }

  public TemplateVariable defaultValue(String defaultValue) {
    this.defaultValue = defaultValue;
    return this;
  }

  /**
   * Get defaultValue
   * @return defaultValue
   */
  
  @Schema(name = "defaultValue", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("defaultValue")
  public String getDefaultValue() {
    return defaultValue;
  }

  public void setDefaultValue(String defaultValue) {
    this.defaultValue = defaultValue;
  }

  public TemplateVariable isRequired(Boolean isRequired) {
    this.isRequired = isRequired;
    return this;
  }

  /**
   * Get isRequired
   * @return isRequired
   */
  
  @Schema(name = "isRequired", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("isRequired")
  public Boolean getIsRequired() {
    return isRequired;
  }

  public void setIsRequired(Boolean isRequired) {
    this.isRequired = isRequired;
  }

  public TemplateVariable validationRules(String validationRules) {
    this.validationRules = validationRules;
    return this;
  }

  /**
   * Get validationRules
   * @return validationRules
   */
  
  @Schema(name = "validationRules", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("validationRules")
  public String getValidationRules() {
    return validationRules;
  }

  public void setValidationRules(String validationRules) {
    this.validationRules = validationRules;
  }

  public TemplateVariable dataSource(String dataSource) {
    this.dataSource = dataSource;
    return this;
  }

  /**
   * Get dataSource
   * @return dataSource
   */
  
  @Schema(name = "dataSource", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("dataSource")
  public String getDataSource() {
    return dataSource;
  }

  public void setDataSource(String dataSource) {
    this.dataSource = dataSource;
  }

  public TemplateVariable aiExtractionRules(String aiExtractionRules) {
    this.aiExtractionRules = aiExtractionRules;
    return this;
  }

  /**
   * Get aiExtractionRules
   * @return aiExtractionRules
   */
  
  @Schema(name = "aiExtractionRules", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("aiExtractionRules")
  public String getAiExtractionRules() {
    return aiExtractionRules;
  }

  public void setAiExtractionRules(String aiExtractionRules) {
    this.aiExtractionRules = aiExtractionRules;
  }

  public TemplateVariable alternatives(List<String> alternatives) {
    this.alternatives = alternatives;
    return this;
  }

  public TemplateVariable addAlternativesItem(String alternativesItem) {
    if (this.alternatives == null) {
      this.alternatives = new ArrayList<>();
    }
    this.alternatives.add(alternativesItem);
    return this;
  }

  /**
   * Get alternatives
   * @return alternatives
   */
  
  @Schema(name = "alternatives", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("alternatives")
  public List<String> getAlternatives() {
    return alternatives;
  }

  public void setAlternatives(List<String> alternatives) {
    this.alternatives = alternatives;
  }

  public TemplateVariable templateId(String templateId) {
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

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    TemplateVariable templateVariable = (TemplateVariable) o;
    return Objects.equals(this.id, templateVariable.id) &&
        Objects.equals(this.name, templateVariable.name) &&
        Objects.equals(this.description, templateVariable.description) &&
        Objects.equals(this.variableType, templateVariable.variableType) &&
        Objects.equals(this.defaultValue, templateVariable.defaultValue) &&
        Objects.equals(this.isRequired, templateVariable.isRequired) &&
        Objects.equals(this.validationRules, templateVariable.validationRules) &&
        Objects.equals(this.dataSource, templateVariable.dataSource) &&
        Objects.equals(this.aiExtractionRules, templateVariable.aiExtractionRules) &&
        Objects.equals(this.alternatives, templateVariable.alternatives) &&
        Objects.equals(this.templateId, templateVariable.templateId);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, name, description, variableType, defaultValue, isRequired, validationRules, dataSource, aiExtractionRules, alternatives, templateId);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class TemplateVariable {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    name: ").append(toIndentedString(name)).append("\n");
    sb.append("    description: ").append(toIndentedString(description)).append("\n");
    sb.append("    variableType: ").append(toIndentedString(variableType)).append("\n");
    sb.append("    defaultValue: ").append(toIndentedString(defaultValue)).append("\n");
    sb.append("    isRequired: ").append(toIndentedString(isRequired)).append("\n");
    sb.append("    validationRules: ").append(toIndentedString(validationRules)).append("\n");
    sb.append("    dataSource: ").append(toIndentedString(dataSource)).append("\n");
    sb.append("    aiExtractionRules: ").append(toIndentedString(aiExtractionRules)).append("\n");
    sb.append("    alternatives: ").append(toIndentedString(alternatives)).append("\n");
    sb.append("    templateId: ").append(toIndentedString(templateId)).append("\n");
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

