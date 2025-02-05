package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.time.OffsetDateTime;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * SignatureBlock
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-05T07:11:09.755832-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class SignatureBlock {

  private String id;

  private String requestId;

  private String blockType;

  private Integer pageNumber;

  private Float xPosition;

  private Float yPosition;

  private Float width;

  private Float height;

  private Boolean isRequired;

  private String signatureData;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime signedAt;

  public SignatureBlock id(String id) {
    this.id = id;
    return this;
  }

  /**
   * Get id
   * @return id
   */
  
  @Schema(name = "id", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("id")
  public String getId() {
    return id;
  }

  public void setId(String id) {
    this.id = id;
  }

  public SignatureBlock requestId(String requestId) {
    this.requestId = requestId;
    return this;
  }

  /**
   * Get requestId
   * @return requestId
   */
  
  @Schema(name = "requestId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("requestId")
  public String getRequestId() {
    return requestId;
  }

  public void setRequestId(String requestId) {
    this.requestId = requestId;
  }

  public SignatureBlock blockType(String blockType) {
    this.blockType = blockType;
    return this;
  }

  /**
   * Get blockType
   * @return blockType
   */
  
  @Schema(name = "blockType", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("blockType")
  public String getBlockType() {
    return blockType;
  }

  public void setBlockType(String blockType) {
    this.blockType = blockType;
  }

  public SignatureBlock pageNumber(Integer pageNumber) {
    this.pageNumber = pageNumber;
    return this;
  }

  /**
   * Get pageNumber
   * @return pageNumber
   */
  
  @Schema(name = "pageNumber", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("pageNumber")
  public Integer getPageNumber() {
    return pageNumber;
  }

  public void setPageNumber(Integer pageNumber) {
    this.pageNumber = pageNumber;
  }

  public SignatureBlock xPosition(Float xPosition) {
    this.xPosition = xPosition;
    return this;
  }

  /**
   * Get xPosition
   * @return xPosition
   */
  
  @Schema(name = "xPosition", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("xPosition")
  public Float getxPosition() {
    return xPosition;
  }

  public void setxPosition(Float xPosition) {
    this.xPosition = xPosition;
  }

  public SignatureBlock yPosition(Float yPosition) {
    this.yPosition = yPosition;
    return this;
  }

  /**
   * Get yPosition
   * @return yPosition
   */
  
  @Schema(name = "yPosition", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("yPosition")
  public Float getyPosition() {
    return yPosition;
  }

  public void setyPosition(Float yPosition) {
    this.yPosition = yPosition;
  }

  public SignatureBlock width(Float width) {
    this.width = width;
    return this;
  }

  /**
   * Get width
   * @return width
   */
  
  @Schema(name = "width", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("width")
  public Float getWidth() {
    return width;
  }

  public void setWidth(Float width) {
    this.width = width;
  }

  public SignatureBlock height(Float height) {
    this.height = height;
    return this;
  }

  /**
   * Get height
   * @return height
   */
  
  @Schema(name = "height", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("height")
  public Float getHeight() {
    return height;
  }

  public void setHeight(Float height) {
    this.height = height;
  }

  public SignatureBlock isRequired(Boolean isRequired) {
    this.isRequired = isRequired;
    return this;
  }

  /**
   * Get isRequired
   * @return isRequired
   */
  
  @Schema(name = "isRequired", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("isRequired")
  public Boolean getIsRequired() {
    return isRequired;
  }

  public void setIsRequired(Boolean isRequired) {
    this.isRequired = isRequired;
  }

  public SignatureBlock signatureData(String signatureData) {
    this.signatureData = signatureData;
    return this;
  }

  /**
   * Get signatureData
   * @return signatureData
   */
  
  @Schema(name = "signatureData", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("signatureData")
  public String getSignatureData() {
    return signatureData;
  }

  public void setSignatureData(String signatureData) {
    this.signatureData = signatureData;
  }

  public SignatureBlock signedAt(OffsetDateTime signedAt) {
    this.signedAt = signedAt;
    return this;
  }

  /**
   * Get signedAt
   * @return signedAt
   */
  @Valid 
  @Schema(name = "signedAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("signedAt")
  public OffsetDateTime getSignedAt() {
    return signedAt;
  }

  public void setSignedAt(OffsetDateTime signedAt) {
    this.signedAt = signedAt;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    SignatureBlock signatureBlock = (SignatureBlock) o;
    return Objects.equals(this.id, signatureBlock.id) &&
        Objects.equals(this.requestId, signatureBlock.requestId) &&
        Objects.equals(this.blockType, signatureBlock.blockType) &&
        Objects.equals(this.pageNumber, signatureBlock.pageNumber) &&
        Objects.equals(this.xPosition, signatureBlock.xPosition) &&
        Objects.equals(this.yPosition, signatureBlock.yPosition) &&
        Objects.equals(this.width, signatureBlock.width) &&
        Objects.equals(this.height, signatureBlock.height) &&
        Objects.equals(this.isRequired, signatureBlock.isRequired) &&
        Objects.equals(this.signatureData, signatureBlock.signatureData) &&
        Objects.equals(this.signedAt, signatureBlock.signedAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, requestId, blockType, pageNumber, xPosition, yPosition, width, height, isRequired, signatureData, signedAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class SignatureBlock {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    requestId: ").append(toIndentedString(requestId)).append("\n");
    sb.append("    blockType: ").append(toIndentedString(blockType)).append("\n");
    sb.append("    pageNumber: ").append(toIndentedString(pageNumber)).append("\n");
    sb.append("    xPosition: ").append(toIndentedString(xPosition)).append("\n");
    sb.append("    yPosition: ").append(toIndentedString(yPosition)).append("\n");
    sb.append("    width: ").append(toIndentedString(width)).append("\n");
    sb.append("    height: ").append(toIndentedString(height)).append("\n");
    sb.append("    isRequired: ").append(toIndentedString(isRequired)).append("\n");
    sb.append("    signatureData: ").append(toIndentedString(signatureData)).append("\n");
    sb.append("    signedAt: ").append(toIndentedString(signedAt)).append("\n");
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

