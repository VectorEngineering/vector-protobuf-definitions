package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import java.util.HashMap;
import java.util.Map;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * AIAssistanceLog
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-08T10:46:38.744443-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class AIAssistanceLog {

  private String id;

  private String documentId;

  private String interactionType;

  private String userQuery;

  private String aiResponse;

  @Valid
  private Map<String, String> context = new HashMap<>();

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  public AIAssistanceLog id(String id) {
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

  public AIAssistanceLog documentId(String documentId) {
    this.documentId = documentId;
    return this;
  }

  /**
   * Get documentId
   * @return documentId
   */
  
  @Schema(name = "documentId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("documentId")
  public String getDocumentId() {
    return documentId;
  }

  public void setDocumentId(String documentId) {
    this.documentId = documentId;
  }

  public AIAssistanceLog interactionType(String interactionType) {
    this.interactionType = interactionType;
    return this;
  }

  /**
   * Get interactionType
   * @return interactionType
   */
  
  @Schema(name = "interactionType", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("interactionType")
  public String getInteractionType() {
    return interactionType;
  }

  public void setInteractionType(String interactionType) {
    this.interactionType = interactionType;
  }

  public AIAssistanceLog userQuery(String userQuery) {
    this.userQuery = userQuery;
    return this;
  }

  /**
   * Get userQuery
   * @return userQuery
   */
  
  @Schema(name = "userQuery", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("userQuery")
  public String getUserQuery() {
    return userQuery;
  }

  public void setUserQuery(String userQuery) {
    this.userQuery = userQuery;
  }

  public AIAssistanceLog aiResponse(String aiResponse) {
    this.aiResponse = aiResponse;
    return this;
  }

  /**
   * Get aiResponse
   * @return aiResponse
   */
  
  @Schema(name = "aiResponse", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("aiResponse")
  public String getAiResponse() {
    return aiResponse;
  }

  public void setAiResponse(String aiResponse) {
    this.aiResponse = aiResponse;
  }

  public AIAssistanceLog context(Map<String, String> context) {
    this.context = context;
    return this;
  }

  public AIAssistanceLog putContextItem(String key, String contextItem) {
    if (this.context == null) {
      this.context = new HashMap<>();
    }
    this.context.put(key, contextItem);
    return this;
  }

  /**
   * Get context
   * @return context
   */
  
  @Schema(name = "context", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("context")
  public Map<String, String> getContext() {
    return context;
  }

  public void setContext(Map<String, String> context) {
    this.context = context;
  }

  public AIAssistanceLog createdAt(OffsetDateTime createdAt) {
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
    AIAssistanceLog aiAssistanceLog = (AIAssistanceLog) o;
    return Objects.equals(this.id, aiAssistanceLog.id) &&
        Objects.equals(this.documentId, aiAssistanceLog.documentId) &&
        Objects.equals(this.interactionType, aiAssistanceLog.interactionType) &&
        Objects.equals(this.userQuery, aiAssistanceLog.userQuery) &&
        Objects.equals(this.aiResponse, aiAssistanceLog.aiResponse) &&
        Objects.equals(this.context, aiAssistanceLog.context) &&
        Objects.equals(this.createdAt, aiAssistanceLog.createdAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, documentId, interactionType, userQuery, aiResponse, context, createdAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class AIAssistanceLog {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    documentId: ").append(toIndentedString(documentId)).append("\n");
    sb.append("    interactionType: ").append(toIndentedString(interactionType)).append("\n");
    sb.append("    userQuery: ").append(toIndentedString(userQuery)).append("\n");
    sb.append("    aiResponse: ").append(toIndentedString(aiResponse)).append("\n");
    sb.append("    context: ").append(toIndentedString(context)).append("\n");
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

