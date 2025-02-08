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
 * FileEmbeddings
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-08T10:26:51.224811-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class FileEmbeddings {

  private String id;

  @Valid
  private List<Float> embeddings = new ArrayList<>();

  private String modelVersion;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  public FileEmbeddings id(String id) {
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

  public FileEmbeddings embeddings(List<Float> embeddings) {
    this.embeddings = embeddings;
    return this;
  }

  public FileEmbeddings addEmbeddingsItem(Float embeddingsItem) {
    if (this.embeddings == null) {
      this.embeddings = new ArrayList<>();
    }
    this.embeddings.add(embeddingsItem);
    return this;
  }

  /**
   * Get embeddings
   * @return embeddings
   */
  
  @Schema(name = "embeddings", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("embeddings")
  public List<Float> getEmbeddings() {
    return embeddings;
  }

  public void setEmbeddings(List<Float> embeddings) {
    this.embeddings = embeddings;
  }

  public FileEmbeddings modelVersion(String modelVersion) {
    this.modelVersion = modelVersion;
    return this;
  }

  /**
   * Get modelVersion
   * @return modelVersion
   */
  
  @Schema(name = "modelVersion", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("modelVersion")
  public String getModelVersion() {
    return modelVersion;
  }

  public void setModelVersion(String modelVersion) {
    this.modelVersion = modelVersion;
  }

  public FileEmbeddings createdAt(OffsetDateTime createdAt) {
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

  public FileEmbeddings updatedAt(OffsetDateTime updatedAt) {
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
    FileEmbeddings fileEmbeddings = (FileEmbeddings) o;
    return Objects.equals(this.id, fileEmbeddings.id) &&
        Objects.equals(this.embeddings, fileEmbeddings.embeddings) &&
        Objects.equals(this.modelVersion, fileEmbeddings.modelVersion) &&
        Objects.equals(this.createdAt, fileEmbeddings.createdAt) &&
        Objects.equals(this.updatedAt, fileEmbeddings.updatedAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, embeddings, modelVersion, createdAt, updatedAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class FileEmbeddings {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    embeddings: ").append(toIndentedString(embeddings)).append("\n");
    sb.append("    modelVersion: ").append(toIndentedString(modelVersion)).append("\n");
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

