package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import org.openapitools.model.Account;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * CreateAccountRequest
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-19T02:14:00.124734-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class CreateAccountRequest {

  private Account account;

  private String initialWorkspaceName;

  private String organizationId;

  private String tenantId;

  public CreateAccountRequest account(Account account) {
    this.account = account;
    return this;
  }

  /**
   * Get account
   * @return account
   */
  @Valid 
  @Schema(name = "account", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("account")
  public Account getAccount() {
    return account;
  }

  public void setAccount(Account account) {
    this.account = account;
  }

  public CreateAccountRequest initialWorkspaceName(String initialWorkspaceName) {
    this.initialWorkspaceName = initialWorkspaceName;
    return this;
  }

  /**
   * Get initialWorkspaceName
   * @return initialWorkspaceName
   */
  
  @Schema(name = "initialWorkspaceName", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("initialWorkspaceName")
  public String getInitialWorkspaceName() {
    return initialWorkspaceName;
  }

  public void setInitialWorkspaceName(String initialWorkspaceName) {
    this.initialWorkspaceName = initialWorkspaceName;
  }

  public CreateAccountRequest organizationId(String organizationId) {
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

  public CreateAccountRequest tenantId(String tenantId) {
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
    CreateAccountRequest createAccountRequest = (CreateAccountRequest) o;
    return Objects.equals(this.account, createAccountRequest.account) &&
        Objects.equals(this.initialWorkspaceName, createAccountRequest.initialWorkspaceName) &&
        Objects.equals(this.organizationId, createAccountRequest.organizationId) &&
        Objects.equals(this.tenantId, createAccountRequest.tenantId);
  }

  @Override
  public int hashCode() {
    return Objects.hash(account, initialWorkspaceName, organizationId, tenantId);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class CreateAccountRequest {\n");
    sb.append("    account: ").append(toIndentedString(account)).append("\n");
    sb.append("    initialWorkspaceName: ").append(toIndentedString(initialWorkspaceName)).append("\n");
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

