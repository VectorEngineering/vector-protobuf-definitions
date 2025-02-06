package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * LimitInfo
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-06T17:12:08.548747-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class LimitInfo {

  private Integer current;

  private Integer limit;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime resetTime;

  private String windowSize;

  public LimitInfo current(Integer current) {
    this.current = current;
    return this;
  }

  /**
   * Get current
   * @return current
   */
  
  @Schema(name = "current", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("current")
  public Integer getCurrent() {
    return current;
  }

  public void setCurrent(Integer current) {
    this.current = current;
  }

  public LimitInfo limit(Integer limit) {
    this.limit = limit;
    return this;
  }

  /**
   * Get limit
   * @return limit
   */
  
  @Schema(name = "limit", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("limit")
  public Integer getLimit() {
    return limit;
  }

  public void setLimit(Integer limit) {
    this.limit = limit;
  }

  public LimitInfo resetTime(OffsetDateTime resetTime) {
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

  public LimitInfo windowSize(String windowSize) {
    this.windowSize = windowSize;
    return this;
  }

  /**
   * Get windowSize
   * @return windowSize
   */
  
  @Schema(name = "windowSize", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("windowSize")
  public String getWindowSize() {
    return windowSize;
  }

  public void setWindowSize(String windowSize) {
    this.windowSize = windowSize;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    LimitInfo limitInfo = (LimitInfo) o;
    return Objects.equals(this.current, limitInfo.current) &&
        Objects.equals(this.limit, limitInfo.limit) &&
        Objects.equals(this.resetTime, limitInfo.resetTime) &&
        Objects.equals(this.windowSize, limitInfo.windowSize);
  }

  @Override
  public int hashCode() {
    return Objects.hash(current, limit, resetTime, windowSize);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class LimitInfo {\n");
    sb.append("    current: ").append(toIndentedString(current)).append("\n");
    sb.append("    limit: ").append(toIndentedString(limit)).append("\n");
    sb.append("    resetTime: ").append(toIndentedString(resetTime)).append("\n");
    sb.append("    windowSize: ").append(toIndentedString(windowSize)).append("\n");
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

