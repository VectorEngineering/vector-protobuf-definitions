package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * SchemaValidation
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-01-29T12:28:25.110741-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class SchemaValidation {

  private String schemaVersion;

  @Valid
  private List<String> invalidPatterns = new ArrayList<>();

  @Valid
  private List<String> typeMismatches = new ArrayList<>();

  public SchemaValidation schemaVersion(String schemaVersion) {
    this.schemaVersion = schemaVersion;
    return this;
  }

  /**
   * Get schemaVersion
   * @return schemaVersion
   */
  
  @Schema(name = "schemaVersion", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("schemaVersion")
  public String getSchemaVersion() {
    return schemaVersion;
  }

  public void setSchemaVersion(String schemaVersion) {
    this.schemaVersion = schemaVersion;
  }

  public SchemaValidation invalidPatterns(List<String> invalidPatterns) {
    this.invalidPatterns = invalidPatterns;
    return this;
  }

  public SchemaValidation addInvalidPatternsItem(String invalidPatternsItem) {
    if (this.invalidPatterns == null) {
      this.invalidPatterns = new ArrayList<>();
    }
    this.invalidPatterns.add(invalidPatternsItem);
    return this;
  }

  /**
   * Get invalidPatterns
   * @return invalidPatterns
   */
  
  @Schema(name = "invalidPatterns", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("invalidPatterns")
  public List<String> getInvalidPatterns() {
    return invalidPatterns;
  }

  public void setInvalidPatterns(List<String> invalidPatterns) {
    this.invalidPatterns = invalidPatterns;
  }

  public SchemaValidation typeMismatches(List<String> typeMismatches) {
    this.typeMismatches = typeMismatches;
    return this;
  }

  public SchemaValidation addTypeMismatchesItem(String typeMismatchesItem) {
    if (this.typeMismatches == null) {
      this.typeMismatches = new ArrayList<>();
    }
    this.typeMismatches.add(typeMismatchesItem);
    return this;
  }

  /**
   * Get typeMismatches
   * @return typeMismatches
   */
  
  @Schema(name = "typeMismatches", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("typeMismatches")
  public List<String> getTypeMismatches() {
    return typeMismatches;
  }

  public void setTypeMismatches(List<String> typeMismatches) {
    this.typeMismatches = typeMismatches;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    SchemaValidation schemaValidation = (SchemaValidation) o;
    return Objects.equals(this.schemaVersion, schemaValidation.schemaVersion) &&
        Objects.equals(this.invalidPatterns, schemaValidation.invalidPatterns) &&
        Objects.equals(this.typeMismatches, schemaValidation.typeMismatches);
  }

  @Override
  public int hashCode() {
    return Objects.hash(schemaVersion, invalidPatterns, typeMismatches);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class SchemaValidation {\n");
    sb.append("    schemaVersion: ").append(toIndentedString(schemaVersion)).append("\n");
    sb.append("    invalidPatterns: ").append(toIndentedString(invalidPatterns)).append("\n");
    sb.append("    typeMismatches: ").append(toIndentedString(typeMismatches)).append("\n");
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

