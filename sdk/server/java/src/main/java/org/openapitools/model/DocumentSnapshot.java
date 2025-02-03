package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import java.util.Arrays;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * DocumentSnapshot
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-03T10:37:17.791439-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class DocumentSnapshot {

  private String id;

  private String fileId;

  private String snapshotHash;

  private byte[] content;

  private String authorId;

  private String reason;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  public DocumentSnapshot id(String id) {
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

  public DocumentSnapshot fileId(String fileId) {
    this.fileId = fileId;
    return this;
  }

  /**
   * Get fileId
   * @return fileId
   */
  
  @Schema(name = "fileId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("fileId")
  public String getFileId() {
    return fileId;
  }

  public void setFileId(String fileId) {
    this.fileId = fileId;
  }

  public DocumentSnapshot snapshotHash(String snapshotHash) {
    this.snapshotHash = snapshotHash;
    return this;
  }

  /**
   * Get snapshotHash
   * @return snapshotHash
   */
  
  @Schema(name = "snapshotHash", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("snapshotHash")
  public String getSnapshotHash() {
    return snapshotHash;
  }

  public void setSnapshotHash(String snapshotHash) {
    this.snapshotHash = snapshotHash;
  }

  public DocumentSnapshot content(byte[] content) {
    this.content = content;
    return this;
  }

  /**
   * Get content
   * @return content
   */
  
  @Schema(name = "content", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("content")
  public byte[] getContent() {
    return content;
  }

  public void setContent(byte[] content) {
    this.content = content;
  }

  public DocumentSnapshot authorId(String authorId) {
    this.authorId = authorId;
    return this;
  }

  /**
   * Get authorId
   * @return authorId
   */
  
  @Schema(name = "authorId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("authorId")
  public String getAuthorId() {
    return authorId;
  }

  public void setAuthorId(String authorId) {
    this.authorId = authorId;
  }

  public DocumentSnapshot reason(String reason) {
    this.reason = reason;
    return this;
  }

  /**
   * Get reason
   * @return reason
   */
  
  @Schema(name = "reason", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("reason")
  public String getReason() {
    return reason;
  }

  public void setReason(String reason) {
    this.reason = reason;
  }

  public DocumentSnapshot createdAt(OffsetDateTime createdAt) {
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
    DocumentSnapshot documentSnapshot = (DocumentSnapshot) o;
    return Objects.equals(this.id, documentSnapshot.id) &&
        Objects.equals(this.fileId, documentSnapshot.fileId) &&
        Objects.equals(this.snapshotHash, documentSnapshot.snapshotHash) &&
        Arrays.equals(this.content, documentSnapshot.content) &&
        Objects.equals(this.authorId, documentSnapshot.authorId) &&
        Objects.equals(this.reason, documentSnapshot.reason) &&
        Objects.equals(this.createdAt, documentSnapshot.createdAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, fileId, snapshotHash, Arrays.hashCode(content), authorId, reason, createdAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class DocumentSnapshot {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    fileId: ").append(toIndentedString(fileId)).append("\n");
    sb.append("    snapshotHash: ").append(toIndentedString(snapshotHash)).append("\n");
    sb.append("    content: ").append(toIndentedString(content)).append("\n");
    sb.append("    authorId: ").append(toIndentedString(authorId)).append("\n");
    sb.append("    reason: ").append(toIndentedString(reason)).append("\n");
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

