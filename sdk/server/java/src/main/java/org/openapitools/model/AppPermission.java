package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
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
 * AppPermission
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-13T18:55:20.876030-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class AppPermission {

  private String id;

  private String appId;

  private String scope;

  private String description;

  private Boolean isRequired;

  @Valid
  private List<String> accessLevels = new ArrayList<>();

  @Valid
  private Map<String, String> constraints = new HashMap<>();

  private Boolean requiresApproval;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  public AppPermission id(String id) {
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

  public AppPermission appId(String appId) {
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

  public AppPermission scope(String scope) {
    this.scope = scope;
    return this;
  }

  /**
   * Get scope
   * @return scope
   */
  
  @Schema(name = "scope", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("scope")
  public String getScope() {
    return scope;
  }

  public void setScope(String scope) {
    this.scope = scope;
  }

  public AppPermission description(String description) {
    this.description = description;
    return this;
  }

  /**
   * Get description
   * @return description
   */
  
  @Schema(name = "description", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("description")
  public String getDescription() {
    return description;
  }

  public void setDescription(String description) {
    this.description = description;
  }

  public AppPermission isRequired(Boolean isRequired) {
    this.isRequired = isRequired;
    return this;
  }

  /**
   * Get isRequired
   * @return isRequired
   */
  
  @Schema(name = "isRequired", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("isRequired")
  public Boolean getIsRequired() {
    return isRequired;
  }

  public void setIsRequired(Boolean isRequired) {
    this.isRequired = isRequired;
  }

  public AppPermission accessLevels(List<String> accessLevels) {
    this.accessLevels = accessLevels;
    return this;
  }

  public AppPermission addAccessLevelsItem(String accessLevelsItem) {
    if (this.accessLevels == null) {
      this.accessLevels = new ArrayList<>();
    }
    this.accessLevels.add(accessLevelsItem);
    return this;
  }

  /**
   * Get accessLevels
   * @return accessLevels
   */
  
  @Schema(name = "accessLevels", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("accessLevels")
  public List<String> getAccessLevels() {
    return accessLevels;
  }

  public void setAccessLevels(List<String> accessLevels) {
    this.accessLevels = accessLevels;
  }

  public AppPermission constraints(Map<String, String> constraints) {
    this.constraints = constraints;
    return this;
  }

  public AppPermission putConstraintsItem(String key, String constraintsItem) {
    if (this.constraints == null) {
      this.constraints = new HashMap<>();
    }
    this.constraints.put(key, constraintsItem);
    return this;
  }

  /**
   * Get constraints
   * @return constraints
   */
  
  @Schema(name = "constraints", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("constraints")
  public Map<String, String> getConstraints() {
    return constraints;
  }

  public void setConstraints(Map<String, String> constraints) {
    this.constraints = constraints;
  }

  public AppPermission requiresApproval(Boolean requiresApproval) {
    this.requiresApproval = requiresApproval;
    return this;
  }

  /**
   * Get requiresApproval
   * @return requiresApproval
   */
  
  @Schema(name = "requiresApproval", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("requiresApproval")
  public Boolean getRequiresApproval() {
    return requiresApproval;
  }

  public void setRequiresApproval(Boolean requiresApproval) {
    this.requiresApproval = requiresApproval;
  }

  public AppPermission createdAt(OffsetDateTime createdAt) {
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

  public AppPermission updatedAt(OffsetDateTime updatedAt) {
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

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    AppPermission appPermission = (AppPermission) o;
    return Objects.equals(this.id, appPermission.id) &&
        Objects.equals(this.appId, appPermission.appId) &&
        Objects.equals(this.scope, appPermission.scope) &&
        Objects.equals(this.description, appPermission.description) &&
        Objects.equals(this.isRequired, appPermission.isRequired) &&
        Objects.equals(this.accessLevels, appPermission.accessLevels) &&
        Objects.equals(this.constraints, appPermission.constraints) &&
        Objects.equals(this.requiresApproval, appPermission.requiresApproval) &&
        Objects.equals(this.createdAt, appPermission.createdAt) &&
        Objects.equals(this.updatedAt, appPermission.updatedAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, appId, scope, description, isRequired, accessLevels, constraints, requiresApproval, createdAt, updatedAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class AppPermission {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    appId: ").append(toIndentedString(appId)).append("\n");
    sb.append("    scope: ").append(toIndentedString(scope)).append("\n");
    sb.append("    description: ").append(toIndentedString(description)).append("\n");
    sb.append("    isRequired: ").append(toIndentedString(isRequired)).append("\n");
    sb.append("    accessLevels: ").append(toIndentedString(accessLevels)).append("\n");
    sb.append("    constraints: ").append(toIndentedString(constraints)).append("\n");
    sb.append("    requiresApproval: ").append(toIndentedString(requiresApproval)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
    sb.append("    updatedAt: ").append(toIndentedString(updatedAt)).append("\n");
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

