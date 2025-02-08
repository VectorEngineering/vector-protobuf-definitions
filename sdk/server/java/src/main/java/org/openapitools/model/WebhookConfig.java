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
import org.openapitools.model.IncludedField;
import org.openapitools.model.PayloadFormat;
import org.openapitools.model.TriggerEvent;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * WebhookConfig
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-08T09:39:48.398562-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class WebhookConfig {

  private String id;

  private String url;

  private String authType;

  private String authToken;

  @Valid
  private Map<String, String> customHeaders = new HashMap<>();

  private Integer maxRetries;

  private String retryInterval;

  @Valid
  private List<TriggerEvent> triggerEvents = new ArrayList<>();

  @Valid
  private List<IncludedField> includedFields = new ArrayList<>();

  private Boolean includeFullResults;

  private PayloadFormat payloadFormat = PayloadFormat.UNSPECIFIED;

  private Boolean verifySsl;

  private String signingSecret;

  private Integer rateLimit;

  private String rateLimitInterval;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime lastTriggeredAt;

  private Integer successfulCalls;

  private Integer failedCalls;

  private Object metadata;

  private String webhookName;

  public WebhookConfig id(String id) {
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

  public WebhookConfig url(String url) {
    this.url = url;
    return this;
  }

  /**
   * Get url
   * @return url
   */
  
  @Schema(name = "url", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("url")
  public String getUrl() {
    return url;
  }

  public void setUrl(String url) {
    this.url = url;
  }

  public WebhookConfig authType(String authType) {
    this.authType = authType;
    return this;
  }

  /**
   * e.g., \"basic\", \"bearer\", \"custom\"
   * @return authType
   */
  
  @Schema(name = "authType", description = "e.g., \"basic\", \"bearer\", \"custom\"", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("authType")
  public String getAuthType() {
    return authType;
  }

  public void setAuthType(String authType) {
    this.authType = authType;
  }

  public WebhookConfig authToken(String authToken) {
    this.authToken = authToken;
    return this;
  }

  /**
   * Get authToken
   * @return authToken
   */
  
  @Schema(name = "authToken", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("authToken")
  public String getAuthToken() {
    return authToken;
  }

  public void setAuthToken(String authToken) {
    this.authToken = authToken;
  }

  public WebhookConfig customHeaders(Map<String, String> customHeaders) {
    this.customHeaders = customHeaders;
    return this;
  }

  public WebhookConfig putCustomHeadersItem(String key, String customHeadersItem) {
    if (this.customHeaders == null) {
      this.customHeaders = new HashMap<>();
    }
    this.customHeaders.put(key, customHeadersItem);
    return this;
  }

  /**
   * Get customHeaders
   * @return customHeaders
   */
  
  @Schema(name = "customHeaders", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("customHeaders")
  public Map<String, String> getCustomHeaders() {
    return customHeaders;
  }

  public void setCustomHeaders(Map<String, String> customHeaders) {
    this.customHeaders = customHeaders;
  }

  public WebhookConfig maxRetries(Integer maxRetries) {
    this.maxRetries = maxRetries;
    return this;
  }

  /**
   * Get maxRetries
   * @return maxRetries
   */
  
  @Schema(name = "maxRetries", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("maxRetries")
  public Integer getMaxRetries() {
    return maxRetries;
  }

  public void setMaxRetries(Integer maxRetries) {
    this.maxRetries = maxRetries;
  }

  public WebhookConfig retryInterval(String retryInterval) {
    this.retryInterval = retryInterval;
    return this;
  }

  /**
   * Get retryInterval
   * @return retryInterval
   */
  
  @Schema(name = "retryInterval", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("retryInterval")
  public String getRetryInterval() {
    return retryInterval;
  }

  public void setRetryInterval(String retryInterval) {
    this.retryInterval = retryInterval;
  }

  public WebhookConfig triggerEvents(List<TriggerEvent> triggerEvents) {
    this.triggerEvents = triggerEvents;
    return this;
  }

  public WebhookConfig addTriggerEventsItem(TriggerEvent triggerEventsItem) {
    if (this.triggerEvents == null) {
      this.triggerEvents = new ArrayList<>();
    }
    this.triggerEvents.add(triggerEventsItem);
    return this;
  }

  /**
   * Get triggerEvents
   * @return triggerEvents
   */
  @Valid 
  @Schema(name = "triggerEvents", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("triggerEvents")
  public List<TriggerEvent> getTriggerEvents() {
    return triggerEvents;
  }

  public void setTriggerEvents(List<TriggerEvent> triggerEvents) {
    this.triggerEvents = triggerEvents;
  }

  public WebhookConfig includedFields(List<IncludedField> includedFields) {
    this.includedFields = includedFields;
    return this;
  }

  public WebhookConfig addIncludedFieldsItem(IncludedField includedFieldsItem) {
    if (this.includedFields == null) {
      this.includedFields = new ArrayList<>();
    }
    this.includedFields.add(includedFieldsItem);
    return this;
  }

  /**
   * Get includedFields
   * @return includedFields
   */
  @Valid 
  @Schema(name = "includedFields", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("includedFields")
  public List<IncludedField> getIncludedFields() {
    return includedFields;
  }

  public void setIncludedFields(List<IncludedField> includedFields) {
    this.includedFields = includedFields;
  }

  public WebhookConfig includeFullResults(Boolean includeFullResults) {
    this.includeFullResults = includeFullResults;
    return this;
  }

  /**
   * Get includeFullResults
   * @return includeFullResults
   */
  
  @Schema(name = "includeFullResults", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("includeFullResults")
  public Boolean getIncludeFullResults() {
    return includeFullResults;
  }

  public void setIncludeFullResults(Boolean includeFullResults) {
    this.includeFullResults = includeFullResults;
  }

  public WebhookConfig payloadFormat(PayloadFormat payloadFormat) {
    this.payloadFormat = payloadFormat;
    return this;
  }

  /**
   * Get payloadFormat
   * @return payloadFormat
   */
  @Valid 
  @Schema(name = "payloadFormat", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("payloadFormat")
  public PayloadFormat getPayloadFormat() {
    return payloadFormat;
  }

  public void setPayloadFormat(PayloadFormat payloadFormat) {
    this.payloadFormat = payloadFormat;
  }

  public WebhookConfig verifySsl(Boolean verifySsl) {
    this.verifySsl = verifySsl;
    return this;
  }

  /**
   * Get verifySsl
   * @return verifySsl
   */
  
  @Schema(name = "verifySsl", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("verifySsl")
  public Boolean getVerifySsl() {
    return verifySsl;
  }

  public void setVerifySsl(Boolean verifySsl) {
    this.verifySsl = verifySsl;
  }

  public WebhookConfig signingSecret(String signingSecret) {
    this.signingSecret = signingSecret;
    return this;
  }

  /**
   * Get signingSecret
   * @return signingSecret
   */
  
  @Schema(name = "signingSecret", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("signingSecret")
  public String getSigningSecret() {
    return signingSecret;
  }

  public void setSigningSecret(String signingSecret) {
    this.signingSecret = signingSecret;
  }

  public WebhookConfig rateLimit(Integer rateLimit) {
    this.rateLimit = rateLimit;
    return this;
  }

  /**
   * Get rateLimit
   * @return rateLimit
   */
  
  @Schema(name = "rateLimit", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("rateLimit")
  public Integer getRateLimit() {
    return rateLimit;
  }

  public void setRateLimit(Integer rateLimit) {
    this.rateLimit = rateLimit;
  }

  public WebhookConfig rateLimitInterval(String rateLimitInterval) {
    this.rateLimitInterval = rateLimitInterval;
    return this;
  }

  /**
   * Get rateLimitInterval
   * @return rateLimitInterval
   */
  
  @Schema(name = "rateLimitInterval", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("rateLimitInterval")
  public String getRateLimitInterval() {
    return rateLimitInterval;
  }

  public void setRateLimitInterval(String rateLimitInterval) {
    this.rateLimitInterval = rateLimitInterval;
  }

  public WebhookConfig createdAt(OffsetDateTime createdAt) {
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

  public WebhookConfig updatedAt(OffsetDateTime updatedAt) {
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

  public WebhookConfig lastTriggeredAt(OffsetDateTime lastTriggeredAt) {
    this.lastTriggeredAt = lastTriggeredAt;
    return this;
  }

  /**
   * Get lastTriggeredAt
   * @return lastTriggeredAt
   */
  @Valid 
  @Schema(name = "lastTriggeredAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("lastTriggeredAt")
  public OffsetDateTime getLastTriggeredAt() {
    return lastTriggeredAt;
  }

  public void setLastTriggeredAt(OffsetDateTime lastTriggeredAt) {
    this.lastTriggeredAt = lastTriggeredAt;
  }

  public WebhookConfig successfulCalls(Integer successfulCalls) {
    this.successfulCalls = successfulCalls;
    return this;
  }

  /**
   * Get successfulCalls
   * @return successfulCalls
   */
  
  @Schema(name = "successfulCalls", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("successfulCalls")
  public Integer getSuccessfulCalls() {
    return successfulCalls;
  }

  public void setSuccessfulCalls(Integer successfulCalls) {
    this.successfulCalls = successfulCalls;
  }

  public WebhookConfig failedCalls(Integer failedCalls) {
    this.failedCalls = failedCalls;
    return this;
  }

  /**
   * Get failedCalls
   * @return failedCalls
   */
  
  @Schema(name = "failedCalls", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("failedCalls")
  public Integer getFailedCalls() {
    return failedCalls;
  }

  public void setFailedCalls(Integer failedCalls) {
    this.failedCalls = failedCalls;
  }

  public WebhookConfig metadata(Object metadata) {
    this.metadata = metadata;
    return this;
  }

  /**
   * Get metadata
   * @return metadata
   */
  
  @Schema(name = "metadata", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("metadata")
  public Object getMetadata() {
    return metadata;
  }

  public void setMetadata(Object metadata) {
    this.metadata = metadata;
  }

  public WebhookConfig webhookName(String webhookName) {
    this.webhookName = webhookName;
    return this;
  }

  /**
   * Get webhookName
   * @return webhookName
   */
  
  @Schema(name = "webhookName", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("webhookName")
  public String getWebhookName() {
    return webhookName;
  }

  public void setWebhookName(String webhookName) {
    this.webhookName = webhookName;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    WebhookConfig webhookConfig = (WebhookConfig) o;
    return Objects.equals(this.id, webhookConfig.id) &&
        Objects.equals(this.url, webhookConfig.url) &&
        Objects.equals(this.authType, webhookConfig.authType) &&
        Objects.equals(this.authToken, webhookConfig.authToken) &&
        Objects.equals(this.customHeaders, webhookConfig.customHeaders) &&
        Objects.equals(this.maxRetries, webhookConfig.maxRetries) &&
        Objects.equals(this.retryInterval, webhookConfig.retryInterval) &&
        Objects.equals(this.triggerEvents, webhookConfig.triggerEvents) &&
        Objects.equals(this.includedFields, webhookConfig.includedFields) &&
        Objects.equals(this.includeFullResults, webhookConfig.includeFullResults) &&
        Objects.equals(this.payloadFormat, webhookConfig.payloadFormat) &&
        Objects.equals(this.verifySsl, webhookConfig.verifySsl) &&
        Objects.equals(this.signingSecret, webhookConfig.signingSecret) &&
        Objects.equals(this.rateLimit, webhookConfig.rateLimit) &&
        Objects.equals(this.rateLimitInterval, webhookConfig.rateLimitInterval) &&
        Objects.equals(this.createdAt, webhookConfig.createdAt) &&
        Objects.equals(this.updatedAt, webhookConfig.updatedAt) &&
        Objects.equals(this.lastTriggeredAt, webhookConfig.lastTriggeredAt) &&
        Objects.equals(this.successfulCalls, webhookConfig.successfulCalls) &&
        Objects.equals(this.failedCalls, webhookConfig.failedCalls) &&
        Objects.equals(this.metadata, webhookConfig.metadata) &&
        Objects.equals(this.webhookName, webhookConfig.webhookName);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, url, authType, authToken, customHeaders, maxRetries, retryInterval, triggerEvents, includedFields, includeFullResults, payloadFormat, verifySsl, signingSecret, rateLimit, rateLimitInterval, createdAt, updatedAt, lastTriggeredAt, successfulCalls, failedCalls, metadata, webhookName);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class WebhookConfig {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    url: ").append(toIndentedString(url)).append("\n");
    sb.append("    authType: ").append(toIndentedString(authType)).append("\n");
    sb.append("    authToken: ").append(toIndentedString(authToken)).append("\n");
    sb.append("    customHeaders: ").append(toIndentedString(customHeaders)).append("\n");
    sb.append("    maxRetries: ").append(toIndentedString(maxRetries)).append("\n");
    sb.append("    retryInterval: ").append(toIndentedString(retryInterval)).append("\n");
    sb.append("    triggerEvents: ").append(toIndentedString(triggerEvents)).append("\n");
    sb.append("    includedFields: ").append(toIndentedString(includedFields)).append("\n");
    sb.append("    includeFullResults: ").append(toIndentedString(includeFullResults)).append("\n");
    sb.append("    payloadFormat: ").append(toIndentedString(payloadFormat)).append("\n");
    sb.append("    verifySsl: ").append(toIndentedString(verifySsl)).append("\n");
    sb.append("    signingSecret: ").append(toIndentedString(signingSecret)).append("\n");
    sb.append("    rateLimit: ").append(toIndentedString(rateLimit)).append("\n");
    sb.append("    rateLimitInterval: ").append(toIndentedString(rateLimitInterval)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
    sb.append("    updatedAt: ").append(toIndentedString(updatedAt)).append("\n");
    sb.append("    lastTriggeredAt: ").append(toIndentedString(lastTriggeredAt)).append("\n");
    sb.append("    successfulCalls: ").append(toIndentedString(successfulCalls)).append("\n");
    sb.append("    failedCalls: ").append(toIndentedString(failedCalls)).append("\n");
    sb.append("    metadata: ").append(toIndentedString(metadata)).append("\n");
    sb.append("    webhookName: ").append(toIndentedString(webhookName)).append("\n");
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

