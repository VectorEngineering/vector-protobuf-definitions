package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.Account;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * ListAccountsResponse
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-01-29T05:44:51.050519-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ListAccountsResponse {

  @Valid
  private List<@Valid Account> accounts = new ArrayList<>();

  private Integer total;

  public ListAccountsResponse accounts(List<@Valid Account> accounts) {
    this.accounts = accounts;
    return this;
  }

  public ListAccountsResponse addAccountsItem(Account accountsItem) {
    if (this.accounts == null) {
      this.accounts = new ArrayList<>();
    }
    this.accounts.add(accountsItem);
    return this;
  }

  /**
   * Get accounts
   * @return accounts
   */
  @Valid 
  @Schema(name = "accounts", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("accounts")
  public List<@Valid Account> getAccounts() {
    return accounts;
  }

  public void setAccounts(List<@Valid Account> accounts) {
    this.accounts = accounts;
  }

  public ListAccountsResponse total(Integer total) {
    this.total = total;
    return this;
  }

  /**
   * Get total
   * @return total
   */
  
  @Schema(name = "total", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("total")
  public Integer getTotal() {
    return total;
  }

  public void setTotal(Integer total) {
    this.total = total;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    ListAccountsResponse listAccountsResponse = (ListAccountsResponse) o;
    return Objects.equals(this.accounts, listAccountsResponse.accounts) &&
        Objects.equals(this.total, listAccountsResponse.total);
  }

  @Override
  public int hashCode() {
    return Objects.hash(accounts, total);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ListAccountsResponse {\n");
    sb.append("    accounts: ").append(toIndentedString(accounts)).append("\n");
    sb.append("    total: ").append(toIndentedString(total)).append("\n");
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

