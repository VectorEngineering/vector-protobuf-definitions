package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import java.time.OffsetDateTime;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import org.openapitools.model.ContractIntelligence;
import org.openapitools.model.DocumentInstance;
import org.openapitools.model.ExplanationBlock;
import org.openapitools.model.TemplateType;
import org.openapitools.model.TemplateVariable;
import org.openapitools.model.TemplateVersion;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * DocumentTemplate
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-05T06:54:24.429429-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class DocumentTemplate {

  private String id;

  private String name;

  private String description;

  private String industryType;

  private String documentType;

  private String baseContent;

  private Boolean isAdaptive;

  @Valid
  private Map<String, String> metadata = new HashMap<>();

  @Valid
  private List<String> supportedLanguages = new ArrayList<>();

  @Valid
  private List<String> requiredDataFields = new ArrayList<>();

  private String version;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  private TemplateType templateType = TemplateType.UNSPECIFIED;

  private String content;

  @Valid
  private List<String> detectedVariables = new ArrayList<>();

  @Valid
  private Map<String, String> variableMappings = new HashMap<>();

  private Boolean isContextAware;

  @Valid
  private List<String> supportedContexts = new ArrayList<>();

  @Valid
  private Map<String, String> formattingRules = new HashMap<>();

  @Valid
  private List<String> conditionalSections = new ArrayList<>();

  @Valid
  private List<@Valid TemplateVersion> versions = new ArrayList<>();

  @Valid
  private List<@Valid TemplateVariable> variables = new ArrayList<>();

  @Valid
  private List<@Valid DocumentInstance> instances = new ArrayList<>();

  @Valid
  private List<@Valid ExplanationBlock> explanations = new ArrayList<>();

  @Valid
  private List<@Valid ContractIntelligence> intelligence = new ArrayList<>();

  public DocumentTemplate id(String id) {
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

  public DocumentTemplate name(String name) {
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

  public DocumentTemplate description(String description) {
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

  public DocumentTemplate industryType(String industryType) {
    this.industryType = industryType;
    return this;
  }

  /**
   * Get industryType
   * @return industryType
   */
  
  @Schema(name = "industryType", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("industryType")
  public String getIndustryType() {
    return industryType;
  }

  public void setIndustryType(String industryType) {
    this.industryType = industryType;
  }

  public DocumentTemplate documentType(String documentType) {
    this.documentType = documentType;
    return this;
  }

  /**
   * Get documentType
   * @return documentType
   */
  
  @Schema(name = "documentType", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("documentType")
  public String getDocumentType() {
    return documentType;
  }

  public void setDocumentType(String documentType) {
    this.documentType = documentType;
  }

  public DocumentTemplate baseContent(String baseContent) {
    this.baseContent = baseContent;
    return this;
  }

  /**
   * Get baseContent
   * @return baseContent
   */
  
  @Schema(name = "baseContent", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("baseContent")
  public String getBaseContent() {
    return baseContent;
  }

  public void setBaseContent(String baseContent) {
    this.baseContent = baseContent;
  }

  public DocumentTemplate isAdaptive(Boolean isAdaptive) {
    this.isAdaptive = isAdaptive;
    return this;
  }

  /**
   * Get isAdaptive
   * @return isAdaptive
   */
  
  @Schema(name = "isAdaptive", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("isAdaptive")
  public Boolean getIsAdaptive() {
    return isAdaptive;
  }

  public void setIsAdaptive(Boolean isAdaptive) {
    this.isAdaptive = isAdaptive;
  }

  public DocumentTemplate metadata(Map<String, String> metadata) {
    this.metadata = metadata;
    return this;
  }

  public DocumentTemplate putMetadataItem(String key, String metadataItem) {
    if (this.metadata == null) {
      this.metadata = new HashMap<>();
    }
    this.metadata.put(key, metadataItem);
    return this;
  }

  /**
   * Get metadata
   * @return metadata
   */
  
  @Schema(name = "metadata", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("metadata")
  public Map<String, String> getMetadata() {
    return metadata;
  }

  public void setMetadata(Map<String, String> metadata) {
    this.metadata = metadata;
  }

  public DocumentTemplate supportedLanguages(List<String> supportedLanguages) {
    this.supportedLanguages = supportedLanguages;
    return this;
  }

  public DocumentTemplate addSupportedLanguagesItem(String supportedLanguagesItem) {
    if (this.supportedLanguages == null) {
      this.supportedLanguages = new ArrayList<>();
    }
    this.supportedLanguages.add(supportedLanguagesItem);
    return this;
  }

  /**
   * Get supportedLanguages
   * @return supportedLanguages
   */
  
  @Schema(name = "supportedLanguages", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("supportedLanguages")
  public List<String> getSupportedLanguages() {
    return supportedLanguages;
  }

  public void setSupportedLanguages(List<String> supportedLanguages) {
    this.supportedLanguages = supportedLanguages;
  }

  public DocumentTemplate requiredDataFields(List<String> requiredDataFields) {
    this.requiredDataFields = requiredDataFields;
    return this;
  }

  public DocumentTemplate addRequiredDataFieldsItem(String requiredDataFieldsItem) {
    if (this.requiredDataFields == null) {
      this.requiredDataFields = new ArrayList<>();
    }
    this.requiredDataFields.add(requiredDataFieldsItem);
    return this;
  }

  /**
   * Get requiredDataFields
   * @return requiredDataFields
   */
  
  @Schema(name = "requiredDataFields", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("requiredDataFields")
  public List<String> getRequiredDataFields() {
    return requiredDataFields;
  }

  public void setRequiredDataFields(List<String> requiredDataFields) {
    this.requiredDataFields = requiredDataFields;
  }

  public DocumentTemplate version(String version) {
    this.version = version;
    return this;
  }

  /**
   * Get version
   * @return version
   */
  
  @Schema(name = "version", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("version")
  public String getVersion() {
    return version;
  }

  public void setVersion(String version) {
    this.version = version;
  }

  public DocumentTemplate createdAt(OffsetDateTime createdAt) {
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

  public DocumentTemplate updatedAt(OffsetDateTime updatedAt) {
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

  public DocumentTemplate templateType(TemplateType templateType) {
    this.templateType = templateType;
    return this;
  }

  /**
   * Get templateType
   * @return templateType
   */
  @Valid 
  @Schema(name = "templateType", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("templateType")
  public TemplateType getTemplateType() {
    return templateType;
  }

  public void setTemplateType(TemplateType templateType) {
    this.templateType = templateType;
  }

  public DocumentTemplate content(String content) {
    this.content = content;
    return this;
  }

  /**
   * Get content
   * @return content
   */
  
  @Schema(name = "content", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("content")
  public String getContent() {
    return content;
  }

  public void setContent(String content) {
    this.content = content;
  }

  public DocumentTemplate detectedVariables(List<String> detectedVariables) {
    this.detectedVariables = detectedVariables;
    return this;
  }

  public DocumentTemplate addDetectedVariablesItem(String detectedVariablesItem) {
    if (this.detectedVariables == null) {
      this.detectedVariables = new ArrayList<>();
    }
    this.detectedVariables.add(detectedVariablesItem);
    return this;
  }

  /**
   * Get detectedVariables
   * @return detectedVariables
   */
  
  @Schema(name = "detectedVariables", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("detectedVariables")
  public List<String> getDetectedVariables() {
    return detectedVariables;
  }

  public void setDetectedVariables(List<String> detectedVariables) {
    this.detectedVariables = detectedVariables;
  }

  public DocumentTemplate variableMappings(Map<String, String> variableMappings) {
    this.variableMappings = variableMappings;
    return this;
  }

  public DocumentTemplate putVariableMappingsItem(String key, String variableMappingsItem) {
    if (this.variableMappings == null) {
      this.variableMappings = new HashMap<>();
    }
    this.variableMappings.put(key, variableMappingsItem);
    return this;
  }

  /**
   * Get variableMappings
   * @return variableMappings
   */
  
  @Schema(name = "variableMappings", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("variableMappings")
  public Map<String, String> getVariableMappings() {
    return variableMappings;
  }

  public void setVariableMappings(Map<String, String> variableMappings) {
    this.variableMappings = variableMappings;
  }

  public DocumentTemplate isContextAware(Boolean isContextAware) {
    this.isContextAware = isContextAware;
    return this;
  }

  /**
   * Get isContextAware
   * @return isContextAware
   */
  
  @Schema(name = "isContextAware", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("isContextAware")
  public Boolean getIsContextAware() {
    return isContextAware;
  }

  public void setIsContextAware(Boolean isContextAware) {
    this.isContextAware = isContextAware;
  }

  public DocumentTemplate supportedContexts(List<String> supportedContexts) {
    this.supportedContexts = supportedContexts;
    return this;
  }

  public DocumentTemplate addSupportedContextsItem(String supportedContextsItem) {
    if (this.supportedContexts == null) {
      this.supportedContexts = new ArrayList<>();
    }
    this.supportedContexts.add(supportedContextsItem);
    return this;
  }

  /**
   * Get supportedContexts
   * @return supportedContexts
   */
  
  @Schema(name = "supportedContexts", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("supportedContexts")
  public List<String> getSupportedContexts() {
    return supportedContexts;
  }

  public void setSupportedContexts(List<String> supportedContexts) {
    this.supportedContexts = supportedContexts;
  }

  public DocumentTemplate formattingRules(Map<String, String> formattingRules) {
    this.formattingRules = formattingRules;
    return this;
  }

  public DocumentTemplate putFormattingRulesItem(String key, String formattingRulesItem) {
    if (this.formattingRules == null) {
      this.formattingRules = new HashMap<>();
    }
    this.formattingRules.put(key, formattingRulesItem);
    return this;
  }

  /**
   * Get formattingRules
   * @return formattingRules
   */
  
  @Schema(name = "formattingRules", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("formattingRules")
  public Map<String, String> getFormattingRules() {
    return formattingRules;
  }

  public void setFormattingRules(Map<String, String> formattingRules) {
    this.formattingRules = formattingRules;
  }

  public DocumentTemplate conditionalSections(List<String> conditionalSections) {
    this.conditionalSections = conditionalSections;
    return this;
  }

  public DocumentTemplate addConditionalSectionsItem(String conditionalSectionsItem) {
    if (this.conditionalSections == null) {
      this.conditionalSections = new ArrayList<>();
    }
    this.conditionalSections.add(conditionalSectionsItem);
    return this;
  }

  /**
   * Get conditionalSections
   * @return conditionalSections
   */
  
  @Schema(name = "conditionalSections", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("conditionalSections")
  public List<String> getConditionalSections() {
    return conditionalSections;
  }

  public void setConditionalSections(List<String> conditionalSections) {
    this.conditionalSections = conditionalSections;
  }

  public DocumentTemplate versions(List<@Valid TemplateVersion> versions) {
    this.versions = versions;
    return this;
  }

  public DocumentTemplate addVersionsItem(TemplateVersion versionsItem) {
    if (this.versions == null) {
      this.versions = new ArrayList<>();
    }
    this.versions.add(versionsItem);
    return this;
  }

  /**
   * Get versions
   * @return versions
   */
  @Valid 
  @Schema(name = "versions", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("versions")
  public List<@Valid TemplateVersion> getVersions() {
    return versions;
  }

  public void setVersions(List<@Valid TemplateVersion> versions) {
    this.versions = versions;
  }

  public DocumentTemplate variables(List<@Valid TemplateVariable> variables) {
    this.variables = variables;
    return this;
  }

  public DocumentTemplate addVariablesItem(TemplateVariable variablesItem) {
    if (this.variables == null) {
      this.variables = new ArrayList<>();
    }
    this.variables.add(variablesItem);
    return this;
  }

  /**
   * Get variables
   * @return variables
   */
  @Valid 
  @Schema(name = "variables", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("variables")
  public List<@Valid TemplateVariable> getVariables() {
    return variables;
  }

  public void setVariables(List<@Valid TemplateVariable> variables) {
    this.variables = variables;
  }

  public DocumentTemplate instances(List<@Valid DocumentInstance> instances) {
    this.instances = instances;
    return this;
  }

  public DocumentTemplate addInstancesItem(DocumentInstance instancesItem) {
    if (this.instances == null) {
      this.instances = new ArrayList<>();
    }
    this.instances.add(instancesItem);
    return this;
  }

  /**
   * Get instances
   * @return instances
   */
  @Valid 
  @Schema(name = "instances", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("instances")
  public List<@Valid DocumentInstance> getInstances() {
    return instances;
  }

  public void setInstances(List<@Valid DocumentInstance> instances) {
    this.instances = instances;
  }

  public DocumentTemplate explanations(List<@Valid ExplanationBlock> explanations) {
    this.explanations = explanations;
    return this;
  }

  public DocumentTemplate addExplanationsItem(ExplanationBlock explanationsItem) {
    if (this.explanations == null) {
      this.explanations = new ArrayList<>();
    }
    this.explanations.add(explanationsItem);
    return this;
  }

  /**
   * Get explanations
   * @return explanations
   */
  @Valid 
  @Schema(name = "explanations", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("explanations")
  public List<@Valid ExplanationBlock> getExplanations() {
    return explanations;
  }

  public void setExplanations(List<@Valid ExplanationBlock> explanations) {
    this.explanations = explanations;
  }

  public DocumentTemplate intelligence(List<@Valid ContractIntelligence> intelligence) {
    this.intelligence = intelligence;
    return this;
  }

  public DocumentTemplate addIntelligenceItem(ContractIntelligence intelligenceItem) {
    if (this.intelligence == null) {
      this.intelligence = new ArrayList<>();
    }
    this.intelligence.add(intelligenceItem);
    return this;
  }

  /**
   * Get intelligence
   * @return intelligence
   */
  @Valid 
  @Schema(name = "intelligence", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("intelligence")
  public List<@Valid ContractIntelligence> getIntelligence() {
    return intelligence;
  }

  public void setIntelligence(List<@Valid ContractIntelligence> intelligence) {
    this.intelligence = intelligence;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    DocumentTemplate documentTemplate = (DocumentTemplate) o;
    return Objects.equals(this.id, documentTemplate.id) &&
        Objects.equals(this.name, documentTemplate.name) &&
        Objects.equals(this.description, documentTemplate.description) &&
        Objects.equals(this.industryType, documentTemplate.industryType) &&
        Objects.equals(this.documentType, documentTemplate.documentType) &&
        Objects.equals(this.baseContent, documentTemplate.baseContent) &&
        Objects.equals(this.isAdaptive, documentTemplate.isAdaptive) &&
        Objects.equals(this.metadata, documentTemplate.metadata) &&
        Objects.equals(this.supportedLanguages, documentTemplate.supportedLanguages) &&
        Objects.equals(this.requiredDataFields, documentTemplate.requiredDataFields) &&
        Objects.equals(this.version, documentTemplate.version) &&
        Objects.equals(this.createdAt, documentTemplate.createdAt) &&
        Objects.equals(this.updatedAt, documentTemplate.updatedAt) &&
        Objects.equals(this.templateType, documentTemplate.templateType) &&
        Objects.equals(this.content, documentTemplate.content) &&
        Objects.equals(this.detectedVariables, documentTemplate.detectedVariables) &&
        Objects.equals(this.variableMappings, documentTemplate.variableMappings) &&
        Objects.equals(this.isContextAware, documentTemplate.isContextAware) &&
        Objects.equals(this.supportedContexts, documentTemplate.supportedContexts) &&
        Objects.equals(this.formattingRules, documentTemplate.formattingRules) &&
        Objects.equals(this.conditionalSections, documentTemplate.conditionalSections) &&
        Objects.equals(this.versions, documentTemplate.versions) &&
        Objects.equals(this.variables, documentTemplate.variables) &&
        Objects.equals(this.instances, documentTemplate.instances) &&
        Objects.equals(this.explanations, documentTemplate.explanations) &&
        Objects.equals(this.intelligence, documentTemplate.intelligence);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, name, description, industryType, documentType, baseContent, isAdaptive, metadata, supportedLanguages, requiredDataFields, version, createdAt, updatedAt, templateType, content, detectedVariables, variableMappings, isContextAware, supportedContexts, formattingRules, conditionalSections, versions, variables, instances, explanations, intelligence);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class DocumentTemplate {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    name: ").append(toIndentedString(name)).append("\n");
    sb.append("    description: ").append(toIndentedString(description)).append("\n");
    sb.append("    industryType: ").append(toIndentedString(industryType)).append("\n");
    sb.append("    documentType: ").append(toIndentedString(documentType)).append("\n");
    sb.append("    baseContent: ").append(toIndentedString(baseContent)).append("\n");
    sb.append("    isAdaptive: ").append(toIndentedString(isAdaptive)).append("\n");
    sb.append("    metadata: ").append(toIndentedString(metadata)).append("\n");
    sb.append("    supportedLanguages: ").append(toIndentedString(supportedLanguages)).append("\n");
    sb.append("    requiredDataFields: ").append(toIndentedString(requiredDataFields)).append("\n");
    sb.append("    version: ").append(toIndentedString(version)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
    sb.append("    updatedAt: ").append(toIndentedString(updatedAt)).append("\n");
    sb.append("    templateType: ").append(toIndentedString(templateType)).append("\n");
    sb.append("    content: ").append(toIndentedString(content)).append("\n");
    sb.append("    detectedVariables: ").append(toIndentedString(detectedVariables)).append("\n");
    sb.append("    variableMappings: ").append(toIndentedString(variableMappings)).append("\n");
    sb.append("    isContextAware: ").append(toIndentedString(isContextAware)).append("\n");
    sb.append("    supportedContexts: ").append(toIndentedString(supportedContexts)).append("\n");
    sb.append("    formattingRules: ").append(toIndentedString(formattingRules)).append("\n");
    sb.append("    conditionalSections: ").append(toIndentedString(conditionalSections)).append("\n");
    sb.append("    versions: ").append(toIndentedString(versions)).append("\n");
    sb.append("    variables: ").append(toIndentedString(variables)).append("\n");
    sb.append("    instances: ").append(toIndentedString(instances)).append("\n");
    sb.append("    explanations: ").append(toIndentedString(explanations)).append("\n");
    sb.append("    intelligence: ").append(toIndentedString(intelligence)).append("\n");
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

