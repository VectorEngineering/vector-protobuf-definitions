package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import org.openapitools.model.ErrorResponse;
import org.openapitools.model.InternalErrorCode;
import org.openapitools.model.LimitInfo;
import org.openapitools.model.QuotaInfo;
import org.openapitools.model.RateLimitContext;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * Represents rate limiting and quota exceeded errors
 */

@Schema(name = "RateLimitErrorMessageResponse", description = "Represents rate limiting and quota exceeded errors")
@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T07:41:41.360877-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class RateLimitErrorMessageResponse {

  private InternalErrorCode code = InternalErrorCode.NO_INTERNAL_ERROR;

  private String message;

  private LimitInfo limitInfo;

  private QuotaInfo quotaInfo;

  private RateLimitContext context;

  private ErrorResponse errorResponse;

  public RateLimitErrorMessageResponse code(InternalErrorCode code) {
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
  public InternalErrorCode getCode() {
    return code;
  }

  public void setCode(InternalErrorCode code) {
    this.code = code;
  }

  public RateLimitErrorMessageResponse message(String message) {
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

  public RateLimitErrorMessageResponse limitInfo(LimitInfo limitInfo) {
    this.limitInfo = limitInfo;
    return this;
  }

  /**
   * Get limitInfo
   * @return limitInfo
   */
  @Valid 
  @Schema(name = "limitInfo", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("limitInfo")
  public LimitInfo getLimitInfo() {
    return limitInfo;
  }

  public void setLimitInfo(LimitInfo limitInfo) {
    this.limitInfo = limitInfo;
  }

  public RateLimitErrorMessageResponse quotaInfo(QuotaInfo quotaInfo) {
    this.quotaInfo = quotaInfo;
    return this;
  }

  /**
   * Get quotaInfo
   * @return quotaInfo
   */
  @Valid 
  @Schema(name = "quotaInfo", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("quotaInfo")
  public QuotaInfo getQuotaInfo() {
    return quotaInfo;
  }

  public void setQuotaInfo(QuotaInfo quotaInfo) {
    this.quotaInfo = quotaInfo;
  }

  public RateLimitErrorMessageResponse context(RateLimitContext context) {
    this.context = context;
    return this;
  }

  /**
   * Get context
   * @return context
   */
  @Valid 
  @Schema(name = "context", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("context")
  public RateLimitContext getContext() {
    return context;
  }

  public void setContext(RateLimitContext context) {
    this.context = context;
  }

  public RateLimitErrorMessageResponse errorResponse(ErrorResponse errorResponse) {
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
    RateLimitErrorMessageResponse rateLimitErrorMessageResponse = (RateLimitErrorMessageResponse) o;
    return Objects.equals(this.code, rateLimitErrorMessageResponse.code) &&
        Objects.equals(this.message, rateLimitErrorMessageResponse.message) &&
        Objects.equals(this.limitInfo, rateLimitErrorMessageResponse.limitInfo) &&
        Objects.equals(this.quotaInfo, rateLimitErrorMessageResponse.quotaInfo) &&
        Objects.equals(this.context, rateLimitErrorMessageResponse.context) &&
        Objects.equals(this.errorResponse, rateLimitErrorMessageResponse.errorResponse);
  }

  @Override
  public int hashCode() {
    return Objects.hash(code, message, limitInfo, quotaInfo, context, errorResponse);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class RateLimitErrorMessageResponse {\n");
    sb.append("    code: ").append(toIndentedString(code)).append("\n");
    sb.append("    message: ").append(toIndentedString(message)).append("\n");
    sb.append("    limitInfo: ").append(toIndentedString(limitInfo)).append("\n");
    sb.append("    quotaInfo: ").append(toIndentedString(quotaInfo)).append("\n");
    sb.append("    context: ").append(toIndentedString(context)).append("\n");
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

