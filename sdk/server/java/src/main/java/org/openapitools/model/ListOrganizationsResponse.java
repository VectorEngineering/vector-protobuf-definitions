package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.Organization;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * ListOrganizationsResponse
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-19T01:39:43.907035-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ListOrganizationsResponse {

  @Valid
  private List<@Valid Organization> organizations = new ArrayList<>();

  private Integer totalCount;

  private Integer nextPageNumber;

  public ListOrganizationsResponse organizations(List<@Valid Organization> organizations) {
    this.organizations = organizations;
    return this;
  }

  public ListOrganizationsResponse addOrganizationsItem(Organization organizationsItem) {
    if (this.organizations == null) {
      this.organizations = new ArrayList<>();
    }
    this.organizations.add(organizationsItem);
    return this;
  }

  /**
   * Get organizations
   * @return organizations
   */
  @Valid 
  @Schema(name = "organizations", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("organizations")
  public List<@Valid Organization> getOrganizations() {
    return organizations;
  }

  public void setOrganizations(List<@Valid Organization> organizations) {
    this.organizations = organizations;
  }

  public ListOrganizationsResponse totalCount(Integer totalCount) {
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

  public ListOrganizationsResponse nextPageNumber(Integer nextPageNumber) {
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
    ListOrganizationsResponse listOrganizationsResponse = (ListOrganizationsResponse) o;
    return Objects.equals(this.organizations, listOrganizationsResponse.organizations) &&
        Objects.equals(this.totalCount, listOrganizationsResponse.totalCount) &&
        Objects.equals(this.nextPageNumber, listOrganizationsResponse.nextPageNumber);
  }

  @Override
  public int hashCode() {
    return Objects.hash(organizations, totalCount, nextPageNumber);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ListOrganizationsResponse {\n");
    sb.append("    organizations: ").append(toIndentedString(organizations)).append("\n");
    sb.append("    totalCount: ").append(toIndentedString(totalCount)).append("\n");
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

