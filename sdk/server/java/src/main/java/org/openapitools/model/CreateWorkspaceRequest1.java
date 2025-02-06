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
 * CreateWorkspaceRequest1
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-06T17:12:08.548747-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class CreateWorkspaceRequest1 {

  private String accountId;

  private String name;

  private String storageQuota;

  private Boolean allowPublicSharing;

  private Boolean requireApproval;

  private Boolean gdprCompliant;

  private Boolean hipaaCompliant;

  public CreateWorkspaceRequest1() {
    super();
  }

  /**
   * Constructor with only required parameters
   */
  public CreateWorkspaceRequest1(String name) {
    this.name = name;
  }

  public CreateWorkspaceRequest1 accountId(String accountId) {
    this.accountId = accountId;
    return this;
  }

  /**
   * Get accountId
   * @return accountId
   */
  
  @Schema(name = "accountId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("accountId")
  public String getAccountId() {
    return accountId;
  }

  public void setAccountId(String accountId) {
    this.accountId = accountId;
  }

  public CreateWorkspaceRequest1 name(String name) {
    this.name = name;
    return this;
  }

  /**
   * Get name
   * @return name
   */
  @NotNull 
  @Schema(name = "name", requiredMode = Schema.RequiredMode.REQUIRED)
  @JsonProperty("name")
  public String getName() {
    return name;
  }

  public void setName(String name) {
    this.name = name;
  }

  public CreateWorkspaceRequest1 storageQuota(String storageQuota) {
    this.storageQuota = storageQuota;
    return this;
  }

  /**
   * Get storageQuota
   * @return storageQuota
   */
  
  @Schema(name = "storageQuota", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("storageQuota")
  public String getStorageQuota() {
    return storageQuota;
  }

  public void setStorageQuota(String storageQuota) {
    this.storageQuota = storageQuota;
  }

  public CreateWorkspaceRequest1 allowPublicSharing(Boolean allowPublicSharing) {
    this.allowPublicSharing = allowPublicSharing;
    return this;
  }

  /**
   * Get allowPublicSharing
   * @return allowPublicSharing
   */
  
  @Schema(name = "allowPublicSharing", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("allowPublicSharing")
  public Boolean getAllowPublicSharing() {
    return allowPublicSharing;
  }

  public void setAllowPublicSharing(Boolean allowPublicSharing) {
    this.allowPublicSharing = allowPublicSharing;
  }

  public CreateWorkspaceRequest1 requireApproval(Boolean requireApproval) {
    this.requireApproval = requireApproval;
    return this;
  }

  /**
   * Get requireApproval
   * @return requireApproval
   */
  
  @Schema(name = "requireApproval", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("requireApproval")
  public Boolean getRequireApproval() {
    return requireApproval;
  }

  public void setRequireApproval(Boolean requireApproval) {
    this.requireApproval = requireApproval;
  }

  public CreateWorkspaceRequest1 gdprCompliant(Boolean gdprCompliant) {
    this.gdprCompliant = gdprCompliant;
    return this;
  }

  /**
   * Get gdprCompliant
   * @return gdprCompliant
   */
  
  @Schema(name = "gdprCompliant", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("gdprCompliant")
  public Boolean getGdprCompliant() {
    return gdprCompliant;
  }

  public void setGdprCompliant(Boolean gdprCompliant) {
    this.gdprCompliant = gdprCompliant;
  }

  public CreateWorkspaceRequest1 hipaaCompliant(Boolean hipaaCompliant) {
    this.hipaaCompliant = hipaaCompliant;
    return this;
  }

  /**
   * Get hipaaCompliant
   * @return hipaaCompliant
   */
  
  @Schema(name = "hipaaCompliant", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("hipaaCompliant")
  public Boolean getHipaaCompliant() {
    return hipaaCompliant;
  }

  public void setHipaaCompliant(Boolean hipaaCompliant) {
    this.hipaaCompliant = hipaaCompliant;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    CreateWorkspaceRequest1 createWorkspaceRequest1 = (CreateWorkspaceRequest1) o;
    return Objects.equals(this.accountId, createWorkspaceRequest1.accountId) &&
        Objects.equals(this.name, createWorkspaceRequest1.name) &&
        Objects.equals(this.storageQuota, createWorkspaceRequest1.storageQuota) &&
        Objects.equals(this.allowPublicSharing, createWorkspaceRequest1.allowPublicSharing) &&
        Objects.equals(this.requireApproval, createWorkspaceRequest1.requireApproval) &&
        Objects.equals(this.gdprCompliant, createWorkspaceRequest1.gdprCompliant) &&
        Objects.equals(this.hipaaCompliant, createWorkspaceRequest1.hipaaCompliant);
  }

  @Override
  public int hashCode() {
    return Objects.hash(accountId, name, storageQuota, allowPublicSharing, requireApproval, gdprCompliant, hipaaCompliant);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class CreateWorkspaceRequest1 {\n");
    sb.append("    accountId: ").append(toIndentedString(accountId)).append("\n");
    sb.append("    name: ").append(toIndentedString(name)).append("\n");
    sb.append("    storageQuota: ").append(toIndentedString(storageQuota)).append("\n");
    sb.append("    allowPublicSharing: ").append(toIndentedString(allowPublicSharing)).append("\n");
    sb.append("    requireApproval: ").append(toIndentedString(requireApproval)).append("\n");
    sb.append("    gdprCompliant: ").append(toIndentedString(gdprCompliant)).append("\n");
    sb.append("    hipaaCompliant: ").append(toIndentedString(hipaaCompliant)).append("\n");
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

