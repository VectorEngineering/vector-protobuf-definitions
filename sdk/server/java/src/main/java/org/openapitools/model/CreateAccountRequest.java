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
 * CreateAccountRequest
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-01-29T05:44:51.050519-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class CreateAccountRequest {

  private String authPlatformUserId;

  private String orgId;

  private String tenantId;

  private String email;

  public CreateAccountRequest() {
    super();
  }

  /**
   * Constructor with only required parameters
   */
  public CreateAccountRequest(String authPlatformUserId, String orgId, String tenantId, String email) {
    this.authPlatformUserId = authPlatformUserId;
    this.orgId = orgId;
    this.tenantId = tenantId;
    this.email = email;
  }

  public CreateAccountRequest authPlatformUserId(String authPlatformUserId) {
    this.authPlatformUserId = authPlatformUserId;
    return this;
  }

  /**
   * Get authPlatformUserId
   * @return authPlatformUserId
   */
  @NotNull 
  @Schema(name = "authPlatformUserId", requiredMode = Schema.RequiredMode.REQUIRED)
  @JsonProperty("authPlatformUserId")
  public String getAuthPlatformUserId() {
    return authPlatformUserId;
  }

  public void setAuthPlatformUserId(String authPlatformUserId) {
    this.authPlatformUserId = authPlatformUserId;
  }

  public CreateAccountRequest orgId(String orgId) {
    this.orgId = orgId;
    return this;
  }

  /**
   * Get orgId
   * @return orgId
   */
  @NotNull 
  @Schema(name = "orgId", requiredMode = Schema.RequiredMode.REQUIRED)
  @JsonProperty("orgId")
  public String getOrgId() {
    return orgId;
  }

  public void setOrgId(String orgId) {
    this.orgId = orgId;
  }

  public CreateAccountRequest tenantId(String tenantId) {
    this.tenantId = tenantId;
    return this;
  }

  /**
   * Get tenantId
   * @return tenantId
   */
  @NotNull 
  @Schema(name = "tenantId", requiredMode = Schema.RequiredMode.REQUIRED)
  @JsonProperty("tenantId")
  public String getTenantId() {
    return tenantId;
  }

  public void setTenantId(String tenantId) {
    this.tenantId = tenantId;
  }

  public CreateAccountRequest email(String email) {
    this.email = email;
    return this;
  }

  /**
   * Get email
   * @return email
   */
  @NotNull 
  @Schema(name = "email", requiredMode = Schema.RequiredMode.REQUIRED)
  @JsonProperty("email")
  public String getEmail() {
    return email;
  }

  public void setEmail(String email) {
    this.email = email;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    CreateAccountRequest createAccountRequest = (CreateAccountRequest) o;
    return Objects.equals(this.authPlatformUserId, createAccountRequest.authPlatformUserId) &&
        Objects.equals(this.orgId, createAccountRequest.orgId) &&
        Objects.equals(this.tenantId, createAccountRequest.tenantId) &&
        Objects.equals(this.email, createAccountRequest.email);
  }

  @Override
  public int hashCode() {
    return Objects.hash(authPlatformUserId, orgId, tenantId, email);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class CreateAccountRequest {\n");
    sb.append("    authPlatformUserId: ").append(toIndentedString(authPlatformUserId)).append("\n");
    sb.append("    orgId: ").append(toIndentedString(orgId)).append("\n");
    sb.append("    tenantId: ").append(toIndentedString(tenantId)).append("\n");
    sb.append("    email: ").append(toIndentedString(email)).append("\n");
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

