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
import org.openapitools.model.Dependency;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * ServiceStatus
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-01-29T12:50:06.029660-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ServiceStatus {

  private String name;

  private String status;

  @Valid
  private Map<String, String> metrics = new HashMap<>();

  @Valid
  private List<@Valid Dependency> dependencies = new ArrayList<>();

  public ServiceStatus name(String name) {
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

  public ServiceStatus status(String status) {
    this.status = status;
    return this;
  }

  /**
   * Get status
   * @return status
   */
  
  @Schema(name = "status", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("status")
  public String getStatus() {
    return status;
  }

  public void setStatus(String status) {
    this.status = status;
  }

  public ServiceStatus metrics(Map<String, String> metrics) {
    this.metrics = metrics;
    return this;
  }

  public ServiceStatus putMetricsItem(String key, String metricsItem) {
    if (this.metrics == null) {
      this.metrics = new HashMap<>();
    }
    this.metrics.put(key, metricsItem);
    return this;
  }

  /**
   * Get metrics
   * @return metrics
   */
  
  @Schema(name = "metrics", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("metrics")
  public Map<String, String> getMetrics() {
    return metrics;
  }

  public void setMetrics(Map<String, String> metrics) {
    this.metrics = metrics;
  }

  public ServiceStatus dependencies(List<@Valid Dependency> dependencies) {
    this.dependencies = dependencies;
    return this;
  }

  public ServiceStatus addDependenciesItem(Dependency dependenciesItem) {
    if (this.dependencies == null) {
      this.dependencies = new ArrayList<>();
    }
    this.dependencies.add(dependenciesItem);
    return this;
  }

  /**
   * Get dependencies
   * @return dependencies
   */
  @Valid 
  @Schema(name = "dependencies", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("dependencies")
  public List<@Valid Dependency> getDependencies() {
    return dependencies;
  }

  public void setDependencies(List<@Valid Dependency> dependencies) {
    this.dependencies = dependencies;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    ServiceStatus serviceStatus = (ServiceStatus) o;
    return Objects.equals(this.name, serviceStatus.name) &&
        Objects.equals(this.status, serviceStatus.status) &&
        Objects.equals(this.metrics, serviceStatus.metrics) &&
        Objects.equals(this.dependencies, serviceStatus.dependencies);
  }

  @Override
  public int hashCode() {
    return Objects.hash(name, status, metrics, dependencies);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ServiceStatus {\n");
    sb.append("    name: ").append(toIndentedString(name)).append("\n");
    sb.append("    status: ").append(toIndentedString(status)).append("\n");
    sb.append("    metrics: ").append(toIndentedString(metrics)).append("\n");
    sb.append("    dependencies: ").append(toIndentedString(dependencies)).append("\n");
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

