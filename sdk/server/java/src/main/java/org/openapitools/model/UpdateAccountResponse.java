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
 * UpdateAccountResponse
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-03T06:30:06.935816-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class UpdateAccountResponse {

  private Account account;

  public UpdateAccountResponse account(Account account) {
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

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    UpdateAccountResponse updateAccountResponse = (UpdateAccountResponse) o;
    return Objects.equals(this.account, updateAccountResponse.account);
  }

  @Override
  public int hashCode() {
    return Objects.hash(account);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class UpdateAccountResponse {\n");
    sb.append("    account: ").append(toIndentedString(account)).append("\n");
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

