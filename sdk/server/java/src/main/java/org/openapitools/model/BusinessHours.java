package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import org.openapitools.model.DayOfWeek;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * BusinessHours
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-01-29T12:50:06.029660-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class BusinessHours {

  private String id;

  private DayOfWeek day = DayOfWeek.UNSPECIFIED;

  private String openTime;

  private String closeTime;

  private Boolean closed;

  private String leadId;

  public BusinessHours id(String id) {
    this.id = id;
    return this;
  }

  /**
   * Get id
   * @return id
   */
  
  @Schema(name = "id", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("id")
  public String getId() {
    return id;
  }

  public void setId(String id) {
    this.id = id;
  }

  public BusinessHours day(DayOfWeek day) {
    this.day = day;
    return this;
  }

  /**
   * Get day
   * @return day
   */
  @Valid 
  @Schema(name = "day", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("day")
  public DayOfWeek getDay() {
    return day;
  }

  public void setDay(DayOfWeek day) {
    this.day = day;
  }

  public BusinessHours openTime(String openTime) {
    this.openTime = openTime;
    return this;
  }

  /**
   * Get openTime
   * @return openTime
   */
  
  @Schema(name = "openTime", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("openTime")
  public String getOpenTime() {
    return openTime;
  }

  public void setOpenTime(String openTime) {
    this.openTime = openTime;
  }

  public BusinessHours closeTime(String closeTime) {
    this.closeTime = closeTime;
    return this;
  }

  /**
   * Get closeTime
   * @return closeTime
   */
  
  @Schema(name = "closeTime", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("closeTime")
  public String getCloseTime() {
    return closeTime;
  }

  public void setCloseTime(String closeTime) {
    this.closeTime = closeTime;
  }

  public BusinessHours closed(Boolean closed) {
    this.closed = closed;
    return this;
  }

  /**
   * Get closed
   * @return closed
   */
  
  @Schema(name = "closed", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("closed")
  public Boolean getClosed() {
    return closed;
  }

  public void setClosed(Boolean closed) {
    this.closed = closed;
  }

  public BusinessHours leadId(String leadId) {
    this.leadId = leadId;
    return this;
  }

  /**
   * Get leadId
   * @return leadId
   */
  
  @Schema(name = "leadId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("leadId")
  public String getLeadId() {
    return leadId;
  }

  public void setLeadId(String leadId) {
    this.leadId = leadId;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    BusinessHours businessHours = (BusinessHours) o;
    return Objects.equals(this.id, businessHours.id) &&
        Objects.equals(this.day, businessHours.day) &&
        Objects.equals(this.openTime, businessHours.openTime) &&
        Objects.equals(this.closeTime, businessHours.closeTime) &&
        Objects.equals(this.closed, businessHours.closed) &&
        Objects.equals(this.leadId, businessHours.leadId);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, day, openTime, closeTime, closed, leadId);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class BusinessHours {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    day: ").append(toIndentedString(day)).append("\n");
    sb.append("    openTime: ").append(toIndentedString(openTime)).append("\n");
    sb.append("    closeTime: ").append(toIndentedString(closeTime)).append("\n");
    sb.append("    closed: ").append(toIndentedString(closed)).append("\n");
    sb.append("    leadId: ").append(toIndentedString(leadId)).append("\n");
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

