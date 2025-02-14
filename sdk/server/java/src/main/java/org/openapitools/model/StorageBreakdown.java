package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * StorageBreakdown
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T08:15:02.658504-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class StorageBreakdown {

  private String fileType;

  private String size;

  private Integer fileCount;

  public StorageBreakdown fileType(String fileType) {
    this.fileType = fileType;
    return this;
  }

  /**
   * Get fileType
   * @return fileType
   */
  
  @Schema(name = "fileType", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("fileType")
  public String getFileType() {
    return fileType;
  }

  public void setFileType(String fileType) {
    this.fileType = fileType;
  }

  public StorageBreakdown size(String size) {
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

  public StorageBreakdown fileCount(Integer fileCount) {
    this.fileCount = fileCount;
    return this;
  }

  /**
   * Get fileCount
   * @return fileCount
   */
  
  @Schema(name = "fileCount", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("fileCount")
  public Integer getFileCount() {
    return fileCount;
  }

  public void setFileCount(Integer fileCount) {
    this.fileCount = fileCount;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    StorageBreakdown storageBreakdown = (StorageBreakdown) o;
    return Objects.equals(this.fileType, storageBreakdown.fileType) &&
        Objects.equals(this.size, storageBreakdown.size) &&
        Objects.equals(this.fileCount, storageBreakdown.fileCount);
  }

  @Override
  public int hashCode() {
    return Objects.hash(fileType, size, fileCount);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class StorageBreakdown {\n");
    sb.append("    fileType: ").append(toIndentedString(fileType)).append("\n");
    sb.append("    size: ").append(toIndentedString(size)).append("\n");
    sb.append("    fileCount: ").append(toIndentedString(fileCount)).append("\n");
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

