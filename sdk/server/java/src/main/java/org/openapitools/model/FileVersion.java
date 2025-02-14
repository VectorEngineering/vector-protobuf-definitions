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
 * FileVersion
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T07:41:41.360877-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class FileVersion {

  private String id;

  private String fileId;

  private Integer versionNumber;

  private String s3Key;

  private String size;

  private String commitMessage;

  private String authorId;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  public FileVersion id(String id) {
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

  public FileVersion fileId(String fileId) {
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

  public FileVersion versionNumber(Integer versionNumber) {
    this.versionNumber = versionNumber;
    return this;
  }

  /**
   * Get versionNumber
   * @return versionNumber
   */
  
  @Schema(name = "versionNumber", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("versionNumber")
  public Integer getVersionNumber() {
    return versionNumber;
  }

  public void setVersionNumber(Integer versionNumber) {
    this.versionNumber = versionNumber;
  }

  public FileVersion s3Key(String s3Key) {
    this.s3Key = s3Key;
    return this;
  }

  /**
   * Get s3Key
   * @return s3Key
   */
  
  @Schema(name = "s3Key", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("s3Key")
  public String getS3Key() {
    return s3Key;
  }

  public void setS3Key(String s3Key) {
    this.s3Key = s3Key;
  }

  public FileVersion size(String size) {
    this.size = size;
    return this;
  }

  /**
   * Get size
   * @return size
   */
  
  @Schema(name = "size", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("size")
  public String getSize() {
    return size;
  }

  public void setSize(String size) {
    this.size = size;
  }

  public FileVersion commitMessage(String commitMessage) {
    this.commitMessage = commitMessage;
    return this;
  }

  /**
   * Get commitMessage
   * @return commitMessage
   */
  
  @Schema(name = "commitMessage", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("commitMessage")
  public String getCommitMessage() {
    return commitMessage;
  }

  public void setCommitMessage(String commitMessage) {
    this.commitMessage = commitMessage;
  }

  public FileVersion authorId(String authorId) {
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

  public FileVersion createdAt(OffsetDateTime createdAt) {
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
    FileVersion fileVersion = (FileVersion) o;
    return Objects.equals(this.id, fileVersion.id) &&
        Objects.equals(this.fileId, fileVersion.fileId) &&
        Objects.equals(this.versionNumber, fileVersion.versionNumber) &&
        Objects.equals(this.s3Key, fileVersion.s3Key) &&
        Objects.equals(this.size, fileVersion.size) &&
        Objects.equals(this.commitMessage, fileVersion.commitMessage) &&
        Objects.equals(this.authorId, fileVersion.authorId) &&
        Objects.equals(this.createdAt, fileVersion.createdAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, fileId, versionNumber, s3Key, size, commitMessage, authorId, createdAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class FileVersion {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    fileId: ").append(toIndentedString(fileId)).append("\n");
    sb.append("    versionNumber: ").append(toIndentedString(versionNumber)).append("\n");
    sb.append("    s3Key: ").append(toIndentedString(s3Key)).append("\n");
    sb.append("    size: ").append(toIndentedString(size)).append("\n");
    sb.append("    commitMessage: ").append(toIndentedString(commitMessage)).append("\n");
    sb.append("    authorId: ").append(toIndentedString(authorId)).append("\n");
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

