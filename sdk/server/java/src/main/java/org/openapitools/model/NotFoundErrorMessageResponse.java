package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import org.openapitools.model.APIInfo;
import org.openapitools.model.ErrorResponse;
import org.openapitools.model.NotFoundErrorCode;
import org.openapitools.model.ResourceInfo;
import org.openapitools.model.Suggestions;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * Represents resource not found errors
 */

@Schema(name = "NotFoundErrorMessageResponse", description = "Represents resource not found errors")
@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-05T07:29:26.257343-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class NotFoundErrorMessageResponse {

  private NotFoundErrorCode code = NotFoundErrorCode.NO_NOT_FOUND_ERROR;

  private String message;

  private ResourceInfo resourceInfo;

  private Suggestions suggestions;

  private APIInfo apiInfo;

  private ErrorResponse errorResponse;

  public NotFoundErrorMessageResponse code(NotFoundErrorCode code) {
    this.code = code;
    return this;
  }

  /**
   * Get code
   * @return code
   */
  @Valid 
  @Schema(name = "code", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("code")
  public NotFoundErrorCode getCode() {
    return code;
  }

  public void setCode(NotFoundErrorCode code) {
    this.code = code;
  }

  public NotFoundErrorMessageResponse message(String message) {
    this.message = message;
    return this;
  }

  /**
   * Get message
   * @return message
   */
  
  @Schema(name = "message", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("message")
  public String getMessage() {
    return message;
  }

  public void setMessage(String message) {
    this.message = message;
  }

  public NotFoundErrorMessageResponse resourceInfo(ResourceInfo resourceInfo) {
    this.resourceInfo = resourceInfo;
    return this;
  }

  /**
   * Get resourceInfo
   * @return resourceInfo
   */
  @Valid 
  @Schema(name = "resourceInfo", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("resourceInfo")
  public ResourceInfo getResourceInfo() {
    return resourceInfo;
  }

  public void setResourceInfo(ResourceInfo resourceInfo) {
    this.resourceInfo = resourceInfo;
  }

  public NotFoundErrorMessageResponse suggestions(Suggestions suggestions) {
    this.suggestions = suggestions;
    return this;
  }

  /**
   * Get suggestions
   * @return suggestions
   */
  @Valid 
  @Schema(name = "suggestions", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("suggestions")
  public Suggestions getSuggestions() {
    return suggestions;
  }

  public void setSuggestions(Suggestions suggestions) {
    this.suggestions = suggestions;
  }

  public NotFoundErrorMessageResponse apiInfo(APIInfo apiInfo) {
    this.apiInfo = apiInfo;
    return this;
  }

  /**
   * Get apiInfo
   * @return apiInfo
   */
  @Valid 
  @Schema(name = "apiInfo", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("apiInfo")
  public APIInfo getApiInfo() {
    return apiInfo;
  }

  public void setApiInfo(APIInfo apiInfo) {
    this.apiInfo = apiInfo;
  }

  public NotFoundErrorMessageResponse errorResponse(ErrorResponse errorResponse) {
    this.errorResponse = errorResponse;
    return this;
  }

  /**
   * Get errorResponse
   * @return errorResponse
   */
  @Valid 
  @Schema(name = "errorResponse", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("errorResponse")
  public ErrorResponse getErrorResponse() {
    return errorResponse;
  }

  public void setErrorResponse(ErrorResponse errorResponse) {
    this.errorResponse = errorResponse;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    NotFoundErrorMessageResponse notFoundErrorMessageResponse = (NotFoundErrorMessageResponse) o;
    return Objects.equals(this.code, notFoundErrorMessageResponse.code) &&
        Objects.equals(this.message, notFoundErrorMessageResponse.message) &&
        Objects.equals(this.resourceInfo, notFoundErrorMessageResponse.resourceInfo) &&
        Objects.equals(this.suggestions, notFoundErrorMessageResponse.suggestions) &&
        Objects.equals(this.apiInfo, notFoundErrorMessageResponse.apiInfo) &&
        Objects.equals(this.errorResponse, notFoundErrorMessageResponse.errorResponse);
  }

  @Override
  public int hashCode() {
    return Objects.hash(code, message, resourceInfo, suggestions, apiInfo, errorResponse);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class NotFoundErrorMessageResponse {\n");
    sb.append("    code: ").append(toIndentedString(code)).append("\n");
    sb.append("    message: ").append(toIndentedString(message)).append("\n");
    sb.append("    resourceInfo: ").append(toIndentedString(resourceInfo)).append("\n");
    sb.append("    suggestions: ").append(toIndentedString(suggestions)).append("\n");
    sb.append("    apiInfo: ").append(toIndentedString(apiInfo)).append("\n");
    sb.append("    errorResponse: ").append(toIndentedString(errorResponse)).append("\n");
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

