package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * RateLimitContext
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T14:04:25.138120-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class RateLimitContext {

  private String userId;

  private String ipAddress;

  @Valid
  private List<String> affectedEndpoints = new ArrayList<>();

  @Valid
  private Map<String, String> headers = new HashMap<>();

  public RateLimitContext userId(String userId) {
    this.userId = userId;
    return this;
  }

  /**
   * Get userId
   * @return userId
   */
  
  @Schema(name = "userId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("userId")
  public String getUserId() {
    return userId;
  }

  public void setUserId(String userId) {
    this.userId = userId;
  }

  public RateLimitContext ipAddress(String ipAddress) {
    this.ipAddress = ipAddress;
    return this;
  }

  /**
   * Get ipAddress
   * @return ipAddress
   */
  
  @Schema(name = "ipAddress", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("ipAddress")
  public String getIpAddress() {
    return ipAddress;
  }

  public void setIpAddress(String ipAddress) {
    this.ipAddress = ipAddress;
  }

  public RateLimitContext affectedEndpoints(List<String> affectedEndpoints) {
    this.affectedEndpoints = affectedEndpoints;
    return this;
  }

  public RateLimitContext addAffectedEndpointsItem(String affectedEndpointsItem) {
    if (this.affectedEndpoints == null) {
      this.affectedEndpoints = new ArrayList<>();
    }
    this.affectedEndpoints.add(affectedEndpointsItem);
    return this;
  }

  /**
   * Get affectedEndpoints
   * @return affectedEndpoints
   */
  
  @Schema(name = "affectedEndpoints", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("affectedEndpoints")
  public List<String> getAffectedEndpoints() {
    return affectedEndpoints;
  }

  public void setAffectedEndpoints(List<String> affectedEndpoints) {
    this.affectedEndpoints = affectedEndpoints;
  }

  public RateLimitContext headers(Map<String, String> headers) {
    this.headers = headers;
    return this;
  }

  public RateLimitContext putHeadersItem(String key, String headersItem) {
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

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    RateLimitContext rateLimitContext = (RateLimitContext) o;
    return Objects.equals(this.userId, rateLimitContext.userId) &&
        Objects.equals(this.ipAddress, rateLimitContext.ipAddress) &&
        Objects.equals(this.affectedEndpoints, rateLimitContext.affectedEndpoints) &&
        Objects.equals(this.headers, rateLimitContext.headers);
  }

  @Override
  public int hashCode() {
    return Objects.hash(userId, ipAddress, affectedEndpoints, headers);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class RateLimitContext {\n");
    sb.append("    userId: ").append(toIndentedString(userId)).append("\n");
    sb.append("    ipAddress: ").append(toIndentedString(ipAddress)).append("\n");
    sb.append("    affectedEndpoints: ").append(toIndentedString(affectedEndpoints)).append("\n");
    sb.append("    headers: ").append(toIndentedString(headers)).append("\n");
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

