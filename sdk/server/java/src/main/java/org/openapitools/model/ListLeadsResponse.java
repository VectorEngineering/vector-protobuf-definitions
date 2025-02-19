package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.Lead;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * ListLeadsResponse
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-19T11:22:55.992070-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ListLeadsResponse {

  @Valid
  private List<@Valid Lead> leads = new ArrayList<>();

  private Integer totalCount;

  private Integer nextPageNumber;

  public ListLeadsResponse leads(List<@Valid Lead> leads) {
    this.leads = leads;
    return this;
  }

  public ListLeadsResponse addLeadsItem(Lead leadsItem) {
    if (this.leads == null) {
      this.leads = new ArrayList<>();
    }
    this.leads.add(leadsItem);
    return this;
  }

  /**
   * Get leads
   * @return leads
   */
  @Valid 
  @Schema(name = "leads", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("leads")
  public List<@Valid Lead> getLeads() {
    return leads;
  }

  public void setLeads(List<@Valid Lead> leads) {
    this.leads = leads;
  }

  public ListLeadsResponse totalCount(Integer totalCount) {
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

  public ListLeadsResponse nextPageNumber(Integer nextPageNumber) {
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
    ListLeadsResponse listLeadsResponse = (ListLeadsResponse) o;
    return Objects.equals(this.leads, listLeadsResponse.leads) &&
        Objects.equals(this.totalCount, listLeadsResponse.totalCount) &&
        Objects.equals(this.nextPageNumber, listLeadsResponse.nextPageNumber);
  }

  @Override
  public int hashCode() {
    return Objects.hash(leads, totalCount, nextPageNumber);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ListLeadsResponse {\n");
    sb.append("    leads: ").append(toIndentedString(leads)).append("\n");
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

