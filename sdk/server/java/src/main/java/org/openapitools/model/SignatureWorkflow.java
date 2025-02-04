package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * SignatureWorkflow
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-03T20:00:13.934981-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class SignatureWorkflow {

  private String id;

  private String requestId;

  @Valid
  private List<String> signingOrder = new ArrayList<>();

  private String currentSigner;

  private String workflowStatus;

  private Boolean parallelSigning;

  private Boolean requireAllSignatures;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime deadline;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  public SignatureWorkflow id(String id) {
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

  public SignatureWorkflow requestId(String requestId) {
    this.requestId = requestId;
    return this;
  }

  /**
   * Get requestId
   * @return requestId
   */
  
  @Schema(name = "requestId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("requestId")
  public String getRequestId() {
    return requestId;
  }

  public void setRequestId(String requestId) {
    this.requestId = requestId;
  }

  public SignatureWorkflow signingOrder(List<String> signingOrder) {
    this.signingOrder = signingOrder;
    return this;
  }

  public SignatureWorkflow addSigningOrderItem(String signingOrderItem) {
    if (this.signingOrder == null) {
      this.signingOrder = new ArrayList<>();
    }
    this.signingOrder.add(signingOrderItem);
    return this;
  }

  /**
   * Get signingOrder
   * @return signingOrder
   */
  
  @Schema(name = "signingOrder", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("signingOrder")
  public List<String> getSigningOrder() {
    return signingOrder;
  }

  public void setSigningOrder(List<String> signingOrder) {
    this.signingOrder = signingOrder;
  }

  public SignatureWorkflow currentSigner(String currentSigner) {
    this.currentSigner = currentSigner;
    return this;
  }

  /**
   * Get currentSigner
   * @return currentSigner
   */
  
  @Schema(name = "currentSigner", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("currentSigner")
  public String getCurrentSigner() {
    return currentSigner;
  }

  public void setCurrentSigner(String currentSigner) {
    this.currentSigner = currentSigner;
  }

  public SignatureWorkflow workflowStatus(String workflowStatus) {
    this.workflowStatus = workflowStatus;
    return this;
  }

  /**
   * Get workflowStatus
   * @return workflowStatus
   */
  
  @Schema(name = "workflowStatus", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("workflowStatus")
  public String getWorkflowStatus() {
    return workflowStatus;
  }

  public void setWorkflowStatus(String workflowStatus) {
    this.workflowStatus = workflowStatus;
  }

  public SignatureWorkflow parallelSigning(Boolean parallelSigning) {
    this.parallelSigning = parallelSigning;
    return this;
  }

  /**
   * Get parallelSigning
   * @return parallelSigning
   */
  
  @Schema(name = "parallelSigning", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("parallelSigning")
  public Boolean getParallelSigning() {
    return parallelSigning;
  }

  public void setParallelSigning(Boolean parallelSigning) {
    this.parallelSigning = parallelSigning;
  }

  public SignatureWorkflow requireAllSignatures(Boolean requireAllSignatures) {
    this.requireAllSignatures = requireAllSignatures;
    return this;
  }

  /**
   * Get requireAllSignatures
   * @return requireAllSignatures
   */
  
  @Schema(name = "requireAllSignatures", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("requireAllSignatures")
  public Boolean getRequireAllSignatures() {
    return requireAllSignatures;
  }

  public void setRequireAllSignatures(Boolean requireAllSignatures) {
    this.requireAllSignatures = requireAllSignatures;
  }

  public SignatureWorkflow deadline(OffsetDateTime deadline) {
    this.deadline = deadline;
    return this;
  }

  /**
   * Get deadline
   * @return deadline
   */
  @Valid 
  @Schema(name = "deadline", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("deadline")
  public OffsetDateTime getDeadline() {
    return deadline;
  }

  public void setDeadline(OffsetDateTime deadline) {
    this.deadline = deadline;
  }

  public SignatureWorkflow createdAt(OffsetDateTime createdAt) {
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

  public SignatureWorkflow updatedAt(OffsetDateTime updatedAt) {
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
    SignatureWorkflow signatureWorkflow = (SignatureWorkflow) o;
    return Objects.equals(this.id, signatureWorkflow.id) &&
        Objects.equals(this.requestId, signatureWorkflow.requestId) &&
        Objects.equals(this.signingOrder, signatureWorkflow.signingOrder) &&
        Objects.equals(this.currentSigner, signatureWorkflow.currentSigner) &&
        Objects.equals(this.workflowStatus, signatureWorkflow.workflowStatus) &&
        Objects.equals(this.parallelSigning, signatureWorkflow.parallelSigning) &&
        Objects.equals(this.requireAllSignatures, signatureWorkflow.requireAllSignatures) &&
        Objects.equals(this.deadline, signatureWorkflow.deadline) &&
        Objects.equals(this.createdAt, signatureWorkflow.createdAt) &&
        Objects.equals(this.updatedAt, signatureWorkflow.updatedAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, requestId, signingOrder, currentSigner, workflowStatus, parallelSigning, requireAllSignatures, deadline, createdAt, updatedAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class SignatureWorkflow {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    requestId: ").append(toIndentedString(requestId)).append("\n");
    sb.append("    signingOrder: ").append(toIndentedString(signingOrder)).append("\n");
    sb.append("    currentSigner: ").append(toIndentedString(currentSigner)).append("\n");
    sb.append("    workflowStatus: ").append(toIndentedString(workflowStatus)).append("\n");
    sb.append("    parallelSigning: ").append(toIndentedString(parallelSigning)).append("\n");
    sb.append("    requireAllSignatures: ").append(toIndentedString(requireAllSignatures)).append("\n");
    sb.append("    deadline: ").append(toIndentedString(deadline)).append("\n");
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

