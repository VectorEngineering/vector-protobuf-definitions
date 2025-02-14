package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import org.openapitools.model.APIKey;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * CreateAPIKeyResponse
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T12:30:40.954679-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class CreateAPIKeyResponse {

  private APIKey apiKey;

  private String keyValue;

  public CreateAPIKeyResponse apiKey(APIKey apiKey) {
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
  public APIKey getApiKey() {
    return apiKey;
  }

  public void setApiKey(APIKey apiKey) {
    this.apiKey = apiKey;
  }

  public CreateAPIKeyResponse keyValue(String keyValue) {
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
    CreateAPIKeyResponse createAPIKeyResponse = (CreateAPIKeyResponse) o;
    return Objects.equals(this.apiKey, createAPIKeyResponse.apiKey) &&
        Objects.equals(this.keyValue, createAPIKeyResponse.keyValue);
  }

  @Override
  public int hashCode() {
    return Objects.hash(apiKey, keyValue);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class CreateAPIKeyResponse {\n");
    sb.append("    apiKey: ").append(toIndentedString(apiKey)).append("\n");
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

