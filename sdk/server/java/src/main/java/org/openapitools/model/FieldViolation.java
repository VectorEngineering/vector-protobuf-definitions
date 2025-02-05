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
 * FieldViolation
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-05T06:54:24.429429-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class FieldViolation {

  private String field;

  private String validation;

  private String message;

  private String expected;

  private String actual;

  public FieldViolation field(String field) {
    this.field = field;
    return this;
  }

  /**
   * Get field
   * @return field
   */
  
  @Schema(name = "field", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("field")
  public String getField() {
    return field;
  }

  public void setField(String field) {
    this.field = field;
  }

  public FieldViolation validation(String validation) {
    this.validation = validation;
    return this;
  }

  /**
   * Get validation
   * @return validation
   */
  
  @Schema(name = "validation", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("validation")
  public String getValidation() {
    return validation;
  }

  public void setValidation(String validation) {
    this.validation = validation;
  }

  public FieldViolation message(String message) {
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

  public FieldViolation expected(String expected) {
    this.expected = expected;
    return this;
  }

  /**
   * Get expected
   * @return expected
   */
  
  @Schema(name = "expected", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("expected")
  public String getExpected() {
    return expected;
  }

  public void setExpected(String expected) {
    this.expected = expected;
  }

  public FieldViolation actual(String actual) {
    this.actual = actual;
    return this;
  }

  /**
   * Get actual
   * @return actual
   */
  
  @Schema(name = "actual", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("actual")
  public String getActual() {
    return actual;
  }

  public void setActual(String actual) {
    this.actual = actual;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    FieldViolation fieldViolation = (FieldViolation) o;
    return Objects.equals(this.field, fieldViolation.field) &&
        Objects.equals(this.validation, fieldViolation.validation) &&
        Objects.equals(this.message, fieldViolation.message) &&
        Objects.equals(this.expected, fieldViolation.expected) &&
        Objects.equals(this.actual, fieldViolation.actual);
  }

  @Override
  public int hashCode() {
    return Objects.hash(field, validation, message, expected, actual);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class FieldViolation {\n");
    sb.append("    field: ").append(toIndentedString(field)).append("\n");
    sb.append("    validation: ").append(toIndentedString(validation)).append("\n");
    sb.append("    message: ").append(toIndentedString(message)).append("\n");
    sb.append("    expected: ").append(toIndentedString(expected)).append("\n");
    sb.append("    actual: ").append(toIndentedString(actual)).append("\n");
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

