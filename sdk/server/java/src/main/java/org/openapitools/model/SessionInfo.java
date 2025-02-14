package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * SessionInfo
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T08:10:47.537779-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class SessionInfo {

  private String sessionId;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime lastActive;

  private String deviceId;

  private String ipAddress;

  public SessionInfo sessionId(String sessionId) {
    this.sessionId = sessionId;
    return this;
  }

  /**
   * Get sessionId
   * @return sessionId
   */
  
  @Schema(name = "sessionId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("sessionId")
  public String getSessionId() {
    return sessionId;
  }

  public void setSessionId(String sessionId) {
    this.sessionId = sessionId;
  }

  public SessionInfo lastActive(OffsetDateTime lastActive) {
    this.lastActive = lastActive;
    return this;
  }

  /**
   * Get lastActive
   * @return lastActive
   */
  @Valid 
  @Schema(name = "lastActive", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("lastActive")
  public OffsetDateTime getLastActive() {
    return lastActive;
  }

  public void setLastActive(OffsetDateTime lastActive) {
    this.lastActive = lastActive;
  }

  public SessionInfo deviceId(String deviceId) {
    this.deviceId = deviceId;
    return this;
  }

  /**
   * Get deviceId
   * @return deviceId
   */
  
  @Schema(name = "deviceId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("deviceId")
  public String getDeviceId() {
    return deviceId;
  }

  public void setDeviceId(String deviceId) {
    this.deviceId = deviceId;
  }

  public SessionInfo ipAddress(String ipAddress) {
    this.ipAddress = ipAddress;
    return this;
  }

  /**
   * Get ipAddress
   * @return ipAddress
   */
  
  @Schema(name = "ipAddress", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("ipAddress")
  public String getIpAddress() {
    return ipAddress;
  }

  public void setIpAddress(String ipAddress) {
    this.ipAddress = ipAddress;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    SessionInfo sessionInfo = (SessionInfo) o;
    return Objects.equals(this.sessionId, sessionInfo.sessionId) &&
        Objects.equals(this.lastActive, sessionInfo.lastActive) &&
        Objects.equals(this.deviceId, sessionInfo.deviceId) &&
        Objects.equals(this.ipAddress, sessionInfo.ipAddress);
  }

  @Override
  public int hashCode() {
    return Objects.hash(sessionId, lastActive, deviceId, ipAddress);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class SessionInfo {\n");
    sb.append("    sessionId: ").append(toIndentedString(sessionId)).append("\n");
    sb.append("    lastActive: ").append(toIndentedString(lastActive)).append("\n");
    sb.append("    deviceId: ").append(toIndentedString(deviceId)).append("\n");
    sb.append("    ipAddress: ").append(toIndentedString(ipAddress)).append("\n");
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

