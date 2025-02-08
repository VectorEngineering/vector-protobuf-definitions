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
 * NegotiationRound
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-08T10:46:38.744443-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class NegotiationRound {

  private String id;

  private String historyId;

  private Integer roundNumber;

  private String proposerId;

  private String proposalContent;

  private String responseContent;

  private String status;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  public NegotiationRound id(String id) {
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

  public NegotiationRound historyId(String historyId) {
    this.historyId = historyId;
    return this;
  }

  /**
   * Get historyId
   * @return historyId
   */
  
  @Schema(name = "historyId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("historyId")
  public String getHistoryId() {
    return historyId;
  }

  public void setHistoryId(String historyId) {
    this.historyId = historyId;
  }

  public NegotiationRound roundNumber(Integer roundNumber) {
    this.roundNumber = roundNumber;
    return this;
  }

  /**
   * Get roundNumber
   * @return roundNumber
   */
  
  @Schema(name = "roundNumber", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("roundNumber")
  public Integer getRoundNumber() {
    return roundNumber;
  }

  public void setRoundNumber(Integer roundNumber) {
    this.roundNumber = roundNumber;
  }

  public NegotiationRound proposerId(String proposerId) {
    this.proposerId = proposerId;
    return this;
  }

  /**
   * Get proposerId
   * @return proposerId
   */
  
  @Schema(name = "proposerId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("proposerId")
  public String getProposerId() {
    return proposerId;
  }

  public void setProposerId(String proposerId) {
    this.proposerId = proposerId;
  }

  public NegotiationRound proposalContent(String proposalContent) {
    this.proposalContent = proposalContent;
    return this;
  }

  /**
   * Get proposalContent
   * @return proposalContent
   */
  
  @Schema(name = "proposalContent", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("proposalContent")
  public String getProposalContent() {
    return proposalContent;
  }

  public void setProposalContent(String proposalContent) {
    this.proposalContent = proposalContent;
  }

  public NegotiationRound responseContent(String responseContent) {
    this.responseContent = responseContent;
    return this;
  }

  /**
   * Get responseContent
   * @return responseContent
   */
  
  @Schema(name = "responseContent", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("responseContent")
  public String getResponseContent() {
    return responseContent;
  }

  public void setResponseContent(String responseContent) {
    this.responseContent = responseContent;
  }

  public NegotiationRound status(String status) {
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

  public NegotiationRound createdAt(OffsetDateTime createdAt) {
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

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    NegotiationRound negotiationRound = (NegotiationRound) o;
    return Objects.equals(this.id, negotiationRound.id) &&
        Objects.equals(this.historyId, negotiationRound.historyId) &&
        Objects.equals(this.roundNumber, negotiationRound.roundNumber) &&
        Objects.equals(this.proposerId, negotiationRound.proposerId) &&
        Objects.equals(this.proposalContent, negotiationRound.proposalContent) &&
        Objects.equals(this.responseContent, negotiationRound.responseContent) &&
        Objects.equals(this.status, negotiationRound.status) &&
        Objects.equals(this.createdAt, negotiationRound.createdAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, historyId, roundNumber, proposerId, proposalContent, responseContent, status, createdAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class NegotiationRound {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    historyId: ").append(toIndentedString(historyId)).append("\n");
    sb.append("    roundNumber: ").append(toIndentedString(roundNumber)).append("\n");
    sb.append("    proposerId: ").append(toIndentedString(proposerId)).append("\n");
    sb.append("    proposalContent: ").append(toIndentedString(proposalContent)).append("\n");
    sb.append("    responseContent: ").append(toIndentedString(responseContent)).append("\n");
    sb.append("    status: ").append(toIndentedString(status)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
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

