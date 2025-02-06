package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import org.openapitools.model.TenantAPIKey;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * RotateTenantAPIKeyResponse
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-06T17:12:08.548747-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class RotateTenantAPIKeyResponse {

  private TenantAPIKey newApiKey;

  private String newKeyValue;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime oldKeyExpiry;

  private String tenantId;

  private String organizationId;

  public RotateTenantAPIKeyResponse newApiKey(TenantAPIKey newApiKey) {
    this.newApiKey = newApiKey;
    return this;
  }

  /**
   * Get newApiKey
   * @return newApiKey
   */
  @Valid 
  @Schema(name = "newApiKey", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("newApiKey")
  public TenantAPIKey getNewApiKey() {
    return newApiKey;
  }

  public void setNewApiKey(TenantAPIKey newApiKey) {
    this.newApiKey = newApiKey;
  }

  public RotateTenantAPIKeyResponse newKeyValue(String newKeyValue) {
    this.newKeyValue = newKeyValue;
    return this;
  }

  /**
   * Get newKeyValue
   * @return newKeyValue
   */
  
  @Schema(name = "newKeyValue", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("newKeyValue")
  public String getNewKeyValue() {
    return newKeyValue;
  }

  public void setNewKeyValue(String newKeyValue) {
    this.newKeyValue = newKeyValue;
  }

  public RotateTenantAPIKeyResponse oldKeyExpiry(OffsetDateTime oldKeyExpiry) {
    this.oldKeyExpiry = oldKeyExpiry;
    return this;
  }

  /**
   * Get oldKeyExpiry
   * @return oldKeyExpiry
   */
  @Valid 
  @Schema(name = "oldKeyExpiry", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("oldKeyExpiry")
  public OffsetDateTime getOldKeyExpiry() {
    return oldKeyExpiry;
  }

  public void setOldKeyExpiry(OffsetDateTime oldKeyExpiry) {
    this.oldKeyExpiry = oldKeyExpiry;
  }

  public RotateTenantAPIKeyResponse tenantId(String tenantId) {
    this.tenantId = tenantId;
    return this;
  }

  /**
   * Get tenantId
   * @return tenantId
   */
  
  @Schema(name = "tenantId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("tenantId")
  public String getTenantId() {
    return tenantId;
  }

  public void setTenantId(String tenantId) {
    this.tenantId = tenantId;
  }

  public RotateTenantAPIKeyResponse organizationId(String organizationId) {
    this.organizationId = organizationId;
    return this;
  }

  /**
   * Get organizationId
   * @return organizationId
   */
  
  @Schema(name = "organizationId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("organizationId")
  public String getOrganizationId() {
    return organizationId;
  }

  public void setOrganizationId(String organizationId) {
    this.organizationId = organizationId;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    RotateTenantAPIKeyResponse rotateTenantAPIKeyResponse = (RotateTenantAPIKeyResponse) o;
    return Objects.equals(this.newApiKey, rotateTenantAPIKeyResponse.newApiKey) &&
        Objects.equals(this.newKeyValue, rotateTenantAPIKeyResponse.newKeyValue) &&
        Objects.equals(this.oldKeyExpiry, rotateTenantAPIKeyResponse.oldKeyExpiry) &&
        Objects.equals(this.tenantId, rotateTenantAPIKeyResponse.tenantId) &&
        Objects.equals(this.organizationId, rotateTenantAPIKeyResponse.organizationId);
  }

  @Override
  public int hashCode() {
    return Objects.hash(newApiKey, newKeyValue, oldKeyExpiry, tenantId, organizationId);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class RotateTenantAPIKeyResponse {\n");
    sb.append("    newApiKey: ").append(toIndentedString(newApiKey)).append("\n");
    sb.append("    newKeyValue: ").append(toIndentedString(newKeyValue)).append("\n");
    sb.append("    oldKeyExpiry: ").append(toIndentedString(oldKeyExpiry)).append("\n");
    sb.append("    tenantId: ").append(toIndentedString(tenantId)).append("\n");
    sb.append("    organizationId: ").append(toIndentedString(organizationId)).append("\n");
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

