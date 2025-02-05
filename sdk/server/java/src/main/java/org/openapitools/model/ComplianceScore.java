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
 * ComplianceScore
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-05T07:11:09.755832-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ComplianceScore {

  private String category;

  private Float score;

  private String status;

  @Valid
  private List<String> improvements = new ArrayList<>();

  public ComplianceScore category(String category) {
    this.category = category;
    return this;
  }

  /**
   * Get category
   * @return category
   */
  
  @Schema(name = "category", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("category")
  public String getCategory() {
    return category;
  }

  public void setCategory(String category) {
    this.category = category;
  }

  public ComplianceScore score(Float score) {
    this.score = score;
    return this;
  }

  /**
   * Get score
   * @return score
   */
  
  @Schema(name = "score", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("score")
  public Float getScore() {
    return score;
  }

  public void setScore(Float score) {
    this.score = score;
  }

  public ComplianceScore status(String status) {
    this.status = status;
    return this;
  }

  /**
   * Get status
   * @return status
   */
  
  @Schema(name = "status", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("status")
  public String getStatus() {
    return status;
  }

  public void setStatus(String status) {
    this.status = status;
  }

  public ComplianceScore improvements(List<String> improvements) {
    this.improvements = improvements;
    return this;
  }

  public ComplianceScore addImprovementsItem(String improvementsItem) {
    if (this.improvements == null) {
      this.improvements = new ArrayList<>();
    }
    this.improvements.add(improvementsItem);
    return this;
  }

  /**
   * Get improvements
   * @return improvements
   */
  
  @Schema(name = "improvements", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("improvements")
  public List<String> getImprovements() {
    return improvements;
  }

  public void setImprovements(List<String> improvements) {
    this.improvements = improvements;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    ComplianceScore complianceScore = (ComplianceScore) o;
    return Objects.equals(this.category, complianceScore.category) &&
        Objects.equals(this.score, complianceScore.score) &&
        Objects.equals(this.status, complianceScore.status) &&
        Objects.equals(this.improvements, complianceScore.improvements);
  }

  @Override
  public int hashCode() {
    return Objects.hash(category, score, status, improvements);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ComplianceScore {\n");
    sb.append("    category: ").append(toIndentedString(category)).append("\n");
    sb.append("    score: ").append(toIndentedString(score)).append("\n");
    sb.append("    status: ").append(toIndentedString(status)).append("\n");
    sb.append("    improvements: ").append(toIndentedString(improvements)).append("\n");
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

