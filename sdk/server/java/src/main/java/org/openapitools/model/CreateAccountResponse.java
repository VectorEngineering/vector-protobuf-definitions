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
 * CreateAccountResponse
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T00:26:22.645676-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class CreateAccountResponse {

  private Account account;

  private String initialWorkspaceId;

  public CreateAccountResponse account(Account account) {
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

  public CreateAccountResponse initialWorkspaceId(String initialWorkspaceId) {
    this.initialWorkspaceId = initialWorkspaceId;
    return this;
  }

  /**
   * Get initialWorkspaceId
   * @return initialWorkspaceId
   */
  
  @Schema(name = "initialWorkspaceId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("initialWorkspaceId")
  public String getInitialWorkspaceId() {
    return initialWorkspaceId;
  }

  public void setInitialWorkspaceId(String initialWorkspaceId) {
    this.initialWorkspaceId = initialWorkspaceId;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    CreateAccountResponse createAccountResponse = (CreateAccountResponse) o;
    return Objects.equals(this.account, createAccountResponse.account) &&
        Objects.equals(this.initialWorkspaceId, createAccountResponse.initialWorkspaceId);
  }

  @Override
  public int hashCode() {
    return Objects.hash(account, initialWorkspaceId);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class CreateAccountResponse {\n");
    sb.append("    account: ").append(toIndentedString(account)).append("\n");
    sb.append("    initialWorkspaceId: ").append(toIndentedString(initialWorkspaceId)).append("\n");
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

