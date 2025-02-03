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
 * AppInstallation
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-03T15:12:04.029446-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class AppInstallation {

  private String id;

  private String appId;

  private String workspaceId;

  private String versionInstalled;

  @Valid
  private Map<String, String> _configuration = new HashMap<>();

  private String installationStatus;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime installedAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime lastUsed;

  public AppInstallation id(String id) {
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

  public AppInstallation appId(String appId) {
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

  public AppInstallation workspaceId(String workspaceId) {
    this.workspaceId = workspaceId;
    return this;
  }

  /**
   * Get workspaceId
   * @return workspaceId
   */
  
  @Schema(name = "workspaceId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("workspaceId")
  public String getWorkspaceId() {
    return workspaceId;
  }

  public void setWorkspaceId(String workspaceId) {
    this.workspaceId = workspaceId;
  }

  public AppInstallation versionInstalled(String versionInstalled) {
    this.versionInstalled = versionInstalled;
    return this;
  }

  /**
   * Get versionInstalled
   * @return versionInstalled
   */
  
  @Schema(name = "versionInstalled", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("versionInstalled")
  public String getVersionInstalled() {
    return versionInstalled;
  }

  public void setVersionInstalled(String versionInstalled) {
    this.versionInstalled = versionInstalled;
  }

  public AppInstallation _configuration(Map<String, String> _configuration) {
    this._configuration = _configuration;
    return this;
  }

  public AppInstallation putConfigurationItem(String key, String _configurationItem) {
    if (this._configuration == null) {
      this._configuration = new HashMap<>();
    }
    this._configuration.put(key, _configurationItem);
    return this;
  }

  /**
   * Get _configuration
   * @return _configuration
   */
  
  @Schema(name = "configuration", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("configuration")
  public Map<String, String> getConfiguration() {
    return _configuration;
  }

  public void setConfiguration(Map<String, String> _configuration) {
    this._configuration = _configuration;
  }

  public AppInstallation installationStatus(String installationStatus) {
    this.installationStatus = installationStatus;
    return this;
  }

  /**
   * Get installationStatus
   * @return installationStatus
   */
  
  @Schema(name = "installationStatus", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("installationStatus")
  public String getInstallationStatus() {
    return installationStatus;
  }

  public void setInstallationStatus(String installationStatus) {
    this.installationStatus = installationStatus;
  }

  public AppInstallation installedAt(OffsetDateTime installedAt) {
    this.installedAt = installedAt;
    return this;
  }

  /**
   * Get installedAt
   * @return installedAt
   */
  @Valid 
  @Schema(name = "installedAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("installedAt")
  public OffsetDateTime getInstalledAt() {
    return installedAt;
  }

  public void setInstalledAt(OffsetDateTime installedAt) {
    this.installedAt = installedAt;
  }

  public AppInstallation lastUsed(OffsetDateTime lastUsed) {
    this.lastUsed = lastUsed;
    return this;
  }

  /**
   * Get lastUsed
   * @return lastUsed
   */
  @Valid 
  @Schema(name = "lastUsed", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("lastUsed")
  public OffsetDateTime getLastUsed() {
    return lastUsed;
  }

  public void setLastUsed(OffsetDateTime lastUsed) {
    this.lastUsed = lastUsed;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    AppInstallation appInstallation = (AppInstallation) o;
    return Objects.equals(this.id, appInstallation.id) &&
        Objects.equals(this.appId, appInstallation.appId) &&
        Objects.equals(this.workspaceId, appInstallation.workspaceId) &&
        Objects.equals(this.versionInstalled, appInstallation.versionInstalled) &&
        Objects.equals(this._configuration, appInstallation._configuration) &&
        Objects.equals(this.installationStatus, appInstallation.installationStatus) &&
        Objects.equals(this.installedAt, appInstallation.installedAt) &&
        Objects.equals(this.lastUsed, appInstallation.lastUsed);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, appId, workspaceId, versionInstalled, _configuration, installationStatus, installedAt, lastUsed);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class AppInstallation {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    appId: ").append(toIndentedString(appId)).append("\n");
    sb.append("    workspaceId: ").append(toIndentedString(workspaceId)).append("\n");
    sb.append("    versionInstalled: ").append(toIndentedString(versionInstalled)).append("\n");
    sb.append("    _configuration: ").append(toIndentedString(_configuration)).append("\n");
    sb.append("    installationStatus: ").append(toIndentedString(installationStatus)).append("\n");
    sb.append("    installedAt: ").append(toIndentedString(installedAt)).append("\n");
    sb.append("    lastUsed: ").append(toIndentedString(lastUsed)).append("\n");
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

