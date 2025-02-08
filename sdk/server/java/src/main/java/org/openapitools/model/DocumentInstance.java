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
import org.openapitools.model.AIAssistanceLog;
import org.openapitools.model.DocumentStatus;
import org.openapitools.model.DocumentVersion;
import org.openapitools.model.NegotiationHistory;
import org.openapitools.model.SignatureRequest;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * DocumentInstance
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-08T10:13:54.615102-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class DocumentInstance {

  private String id;

  private String templateId;

  private DocumentStatus status = DocumentStatus.UNSPECIFIED;

  @Valid
  private Map<String, String> fieldValues = new HashMap<>();

  @Valid
  private List<String> signers = new ArrayList<>();

  private String language;

  private Boolean isCompleted;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime dueDate;

  private String generatedContent;

  @Valid
  private List<String> attachments = new ArrayList<>();

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  @Valid
  private List<@Valid SignatureRequest> signatureRequests = new ArrayList<>();

  @Valid
  private List<@Valid AIAssistanceLog> aiAssistance = new ArrayList<>();

  private NegotiationHistory negotiation;

  @Valid
  private List<@Valid DocumentVersion> versions = new ArrayList<>();

  public DocumentInstance id(String id) {
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

  public DocumentInstance templateId(String templateId) {
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

  public DocumentInstance status(DocumentStatus status) {
    this.status = status;
    return this;
  }

  /**
   * Get status
   * @return status
   */
  @Valid 
  @Schema(name = "status", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("status")
  public DocumentStatus getStatus() {
    return status;
  }

  public void setStatus(DocumentStatus status) {
    this.status = status;
  }

  public DocumentInstance fieldValues(Map<String, String> fieldValues) {
    this.fieldValues = fieldValues;
    return this;
  }

  public DocumentInstance putFieldValuesItem(String key, String fieldValuesItem) {
    if (this.fieldValues == null) {
      this.fieldValues = new HashMap<>();
    }
    this.fieldValues.put(key, fieldValuesItem);
    return this;
  }

  /**
   * Get fieldValues
   * @return fieldValues
   */
  
  @Schema(name = "fieldValues", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("fieldValues")
  public Map<String, String> getFieldValues() {
    return fieldValues;
  }

  public void setFieldValues(Map<String, String> fieldValues) {
    this.fieldValues = fieldValues;
  }

  public DocumentInstance signers(List<String> signers) {
    this.signers = signers;
    return this;
  }

  public DocumentInstance addSignersItem(String signersItem) {
    if (this.signers == null) {
      this.signers = new ArrayList<>();
    }
    this.signers.add(signersItem);
    return this;
  }

  /**
   * Get signers
   * @return signers
   */
  
  @Schema(name = "signers", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("signers")
  public List<String> getSigners() {
    return signers;
  }

  public void setSigners(List<String> signers) {
    this.signers = signers;
  }

  public DocumentInstance language(String language) {
    this.language = language;
    return this;
  }

  /**
   * Get language
   * @return language
   */
  
  @Schema(name = "language", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("language")
  public String getLanguage() {
    return language;
  }

  public void setLanguage(String language) {
    this.language = language;
  }

  public DocumentInstance isCompleted(Boolean isCompleted) {
    this.isCompleted = isCompleted;
    return this;
  }

  /**
   * Get isCompleted
   * @return isCompleted
   */
  
  @Schema(name = "isCompleted", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("isCompleted")
  public Boolean getIsCompleted() {
    return isCompleted;
  }

  public void setIsCompleted(Boolean isCompleted) {
    this.isCompleted = isCompleted;
  }

  public DocumentInstance dueDate(OffsetDateTime dueDate) {
    this.dueDate = dueDate;
    return this;
  }

  /**
   * Get dueDate
   * @return dueDate
   */
  @Valid 
  @Schema(name = "dueDate", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("dueDate")
  public OffsetDateTime getDueDate() {
    return dueDate;
  }

  public void setDueDate(OffsetDateTime dueDate) {
    this.dueDate = dueDate;
  }

  public DocumentInstance generatedContent(String generatedContent) {
    this.generatedContent = generatedContent;
    return this;
  }

  /**
   * Get generatedContent
   * @return generatedContent
   */
  
  @Schema(name = "generatedContent", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("generatedContent")
  public String getGeneratedContent() {
    return generatedContent;
  }

  public void setGeneratedContent(String generatedContent) {
    this.generatedContent = generatedContent;
  }

  public DocumentInstance attachments(List<String> attachments) {
    this.attachments = attachments;
    return this;
  }

  public DocumentInstance addAttachmentsItem(String attachmentsItem) {
    if (this.attachments == null) {
      this.attachments = new ArrayList<>();
    }
    this.attachments.add(attachmentsItem);
    return this;
  }

  /**
   * Get attachments
   * @return attachments
   */
  
  @Schema(name = "attachments", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("attachments")
  public List<String> getAttachments() {
    return attachments;
  }

  public void setAttachments(List<String> attachments) {
    this.attachments = attachments;
  }

  public DocumentInstance createdAt(OffsetDateTime createdAt) {
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

  public DocumentInstance updatedAt(OffsetDateTime updatedAt) {
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

  public DocumentInstance signatureRequests(List<@Valid SignatureRequest> signatureRequests) {
    this.signatureRequests = signatureRequests;
    return this;
  }

  public DocumentInstance addSignatureRequestsItem(SignatureRequest signatureRequestsItem) {
    if (this.signatureRequests == null) {
      this.signatureRequests = new ArrayList<>();
    }
    this.signatureRequests.add(signatureRequestsItem);
    return this;
  }

  /**
   * Get signatureRequests
   * @return signatureRequests
   */
  @Valid 
  @Schema(name = "signatureRequests", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("signatureRequests")
  public List<@Valid SignatureRequest> getSignatureRequests() {
    return signatureRequests;
  }

  public void setSignatureRequests(List<@Valid SignatureRequest> signatureRequests) {
    this.signatureRequests = signatureRequests;
  }

  public DocumentInstance aiAssistance(List<@Valid AIAssistanceLog> aiAssistance) {
    this.aiAssistance = aiAssistance;
    return this;
  }

  public DocumentInstance addAiAssistanceItem(AIAssistanceLog aiAssistanceItem) {
    if (this.aiAssistance == null) {
      this.aiAssistance = new ArrayList<>();
    }
    this.aiAssistance.add(aiAssistanceItem);
    return this;
  }

  /**
   * Get aiAssistance
   * @return aiAssistance
   */
  @Valid 
  @Schema(name = "aiAssistance", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("aiAssistance")
  public List<@Valid AIAssistanceLog> getAiAssistance() {
    return aiAssistance;
  }

  public void setAiAssistance(List<@Valid AIAssistanceLog> aiAssistance) {
    this.aiAssistance = aiAssistance;
  }

  public DocumentInstance negotiation(NegotiationHistory negotiation) {
    this.negotiation = negotiation;
    return this;
  }

  /**
   * Get negotiation
   * @return negotiation
   */
  @Valid 
  @Schema(name = "negotiation", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("negotiation")
  public NegotiationHistory getNegotiation() {
    return negotiation;
  }

  public void setNegotiation(NegotiationHistory negotiation) {
    this.negotiation = negotiation;
  }

  public DocumentInstance versions(List<@Valid DocumentVersion> versions) {
    this.versions = versions;
    return this;
  }

  public DocumentInstance addVersionsItem(DocumentVersion versionsItem) {
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
  public List<@Valid DocumentVersion> getVersions() {
    return versions;
  }

  public void setVersions(List<@Valid DocumentVersion> versions) {
    this.versions = versions;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    DocumentInstance documentInstance = (DocumentInstance) o;
    return Objects.equals(this.id, documentInstance.id) &&
        Objects.equals(this.templateId, documentInstance.templateId) &&
        Objects.equals(this.status, documentInstance.status) &&
        Objects.equals(this.fieldValues, documentInstance.fieldValues) &&
        Objects.equals(this.signers, documentInstance.signers) &&
        Objects.equals(this.language, documentInstance.language) &&
        Objects.equals(this.isCompleted, documentInstance.isCompleted) &&
        Objects.equals(this.dueDate, documentInstance.dueDate) &&
        Objects.equals(this.generatedContent, documentInstance.generatedContent) &&
        Objects.equals(this.attachments, documentInstance.attachments) &&
        Objects.equals(this.createdAt, documentInstance.createdAt) &&
        Objects.equals(this.updatedAt, documentInstance.updatedAt) &&
        Objects.equals(this.signatureRequests, documentInstance.signatureRequests) &&
        Objects.equals(this.aiAssistance, documentInstance.aiAssistance) &&
        Objects.equals(this.negotiation, documentInstance.negotiation) &&
        Objects.equals(this.versions, documentInstance.versions);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, templateId, status, fieldValues, signers, language, isCompleted, dueDate, generatedContent, attachments, createdAt, updatedAt, signatureRequests, aiAssistance, negotiation, versions);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class DocumentInstance {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    templateId: ").append(toIndentedString(templateId)).append("\n");
    sb.append("    status: ").append(toIndentedString(status)).append("\n");
    sb.append("    fieldValues: ").append(toIndentedString(fieldValues)).append("\n");
    sb.append("    signers: ").append(toIndentedString(signers)).append("\n");
    sb.append("    language: ").append(toIndentedString(language)).append("\n");
    sb.append("    isCompleted: ").append(toIndentedString(isCompleted)).append("\n");
    sb.append("    dueDate: ").append(toIndentedString(dueDate)).append("\n");
    sb.append("    generatedContent: ").append(toIndentedString(generatedContent)).append("\n");
    sb.append("    attachments: ").append(toIndentedString(attachments)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
    sb.append("    updatedAt: ").append(toIndentedString(updatedAt)).append("\n");
    sb.append("    signatureRequests: ").append(toIndentedString(signatureRequests)).append("\n");
    sb.append("    aiAssistance: ").append(toIndentedString(aiAssistance)).append("\n");
    sb.append("    negotiation: ").append(toIndentedString(negotiation)).append("\n");
    sb.append("    versions: ").append(toIndentedString(versions)).append("\n");
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

