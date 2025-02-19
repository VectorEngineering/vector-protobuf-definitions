package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.ErrorResponse;
import org.openapitools.model.FieldViolation;
import org.openapitools.model.ResourceValidation;
import org.openapitools.model.SchemaValidation;
import org.openapitools.model.ValidationErrorCode;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * Represents data validation and business logic errors
 */

@Schema(name = "ValidationErrorMessageResponse", description = "Represents data validation and business logic errors")
@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-19T11:32:59.900685-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ValidationErrorMessageResponse {

  private ValidationErrorCode code = ValidationErrorCode.NO_ERROR;

  private String message;

  @Valid
  private List<@Valid FieldViolation> fieldViolations = new ArrayList<>();

  private SchemaValidation schemaValidation;

  private ResourceValidation resourceValidation;

  private ErrorResponse errorResponse;

  public ValidationErrorMessageResponse code(ValidationErrorCode code) {
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
  public ValidationErrorCode getCode() {
    return code;
  }

  public void setCode(ValidationErrorCode code) {
    this.code = code;
  }

  public ValidationErrorMessageResponse message(String message) {
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

  public ValidationErrorMessageResponse fieldViolations(List<@Valid FieldViolation> fieldViolations) {
    this.fieldViolations = fieldViolations;
    return this;
  }

  public ValidationErrorMessageResponse addFieldViolationsItem(FieldViolation fieldViolationsItem) {
    if (this.fieldViolations == null) {
      this.fieldViolations = new ArrayList<>();
    }
    this.fieldViolations.add(fieldViolationsItem);
    return this;
  }

  /**
   * Get fieldViolations
   * @return fieldViolations
   */
  @Valid 
  @Schema(name = "fieldViolations", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("fieldViolations")
  public List<@Valid FieldViolation> getFieldViolations() {
    return fieldViolations;
  }

  public void setFieldViolations(List<@Valid FieldViolation> fieldViolations) {
    this.fieldViolations = fieldViolations;
  }

  public ValidationErrorMessageResponse schemaValidation(SchemaValidation schemaValidation) {
    this.schemaValidation = schemaValidation;
    return this;
  }

  /**
   * Get schemaValidation
   * @return schemaValidation
   */
  @Valid 
  @Schema(name = "schemaValidation", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("schemaValidation")
  public SchemaValidation getSchemaValidation() {
    return schemaValidation;
  }

  public void setSchemaValidation(SchemaValidation schemaValidation) {
    this.schemaValidation = schemaValidation;
  }

  public ValidationErrorMessageResponse resourceValidation(ResourceValidation resourceValidation) {
    this.resourceValidation = resourceValidation;
    return this;
  }

  /**
   * Get resourceValidation
   * @return resourceValidation
   */
  @Valid 
  @Schema(name = "resourceValidation", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("resourceValidation")
  public ResourceValidation getResourceValidation() {
    return resourceValidation;
  }

  public void setResourceValidation(ResourceValidation resourceValidation) {
    this.resourceValidation = resourceValidation;
  }

  public ValidationErrorMessageResponse errorResponse(ErrorResponse errorResponse) {
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
    ValidationErrorMessageResponse validationErrorMessageResponse = (ValidationErrorMessageResponse) o;
    return Objects.equals(this.code, validationErrorMessageResponse.code) &&
        Objects.equals(this.message, validationErrorMessageResponse.message) &&
        Objects.equals(this.fieldViolations, validationErrorMessageResponse.fieldViolations) &&
        Objects.equals(this.schemaValidation, validationErrorMessageResponse.schemaValidation) &&
        Objects.equals(this.resourceValidation, validationErrorMessageResponse.resourceValidation) &&
        Objects.equals(this.errorResponse, validationErrorMessageResponse.errorResponse);
  }

  @Override
  public int hashCode() {
    return Objects.hash(code, message, fieldViolations, schemaValidation, resourceValidation, errorResponse);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ValidationErrorMessageResponse {\n");
    sb.append("    code: ").append(toIndentedString(code)).append("\n");
    sb.append("    message: ").append(toIndentedString(message)).append("\n");
    sb.append("    fieldViolations: ").append(toIndentedString(fieldViolations)).append("\n");
    sb.append("    schemaValidation: ").append(toIndentedString(schemaValidation)).append("\n");
    sb.append("    resourceValidation: ").append(toIndentedString(resourceValidation)).append("\n");
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

