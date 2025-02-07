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
 * AppVersion
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T17:56:14.620528-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class AppVersion {

  private String id;

  private String appId;

  private String versionNumber;

  private String changelog;

  @Valid
  private Map<String, String> requirements = new HashMap<>();

  private Boolean isPublic;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime releaseDate;

  public AppVersion id(String id) {
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

  public AppVersion appId(String appId) {
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

  public AppVersion versionNumber(String versionNumber) {
    this.versionNumber = versionNumber;
    return this;
  }

  /**
   * Get versionNumber
   * @return versionNumber
   */
  
  @Schema(name = "versionNumber", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("versionNumber")
  public String getVersionNumber() {
    return versionNumber;
  }

  public void setVersionNumber(String versionNumber) {
    this.versionNumber = versionNumber;
  }

  public AppVersion changelog(String changelog) {
    this.changelog = changelog;
    return this;
  }

  /**
   * Get changelog
   * @return changelog
   */
  
  @Schema(name = "changelog", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("changelog")
  public String getChangelog() {
    return changelog;
  }

  public void setChangelog(String changelog) {
    this.changelog = changelog;
  }

  public AppVersion requirements(Map<String, String> requirements) {
    this.requirements = requirements;
    return this;
  }

  public AppVersion putRequirementsItem(String key, String requirementsItem) {
    if (this.requirements == null) {
      this.requirements = new HashMap<>();
    }
    this.requirements.put(key, requirementsItem);
    return this;
  }

  /**
   * Get requirements
   * @return requirements
   */
  
  @Schema(name = "requirements", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("requirements")
  public Map<String, String> getRequirements() {
    return requirements;
  }

  public void setRequirements(Map<String, String> requirements) {
    this.requirements = requirements;
  }

  public AppVersion isPublic(Boolean isPublic) {
    this.isPublic = isPublic;
    return this;
  }

  /**
   * Get isPublic
   * @return isPublic
   */
  
  @Schema(name = "isPublic", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("isPublic")
  public Boolean getIsPublic() {
    return isPublic;
  }

  public void setIsPublic(Boolean isPublic) {
    this.isPublic = isPublic;
  }

  public AppVersion releaseDate(OffsetDateTime releaseDate) {
    this.releaseDate = releaseDate;
    return this;
  }

  /**
   * Get releaseDate
   * @return releaseDate
   */
  @Valid 
  @Schema(name = "releaseDate", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("releaseDate")
  public OffsetDateTime getReleaseDate() {
    return releaseDate;
  }

  public void setReleaseDate(OffsetDateTime releaseDate) {
    this.releaseDate = releaseDate;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    AppVersion appVersion = (AppVersion) o;
    return Objects.equals(this.id, appVersion.id) &&
        Objects.equals(this.appId, appVersion.appId) &&
        Objects.equals(this.versionNumber, appVersion.versionNumber) &&
        Objects.equals(this.changelog, appVersion.changelog) &&
        Objects.equals(this.requirements, appVersion.requirements) &&
        Objects.equals(this.isPublic, appVersion.isPublic) &&
        Objects.equals(this.releaseDate, appVersion.releaseDate);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, appId, versionNumber, changelog, requirements, isPublic, releaseDate);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class AppVersion {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    appId: ").append(toIndentedString(appId)).append("\n");
    sb.append("    versionNumber: ").append(toIndentedString(versionNumber)).append("\n");
    sb.append("    changelog: ").append(toIndentedString(changelog)).append("\n");
    sb.append("    requirements: ").append(toIndentedString(requirements)).append("\n");
    sb.append("    isPublic: ").append(toIndentedString(isPublic)).append("\n");
    sb.append("    releaseDate: ").append(toIndentedString(releaseDate)).append("\n");
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

