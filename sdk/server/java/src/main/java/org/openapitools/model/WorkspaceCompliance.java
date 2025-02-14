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
 * WorkspaceCompliance
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T08:12:58.450907-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class WorkspaceCompliance {

  private String id;

  private String complianceType;

  private String status;

  private String certificationId;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime validUntil;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  public WorkspaceCompliance id(String id) {
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

  public WorkspaceCompliance complianceType(String complianceType) {
    this.complianceType = complianceType;
    return this;
  }

  /**
   * Get complianceType
   * @return complianceType
   */
  
  @Schema(name = "complianceType", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("complianceType")
  public String getComplianceType() {
    return complianceType;
  }

  public void setComplianceType(String complianceType) {
    this.complianceType = complianceType;
  }

  public WorkspaceCompliance status(String status) {
    this.status = status;
    return this;
  }

  /**
   * Get status
   * @return status
   */
  
  @Schema(name = "status", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("status")
  public String getStatus() {
    return status;
  }

  public void setStatus(String status) {
    this.status = status;
  }

  public WorkspaceCompliance certificationId(String certificationId) {
    this.certificationId = certificationId;
    return this;
  }

  /**
   * Get certificationId
   * @return certificationId
   */
  
  @Schema(name = "certificationId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("certificationId")
  public String getCertificationId() {
    return certificationId;
  }

  public void setCertificationId(String certificationId) {
    this.certificationId = certificationId;
  }

  public WorkspaceCompliance validUntil(OffsetDateTime validUntil) {
    this.validUntil = validUntil;
    return this;
  }

  /**
   * Get validUntil
   * @return validUntil
   */
  @Valid 
  @Schema(name = "validUntil", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("validUntil")
  public OffsetDateTime getValidUntil() {
    return validUntil;
  }

  public void setValidUntil(OffsetDateTime validUntil) {
    this.validUntil = validUntil;
  }

  public WorkspaceCompliance createdAt(OffsetDateTime createdAt) {
    this.createdAt = createdAt;
    return this;
  }

  /**
   * Get createdAt
   * @return createdAt
   */
  @Valid 
  @Schema(name = "createdAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("createdAt")
  public OffsetDateTime getCreatedAt() {
    return createdAt;
  }

  public void setCreatedAt(OffsetDateTime createdAt) {
    this.createdAt = createdAt;
  }

  public WorkspaceCompliance updatedAt(OffsetDateTime updatedAt) {
    this.updatedAt = updatedAt;
    return this;
  }

  /**
   * Get updatedAt
   * @return updatedAt
   */
  @Valid 
  @Schema(name = "updatedAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("updatedAt")
  public OffsetDateTime getUpdatedAt() {
    return updatedAt;
  }

  public void setUpdatedAt(OffsetDateTime updatedAt) {
    this.updatedAt = updatedAt;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    WorkspaceCompliance workspaceCompliance = (WorkspaceCompliance) o;
    return Objects.equals(this.id, workspaceCompliance.id) &&
        Objects.equals(this.complianceType, workspaceCompliance.complianceType) &&
        Objects.equals(this.status, workspaceCompliance.status) &&
        Objects.equals(this.certificationId, workspaceCompliance.certificationId) &&
        Objects.equals(this.validUntil, workspaceCompliance.validUntil) &&
        Objects.equals(this.createdAt, workspaceCompliance.createdAt) &&
        Objects.equals(this.updatedAt, workspaceCompliance.updatedAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, complianceType, status, certificationId, validUntil, createdAt, updatedAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class WorkspaceCompliance {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    complianceType: ").append(toIndentedString(complianceType)).append("\n");
    sb.append("    status: ").append(toIndentedString(status)).append("\n");
    sb.append("    certificationId: ").append(toIndentedString(certificationId)).append("\n");
    sb.append("    validUntil: ").append(toIndentedString(validUntil)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
    sb.append("    updatedAt: ").append(toIndentedString(updatedAt)).append("\n");
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

