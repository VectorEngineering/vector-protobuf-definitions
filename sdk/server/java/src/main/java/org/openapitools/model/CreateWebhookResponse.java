package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import org.openapitools.model.WebhookConfig;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * CreateWebhookResponse
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-19T01:32:43.436930-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class CreateWebhookResponse {

  private WebhookConfig webhook;

  public CreateWebhookResponse webhook(WebhookConfig webhook) {
    this.webhook = webhook;
    return this;
  }

  /**
   * Get webhook
   * @return webhook
   */
  @Valid 
  @Schema(name = "webhook", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("webhook")
  public WebhookConfig getWebhook() {
    return webhook;
  }

  public void setWebhook(WebhookConfig webhook) {
    this.webhook = webhook;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    CreateWebhookResponse createWebhookResponse = (CreateWebhookResponse) o;
    return Objects.equals(this.webhook, createWebhookResponse.webhook);
  }

  @Override
  public int hashCode() {
    return Objects.hash(webhook);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class CreateWebhookResponse {\n");
    sb.append("    webhook: ").append(toIndentedString(webhook)).append("\n");
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

