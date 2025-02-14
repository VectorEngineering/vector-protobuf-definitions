package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import org.openapitools.model.UpdateAccountRequestPayload;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * UpdateAccountRequest
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T12:22:33.767757-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class UpdateAccountRequest {

  private UpdateAccountRequestPayload payload;

  public UpdateAccountRequest payload(UpdateAccountRequestPayload payload) {
    this.payload = payload;
    return this;
  }

  /**
   * Get payload
   * @return payload
   */
  @Valid 
  @Schema(name = "payload", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("payload")
  public UpdateAccountRequestPayload getPayload() {
    return payload;
  }

  public void setPayload(UpdateAccountRequestPayload payload) {
    this.payload = payload;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    UpdateAccountRequest updateAccountRequest = (UpdateAccountRequest) o;
    return Objects.equals(this.payload, updateAccountRequest.payload);
  }

  @Override
  public int hashCode() {
    return Objects.hash(payload);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class UpdateAccountRequest {\n");
    sb.append("    payload: ").append(toIndentedString(payload)).append("\n");
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

