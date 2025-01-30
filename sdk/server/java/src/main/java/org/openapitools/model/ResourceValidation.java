package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * ResourceValidation
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-01-29T23:23:25.497382-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ResourceValidation {

  @Valid
  private List<String> missingResources = new ArrayList<>();

  @Valid
  private List<String> invalidReferences = new ArrayList<>();

  @Valid
  private Map<String, String> idViolations = new HashMap<>();

  public ResourceValidation missingResources(List<String> missingResources) {
    this.missingResources = missingResources;
    return this;
  }

  public ResourceValidation addMissingResourcesItem(String missingResourcesItem) {
    if (this.missingResources == null) {
      this.missingResources = new ArrayList<>();
    }
    this.missingResources.add(missingResourcesItem);
    return this;
  }

  /**
   * Get missingResources
   * @return missingResources
   */
  
  @Schema(name = "missingResources", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("missingResources")
  public List<String> getMissingResources() {
    return missingResources;
  }

  public void setMissingResources(List<String> missingResources) {
    this.missingResources = missingResources;
  }

  public ResourceValidation invalidReferences(List<String> invalidReferences) {
    this.invalidReferences = invalidReferences;
    return this;
  }

  public ResourceValidation addInvalidReferencesItem(String invalidReferencesItem) {
    if (this.invalidReferences == null) {
      this.invalidReferences = new ArrayList<>();
    }
    this.invalidReferences.add(invalidReferencesItem);
    return this;
  }

  /**
   * Get invalidReferences
   * @return invalidReferences
   */
  
  @Schema(name = "invalidReferences", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("invalidReferences")
  public List<String> getInvalidReferences() {
    return invalidReferences;
  }

  public void setInvalidReferences(List<String> invalidReferences) {
    this.invalidReferences = invalidReferences;
  }

  public ResourceValidation idViolations(Map<String, String> idViolations) {
    this.idViolations = idViolations;
    return this;
  }

  public ResourceValidation putIdViolationsItem(String key, String idViolationsItem) {
    if (this.idViolations == null) {
      this.idViolations = new HashMap<>();
    }
    this.idViolations.put(key, idViolationsItem);
    return this;
  }

  /**
   * Get idViolations
   * @return idViolations
   */
  
  @Schema(name = "idViolations", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("idViolations")
  public Map<String, String> getIdViolations() {
    return idViolations;
  }

  public void setIdViolations(Map<String, String> idViolations) {
    this.idViolations = idViolations;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    ResourceValidation resourceValidation = (ResourceValidation) o;
    return Objects.equals(this.missingResources, resourceValidation.missingResources) &&
        Objects.equals(this.invalidReferences, resourceValidation.invalidReferences) &&
        Objects.equals(this.idViolations, resourceValidation.idViolations);
  }

  @Override
  public int hashCode() {
    return Objects.hash(missingResources, invalidReferences, idViolations);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ResourceValidation {\n");
    sb.append("    missingResources: ").append(toIndentedString(missingResources)).append("\n");
    sb.append("    invalidReferences: ").append(toIndentedString(invalidReferences)).append("\n");
    sb.append("    idViolations: ").append(toIndentedString(idViolations)).append("\n");
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

