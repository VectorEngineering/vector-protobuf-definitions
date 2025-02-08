package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * APIInfo
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T21:14:09.107639-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class APIInfo {

  private String version;

  @Valid
  private List<String> supportedVersions = new ArrayList<>();

  private Boolean isDeprecated;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime sunsetDate;

  public APIInfo version(String version) {
    this.version = version;
    return this;
  }

  /**
   * Get version
   * @return version
   */
  
  @Schema(name = "version", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("version")
  public String getVersion() {
    return version;
  }

  public void setVersion(String version) {
    this.version = version;
  }

  public APIInfo supportedVersions(List<String> supportedVersions) {
    this.supportedVersions = supportedVersions;
    return this;
  }

  public APIInfo addSupportedVersionsItem(String supportedVersionsItem) {
    if (this.supportedVersions == null) {
      this.supportedVersions = new ArrayList<>();
    }
    this.supportedVersions.add(supportedVersionsItem);
    return this;
  }

  /**
   * Get supportedVersions
   * @return supportedVersions
   */
  
  @Schema(name = "supportedVersions", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("supportedVersions")
  public List<String> getSupportedVersions() {
    return supportedVersions;
  }

  public void setSupportedVersions(List<String> supportedVersions) {
    this.supportedVersions = supportedVersions;
  }

  public APIInfo isDeprecated(Boolean isDeprecated) {
    this.isDeprecated = isDeprecated;
    return this;
  }

  /**
   * Get isDeprecated
   * @return isDeprecated
   */
  
  @Schema(name = "isDeprecated", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("isDeprecated")
  public Boolean getIsDeprecated() {
    return isDeprecated;
  }

  public void setIsDeprecated(Boolean isDeprecated) {
    this.isDeprecated = isDeprecated;
  }

  public APIInfo sunsetDate(OffsetDateTime sunsetDate) {
    this.sunsetDate = sunsetDate;
    return this;
  }

  /**
   * Get sunsetDate
   * @return sunsetDate
   */
  @Valid 
  @Schema(name = "sunsetDate", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("sunsetDate")
  public OffsetDateTime getSunsetDate() {
    return sunsetDate;
  }

  public void setSunsetDate(OffsetDateTime sunsetDate) {
    this.sunsetDate = sunsetDate;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    APIInfo apIInfo = (APIInfo) o;
    return Objects.equals(this.version, apIInfo.version) &&
        Objects.equals(this.supportedVersions, apIInfo.supportedVersions) &&
        Objects.equals(this.isDeprecated, apIInfo.isDeprecated) &&
        Objects.equals(this.sunsetDate, apIInfo.sunsetDate);
  }

  @Override
  public int hashCode() {
    return Objects.hash(version, supportedVersions, isDeprecated, sunsetDate);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class APIInfo {\n");
    sb.append("    version: ").append(toIndentedString(version)).append("\n");
    sb.append("    supportedVersions: ").append(toIndentedString(supportedVersions)).append("\n");
    sb.append("    isDeprecated: ").append(toIndentedString(isDeprecated)).append("\n");
    sb.append("    sunsetDate: ").append(toIndentedString(sunsetDate)).append("\n");
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

