package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * RotateTenantAPIKeyBody
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-05T07:37:18.524575-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class RotateTenantAPIKeyBody {

  private String organizationId;

  private String tenantId;

  public RotateTenantAPIKeyBody organizationId(String organizationId) {
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

  public RotateTenantAPIKeyBody tenantId(String tenantId) {
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

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    RotateTenantAPIKeyBody rotateTenantAPIKeyBody = (RotateTenantAPIKeyBody) o;
    return Objects.equals(this.organizationId, rotateTenantAPIKeyBody.organizationId) &&
        Objects.equals(this.tenantId, rotateTenantAPIKeyBody.tenantId);
  }

  @Override
  public int hashCode() {
    return Objects.hash(organizationId, tenantId);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class RotateTenantAPIKeyBody {\n");
    sb.append("    organizationId: ").append(toIndentedString(organizationId)).append("\n");
    sb.append("    tenantId: ").append(toIndentedString(tenantId)).append("\n");
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

