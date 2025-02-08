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
 * UserActivity
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T21:14:09.107639-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class UserActivity {

  private String userId;

  private String email;

  private Integer fileOperations;

  private Integer commentsMade;

  private Integer documentsProcessed;

  public UserActivity userId(String userId) {
    this.userId = userId;
    return this;
  }

  /**
   * Get userId
   * @return userId
   */
  
  @Schema(name = "userId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("userId")
  public String getUserId() {
    return userId;
  }

  public void setUserId(String userId) {
    this.userId = userId;
  }

  public UserActivity email(String email) {
    this.email = email;
    return this;
  }

  /**
   * Get email
   * @return email
   */
  
  @Schema(name = "email", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("email")
  public String getEmail() {
    return email;
  }

  public void setEmail(String email) {
    this.email = email;
  }

  public UserActivity fileOperations(Integer fileOperations) {
    this.fileOperations = fileOperations;
    return this;
  }

  /**
   * Get fileOperations
   * @return fileOperations
   */
  
  @Schema(name = "fileOperations", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("fileOperations")
  public Integer getFileOperations() {
    return fileOperations;
  }

  public void setFileOperations(Integer fileOperations) {
    this.fileOperations = fileOperations;
  }

  public UserActivity commentsMade(Integer commentsMade) {
    this.commentsMade = commentsMade;
    return this;
  }

  /**
   * Get commentsMade
   * @return commentsMade
   */
  
  @Schema(name = "commentsMade", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("commentsMade")
  public Integer getCommentsMade() {
    return commentsMade;
  }

  public void setCommentsMade(Integer commentsMade) {
    this.commentsMade = commentsMade;
  }

  public UserActivity documentsProcessed(Integer documentsProcessed) {
    this.documentsProcessed = documentsProcessed;
    return this;
  }

  /**
   * Get documentsProcessed
   * @return documentsProcessed
   */
  
  @Schema(name = "documentsProcessed", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("documentsProcessed")
  public Integer getDocumentsProcessed() {
    return documentsProcessed;
  }

  public void setDocumentsProcessed(Integer documentsProcessed) {
    this.documentsProcessed = documentsProcessed;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    UserActivity userActivity = (UserActivity) o;
    return Objects.equals(this.userId, userActivity.userId) &&
        Objects.equals(this.email, userActivity.email) &&
        Objects.equals(this.fileOperations, userActivity.fileOperations) &&
        Objects.equals(this.commentsMade, userActivity.commentsMade) &&
        Objects.equals(this.documentsProcessed, userActivity.documentsProcessed);
  }

  @Override
  public int hashCode() {
    return Objects.hash(userId, email, fileOperations, commentsMade, documentsProcessed);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class UserActivity {\n");
    sb.append("    userId: ").append(toIndentedString(userId)).append("\n");
    sb.append("    email: ").append(toIndentedString(email)).append("\n");
    sb.append("    fileOperations: ").append(toIndentedString(fileOperations)).append("\n");
    sb.append("    commentsMade: ").append(toIndentedString(commentsMade)).append("\n");
    sb.append("    documentsProcessed: ").append(toIndentedString(documentsProcessed)).append("\n");
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

