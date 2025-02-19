package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import java.time.OffsetDateTime;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.TenantAPIKeyScope;
import org.openapitools.model.V1Status;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * TenantAPIKey
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-18T23:01:10.850512-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class TenantAPIKey {

  private String id;

  private String keyHash;

  private String keyPrefix;

  private String name;

  private String description;

  private V1Status status = V1Status.UNSPECIFIED;

  @Valid
  private List<TenantAPIKeyScope> scopes = new ArrayList<>();

  private Integer maxUses;

  @Valid
  private List<String> allowedIps = new ArrayList<>();

  private Integer useCount;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime expiresAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime deletedAt;

  public TenantAPIKey id(String id) {
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

  public TenantAPIKey keyHash(String keyHash) {
    this.keyHash = keyHash;
    return this;
  }

  /**
   * Get keyHash
   * @return keyHash
   */
  
  @Schema(name = "keyHash", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("keyHash")
  public String getKeyHash() {
    return keyHash;
  }

  public void setKeyHash(String keyHash) {
    this.keyHash = keyHash;
  }

  public TenantAPIKey keyPrefix(String keyPrefix) {
    this.keyPrefix = keyPrefix;
    return this;
  }

  /**
   * Get keyPrefix
   * @return keyPrefix
   */
  
  @Schema(name = "keyPrefix", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("keyPrefix")
  public String getKeyPrefix() {
    return keyPrefix;
  }

  public void setKeyPrefix(String keyPrefix) {
    this.keyPrefix = keyPrefix;
  }

  public TenantAPIKey name(String name) {
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

  public TenantAPIKey description(String description) {
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

  public TenantAPIKey status(V1Status status) {
    this.status = status;
    return this;
  }

  /**
   * Get status
   * @return status
   */
  @Valid 
  @Schema(name = "status", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("status")
  public V1Status getStatus() {
    return status;
  }

  public void setStatus(V1Status status) {
    this.status = status;
  }

  public TenantAPIKey scopes(List<TenantAPIKeyScope> scopes) {
    this.scopes = scopes;
    return this;
  }

  public TenantAPIKey addScopesItem(TenantAPIKeyScope scopesItem) {
    if (this.scopes == null) {
      this.scopes = new ArrayList<>();
    }
    this.scopes.add(scopesItem);
    return this;
  }

  /**
   * Get scopes
   * @return scopes
   */
  @Valid 
  @Schema(name = "scopes", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("scopes")
  public List<TenantAPIKeyScope> getScopes() {
    return scopes;
  }

  public void setScopes(List<TenantAPIKeyScope> scopes) {
    this.scopes = scopes;
  }

  public TenantAPIKey maxUses(Integer maxUses) {
    this.maxUses = maxUses;
    return this;
  }

  /**
   * Get maxUses
   * @return maxUses
   */
  
  @Schema(name = "maxUses", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("maxUses")
  public Integer getMaxUses() {
    return maxUses;
  }

  public void setMaxUses(Integer maxUses) {
    this.maxUses = maxUses;
  }

  public TenantAPIKey allowedIps(List<String> allowedIps) {
    this.allowedIps = allowedIps;
    return this;
  }

  public TenantAPIKey addAllowedIpsItem(String allowedIpsItem) {
    if (this.allowedIps == null) {
      this.allowedIps = new ArrayList<>();
    }
    this.allowedIps.add(allowedIpsItem);
    return this;
  }

  /**
   * Get allowedIps
   * @return allowedIps
   */
  
  @Schema(name = "allowedIps", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("allowedIps")
  public List<String> getAllowedIps() {
    return allowedIps;
  }

  public void setAllowedIps(List<String> allowedIps) {
    this.allowedIps = allowedIps;
  }

  public TenantAPIKey useCount(Integer useCount) {
    this.useCount = useCount;
    return this;
  }

  /**
   * Get useCount
   * @return useCount
   */
  
  @Schema(name = "useCount", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("useCount")
  public Integer getUseCount() {
    return useCount;
  }

  public void setUseCount(Integer useCount) {
    this.useCount = useCount;
  }

  public TenantAPIKey expiresAt(OffsetDateTime expiresAt) {
    this.expiresAt = expiresAt;
    return this;
  }

  /**
   * Get expiresAt
   * @return expiresAt
   */
  @Valid 
  @Schema(name = "expiresAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("expiresAt")
  public OffsetDateTime getExpiresAt() {
    return expiresAt;
  }

  public void setExpiresAt(OffsetDateTime expiresAt) {
    this.expiresAt = expiresAt;
  }

  public TenantAPIKey createdAt(OffsetDateTime createdAt) {
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

  public TenantAPIKey updatedAt(OffsetDateTime updatedAt) {
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

  public TenantAPIKey deletedAt(OffsetDateTime deletedAt) {
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
    TenantAPIKey tenantAPIKey = (TenantAPIKey) o;
    return Objects.equals(this.id, tenantAPIKey.id) &&
        Objects.equals(this.keyHash, tenantAPIKey.keyHash) &&
        Objects.equals(this.keyPrefix, tenantAPIKey.keyPrefix) &&
        Objects.equals(this.name, tenantAPIKey.name) &&
        Objects.equals(this.description, tenantAPIKey.description) &&
        Objects.equals(this.status, tenantAPIKey.status) &&
        Objects.equals(this.scopes, tenantAPIKey.scopes) &&
        Objects.equals(this.maxUses, tenantAPIKey.maxUses) &&
        Objects.equals(this.allowedIps, tenantAPIKey.allowedIps) &&
        Objects.equals(this.useCount, tenantAPIKey.useCount) &&
        Objects.equals(this.expiresAt, tenantAPIKey.expiresAt) &&
        Objects.equals(this.createdAt, tenantAPIKey.createdAt) &&
        Objects.equals(this.updatedAt, tenantAPIKey.updatedAt) &&
        Objects.equals(this.deletedAt, tenantAPIKey.deletedAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, keyHash, keyPrefix, name, description, status, scopes, maxUses, allowedIps, useCount, expiresAt, createdAt, updatedAt, deletedAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class TenantAPIKey {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    keyHash: ").append(toIndentedString(keyHash)).append("\n");
    sb.append("    keyPrefix: ").append(toIndentedString(keyPrefix)).append("\n");
    sb.append("    name: ").append(toIndentedString(name)).append("\n");
    sb.append("    description: ").append(toIndentedString(description)).append("\n");
    sb.append("    status: ").append(toIndentedString(status)).append("\n");
    sb.append("    scopes: ").append(toIndentedString(scopes)).append("\n");
    sb.append("    maxUses: ").append(toIndentedString(maxUses)).append("\n");
    sb.append("    allowedIps: ").append(toIndentedString(allowedIps)).append("\n");
    sb.append("    useCount: ").append(toIndentedString(useCount)).append("\n");
    sb.append("    expiresAt: ").append(toIndentedString(expiresAt)).append("\n");
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

