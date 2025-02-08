package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * TokenInfo
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-08T10:46:38.744443-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class TokenInfo {

  private String tokenId;

  private String tokenType;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime expiry;

  private String issuer;

  @Valid
  private List<String> audiences = new ArrayList<>();

  public TokenInfo tokenId(String tokenId) {
    this.tokenId = tokenId;
    return this;
  }

  /**
   * Get tokenId
   * @return tokenId
   */
  
  @Schema(name = "tokenId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("tokenId")
  public String getTokenId() {
    return tokenId;
  }

  public void setTokenId(String tokenId) {
    this.tokenId = tokenId;
  }

  public TokenInfo tokenType(String tokenType) {
    this.tokenType = tokenType;
    return this;
  }

  /**
   * Get tokenType
   * @return tokenType
   */
  
  @Schema(name = "tokenType", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("tokenType")
  public String getTokenType() {
    return tokenType;
  }

  public void setTokenType(String tokenType) {
    this.tokenType = tokenType;
  }

  public TokenInfo expiry(OffsetDateTime expiry) {
    this.expiry = expiry;
    return this;
  }

  /**
   * Get expiry
   * @return expiry
   */
  @Valid 
  @Schema(name = "expiry", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("expiry")
  public OffsetDateTime getExpiry() {
    return expiry;
  }

  public void setExpiry(OffsetDateTime expiry) {
    this.expiry = expiry;
  }

  public TokenInfo issuer(String issuer) {
    this.issuer = issuer;
    return this;
  }

  /**
   * Get issuer
   * @return issuer
   */
  
  @Schema(name = "issuer", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("issuer")
  public String getIssuer() {
    return issuer;
  }

  public void setIssuer(String issuer) {
    this.issuer = issuer;
  }

  public TokenInfo audiences(List<String> audiences) {
    this.audiences = audiences;
    return this;
  }

  public TokenInfo addAudiencesItem(String audiencesItem) {
    if (this.audiences == null) {
      this.audiences = new ArrayList<>();
    }
    this.audiences.add(audiencesItem);
    return this;
  }

  /**
   * Get audiences
   * @return audiences
   */
  
  @Schema(name = "audiences", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("audiences")
  public List<String> getAudiences() {
    return audiences;
  }

  public void setAudiences(List<String> audiences) {
    this.audiences = audiences;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    TokenInfo tokenInfo = (TokenInfo) o;
    return Objects.equals(this.tokenId, tokenInfo.tokenId) &&
        Objects.equals(this.tokenType, tokenInfo.tokenType) &&
        Objects.equals(this.expiry, tokenInfo.expiry) &&
        Objects.equals(this.issuer, tokenInfo.issuer) &&
        Objects.equals(this.audiences, tokenInfo.audiences);
  }

  @Override
  public int hashCode() {
    return Objects.hash(tokenId, tokenType, expiry, issuer, audiences);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class TokenInfo {\n");
    sb.append("    tokenId: ").append(toIndentedString(tokenId)).append("\n");
    sb.append("    tokenType: ").append(toIndentedString(tokenType)).append("\n");
    sb.append("    expiry: ").append(toIndentedString(expiry)).append("\n");
    sb.append("    issuer: ").append(toIndentedString(issuer)).append("\n");
    sb.append("    audiences: ").append(toIndentedString(audiences)).append("\n");
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

