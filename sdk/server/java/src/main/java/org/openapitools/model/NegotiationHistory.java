package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.NegotiationRound;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * NegotiationHistory
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-09T17:57:27.369576-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class NegotiationHistory {

  private String id;

  private String documentId;

  @Valid
  private List<@Valid NegotiationRound> rounds = new ArrayList<>();

  private String currentStatus;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime startedAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime lastUpdated;

  public NegotiationHistory id(String id) {
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

  public NegotiationHistory documentId(String documentId) {
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

  public NegotiationHistory rounds(List<@Valid NegotiationRound> rounds) {
    this.rounds = rounds;
    return this;
  }

  public NegotiationHistory addRoundsItem(NegotiationRound roundsItem) {
    if (this.rounds == null) {
      this.rounds = new ArrayList<>();
    }
    this.rounds.add(roundsItem);
    return this;
  }

  /**
   * Get rounds
   * @return rounds
   */
  @Valid 
  @Schema(name = "rounds", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("rounds")
  public List<@Valid NegotiationRound> getRounds() {
    return rounds;
  }

  public void setRounds(List<@Valid NegotiationRound> rounds) {
    this.rounds = rounds;
  }

  public NegotiationHistory currentStatus(String currentStatus) {
    this.currentStatus = currentStatus;
    return this;
  }

  /**
   * Get currentStatus
   * @return currentStatus
   */
  
  @Schema(name = "currentStatus", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("currentStatus")
  public String getCurrentStatus() {
    return currentStatus;
  }

  public void setCurrentStatus(String currentStatus) {
    this.currentStatus = currentStatus;
  }

  public NegotiationHistory startedAt(OffsetDateTime startedAt) {
    this.startedAt = startedAt;
    return this;
  }

  /**
   * Get startedAt
   * @return startedAt
   */
  @Valid 
  @Schema(name = "startedAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("startedAt")
  public OffsetDateTime getStartedAt() {
    return startedAt;
  }

  public void setStartedAt(OffsetDateTime startedAt) {
    this.startedAt = startedAt;
  }

  public NegotiationHistory lastUpdated(OffsetDateTime lastUpdated) {
    this.lastUpdated = lastUpdated;
    return this;
  }

  /**
   * Get lastUpdated
   * @return lastUpdated
   */
  @Valid 
  @Schema(name = "lastUpdated", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("lastUpdated")
  public OffsetDateTime getLastUpdated() {
    return lastUpdated;
  }

  public void setLastUpdated(OffsetDateTime lastUpdated) {
    this.lastUpdated = lastUpdated;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    NegotiationHistory negotiationHistory = (NegotiationHistory) o;
    return Objects.equals(this.id, negotiationHistory.id) &&
        Objects.equals(this.documentId, negotiationHistory.documentId) &&
        Objects.equals(this.rounds, negotiationHistory.rounds) &&
        Objects.equals(this.currentStatus, negotiationHistory.currentStatus) &&
        Objects.equals(this.startedAt, negotiationHistory.startedAt) &&
        Objects.equals(this.lastUpdated, negotiationHistory.lastUpdated);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, documentId, rounds, currentStatus, startedAt, lastUpdated);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class NegotiationHistory {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    documentId: ").append(toIndentedString(documentId)).append("\n");
    sb.append("    rounds: ").append(toIndentedString(rounds)).append("\n");
    sb.append("    currentStatus: ").append(toIndentedString(currentStatus)).append("\n");
    sb.append("    startedAt: ").append(toIndentedString(startedAt)).append("\n");
    sb.append("    lastUpdated: ").append(toIndentedString(lastUpdated)).append("\n");
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

