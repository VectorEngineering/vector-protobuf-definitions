package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.HashMap;
import java.util.Map;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * ResourceUtilization
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T12:30:40.954679-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ResourceUtilization {

  private Double cpuUsage;

  private Double memoryUsage;

  private Integer activeConnections;

  @Valid
  private Map<String, Double> quotas = new HashMap<>();

  public ResourceUtilization cpuUsage(Double cpuUsage) {
    this.cpuUsage = cpuUsage;
    return this;
  }

  /**
   * Get cpuUsage
   * @return cpuUsage
   */
  
  @Schema(name = "cpuUsage", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("cpuUsage")
  public Double getCpuUsage() {
    return cpuUsage;
  }

  public void setCpuUsage(Double cpuUsage) {
    this.cpuUsage = cpuUsage;
  }

  public ResourceUtilization memoryUsage(Double memoryUsage) {
    this.memoryUsage = memoryUsage;
    return this;
  }

  /**
   * Get memoryUsage
   * @return memoryUsage
   */
  
  @Schema(name = "memoryUsage", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("memoryUsage")
  public Double getMemoryUsage() {
    return memoryUsage;
  }

  public void setMemoryUsage(Double memoryUsage) {
    this.memoryUsage = memoryUsage;
  }

  public ResourceUtilization activeConnections(Integer activeConnections) {
    this.activeConnections = activeConnections;
    return this;
  }

  /**
   * Get activeConnections
   * @return activeConnections
   */
  
  @Schema(name = "activeConnections", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("activeConnections")
  public Integer getActiveConnections() {
    return activeConnections;
  }

  public void setActiveConnections(Integer activeConnections) {
    this.activeConnections = activeConnections;
  }

  public ResourceUtilization quotas(Map<String, Double> quotas) {
    this.quotas = quotas;
    return this;
  }

  public ResourceUtilization putQuotasItem(String key, Double quotasItem) {
    if (this.quotas == null) {
      this.quotas = new HashMap<>();
    }
    this.quotas.put(key, quotasItem);
    return this;
  }

  /**
   * Get quotas
   * @return quotas
   */
  
  @Schema(name = "quotas", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("quotas")
  public Map<String, Double> getQuotas() {
    return quotas;
  }

  public void setQuotas(Map<String, Double> quotas) {
    this.quotas = quotas;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    ResourceUtilization resourceUtilization = (ResourceUtilization) o;
    return Objects.equals(this.cpuUsage, resourceUtilization.cpuUsage) &&
        Objects.equals(this.memoryUsage, resourceUtilization.memoryUsage) &&
        Objects.equals(this.activeConnections, resourceUtilization.activeConnections) &&
        Objects.equals(this.quotas, resourceUtilization.quotas);
  }

  @Override
  public int hashCode() {
    return Objects.hash(cpuUsage, memoryUsage, activeConnections, quotas);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ResourceUtilization {\n");
    sb.append("    cpuUsage: ").append(toIndentedString(cpuUsage)).append("\n");
    sb.append("    memoryUsage: ").append(toIndentedString(memoryUsage)).append("\n");
    sb.append("    activeConnections: ").append(toIndentedString(activeConnections)).append("\n");
    sb.append("    quotas: ").append(toIndentedString(quotas)).append("\n");
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

