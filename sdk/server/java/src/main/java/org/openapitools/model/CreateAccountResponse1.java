package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import org.openapitools.model.Account1;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * CreateAccountResponse1
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-05T09:09:05.453051-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class CreateAccountResponse1 {

  private Account1 account;

  private String initialWorkspaceName;

  public CreateAccountResponse1 account(Account1 account) {
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
  public Account1 getAccount() {
    return account;
  }

  public void setAccount(Account1 account) {
    this.account = account;
  }

  public CreateAccountResponse1 initialWorkspaceName(String initialWorkspaceName) {
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

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    CreateAccountResponse1 createAccountResponse1 = (CreateAccountResponse1) o;
    return Objects.equals(this.account, createAccountResponse1.account) &&
        Objects.equals(this.initialWorkspaceName, createAccountResponse1.initialWorkspaceName);
  }

  @Override
  public int hashCode() {
    return Objects.hash(account, initialWorkspaceName);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class CreateAccountResponse1 {\n");
    sb.append("    account: ").append(toIndentedString(account)).append("\n");
    sb.append("    initialWorkspaceName: ").append(toIndentedString(initialWorkspaceName)).append("\n");
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

