package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import org.openapitools.model.TenantAPIKey;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * UpdateTenantAPIKeyRequest
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-08T10:46:38.744443-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class UpdateTenantAPIKeyRequest {

  private TenantAPIKey apiKey;

  public UpdateTenantAPIKeyRequest apiKey(TenantAPIKey apiKey) {
    this.apiKey = apiKey;
    return this;
  }

  /**
   * Get apiKey
   * @return apiKey
   */
  @Valid 
  @Schema(name = "apiKey", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("apiKey")
  public TenantAPIKey getApiKey() {
    return apiKey;
  }

  public void setApiKey(TenantAPIKey apiKey) {
    this.apiKey = apiKey;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    UpdateTenantAPIKeyRequest updateTenantAPIKeyRequest = (UpdateTenantAPIKeyRequest) o;
    return Objects.equals(this.apiKey, updateTenantAPIKeyRequest.apiKey);
  }

  @Override
  public int hashCode() {
    return Objects.hash(apiKey);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class UpdateTenantAPIKeyRequest {\n");
    sb.append("    apiKey: ").append(toIndentedString(apiKey)).append("\n");
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

