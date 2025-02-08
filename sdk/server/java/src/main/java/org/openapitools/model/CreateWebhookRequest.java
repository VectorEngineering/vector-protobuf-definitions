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
 * CreateWebhookRequest
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-08T09:28:11.323460-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class CreateWebhookRequest {

  private String organizationId;

  private String workspaceId;

  private String tenantId;

  private String accountId;

  private WebhookConfig webhook;

  public CreateWebhookRequest organizationId(String organizationId) {
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

  public CreateWebhookRequest workspaceId(String workspaceId) {
    this.workspaceId = workspaceId;
    return this;
  }

  /**
   * Get workspaceId
   * @return workspaceId
   */
  
  @Schema(name = "workspaceId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("workspaceId")
  public String getWorkspaceId() {
    return workspaceId;
  }

  public void setWorkspaceId(String workspaceId) {
    this.workspaceId = workspaceId;
  }

  public CreateWebhookRequest tenantId(String tenantId) {
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

  public CreateWebhookRequest accountId(String accountId) {
    this.accountId = accountId;
    return this;
  }

  /**
   * Get accountId
   * @return accountId
   */
  
  @Schema(name = "accountId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("accountId")
  public String getAccountId() {
    return accountId;
  }

  public void setAccountId(String accountId) {
    this.accountId = accountId;
  }

  public CreateWebhookRequest webhook(WebhookConfig webhook) {
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
    CreateWebhookRequest createWebhookRequest = (CreateWebhookRequest) o;
    return Objects.equals(this.organizationId, createWebhookRequest.organizationId) &&
        Objects.equals(this.workspaceId, createWebhookRequest.workspaceId) &&
        Objects.equals(this.tenantId, createWebhookRequest.tenantId) &&
        Objects.equals(this.accountId, createWebhookRequest.accountId) &&
        Objects.equals(this.webhook, createWebhookRequest.webhook);
  }

  @Override
  public int hashCode() {
    return Objects.hash(organizationId, workspaceId, tenantId, accountId, webhook);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class CreateWebhookRequest {\n");
    sb.append("    organizationId: ").append(toIndentedString(organizationId)).append("\n");
    sb.append("    workspaceId: ").append(toIndentedString(workspaceId)).append("\n");
    sb.append("    tenantId: ").append(toIndentedString(tenantId)).append("\n");
    sb.append("    accountId: ").append(toIndentedString(accountId)).append("\n");
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

