package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.CommentThread;
import org.openapitools.model.DocumentSnapshot;
import org.openapitools.model.FileEmbeddings;
import org.openapitools.model.FileSharing;
import org.openapitools.model.FileVersion;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * FileMetadata
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T12:21:38.405762-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class FileMetadata {

  private String id;

  private String name;

  private String size;

  private String s3Key;

  private String s3BucketName;

  private Boolean isDeleted;

  private Integer version;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime deletedAt;

  private FileEmbeddings embeddings;

  @Valid
  private List<@Valid FileVersion> versions = new ArrayList<>();

  @Valid
  private List<@Valid CommentThread> comments = new ArrayList<>();

  @Valid
  private List<@Valid FileSharing> sharing = new ArrayList<>();

  @Valid
  private List<@Valid DocumentSnapshot> snapshots = new ArrayList<>();

  public FileMetadata id(String id) {
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

  public FileMetadata name(String name) {
    this.name = name;
    return this;
  }

  /**
   * Get name
   * @return name
   */
  
  @Schema(name = "name", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("name")
  public String getName() {
    return name;
  }

  public void setName(String name) {
    this.name = name;
  }

  public FileMetadata size(String size) {
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

  public FileMetadata s3Key(String s3Key) {
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

  public FileMetadata s3BucketName(String s3BucketName) {
    this.s3BucketName = s3BucketName;
    return this;
  }

  /**
   * Get s3BucketName
   * @return s3BucketName
   */
  
  @Schema(name = "s3BucketName", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("s3BucketName")
  public String getS3BucketName() {
    return s3BucketName;
  }

  public void setS3BucketName(String s3BucketName) {
    this.s3BucketName = s3BucketName;
  }

  public FileMetadata isDeleted(Boolean isDeleted) {
    this.isDeleted = isDeleted;
    return this;
  }

  /**
   * Get isDeleted
   * @return isDeleted
   */
  
  @Schema(name = "isDeleted", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("isDeleted")
  public Boolean getIsDeleted() {
    return isDeleted;
  }

  public void setIsDeleted(Boolean isDeleted) {
    this.isDeleted = isDeleted;
  }

  public FileMetadata version(Integer version) {
    this.version = version;
    return this;
  }

  /**
   * Get version
   * @return version
   */
  
  @Schema(name = "version", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("version")
  public Integer getVersion() {
    return version;
  }

  public void setVersion(Integer version) {
    this.version = version;
  }

  public FileMetadata createdAt(OffsetDateTime createdAt) {
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

  public FileMetadata updatedAt(OffsetDateTime updatedAt) {
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

  public FileMetadata deletedAt(OffsetDateTime deletedAt) {
    this.deletedAt = deletedAt;
    return this;
  }

  /**
   * Get deletedAt
   * @return deletedAt
   */
  @Valid 
  @Schema(name = "deletedAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("deletedAt")
  public OffsetDateTime getDeletedAt() {
    return deletedAt;
  }

  public void setDeletedAt(OffsetDateTime deletedAt) {
    this.deletedAt = deletedAt;
  }

  public FileMetadata embeddings(FileEmbeddings embeddings) {
    this.embeddings = embeddings;
    return this;
  }

  /**
   * Get embeddings
   * @return embeddings
   */
  @Valid 
  @Schema(name = "embeddings", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("embeddings")
  public FileEmbeddings getEmbeddings() {
    return embeddings;
  }

  public void setEmbeddings(FileEmbeddings embeddings) {
    this.embeddings = embeddings;
  }

  public FileMetadata versions(List<@Valid FileVersion> versions) {
    this.versions = versions;
    return this;
  }

  public FileMetadata addVersionsItem(FileVersion versionsItem) {
    if (this.versions == null) {
      this.versions = new ArrayList<>();
    }
    this.versions.add(versionsItem);
    return this;
  }

  /**
   * Get versions
   * @return versions
   */
  @Valid 
  @Schema(name = "versions", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("versions")
  public List<@Valid FileVersion> getVersions() {
    return versions;
  }

  public void setVersions(List<@Valid FileVersion> versions) {
    this.versions = versions;
  }

  public FileMetadata comments(List<@Valid CommentThread> comments) {
    this.comments = comments;
    return this;
  }

  public FileMetadata addCommentsItem(CommentThread commentsItem) {
    if (this.comments == null) {
      this.comments = new ArrayList<>();
    }
    this.comments.add(commentsItem);
    return this;
  }

  /**
   * Get comments
   * @return comments
   */
  @Valid 
  @Schema(name = "comments", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("comments")
  public List<@Valid CommentThread> getComments() {
    return comments;
  }

  public void setComments(List<@Valid CommentThread> comments) {
    this.comments = comments;
  }

  public FileMetadata sharing(List<@Valid FileSharing> sharing) {
    this.sharing = sharing;
    return this;
  }

  public FileMetadata addSharingItem(FileSharing sharingItem) {
    if (this.sharing == null) {
      this.sharing = new ArrayList<>();
    }
    this.sharing.add(sharingItem);
    return this;
  }

  /**
   * Get sharing
   * @return sharing
   */
  @Valid 
  @Schema(name = "sharing", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("sharing")
  public List<@Valid FileSharing> getSharing() {
    return sharing;
  }

  public void setSharing(List<@Valid FileSharing> sharing) {
    this.sharing = sharing;
  }

  public FileMetadata snapshots(List<@Valid DocumentSnapshot> snapshots) {
    this.snapshots = snapshots;
    return this;
  }

  public FileMetadata addSnapshotsItem(DocumentSnapshot snapshotsItem) {
    if (this.snapshots == null) {
      this.snapshots = new ArrayList<>();
    }
    this.snapshots.add(snapshotsItem);
    return this;
  }

  /**
   * Get snapshots
   * @return snapshots
   */
  @Valid 
  @Schema(name = "snapshots", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("snapshots")
  public List<@Valid DocumentSnapshot> getSnapshots() {
    return snapshots;
  }

  public void setSnapshots(List<@Valid DocumentSnapshot> snapshots) {
    this.snapshots = snapshots;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    FileMetadata fileMetadata = (FileMetadata) o;
    return Objects.equals(this.id, fileMetadata.id) &&
        Objects.equals(this.name, fileMetadata.name) &&
        Objects.equals(this.size, fileMetadata.size) &&
        Objects.equals(this.s3Key, fileMetadata.s3Key) &&
        Objects.equals(this.s3BucketName, fileMetadata.s3BucketName) &&
        Objects.equals(this.isDeleted, fileMetadata.isDeleted) &&
        Objects.equals(this.version, fileMetadata.version) &&
        Objects.equals(this.createdAt, fileMetadata.createdAt) &&
        Objects.equals(this.updatedAt, fileMetadata.updatedAt) &&
        Objects.equals(this.deletedAt, fileMetadata.deletedAt) &&
        Objects.equals(this.embeddings, fileMetadata.embeddings) &&
        Objects.equals(this.versions, fileMetadata.versions) &&
        Objects.equals(this.comments, fileMetadata.comments) &&
        Objects.equals(this.sharing, fileMetadata.sharing) &&
        Objects.equals(this.snapshots, fileMetadata.snapshots);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, name, size, s3Key, s3BucketName, isDeleted, version, createdAt, updatedAt, deletedAt, embeddings, versions, comments, sharing, snapshots);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class FileMetadata {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    name: ").append(toIndentedString(name)).append("\n");
    sb.append("    size: ").append(toIndentedString(size)).append("\n");
    sb.append("    s3Key: ").append(toIndentedString(s3Key)).append("\n");
    sb.append("    s3BucketName: ").append(toIndentedString(s3BucketName)).append("\n");
    sb.append("    isDeleted: ").append(toIndentedString(isDeleted)).append("\n");
    sb.append("    version: ").append(toIndentedString(version)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
    sb.append("    updatedAt: ").append(toIndentedString(updatedAt)).append("\n");
    sb.append("    deletedAt: ").append(toIndentedString(deletedAt)).append("\n");
    sb.append("    embeddings: ").append(toIndentedString(embeddings)).append("\n");
    sb.append("    versions: ").append(toIndentedString(versions)).append("\n");
    sb.append("    comments: ").append(toIndentedString(comments)).append("\n");
    sb.append("    sharing: ").append(toIndentedString(sharing)).append("\n");
    sb.append("    snapshots: ").append(toIndentedString(snapshots)).append("\n");
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

