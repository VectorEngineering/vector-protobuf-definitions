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
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * AppWebhook
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T21:14:09.107639-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class AppWebhook {

  private String id;

  private String appId;

  private String url;

  @Valid
  private List<String> subscribedEvents = new ArrayList<>();

  private String secretKey;

  private Integer retryCount;

  private Integer timeoutSeconds;

  private Boolean isActive;

  @Valid
  private Map<String, String> headers = new HashMap<>();

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime lastTriggered;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  public AppWebhook id(String id) {
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

  public AppWebhook appId(String appId) {
    this.appId = appId;
    return this;
  }

  /**
   * Get appId
   * @return appId
   */
  
  @Schema(name = "appId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("appId")
  public String getAppId() {
    return appId;
  }

  public void setAppId(String appId) {
    this.appId = appId;
  }

  public AppWebhook url(String url) {
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

  public AppWebhook subscribedEvents(List<String> subscribedEvents) {
    this.subscribedEvents = subscribedEvents;
    return this;
  }

  public AppWebhook addSubscribedEventsItem(String subscribedEventsItem) {
    if (this.subscribedEvents == null) {
      this.subscribedEvents = new ArrayList<>();
    }
    this.subscribedEvents.add(subscribedEventsItem);
    return this;
  }

  /**
   * Get subscribedEvents
   * @return subscribedEvents
   */
  
  @Schema(name = "subscribedEvents", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("subscribedEvents")
  public List<String> getSubscribedEvents() {
    return subscribedEvents;
  }

  public void setSubscribedEvents(List<String> subscribedEvents) {
    this.subscribedEvents = subscribedEvents;
  }

  public AppWebhook secretKey(String secretKey) {
    this.secretKey = secretKey;
    return this;
  }

  /**
   * Get secretKey
   * @return secretKey
   */
  
  @Schema(name = "secretKey", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("secretKey")
  public String getSecretKey() {
    return secretKey;
  }

  public void setSecretKey(String secretKey) {
    this.secretKey = secretKey;
  }

  public AppWebhook retryCount(Integer retryCount) {
    this.retryCount = retryCount;
    return this;
  }

  /**
   * Get retryCount
   * @return retryCount
   */
  
  @Schema(name = "retryCount", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("retryCount")
  public Integer getRetryCount() {
    return retryCount;
  }

  public void setRetryCount(Integer retryCount) {
    this.retryCount = retryCount;
  }

  public AppWebhook timeoutSeconds(Integer timeoutSeconds) {
    this.timeoutSeconds = timeoutSeconds;
    return this;
  }

  /**
   * Get timeoutSeconds
   * @return timeoutSeconds
   */
  
  @Schema(name = "timeoutSeconds", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("timeoutSeconds")
  public Integer getTimeoutSeconds() {
    return timeoutSeconds;
  }

  public void setTimeoutSeconds(Integer timeoutSeconds) {
    this.timeoutSeconds = timeoutSeconds;
  }

  public AppWebhook isActive(Boolean isActive) {
    this.isActive = isActive;
    return this;
  }

  /**
   * Get isActive
   * @return isActive
   */
  
  @Schema(name = "isActive", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("isActive")
  public Boolean getIsActive() {
    return isActive;
  }

  public void setIsActive(Boolean isActive) {
    this.isActive = isActive;
  }

  public AppWebhook headers(Map<String, String> headers) {
    this.headers = headers;
    return this;
  }

  public AppWebhook putHeadersItem(String key, String headersItem) {
    if (this.headers == null) {
      this.headers = new HashMap<>();
    }
    this.headers.put(key, headersItem);
    return this;
  }

  /**
   * Get headers
   * @return headers
   */
  
  @Schema(name = "headers", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("headers")
  public Map<String, String> getHeaders() {
    return headers;
  }

  public void setHeaders(Map<String, String> headers) {
    this.headers = headers;
  }

  public AppWebhook lastTriggered(OffsetDateTime lastTriggered) {
    this.lastTriggered = lastTriggered;
    return this;
  }

  /**
   * Get lastTriggered
   * @return lastTriggered
   */
  @Valid 
  @Schema(name = "lastTriggered", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("lastTriggered")
  public OffsetDateTime getLastTriggered() {
    return lastTriggered;
  }

  public void setLastTriggered(OffsetDateTime lastTriggered) {
    this.lastTriggered = lastTriggered;
  }

  public AppWebhook createdAt(OffsetDateTime createdAt) {
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
    AppWebhook appWebhook = (AppWebhook) o;
    return Objects.equals(this.id, appWebhook.id) &&
        Objects.equals(this.appId, appWebhook.appId) &&
        Objects.equals(this.url, appWebhook.url) &&
        Objects.equals(this.subscribedEvents, appWebhook.subscribedEvents) &&
        Objects.equals(this.secretKey, appWebhook.secretKey) &&
        Objects.equals(this.retryCount, appWebhook.retryCount) &&
        Objects.equals(this.timeoutSeconds, appWebhook.timeoutSeconds) &&
        Objects.equals(this.isActive, appWebhook.isActive) &&
        Objects.equals(this.headers, appWebhook.headers) &&
        Objects.equals(this.lastTriggered, appWebhook.lastTriggered) &&
        Objects.equals(this.createdAt, appWebhook.createdAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, appId, url, subscribedEvents, secretKey, retryCount, timeoutSeconds, isActive, headers, lastTriggered, createdAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class AppWebhook {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    appId: ").append(toIndentedString(appId)).append("\n");
    sb.append("    url: ").append(toIndentedString(url)).append("\n");
    sb.append("    subscribedEvents: ").append(toIndentedString(subscribedEvents)).append("\n");
    sb.append("    secretKey: ").append(toIndentedString(secretKey)).append("\n");
    sb.append("    retryCount: ").append(toIndentedString(retryCount)).append("\n");
    sb.append("    timeoutSeconds: ").append(toIndentedString(timeoutSeconds)).append("\n");
    sb.append("    isActive: ").append(toIndentedString(isActive)).append("\n");
    sb.append("    headers: ").append(toIndentedString(headers)).append("\n");
    sb.append("    lastTriggered: ").append(toIndentedString(lastTriggered)).append("\n");
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

