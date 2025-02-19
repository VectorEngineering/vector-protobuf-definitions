package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.Tenant;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * ListTenantsResponse
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-19T01:39:43.907035-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ListTenantsResponse {

  @Valid
  private List<@Valid Tenant> tenants = new ArrayList<>();

  private Integer nextPageNumber;

  public ListTenantsResponse tenants(List<@Valid Tenant> tenants) {
    this.tenants = tenants;
    return this;
  }

  public ListTenantsResponse addTenantsItem(Tenant tenantsItem) {
    if (this.tenants == null) {
      this.tenants = new ArrayList<>();
    }
    this.tenants.add(tenantsItem);
    return this;
  }

  /**
   * Get tenants
   * @return tenants
   */
  @Valid 
  @Schema(name = "tenants", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("tenants")
  public List<@Valid Tenant> getTenants() {
    return tenants;
  }

  public void setTenants(List<@Valid Tenant> tenants) {
    this.tenants = tenants;
  }

  public ListTenantsResponse nextPageNumber(Integer nextPageNumber) {
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

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    ListTenantsResponse listTenantsResponse = (ListTenantsResponse) o;
    return Objects.equals(this.tenants, listTenantsResponse.tenants) &&
        Objects.equals(this.nextPageNumber, listTenantsResponse.nextPageNumber);
  }

  @Override
  public int hashCode() {
    return Objects.hash(tenants, nextPageNumber);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ListTenantsResponse {\n");
    sb.append("    tenants: ").append(toIndentedString(tenants)).append("\n");
    sb.append("    nextPageNumber: ").append(toIndentedString(nextPageNumber)).append("\n");
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

