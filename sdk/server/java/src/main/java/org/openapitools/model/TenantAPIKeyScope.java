package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonValue;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

/**
 * Usage: - Multiple scopes can be assigned to a single key - Scopes follow a hierarchical pattern - More specific scopes take precedence over general ones   - TENANT_API_KEY_SCOPE_UNSPECIFIED: Default state, should not be used explicitly  - TENANT_API_KEY_SCOPE_READ_JOBS: Read-only access scopes  View scraping jobs  - TENANT_API_KEY_SCOPE_READ_LEADS: View scraped leads  - TENANT_API_KEY_SCOPE_READ_WORKFLOWS: View workflow configurations  - TENANT_API_KEY_SCOPE_READ_ANALYTICS: View analytics and metrics  - TENANT_API_KEY_SCOPE_READ_SETTINGS: View tenant settings  - TENANT_API_KEY_SCOPE_WRITE_JOBS: Write access scopes  Create/modify scraping jobs  - TENANT_API_KEY_SCOPE_WRITE_LEADS: Modify lead data  - TENANT_API_KEY_SCOPE_WRITE_WORKFLOWS: Create/modify workflows  - TENANT_API_KEY_SCOPE_WRITE_SETTINGS: Modify tenant settings  - TENANT_API_KEY_SCOPE_DELETE_JOBS: Delete access scopes  Delete scraping jobs  - TENANT_API_KEY_SCOPE_DELETE_LEADS: Delete lead data  - TENANT_API_KEY_SCOPE_DELETE_WORKFLOWS: Delete workflows  - TENANT_API_KEY_SCOPE_EXPORT_DATA: Special access scopes  Export data in various formats  - TENANT_API_KEY_SCOPE_MANAGE_KEYS: Manage other API keys  - TENANT_API_KEY_SCOPE_BILLING_READ: View billing information  - TENANT_API_KEY_SCOPE_BILLING_WRITE: Modify billing settings  - TENANT_API_KEY_SCOPE_ADMIN: Full access scopes  Full administrative access
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T15:17:20.852245-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum TenantAPIKeyScope {
  
  UNSPECIFIED("TENANT_API_KEY_SCOPE_UNSPECIFIED"),
  
  READ_JOBS("TENANT_API_KEY_SCOPE_READ_JOBS"),
  
  READ_LEADS("TENANT_API_KEY_SCOPE_READ_LEADS"),
  
  READ_WORKFLOWS("TENANT_API_KEY_SCOPE_READ_WORKFLOWS"),
  
  READ_ANALYTICS("TENANT_API_KEY_SCOPE_READ_ANALYTICS"),
  
  READ_SETTINGS("TENANT_API_KEY_SCOPE_READ_SETTINGS"),
  
  WRITE_JOBS("TENANT_API_KEY_SCOPE_WRITE_JOBS"),
  
  WRITE_LEADS("TENANT_API_KEY_SCOPE_WRITE_LEADS"),
  
  WRITE_WORKFLOWS("TENANT_API_KEY_SCOPE_WRITE_WORKFLOWS"),
  
  WRITE_SETTINGS("TENANT_API_KEY_SCOPE_WRITE_SETTINGS"),
  
  DELETE_JOBS("TENANT_API_KEY_SCOPE_DELETE_JOBS"),
  
  DELETE_LEADS("TENANT_API_KEY_SCOPE_DELETE_LEADS"),
  
  DELETE_WORKFLOWS("TENANT_API_KEY_SCOPE_DELETE_WORKFLOWS"),
  
  EXPORT_DATA("TENANT_API_KEY_SCOPE_EXPORT_DATA"),
  
  MANAGE_KEYS("TENANT_API_KEY_SCOPE_MANAGE_KEYS"),
  
  BILLING_READ("TENANT_API_KEY_SCOPE_BILLING_READ"),
  
  BILLING_WRITE("TENANT_API_KEY_SCOPE_BILLING_WRITE"),
  
  ADMIN("TENANT_API_KEY_SCOPE_ADMIN");

  private String value;

  TenantAPIKeyScope(String value) {
    this.value = value;
  }

  @JsonValue
  public String getValue() {
    return value;
  }

  @Override
  public String toString() {
    return String.valueOf(value);
  }

  @JsonCreator
  public static TenantAPIKeyScope fromValue(String value) {
    for (TenantAPIKeyScope b : TenantAPIKeyScope.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

