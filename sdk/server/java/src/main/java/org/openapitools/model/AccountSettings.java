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
 * AccountSettings
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-08T10:46:38.744443-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class AccountSettings {

  private String id;

  private Boolean emailNotifications;

  private Boolean slackNotifications;

  private String defaultDataRetention;

  private Boolean autoPurgeEnabled;

  private Boolean require2fa;

  private String sessionTimeout;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime deletedAt;

  public AccountSettings id(String id) {
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

  public AccountSettings emailNotifications(Boolean emailNotifications) {
    this.emailNotifications = emailNotifications;
    return this;
  }

  /**
   * Get emailNotifications
   * @return emailNotifications
   */
  
  @Schema(name = "emailNotifications", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("emailNotifications")
  public Boolean getEmailNotifications() {
    return emailNotifications;
  }

  public void setEmailNotifications(Boolean emailNotifications) {
    this.emailNotifications = emailNotifications;
  }

  public AccountSettings slackNotifications(Boolean slackNotifications) {
    this.slackNotifications = slackNotifications;
    return this;
  }

  /**
   * Get slackNotifications
   * @return slackNotifications
   */
  
  @Schema(name = "slackNotifications", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("slackNotifications")
  public Boolean getSlackNotifications() {
    return slackNotifications;
  }

  public void setSlackNotifications(Boolean slackNotifications) {
    this.slackNotifications = slackNotifications;
  }

  public AccountSettings defaultDataRetention(String defaultDataRetention) {
    this.defaultDataRetention = defaultDataRetention;
    return this;
  }

  /**
   * Get defaultDataRetention
   * @return defaultDataRetention
   */
  
  @Schema(name = "defaultDataRetention", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("defaultDataRetention")
  public String getDefaultDataRetention() {
    return defaultDataRetention;
  }

  public void setDefaultDataRetention(String defaultDataRetention) {
    this.defaultDataRetention = defaultDataRetention;
  }

  public AccountSettings autoPurgeEnabled(Boolean autoPurgeEnabled) {
    this.autoPurgeEnabled = autoPurgeEnabled;
    return this;
  }

  /**
   * Get autoPurgeEnabled
   * @return autoPurgeEnabled
   */
  
  @Schema(name = "autoPurgeEnabled", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("autoPurgeEnabled")
  public Boolean getAutoPurgeEnabled() {
    return autoPurgeEnabled;
  }

  public void setAutoPurgeEnabled(Boolean autoPurgeEnabled) {
    this.autoPurgeEnabled = autoPurgeEnabled;
  }

  public AccountSettings require2fa(Boolean require2fa) {
    this.require2fa = require2fa;
    return this;
  }

  /**
   * Get require2fa
   * @return require2fa
   */
  
  @Schema(name = "require2fa", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("require2fa")
  public Boolean getRequire2fa() {
    return require2fa;
  }

  public void setRequire2fa(Boolean require2fa) {
    this.require2fa = require2fa;
  }

  public AccountSettings sessionTimeout(String sessionTimeout) {
    this.sessionTimeout = sessionTimeout;
    return this;
  }

  /**
   * Get sessionTimeout
   * @return sessionTimeout
   */
  
  @Schema(name = "sessionTimeout", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("sessionTimeout")
  public String getSessionTimeout() {
    return sessionTimeout;
  }

  public void setSessionTimeout(String sessionTimeout) {
    this.sessionTimeout = sessionTimeout;
  }

  public AccountSettings createdAt(OffsetDateTime createdAt) {
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

  public AccountSettings updatedAt(OffsetDateTime updatedAt) {
    this.updatedAt = updatedAt;
    return this;
  }

  /**
   * Get updatedAt
   * @return updatedAt
   */
  @Valid 
  @Schema(name = "updatedAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("updatedAt")
  public OffsetDateTime getUpdatedAt() {
    return updatedAt;
  }

  public void setUpdatedAt(OffsetDateTime updatedAt) {
    this.updatedAt = updatedAt;
  }

  public AccountSettings deletedAt(OffsetDateTime deletedAt) {
    this.deletedAt = deletedAt;
    return this;
  }

  /**
   * Get deletedAt
   * @return deletedAt
   */
  @Valid 
  @Schema(name = "deletedAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("deletedAt")
  public OffsetDateTime getDeletedAt() {
    return deletedAt;
  }

  public void setDeletedAt(OffsetDateTime deletedAt) {
    this.deletedAt = deletedAt;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    AccountSettings accountSettings = (AccountSettings) o;
    return Objects.equals(this.id, accountSettings.id) &&
        Objects.equals(this.emailNotifications, accountSettings.emailNotifications) &&
        Objects.equals(this.slackNotifications, accountSettings.slackNotifications) &&
        Objects.equals(this.defaultDataRetention, accountSettings.defaultDataRetention) &&
        Objects.equals(this.autoPurgeEnabled, accountSettings.autoPurgeEnabled) &&
        Objects.equals(this.require2fa, accountSettings.require2fa) &&
        Objects.equals(this.sessionTimeout, accountSettings.sessionTimeout) &&
        Objects.equals(this.createdAt, accountSettings.createdAt) &&
        Objects.equals(this.updatedAt, accountSettings.updatedAt) &&
        Objects.equals(this.deletedAt, accountSettings.deletedAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, emailNotifications, slackNotifications, defaultDataRetention, autoPurgeEnabled, require2fa, sessionTimeout, createdAt, updatedAt, deletedAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class AccountSettings {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    emailNotifications: ").append(toIndentedString(emailNotifications)).append("\n");
    sb.append("    slackNotifications: ").append(toIndentedString(slackNotifications)).append("\n");
    sb.append("    defaultDataRetention: ").append(toIndentedString(defaultDataRetention)).append("\n");
    sb.append("    autoPurgeEnabled: ").append(toIndentedString(autoPurgeEnabled)).append("\n");
    sb.append("    require2fa: ").append(toIndentedString(require2fa)).append("\n");
    sb.append("    sessionTimeout: ").append(toIndentedString(sessionTimeout)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
    sb.append("    updatedAt: ").append(toIndentedString(updatedAt)).append("\n");
    sb.append("    deletedAt: ").append(toIndentedString(deletedAt)).append("\n");
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

