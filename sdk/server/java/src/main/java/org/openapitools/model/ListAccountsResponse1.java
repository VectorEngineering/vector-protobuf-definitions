package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.Account1;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * ListAccountsResponse1
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T00:28:49.378974-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ListAccountsResponse1 {

  @Valid
  private List<@Valid Account1> accounts = new ArrayList<>();

  private String nextPageToken;

  private Integer totalSize;

  public ListAccountsResponse1 accounts(List<@Valid Account1> accounts) {
    this.accounts = accounts;
    return this;
  }

  public ListAccountsResponse1 addAccountsItem(Account1 accountsItem) {
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
  public List<@Valid Account1> getAccounts() {
    return accounts;
  }

  public void setAccounts(List<@Valid Account1> accounts) {
    this.accounts = accounts;
  }

  public ListAccountsResponse1 nextPageToken(String nextPageToken) {
    this.nextPageToken = nextPageToken;
    return this;
  }

  /**
   * Get nextPageToken
   * @return nextPageToken
   */
  
  @Schema(name = "nextPageToken", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("nextPageToken")
  public String getNextPageToken() {
    return nextPageToken;
  }

  public void setNextPageToken(String nextPageToken) {
    this.nextPageToken = nextPageToken;
  }

  public ListAccountsResponse1 totalSize(Integer totalSize) {
    this.totalSize = totalSize;
    return this;
  }

  /**
   * Get totalSize
   * @return totalSize
   */
  
  @Schema(name = "totalSize", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("totalSize")
  public Integer getTotalSize() {
    return totalSize;
  }

  public void setTotalSize(Integer totalSize) {
    this.totalSize = totalSize;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    ListAccountsResponse1 listAccountsResponse1 = (ListAccountsResponse1) o;
    return Objects.equals(this.accounts, listAccountsResponse1.accounts) &&
        Objects.equals(this.nextPageToken, listAccountsResponse1.nextPageToken) &&
        Objects.equals(this.totalSize, listAccountsResponse1.totalSize);
  }

  @Override
  public int hashCode() {
    return Objects.hash(accounts, nextPageToken, totalSize);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ListAccountsResponse1 {\n");
    sb.append("    accounts: ").append(toIndentedString(accounts)).append("\n");
    sb.append("    nextPageToken: ").append(toIndentedString(nextPageToken)).append("\n");
    sb.append("    totalSize: ").append(toIndentedString(totalSize)).append("\n");
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

