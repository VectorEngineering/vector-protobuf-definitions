package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import java.util.HashMap;
import java.util.Map;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * DataProfile
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T08:02:26.368586-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class DataProfile {

  private String id;

  private String name;

  private String profileType;

  @Valid
  private Map<String, String> dataFields = new HashMap<>();

  private Boolean isDefault;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  public DataProfile id(String id) {
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

  public DataProfile name(String name) {
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

  public DataProfile profileType(String profileType) {
    this.profileType = profileType;
    return this;
  }

  /**
   * Get profileType
   * @return profileType
   */
  
  @Schema(name = "profileType", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("profileType")
  public String getProfileType() {
    return profileType;
  }

  public void setProfileType(String profileType) {
    this.profileType = profileType;
  }

  public DataProfile dataFields(Map<String, String> dataFields) {
    this.dataFields = dataFields;
    return this;
  }

  public DataProfile putDataFieldsItem(String key, String dataFieldsItem) {
    if (this.dataFields == null) {
      this.dataFields = new HashMap<>();
    }
    this.dataFields.put(key, dataFieldsItem);
    return this;
  }

  /**
   * Get dataFields
   * @return dataFields
   */
  
  @Schema(name = "dataFields", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("dataFields")
  public Map<String, String> getDataFields() {
    return dataFields;
  }

  public void setDataFields(Map<String, String> dataFields) {
    this.dataFields = dataFields;
  }

  public DataProfile isDefault(Boolean isDefault) {
    this.isDefault = isDefault;
    return this;
  }

  /**
   * Get isDefault
   * @return isDefault
   */
  
  @Schema(name = "isDefault", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("isDefault")
  public Boolean getIsDefault() {
    return isDefault;
  }

  public void setIsDefault(Boolean isDefault) {
    this.isDefault = isDefault;
  }

  public DataProfile createdAt(OffsetDateTime createdAt) {
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

  public DataProfile updatedAt(OffsetDateTime updatedAt) {
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
    DataProfile dataProfile = (DataProfile) o;
    return Objects.equals(this.id, dataProfile.id) &&
        Objects.equals(this.name, dataProfile.name) &&
        Objects.equals(this.profileType, dataProfile.profileType) &&
        Objects.equals(this.dataFields, dataProfile.dataFields) &&
        Objects.equals(this.isDefault, dataProfile.isDefault) &&
        Objects.equals(this.createdAt, dataProfile.createdAt) &&
        Objects.equals(this.updatedAt, dataProfile.updatedAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, name, profileType, dataFields, isDefault, createdAt, updatedAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class DataProfile {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    name: ").append(toIndentedString(name)).append("\n");
    sb.append("    profileType: ").append(toIndentedString(profileType)).append("\n");
    sb.append("    dataFields: ").append(toIndentedString(dataFields)).append("\n");
    sb.append("    isDefault: ").append(toIndentedString(isDefault)).append("\n");
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

