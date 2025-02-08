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
 * AppAnalytics
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T22:05:34.423190-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class AppAnalytics {

  private String id;

  private String appId;

  private String metricName;

  private Float metricValue;

  @Valid
  private Map<String, String> dimensions = new HashMap<>();

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime recordedAt;

  public AppAnalytics id(String id) {
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

  public AppAnalytics appId(String appId) {
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

  public AppAnalytics metricName(String metricName) {
    this.metricName = metricName;
    return this;
  }

  /**
   * Get metricName
   * @return metricName
   */
  
  @Schema(name = "metricName", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("metricName")
  public String getMetricName() {
    return metricName;
  }

  public void setMetricName(String metricName) {
    this.metricName = metricName;
  }

  public AppAnalytics metricValue(Float metricValue) {
    this.metricValue = metricValue;
    return this;
  }

  /**
   * Get metricValue
   * @return metricValue
   */
  
  @Schema(name = "metricValue", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("metricValue")
  public Float getMetricValue() {
    return metricValue;
  }

  public void setMetricValue(Float metricValue) {
    this.metricValue = metricValue;
  }

  public AppAnalytics dimensions(Map<String, String> dimensions) {
    this.dimensions = dimensions;
    return this;
  }

  public AppAnalytics putDimensionsItem(String key, String dimensionsItem) {
    if (this.dimensions == null) {
      this.dimensions = new HashMap<>();
    }
    this.dimensions.put(key, dimensionsItem);
    return this;
  }

  /**
   * Get dimensions
   * @return dimensions
   */
  
  @Schema(name = "dimensions", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("dimensions")
  public Map<String, String> getDimensions() {
    return dimensions;
  }

  public void setDimensions(Map<String, String> dimensions) {
    this.dimensions = dimensions;
  }

  public AppAnalytics recordedAt(OffsetDateTime recordedAt) {
    this.recordedAt = recordedAt;
    return this;
  }

  /**
   * Get recordedAt
   * @return recordedAt
   */
  @Valid 
  @Schema(name = "recordedAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("recordedAt")
  public OffsetDateTime getRecordedAt() {
    return recordedAt;
  }

  public void setRecordedAt(OffsetDateTime recordedAt) {
    this.recordedAt = recordedAt;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    AppAnalytics appAnalytics = (AppAnalytics) o;
    return Objects.equals(this.id, appAnalytics.id) &&
        Objects.equals(this.appId, appAnalytics.appId) &&
        Objects.equals(this.metricName, appAnalytics.metricName) &&
        Objects.equals(this.metricValue, appAnalytics.metricValue) &&
        Objects.equals(this.dimensions, appAnalytics.dimensions) &&
        Objects.equals(this.recordedAt, appAnalytics.recordedAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, appId, metricName, metricValue, dimensions, recordedAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class AppAnalytics {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    appId: ").append(toIndentedString(appId)).append("\n");
    sb.append("    metricName: ").append(toIndentedString(metricName)).append("\n");
    sb.append("    metricValue: ").append(toIndentedString(metricValue)).append("\n");
    sb.append("    dimensions: ").append(toIndentedString(dimensions)).append("\n");
    sb.append("    recordedAt: ").append(toIndentedString(recordedAt)).append("\n");
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

