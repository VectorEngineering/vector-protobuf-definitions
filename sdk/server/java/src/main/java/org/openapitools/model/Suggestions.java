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
 * Suggestions
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-01-29T12:50:06.029660-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class Suggestions {

  @Valid
  private List<String> similarResources = new ArrayList<>();

  @Valid
  private List<String> alternativePaths = new ArrayList<>();

  private String documentationUrl;

  @Valid
  private Map<String, String> hints = new HashMap<>();

  public Suggestions similarResources(List<String> similarResources) {
    this.similarResources = similarResources;
    return this;
  }

  public Suggestions addSimilarResourcesItem(String similarResourcesItem) {
    if (this.similarResources == null) {
      this.similarResources = new ArrayList<>();
    }
    this.similarResources.add(similarResourcesItem);
    return this;
  }

  /**
   * Get similarResources
   * @return similarResources
   */
  
  @Schema(name = "similarResources", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("similarResources")
  public List<String> getSimilarResources() {
    return similarResources;
  }

  public void setSimilarResources(List<String> similarResources) {
    this.similarResources = similarResources;
  }

  public Suggestions alternativePaths(List<String> alternativePaths) {
    this.alternativePaths = alternativePaths;
    return this;
  }

  public Suggestions addAlternativePathsItem(String alternativePathsItem) {
    if (this.alternativePaths == null) {
      this.alternativePaths = new ArrayList<>();
    }
    this.alternativePaths.add(alternativePathsItem);
    return this;
  }

  /**
   * Get alternativePaths
   * @return alternativePaths
   */
  
  @Schema(name = "alternativePaths", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("alternativePaths")
  public List<String> getAlternativePaths() {
    return alternativePaths;
  }

  public void setAlternativePaths(List<String> alternativePaths) {
    this.alternativePaths = alternativePaths;
  }

  public Suggestions documentationUrl(String documentationUrl) {
    this.documentationUrl = documentationUrl;
    return this;
  }

  /**
   * Get documentationUrl
   * @return documentationUrl
   */
  
  @Schema(name = "documentationUrl", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("documentationUrl")
  public String getDocumentationUrl() {
    return documentationUrl;
  }

  public void setDocumentationUrl(String documentationUrl) {
    this.documentationUrl = documentationUrl;
  }

  public Suggestions hints(Map<String, String> hints) {
    this.hints = hints;
    return this;
  }

  public Suggestions putHintsItem(String key, String hintsItem) {
    if (this.hints == null) {
      this.hints = new HashMap<>();
    }
    this.hints.put(key, hintsItem);
    return this;
  }

  /**
   * Get hints
   * @return hints
   */
  
  @Schema(name = "hints", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("hints")
  public Map<String, String> getHints() {
    return hints;
  }

  public void setHints(Map<String, String> hints) {
    this.hints = hints;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    Suggestions suggestions = (Suggestions) o;
    return Objects.equals(this.similarResources, suggestions.similarResources) &&
        Objects.equals(this.alternativePaths, suggestions.alternativePaths) &&
        Objects.equals(this.documentationUrl, suggestions.documentationUrl) &&
        Objects.equals(this.hints, suggestions.hints);
  }

  @Override
  public int hashCode() {
    return Objects.hash(similarResources, alternativePaths, documentationUrl, hints);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class Suggestions {\n");
    sb.append("    similarResources: ").append(toIndentedString(similarResources)).append("\n");
    sb.append("    alternativePaths: ").append(toIndentedString(alternativePaths)).append("\n");
    sb.append("    documentationUrl: ").append(toIndentedString(documentationUrl)).append("\n");
    sb.append("    hints: ").append(toIndentedString(hints)).append("\n");
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

