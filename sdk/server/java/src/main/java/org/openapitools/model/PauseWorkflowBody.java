package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * PauseWorkflowBody
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T08:12:58.450907-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class PauseWorkflowBody {

  private Boolean pause;

  public PauseWorkflowBody pause(Boolean pause) {
    this.pause = pause;
    return this;
  }

  /**
   * Get pause
   * @return pause
   */
  
  @Schema(name = "pause", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("pause")
  public Boolean getPause() {
    return pause;
  }

  public void setPause(Boolean pause) {
    this.pause = pause;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    PauseWorkflowBody pauseWorkflowBody = (PauseWorkflowBody) o;
    return Objects.equals(this.pause, pauseWorkflowBody.pause);
  }

  @Override
  public int hashCode() {
    return Objects.hash(pause);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class PauseWorkflowBody {\n");
    sb.append("    pause: ").append(toIndentedString(pause)).append("\n");
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

