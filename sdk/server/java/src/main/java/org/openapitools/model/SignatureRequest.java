package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import java.time.OffsetDateTime;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.SignatureBlock;
import org.openapitools.model.SignatureStatus;
import org.openapitools.model.SignatureWorkflow;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * SignatureRequest
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-05T07:11:09.755832-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class SignatureRequest {

  private String id;

  private SignatureStatus status = SignatureStatus.UNSPECIFIED;

  private String signerEmail;

  private String signerName;

  private String role;

  private String authenticationMethod;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime expiresAt;

  private Boolean requiresMfa;

  private String signatureType;

  @Valid
  private List<String> reminderSchedule = new ArrayList<>();

  private String documentId;

  @Valid
  private List<@Valid SignatureBlock> signatureBlocks = new ArrayList<>();

  private SignatureWorkflow workflow;

  public SignatureRequest id(String id) {
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

  public SignatureRequest status(SignatureStatus status) {
    this.status = status;
    return this;
  }

  /**
   * Get status
   * @return status
   */
  @Valid 
  @Schema(name = "status", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("status")
  public SignatureStatus getStatus() {
    return status;
  }

  public void setStatus(SignatureStatus status) {
    this.status = status;
  }

  public SignatureRequest signerEmail(String signerEmail) {
    this.signerEmail = signerEmail;
    return this;
  }

  /**
   * Get signerEmail
   * @return signerEmail
   */
  
  @Schema(name = "signerEmail", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("signerEmail")
  public String getSignerEmail() {
    return signerEmail;
  }

  public void setSignerEmail(String signerEmail) {
    this.signerEmail = signerEmail;
  }

  public SignatureRequest signerName(String signerName) {
    this.signerName = signerName;
    return this;
  }

  /**
   * Get signerName
   * @return signerName
   */
  
  @Schema(name = "signerName", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("signerName")
  public String getSignerName() {
    return signerName;
  }

  public void setSignerName(String signerName) {
    this.signerName = signerName;
  }

  public SignatureRequest role(String role) {
    this.role = role;
    return this;
  }

  /**
   * Get role
   * @return role
   */
  
  @Schema(name = "role", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("role")
  public String getRole() {
    return role;
  }

  public void setRole(String role) {
    this.role = role;
  }

  public SignatureRequest authenticationMethod(String authenticationMethod) {
    this.authenticationMethod = authenticationMethod;
    return this;
  }

  /**
   * Get authenticationMethod
   * @return authenticationMethod
   */
  
  @Schema(name = "authenticationMethod", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("authenticationMethod")
  public String getAuthenticationMethod() {
    return authenticationMethod;
  }

  public void setAuthenticationMethod(String authenticationMethod) {
    this.authenticationMethod = authenticationMethod;
  }

  public SignatureRequest expiresAt(OffsetDateTime expiresAt) {
    this.expiresAt = expiresAt;
    return this;
  }

  /**
   * Get expiresAt
   * @return expiresAt
   */
  @Valid 
  @Schema(name = "expiresAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("expiresAt")
  public OffsetDateTime getExpiresAt() {
    return expiresAt;
  }

  public void setExpiresAt(OffsetDateTime expiresAt) {
    this.expiresAt = expiresAt;
  }

  public SignatureRequest requiresMfa(Boolean requiresMfa) {
    this.requiresMfa = requiresMfa;
    return this;
  }

  /**
   * Get requiresMfa
   * @return requiresMfa
   */
  
  @Schema(name = "requiresMfa", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("requiresMfa")
  public Boolean getRequiresMfa() {
    return requiresMfa;
  }

  public void setRequiresMfa(Boolean requiresMfa) {
    this.requiresMfa = requiresMfa;
  }

  public SignatureRequest signatureType(String signatureType) {
    this.signatureType = signatureType;
    return this;
  }

  /**
   * Get signatureType
   * @return signatureType
   */
  
  @Schema(name = "signatureType", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("signatureType")
  public String getSignatureType() {
    return signatureType;
  }

  public void setSignatureType(String signatureType) {
    this.signatureType = signatureType;
  }

  public SignatureRequest reminderSchedule(List<String> reminderSchedule) {
    this.reminderSchedule = reminderSchedule;
    return this;
  }

  public SignatureRequest addReminderScheduleItem(String reminderScheduleItem) {
    if (this.reminderSchedule == null) {
      this.reminderSchedule = new ArrayList<>();
    }
    this.reminderSchedule.add(reminderScheduleItem);
    return this;
  }

  /**
   * Get reminderSchedule
   * @return reminderSchedule
   */
  
  @Schema(name = "reminderSchedule", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("reminderSchedule")
  public List<String> getReminderSchedule() {
    return reminderSchedule;
  }

  public void setReminderSchedule(List<String> reminderSchedule) {
    this.reminderSchedule = reminderSchedule;
  }

  public SignatureRequest documentId(String documentId) {
    this.documentId = documentId;
    return this;
  }

  /**
   * Get documentId
   * @return documentId
   */
  
  @Schema(name = "documentId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("documentId")
  public String getDocumentId() {
    return documentId;
  }

  public void setDocumentId(String documentId) {
    this.documentId = documentId;
  }

  public SignatureRequest signatureBlocks(List<@Valid SignatureBlock> signatureBlocks) {
    this.signatureBlocks = signatureBlocks;
    return this;
  }

  public SignatureRequest addSignatureBlocksItem(SignatureBlock signatureBlocksItem) {
    if (this.signatureBlocks == null) {
      this.signatureBlocks = new ArrayList<>();
    }
    this.signatureBlocks.add(signatureBlocksItem);
    return this;
  }

  /**
   * Get signatureBlocks
   * @return signatureBlocks
   */
  @Valid 
  @Schema(name = "signatureBlocks", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("signatureBlocks")
  public List<@Valid SignatureBlock> getSignatureBlocks() {
    return signatureBlocks;
  }

  public void setSignatureBlocks(List<@Valid SignatureBlock> signatureBlocks) {
    this.signatureBlocks = signatureBlocks;
  }

  public SignatureRequest workflow(SignatureWorkflow workflow) {
    this.workflow = workflow;
    return this;
  }

  /**
   * Get workflow
   * @return workflow
   */
  @Valid 
  @Schema(name = "workflow", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("workflow")
  public SignatureWorkflow getWorkflow() {
    return workflow;
  }

  public void setWorkflow(SignatureWorkflow workflow) {
    this.workflow = workflow;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    SignatureRequest signatureRequest = (SignatureRequest) o;
    return Objects.equals(this.id, signatureRequest.id) &&
        Objects.equals(this.status, signatureRequest.status) &&
        Objects.equals(this.signerEmail, signatureRequest.signerEmail) &&
        Objects.equals(this.signerName, signatureRequest.signerName) &&
        Objects.equals(this.role, signatureRequest.role) &&
        Objects.equals(this.authenticationMethod, signatureRequest.authenticationMethod) &&
        Objects.equals(this.expiresAt, signatureRequest.expiresAt) &&
        Objects.equals(this.requiresMfa, signatureRequest.requiresMfa) &&
        Objects.equals(this.signatureType, signatureRequest.signatureType) &&
        Objects.equals(this.reminderSchedule, signatureRequest.reminderSchedule) &&
        Objects.equals(this.documentId, signatureRequest.documentId) &&
        Objects.equals(this.signatureBlocks, signatureRequest.signatureBlocks) &&
        Objects.equals(this.workflow, signatureRequest.workflow);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, status, signerEmail, signerName, role, authenticationMethod, expiresAt, requiresMfa, signatureType, reminderSchedule, documentId, signatureBlocks, workflow);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class SignatureRequest {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    status: ").append(toIndentedString(status)).append("\n");
    sb.append("    signerEmail: ").append(toIndentedString(signerEmail)).append("\n");
    sb.append("    signerName: ").append(toIndentedString(signerName)).append("\n");
    sb.append("    role: ").append(toIndentedString(role)).append("\n");
    sb.append("    authenticationMethod: ").append(toIndentedString(authenticationMethod)).append("\n");
    sb.append("    expiresAt: ").append(toIndentedString(expiresAt)).append("\n");
    sb.append("    requiresMfa: ").append(toIndentedString(requiresMfa)).append("\n");
    sb.append("    signatureType: ").append(toIndentedString(signatureType)).append("\n");
    sb.append("    reminderSchedule: ").append(toIndentedString(reminderSchedule)).append("\n");
    sb.append("    documentId: ").append(toIndentedString(documentId)).append("\n");
    sb.append("    signatureBlocks: ").append(toIndentedString(signatureBlocks)).append("\n");
    sb.append("    workflow: ").append(toIndentedString(workflow)).append("\n");
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

