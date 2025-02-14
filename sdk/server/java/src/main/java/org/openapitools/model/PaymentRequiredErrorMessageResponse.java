package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import org.openapitools.model.ErrorResponse;
import org.openapitools.model.PaymentInfo;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * Indicates that payment is required to proceed
 */

@Schema(name = "PaymentRequiredErrorMessageResponse", description = "Indicates that payment is required to proceed")
@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T08:15:02.658504-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class PaymentRequiredErrorMessageResponse {

  private Integer code;

  private String message;

  private PaymentInfo paymentInfo;

  private ErrorResponse errorResponse;

  public PaymentRequiredErrorMessageResponse code(Integer code) {
    this.code = code;
    return this;
  }

  /**
   * Get code
   * @return code
   */
  
  @Schema(name = "code", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("code")
  public Integer getCode() {
    return code;
  }

  public void setCode(Integer code) {
    this.code = code;
  }

  public PaymentRequiredErrorMessageResponse message(String message) {
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

  public PaymentRequiredErrorMessageResponse paymentInfo(PaymentInfo paymentInfo) {
    this.paymentInfo = paymentInfo;
    return this;
  }

  /**
   * Get paymentInfo
   * @return paymentInfo
   */
  @Valid 
  @Schema(name = "paymentInfo", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("paymentInfo")
  public PaymentInfo getPaymentInfo() {
    return paymentInfo;
  }

  public void setPaymentInfo(PaymentInfo paymentInfo) {
    this.paymentInfo = paymentInfo;
  }

  public PaymentRequiredErrorMessageResponse errorResponse(ErrorResponse errorResponse) {
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
    PaymentRequiredErrorMessageResponse paymentRequiredErrorMessageResponse = (PaymentRequiredErrorMessageResponse) o;
    return Objects.equals(this.code, paymentRequiredErrorMessageResponse.code) &&
        Objects.equals(this.message, paymentRequiredErrorMessageResponse.message) &&
        Objects.equals(this.paymentInfo, paymentRequiredErrorMessageResponse.paymentInfo) &&
        Objects.equals(this.errorResponse, paymentRequiredErrorMessageResponse.errorResponse);
  }

  @Override
  public int hashCode() {
    return Objects.hash(code, message, paymentInfo, errorResponse);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class PaymentRequiredErrorMessageResponse {\n");
    sb.append("    code: ").append(toIndentedString(code)).append("\n");
    sb.append("    message: ").append(toIndentedString(message)).append("\n");
    sb.append("    paymentInfo: ").append(toIndentedString(paymentInfo)).append("\n");
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

