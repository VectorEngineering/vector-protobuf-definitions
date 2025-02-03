package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.FileMetadata;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * FolderMetadata
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-03T15:12:04.029446-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class FolderMetadata {

  private String id;

  private String name;

  private String s3BucketName;

  private String s3FolderPath;

  private Boolean isDeleted;

  private String parentFolderId;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime deletedAt;

  @Valid
  private List<@Valid FolderMetadata> childFolders = new ArrayList<>();

  @Valid
  private List<@Valid FileMetadata> files = new ArrayList<>();

  public FolderMetadata id(String id) {
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

  public FolderMetadata name(String name) {
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

  public FolderMetadata s3BucketName(String s3BucketName) {
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

  public FolderMetadata s3FolderPath(String s3FolderPath) {
    this.s3FolderPath = s3FolderPath;
    return this;
  }

  /**
   * Get s3FolderPath
   * @return s3FolderPath
   */
  
  @Schema(name = "s3FolderPath", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("s3FolderPath")
  public String getS3FolderPath() {
    return s3FolderPath;
  }

  public void setS3FolderPath(String s3FolderPath) {
    this.s3FolderPath = s3FolderPath;
  }

  public FolderMetadata isDeleted(Boolean isDeleted) {
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

  public FolderMetadata parentFolderId(String parentFolderId) {
    this.parentFolderId = parentFolderId;
    return this;
  }

  /**
   * Get parentFolderId
   * @return parentFolderId
   */
  
  @Schema(name = "parentFolderId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("parentFolderId")
  public String getParentFolderId() {
    return parentFolderId;
  }

  public void setParentFolderId(String parentFolderId) {
    this.parentFolderId = parentFolderId;
  }

  public FolderMetadata createdAt(OffsetDateTime createdAt) {
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

  public FolderMetadata updatedAt(OffsetDateTime updatedAt) {
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

  public FolderMetadata deletedAt(OffsetDateTime deletedAt) {
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

  public FolderMetadata childFolders(List<@Valid FolderMetadata> childFolders) {
    this.childFolders = childFolders;
    return this;
  }

  public FolderMetadata addChildFoldersItem(FolderMetadata childFoldersItem) {
    if (this.childFolders == null) {
      this.childFolders = new ArrayList<>();
    }
    this.childFolders.add(childFoldersItem);
    return this;
  }

  /**
   * Get childFolders
   * @return childFolders
   */
  @Valid 
  @Schema(name = "childFolders", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("childFolders")
  public List<@Valid FolderMetadata> getChildFolders() {
    return childFolders;
  }

  public void setChildFolders(List<@Valid FolderMetadata> childFolders) {
    this.childFolders = childFolders;
  }

  public FolderMetadata files(List<@Valid FileMetadata> files) {
    this.files = files;
    return this;
  }

  public FolderMetadata addFilesItem(FileMetadata filesItem) {
    if (this.files == null) {
      this.files = new ArrayList<>();
    }
    this.files.add(filesItem);
    return this;
  }

  /**
   * Get files
   * @return files
   */
  @Valid 
  @Schema(name = "files", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("files")
  public List<@Valid FileMetadata> getFiles() {
    return files;
  }

  public void setFiles(List<@Valid FileMetadata> files) {
    this.files = files;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    FolderMetadata folderMetadata = (FolderMetadata) o;
    return Objects.equals(this.id, folderMetadata.id) &&
        Objects.equals(this.name, folderMetadata.name) &&
        Objects.equals(this.s3BucketName, folderMetadata.s3BucketName) &&
        Objects.equals(this.s3FolderPath, folderMetadata.s3FolderPath) &&
        Objects.equals(this.isDeleted, folderMetadata.isDeleted) &&
        Objects.equals(this.parentFolderId, folderMetadata.parentFolderId) &&
        Objects.equals(this.createdAt, folderMetadata.createdAt) &&
        Objects.equals(this.updatedAt, folderMetadata.updatedAt) &&
        Objects.equals(this.deletedAt, folderMetadata.deletedAt) &&
        Objects.equals(this.childFolders, folderMetadata.childFolders) &&
        Objects.equals(this.files, folderMetadata.files);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, name, s3BucketName, s3FolderPath, isDeleted, parentFolderId, createdAt, updatedAt, deletedAt, childFolders, files);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class FolderMetadata {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    name: ").append(toIndentedString(name)).append("\n");
    sb.append("    s3BucketName: ").append(toIndentedString(s3BucketName)).append("\n");
    sb.append("    s3FolderPath: ").append(toIndentedString(s3FolderPath)).append("\n");
    sb.append("    isDeleted: ").append(toIndentedString(isDeleted)).append("\n");
    sb.append("    parentFolderId: ").append(toIndentedString(parentFolderId)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
    sb.append("    updatedAt: ").append(toIndentedString(updatedAt)).append("\n");
    sb.append("    deletedAt: ").append(toIndentedString(deletedAt)).append("\n");
    sb.append("    childFolders: ").append(toIndentedString(childFolders)).append("\n");
    sb.append("    files: ").append(toIndentedString(files)).append("\n");
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

