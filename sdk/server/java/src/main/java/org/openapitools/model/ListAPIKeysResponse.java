package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.APIKey;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * ListAPIKeysResponse
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T07:41:41.360877-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ListAPIKeysResponse {

  @Valid
  private List<@Valid APIKey> apiKeys = new ArrayList<>();

  private Integer nextPageNumber;

  private Integer totalCount;

  public ListAPIKeysResponse apiKeys(List<@Valid APIKey> apiKeys) {
    this.apiKeys = apiKeys;
    return this;
  }

  public ListAPIKeysResponse addApiKeysItem(APIKey apiKeysItem) {
    if (this.apiKeys == null) {
      this.apiKeys = new ArrayList<>();
    }
    this.apiKeys.add(apiKeysItem);
    return this;
  }

  /**
   * Get apiKeys
   * @return apiKeys
   */
  @Valid 
  @Schema(name = "apiKeys", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("apiKeys")
  public List<@Valid APIKey> getApiKeys() {
    return apiKeys;
  }

  public void setApiKeys(List<@Valid APIKey> apiKeys) {
    this.apiKeys = apiKeys;
  }

  public ListAPIKeysResponse nextPageNumber(Integer nextPageNumber) {
    this.nextPageNumber = nextPageNumber;
    return this;
  }

  /**
   * Get nextPageNumber
   * @return nextPageNumber
   */
  
  @Schema(name = "nextPageNumber", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("nextPageNumber")
  public Integer getNextPageNumber() {
    return nextPageNumber;
  }

  public void setNextPageNumber(Integer nextPageNumber) {
    this.nextPageNumber = nextPageNumber;
  }

  public ListAPIKeysResponse totalCount(Integer totalCount) {
    this.totalCount = totalCount;
    return this;
  }

  /**
   * Get totalCount
   * @return totalCount
   */
  
  @Schema(name = "totalCount", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("totalCount")
  public Integer getTotalCount() {
    return totalCount;
  }

  public void setTotalCount(Integer totalCount) {
    this.totalCount = totalCount;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    ListAPIKeysResponse listAPIKeysResponse = (ListAPIKeysResponse) o;
    return Objects.equals(this.apiKeys, listAPIKeysResponse.apiKeys) &&
        Objects.equals(this.nextPageNumber, listAPIKeysResponse.nextPageNumber) &&
        Objects.equals(this.totalCount, listAPIKeysResponse.totalCount);
  }

  @Override
  public int hashCode() {
    return Objects.hash(apiKeys, nextPageNumber, totalCount);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ListAPIKeysResponse {\n");
    sb.append("    apiKeys: ").append(toIndentedString(apiKeys)).append("\n");
    sb.append("    nextPageNumber: ").append(toIndentedString(nextPageNumber)).append("\n");
    sb.append("    totalCount: ").append(toIndentedString(totalCount)).append("\n");
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

