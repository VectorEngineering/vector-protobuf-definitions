package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.ActivityMetrics;
import org.openapitools.model.ComplianceMetrics;
import org.openapitools.model.UserActivity;
import org.openapitools.model.WorkspaceActivity;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * GetWorkspaceAnalyticsResponse1
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T12:22:33.767757-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class GetWorkspaceAnalyticsResponse1 {

  private ActivityMetrics activity;

  @Valid
  private List<@Valid UserActivity> userActivities = new ArrayList<>();

  private ComplianceMetrics compliance;

  @Valid
  private List<@Valid WorkspaceActivity> recentActivities = new ArrayList<>();

  public GetWorkspaceAnalyticsResponse1 activity(ActivityMetrics activity) {
    this.activity = activity;
    return this;
  }

  /**
   * Get activity
   * @return activity
   */
  @Valid 
  @Schema(name = "activity", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("activity")
  public ActivityMetrics getActivity() {
    return activity;
  }

  public void setActivity(ActivityMetrics activity) {
    this.activity = activity;
  }

  public GetWorkspaceAnalyticsResponse1 userActivities(List<@Valid UserActivity> userActivities) {
    this.userActivities = userActivities;
    return this;
  }

  public GetWorkspaceAnalyticsResponse1 addUserActivitiesItem(UserActivity userActivitiesItem) {
    if (this.userActivities == null) {
      this.userActivities = new ArrayList<>();
    }
    this.userActivities.add(userActivitiesItem);
    return this;
  }

  /**
   * Get userActivities
   * @return userActivities
   */
  @Valid 
  @Schema(name = "userActivities", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("userActivities")
  public List<@Valid UserActivity> getUserActivities() {
    return userActivities;
  }

  public void setUserActivities(List<@Valid UserActivity> userActivities) {
    this.userActivities = userActivities;
  }

  public GetWorkspaceAnalyticsResponse1 compliance(ComplianceMetrics compliance) {
    this.compliance = compliance;
    return this;
  }

  /**
   * Get compliance
   * @return compliance
   */
  @Valid 
  @Schema(name = "compliance", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("compliance")
  public ComplianceMetrics getCompliance() {
    return compliance;
  }

  public void setCompliance(ComplianceMetrics compliance) {
    this.compliance = compliance;
  }

  public GetWorkspaceAnalyticsResponse1 recentActivities(List<@Valid WorkspaceActivity> recentActivities) {
    this.recentActivities = recentActivities;
    return this;
  }

  public GetWorkspaceAnalyticsResponse1 addRecentActivitiesItem(WorkspaceActivity recentActivitiesItem) {
    if (this.recentActivities == null) {
      this.recentActivities = new ArrayList<>();
    }
    this.recentActivities.add(recentActivitiesItem);
    return this;
  }

  /**
   * Get recentActivities
   * @return recentActivities
   */
  @Valid 
  @Schema(name = "recentActivities", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("recentActivities")
  public List<@Valid WorkspaceActivity> getRecentActivities() {
    return recentActivities;
  }

  public void setRecentActivities(List<@Valid WorkspaceActivity> recentActivities) {
    this.recentActivities = recentActivities;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    GetWorkspaceAnalyticsResponse1 getWorkspaceAnalyticsResponse1 = (GetWorkspaceAnalyticsResponse1) o;
    return Objects.equals(this.activity, getWorkspaceAnalyticsResponse1.activity) &&
        Objects.equals(this.userActivities, getWorkspaceAnalyticsResponse1.userActivities) &&
        Objects.equals(this.compliance, getWorkspaceAnalyticsResponse1.compliance) &&
        Objects.equals(this.recentActivities, getWorkspaceAnalyticsResponse1.recentActivities);
  }

  @Override
  public int hashCode() {
    return Objects.hash(activity, userActivities, compliance, recentActivities);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class GetWorkspaceAnalyticsResponse1 {\n");
    sb.append("    activity: ").append(toIndentedString(activity)).append("\n");
    sb.append("    userActivities: ").append(toIndentedString(userActivities)).append("\n");
    sb.append("    compliance: ").append(toIndentedString(compliance)).append("\n");
    sb.append("    recentActivities: ").append(toIndentedString(recentActivities)).append("\n");
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

