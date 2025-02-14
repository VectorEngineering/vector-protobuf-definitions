package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import org.openapitools.model.Lead;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * GetLeadResponse
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T12:30:40.954679-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class GetLeadResponse {

  private Lead lead;

  public GetLeadResponse lead(Lead lead) {
    this.lead = lead;
    return this;
  }

  /**
   * Get lead
   * @return lead
   */
  @Valid 
  @Schema(name = "lead", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("lead")
  public Lead getLead() {
    return lead;
  }

  public void setLead(Lead lead) {
    this.lead = lead;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    GetLeadResponse getLeadResponse = (GetLeadResponse) o;
    return Objects.equals(this.lead, getLeadResponse.lead);
  }

  @Override
  public int hashCode() {
    return Objects.hash(lead);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class GetLeadResponse {\n");
    sb.append("    lead: ").append(toIndentedString(lead)).append("\n");
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

