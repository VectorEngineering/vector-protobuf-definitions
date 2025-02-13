package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * MFAInfo
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-13T17:57:23.762503-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class MFAInfo {

  private Boolean mfaRequired;

  @Valid
  private List<String> allowedMethods = new ArrayList<>();

  private String failedReason;

  public MFAInfo mfaRequired(Boolean mfaRequired) {
    this.mfaRequired = mfaRequired;
    return this;
  }

  /**
   * Get mfaRequired
   * @return mfaRequired
   */
  
  @Schema(name = "mfaRequired", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("mfaRequired")
  public Boolean getMfaRequired() {
    return mfaRequired;
  }

  public void setMfaRequired(Boolean mfaRequired) {
    this.mfaRequired = mfaRequired;
  }

  public MFAInfo allowedMethods(List<String> allowedMethods) {
    this.allowedMethods = allowedMethods;
    return this;
  }

  public MFAInfo addAllowedMethodsItem(String allowedMethodsItem) {
    if (this.allowedMethods == null) {
      this.allowedMethods = new ArrayList<>();
    }
    this.allowedMethods.add(allowedMethodsItem);
    return this;
  }

  /**
   * Get allowedMethods
   * @return allowedMethods
   */
  
  @Schema(name = "allowedMethods", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("allowedMethods")
  public List<String> getAllowedMethods() {
    return allowedMethods;
  }

  public void setAllowedMethods(List<String> allowedMethods) {
    this.allowedMethods = allowedMethods;
  }

  public MFAInfo failedReason(String failedReason) {
    this.failedReason = failedReason;
    return this;
  }

  /**
   * Get failedReason
   * @return failedReason
   */
  
  @Schema(name = "failedReason", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("failedReason")
  public String getFailedReason() {
    return failedReason;
  }

  public void setFailedReason(String failedReason) {
    this.failedReason = failedReason;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    MFAInfo mfAInfo = (MFAInfo) o;
    return Objects.equals(this.mfaRequired, mfAInfo.mfaRequired) &&
        Objects.equals(this.allowedMethods, mfAInfo.allowedMethods) &&
        Objects.equals(this.failedReason, mfAInfo.failedReason);
  }

  @Override
  public int hashCode() {
    return Objects.hash(mfaRequired, allowedMethods, failedReason);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class MFAInfo {\n");
    sb.append("    mfaRequired: ").append(toIndentedString(mfaRequired)).append("\n");
    sb.append("    allowedMethods: ").append(toIndentedString(allowedMethods)).append("\n");
    sb.append("    failedReason: ").append(toIndentedString(failedReason)).append("\n");
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

