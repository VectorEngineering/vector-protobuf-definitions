package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import org.openapitools.model.APIKey;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * RotateAPIKeyResponse
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T17:07:07.052157-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class RotateAPIKeyResponse {

  private APIKey newApiKey;

  private String newKeyValue;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime oldKeyExpiry;

  public RotateAPIKeyResponse newApiKey(APIKey newApiKey) {
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
  public APIKey getNewApiKey() {
    return newApiKey;
  }

  public void setNewApiKey(APIKey newApiKey) {
    this.newApiKey = newApiKey;
  }

  public RotateAPIKeyResponse newKeyValue(String newKeyValue) {
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

  public RotateAPIKeyResponse oldKeyExpiry(OffsetDateTime oldKeyExpiry) {
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

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    RotateAPIKeyResponse rotateAPIKeyResponse = (RotateAPIKeyResponse) o;
    return Objects.equals(this.newApiKey, rotateAPIKeyResponse.newApiKey) &&
        Objects.equals(this.newKeyValue, rotateAPIKeyResponse.newKeyValue) &&
        Objects.equals(this.oldKeyExpiry, rotateAPIKeyResponse.oldKeyExpiry);
  }

  @Override
  public int hashCode() {
    return Objects.hash(newApiKey, newKeyValue, oldKeyExpiry);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class RotateAPIKeyResponse {\n");
    sb.append("    newApiKey: ").append(toIndentedString(newApiKey)).append("\n");
    sb.append("    newKeyValue: ").append(toIndentedString(newKeyValue)).append("\n");
    sb.append("    oldKeyExpiry: ").append(toIndentedString(oldKeyExpiry)).append("\n");
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

