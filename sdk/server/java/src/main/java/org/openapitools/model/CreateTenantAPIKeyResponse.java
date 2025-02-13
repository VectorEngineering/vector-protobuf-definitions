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
 * CreateTenantAPIKeyResponse
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-13T18:55:20.876030-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class CreateTenantAPIKeyResponse {

  private String keyId;

  private String keyValue;

  public CreateTenantAPIKeyResponse keyId(String keyId) {
    this.keyId = keyId;
    return this;
  }

  /**
   * Get keyId
   * @return keyId
   */
  
  @Schema(name = "keyId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("keyId")
  public String getKeyId() {
    return keyId;
  }

  public void setKeyId(String keyId) {
    this.keyId = keyId;
  }

  public CreateTenantAPIKeyResponse keyValue(String keyValue) {
    this.keyValue = keyValue;
    return this;
  }

  /**
   * Get keyValue
   * @return keyValue
   */
  
  @Schema(name = "keyValue", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("keyValue")
  public String getKeyValue() {
    return keyValue;
  }

  public void setKeyValue(String keyValue) {
    this.keyValue = keyValue;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    CreateTenantAPIKeyResponse createTenantAPIKeyResponse = (CreateTenantAPIKeyResponse) o;
    return Objects.equals(this.keyId, createTenantAPIKeyResponse.keyId) &&
        Objects.equals(this.keyValue, createTenantAPIKeyResponse.keyValue);
  }

  @Override
  public int hashCode() {
    return Objects.hash(keyId, keyValue);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class CreateTenantAPIKeyResponse {\n");
    sb.append("    keyId: ").append(toIndentedString(keyId)).append("\n");
    sb.append("    keyValue: ").append(toIndentedString(keyValue)).append("\n");
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

