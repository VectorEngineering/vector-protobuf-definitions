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
 * AvailabilityInfo
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T14:39:19.806517-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class AvailabilityInfo {

  private Integer retryAfter;

  private String maintenanceWindow;

  @Valid
  private List<String> affectedServices = new ArrayList<>();

  private String statusPageUrl;

  public AvailabilityInfo retryAfter(Integer retryAfter) {
    this.retryAfter = retryAfter;
    return this;
  }

  /**
   * Get retryAfter
   * @return retryAfter
   */
  
  @Schema(name = "retryAfter", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("retryAfter")
  public Integer getRetryAfter() {
    return retryAfter;
  }

  public void setRetryAfter(Integer retryAfter) {
    this.retryAfter = retryAfter;
  }

  public AvailabilityInfo maintenanceWindow(String maintenanceWindow) {
    this.maintenanceWindow = maintenanceWindow;
    return this;
  }

  /**
   * Get maintenanceWindow
   * @return maintenanceWindow
   */
  
  @Schema(name = "maintenanceWindow", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("maintenanceWindow")
  public String getMaintenanceWindow() {
    return maintenanceWindow;
  }

  public void setMaintenanceWindow(String maintenanceWindow) {
    this.maintenanceWindow = maintenanceWindow;
  }

  public AvailabilityInfo affectedServices(List<String> affectedServices) {
    this.affectedServices = affectedServices;
    return this;
  }

  public AvailabilityInfo addAffectedServicesItem(String affectedServicesItem) {
    if (this.affectedServices == null) {
      this.affectedServices = new ArrayList<>();
    }
    this.affectedServices.add(affectedServicesItem);
    return this;
  }

  /**
   * Get affectedServices
   * @return affectedServices
   */
  
  @Schema(name = "affectedServices", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("affectedServices")
  public List<String> getAffectedServices() {
    return affectedServices;
  }

  public void setAffectedServices(List<String> affectedServices) {
    this.affectedServices = affectedServices;
  }

  public AvailabilityInfo statusPageUrl(String statusPageUrl) {
    this.statusPageUrl = statusPageUrl;
    return this;
  }

  /**
   * Get statusPageUrl
   * @return statusPageUrl
   */
  
  @Schema(name = "statusPageUrl", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("statusPageUrl")
  public String getStatusPageUrl() {
    return statusPageUrl;
  }

  public void setStatusPageUrl(String statusPageUrl) {
    this.statusPageUrl = statusPageUrl;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    AvailabilityInfo availabilityInfo = (AvailabilityInfo) o;
    return Objects.equals(this.retryAfter, availabilityInfo.retryAfter) &&
        Objects.equals(this.maintenanceWindow, availabilityInfo.maintenanceWindow) &&
        Objects.equals(this.affectedServices, availabilityInfo.affectedServices) &&
        Objects.equals(this.statusPageUrl, availabilityInfo.statusPageUrl);
  }

  @Override
  public int hashCode() {
    return Objects.hash(retryAfter, maintenanceWindow, affectedServices, statusPageUrl);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class AvailabilityInfo {\n");
    sb.append("    retryAfter: ").append(toIndentedString(retryAfter)).append("\n");
    sb.append("    maintenanceWindow: ").append(toIndentedString(maintenanceWindow)).append("\n");
    sb.append("    affectedServices: ").append(toIndentedString(affectedServices)).append("\n");
    sb.append("    statusPageUrl: ").append(toIndentedString(statusPageUrl)).append("\n");
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

