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
 * QuotaInfo
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-05T07:11:09.755832-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class QuotaInfo {

  private String resource;

  private String used;

  private String limit;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime resetTime;

  @Valid
  private Map<String, String> resourceUsage = new HashMap<>();

  public QuotaInfo resource(String resource) {
    this.resource = resource;
    return this;
  }

  /**
   * Get resource
   * @return resource
   */
  
  @Schema(name = "resource", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("resource")
  public String getResource() {
    return resource;
  }

  public void setResource(String resource) {
    this.resource = resource;
  }

  public QuotaInfo used(String used) {
    this.used = used;
    return this;
  }

  /**
   * Get used
   * @return used
   */
  
  @Schema(name = "used", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("used")
  public String getUsed() {
    return used;
  }

  public void setUsed(String used) {
    this.used = used;
  }

  public QuotaInfo limit(String limit) {
    this.limit = limit;
    return this;
  }

  /**
   * Get limit
   * @return limit
   */
  
  @Schema(name = "limit", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("limit")
  public String getLimit() {
    return limit;
  }

  public void setLimit(String limit) {
    this.limit = limit;
  }

  public QuotaInfo resetTime(OffsetDateTime resetTime) {
    this.resetTime = resetTime;
    return this;
  }

  /**
   * Get resetTime
   * @return resetTime
   */
  @Valid 
  @Schema(name = "resetTime", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("resetTime")
  public OffsetDateTime getResetTime() {
    return resetTime;
  }

  public void setResetTime(OffsetDateTime resetTime) {
    this.resetTime = resetTime;
  }

  public QuotaInfo resourceUsage(Map<String, String> resourceUsage) {
    this.resourceUsage = resourceUsage;
    return this;
  }

  public QuotaInfo putResourceUsageItem(String key, String resourceUsageItem) {
    if (this.resourceUsage == null) {
      this.resourceUsage = new HashMap<>();
    }
    this.resourceUsage.put(key, resourceUsageItem);
    return this;
  }

  /**
   * Get resourceUsage
   * @return resourceUsage
   */
  
  @Schema(name = "resourceUsage", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("resourceUsage")
  public Map<String, String> getResourceUsage() {
    return resourceUsage;
  }

  public void setResourceUsage(Map<String, String> resourceUsage) {
    this.resourceUsage = resourceUsage;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    QuotaInfo quotaInfo = (QuotaInfo) o;
    return Objects.equals(this.resource, quotaInfo.resource) &&
        Objects.equals(this.used, quotaInfo.used) &&
        Objects.equals(this.limit, quotaInfo.limit) &&
        Objects.equals(this.resetTime, quotaInfo.resetTime) &&
        Objects.equals(this.resourceUsage, quotaInfo.resourceUsage);
  }

  @Override
  public int hashCode() {
    return Objects.hash(resource, used, limit, resetTime, resourceUsage);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class QuotaInfo {\n");
    sb.append("    resource: ").append(toIndentedString(resource)).append("\n");
    sb.append("    used: ").append(toIndentedString(used)).append("\n");
    sb.append("    limit: ").append(toIndentedString(limit)).append("\n");
    sb.append("    resetTime: ").append(toIndentedString(resetTime)).append("\n");
    sb.append("    resourceUsage: ").append(toIndentedString(resourceUsage)).append("\n");
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

