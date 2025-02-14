package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.WebhookConfig;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * ListWebhooksResponse
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T12:22:33.767757-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ListWebhooksResponse {

  @Valid
  private List<@Valid WebhookConfig> webhooks = new ArrayList<>();

  private Integer nextPageNumber;

  private Integer totalCount;

  public ListWebhooksResponse webhooks(List<@Valid WebhookConfig> webhooks) {
    this.webhooks = webhooks;
    return this;
  }

  public ListWebhooksResponse addWebhooksItem(WebhookConfig webhooksItem) {
    if (this.webhooks == null) {
      this.webhooks = new ArrayList<>();
    }
    this.webhooks.add(webhooksItem);
    return this;
  }

  /**
   * Get webhooks
   * @return webhooks
   */
  @Valid 
  @Schema(name = "webhooks", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("webhooks")
  public List<@Valid WebhookConfig> getWebhooks() {
    return webhooks;
  }

  public void setWebhooks(List<@Valid WebhookConfig> webhooks) {
    this.webhooks = webhooks;
  }

  public ListWebhooksResponse nextPageNumber(Integer nextPageNumber) {
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

  public ListWebhooksResponse totalCount(Integer totalCount) {
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
    ListWebhooksResponse listWebhooksResponse = (ListWebhooksResponse) o;
    return Objects.equals(this.webhooks, listWebhooksResponse.webhooks) &&
        Objects.equals(this.nextPageNumber, listWebhooksResponse.nextPageNumber) &&
        Objects.equals(this.totalCount, listWebhooksResponse.totalCount);
  }

  @Override
  public int hashCode() {
    return Objects.hash(webhooks, nextPageNumber, totalCount);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ListWebhooksResponse {\n");
    sb.append("    webhooks: ").append(toIndentedString(webhooks)).append("\n");
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

