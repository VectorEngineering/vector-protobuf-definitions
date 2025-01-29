package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * CreateScrapingJobRequest
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-01-29T11:21:34.061205-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class CreateScrapingJobRequest {

  private String userId;

  private String orgId;

  private String tenantId;

  private String name;

  @Valid
  private List<String> keywords = new ArrayList<>();

  private String lang;

  private Integer zoom;

  private String lat;

  private String lon;

  private Boolean fastMode;

  private Integer radius;

  private Integer depth;

  private Boolean email;

  private Integer maxTime;

  @Valid
  private List<String> proxies = new ArrayList<>();

  public CreateScrapingJobRequest() {
    super();
  }

  /**
   * Constructor with only required parameters
   */
  public CreateScrapingJobRequest(String userId, String orgId, String tenantId) {
    this.userId = userId;
    this.orgId = orgId;
    this.tenantId = tenantId;
  }

  public CreateScrapingJobRequest userId(String userId) {
    this.userId = userId;
    return this;
  }

  /**
   * Get userId
   * @return userId
   */
  @NotNull 
  @Schema(name = "userId", requiredMode = Schema.RequiredMode.REQUIRED)
  @JsonProperty("userId")
  public String getUserId() {
    return userId;
  }

  public void setUserId(String userId) {
    this.userId = userId;
  }

  public CreateScrapingJobRequest orgId(String orgId) {
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

  public CreateScrapingJobRequest tenantId(String tenantId) {
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

  public CreateScrapingJobRequest name(String name) {
    this.name = name;
    return this;
  }

  /**
   * Get name
   * @return name
   */
  
  @Schema(name = "name", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("name")
  public String getName() {
    return name;
  }

  public void setName(String name) {
    this.name = name;
  }

  public CreateScrapingJobRequest keywords(List<String> keywords) {
    this.keywords = keywords;
    return this;
  }

  public CreateScrapingJobRequest addKeywordsItem(String keywordsItem) {
    if (this.keywords == null) {
      this.keywords = new ArrayList<>();
    }
    this.keywords.add(keywordsItem);
    return this;
  }

  /**
   * Get keywords
   * @return keywords
   */
  
  @Schema(name = "keywords", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("keywords")
  public List<String> getKeywords() {
    return keywords;
  }

  public void setKeywords(List<String> keywords) {
    this.keywords = keywords;
  }

  public CreateScrapingJobRequest lang(String lang) {
    this.lang = lang;
    return this;
  }

  /**
   * Get lang
   * @return lang
   */
  
  @Schema(name = "lang", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("lang")
  public String getLang() {
    return lang;
  }

  public void setLang(String lang) {
    this.lang = lang;
  }

  public CreateScrapingJobRequest zoom(Integer zoom) {
    this.zoom = zoom;
    return this;
  }

  /**
   * Get zoom
   * @return zoom
   */
  
  @Schema(name = "zoom", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("zoom")
  public Integer getZoom() {
    return zoom;
  }

  public void setZoom(Integer zoom) {
    this.zoom = zoom;
  }

  public CreateScrapingJobRequest lat(String lat) {
    this.lat = lat;
    return this;
  }

  /**
   * Get lat
   * @return lat
   */
  
  @Schema(name = "lat", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("lat")
  public String getLat() {
    return lat;
  }

  public void setLat(String lat) {
    this.lat = lat;
  }

  public CreateScrapingJobRequest lon(String lon) {
    this.lon = lon;
    return this;
  }

  /**
   * Get lon
   * @return lon
   */
  
  @Schema(name = "lon", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("lon")
  public String getLon() {
    return lon;
  }

  public void setLon(String lon) {
    this.lon = lon;
  }

  public CreateScrapingJobRequest fastMode(Boolean fastMode) {
    this.fastMode = fastMode;
    return this;
  }

  /**
   * Get fastMode
   * @return fastMode
   */
  
  @Schema(name = "fastMode", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("fastMode")
  public Boolean getFastMode() {
    return fastMode;
  }

  public void setFastMode(Boolean fastMode) {
    this.fastMode = fastMode;
  }

  public CreateScrapingJobRequest radius(Integer radius) {
    this.radius = radius;
    return this;
  }

  /**
   * Get radius
   * @return radius
   */
  
  @Schema(name = "radius", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("radius")
  public Integer getRadius() {
    return radius;
  }

  public void setRadius(Integer radius) {
    this.radius = radius;
  }

  public CreateScrapingJobRequest depth(Integer depth) {
    this.depth = depth;
    return this;
  }

  /**
   * Get depth
   * @return depth
   */
  
  @Schema(name = "depth", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("depth")
  public Integer getDepth() {
    return depth;
  }

  public void setDepth(Integer depth) {
    this.depth = depth;
  }

  public CreateScrapingJobRequest email(Boolean email) {
    this.email = email;
    return this;
  }

  /**
   * Get email
   * @return email
   */
  
  @Schema(name = "email", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("email")
  public Boolean getEmail() {
    return email;
  }

  public void setEmail(Boolean email) {
    this.email = email;
  }

  public CreateScrapingJobRequest maxTime(Integer maxTime) {
    this.maxTime = maxTime;
    return this;
  }

  /**
   * Get maxTime
   * @return maxTime
   */
  
  @Schema(name = "maxTime", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("maxTime")
  public Integer getMaxTime() {
    return maxTime;
  }

  public void setMaxTime(Integer maxTime) {
    this.maxTime = maxTime;
  }

  public CreateScrapingJobRequest proxies(List<String> proxies) {
    this.proxies = proxies;
    return this;
  }

  public CreateScrapingJobRequest addProxiesItem(String proxiesItem) {
    if (this.proxies == null) {
      this.proxies = new ArrayList<>();
    }
    this.proxies.add(proxiesItem);
    return this;
  }

  /**
   * Get proxies
   * @return proxies
   */
  
  @Schema(name = "proxies", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("proxies")
  public List<String> getProxies() {
    return proxies;
  }

  public void setProxies(List<String> proxies) {
    this.proxies = proxies;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    CreateScrapingJobRequest createScrapingJobRequest = (CreateScrapingJobRequest) o;
    return Objects.equals(this.userId, createScrapingJobRequest.userId) &&
        Objects.equals(this.orgId, createScrapingJobRequest.orgId) &&
        Objects.equals(this.tenantId, createScrapingJobRequest.tenantId) &&
        Objects.equals(this.name, createScrapingJobRequest.name) &&
        Objects.equals(this.keywords, createScrapingJobRequest.keywords) &&
        Objects.equals(this.lang, createScrapingJobRequest.lang) &&
        Objects.equals(this.zoom, createScrapingJobRequest.zoom) &&
        Objects.equals(this.lat, createScrapingJobRequest.lat) &&
        Objects.equals(this.lon, createScrapingJobRequest.lon) &&
        Objects.equals(this.fastMode, createScrapingJobRequest.fastMode) &&
        Objects.equals(this.radius, createScrapingJobRequest.radius) &&
        Objects.equals(this.depth, createScrapingJobRequest.depth) &&
        Objects.equals(this.email, createScrapingJobRequest.email) &&
        Objects.equals(this.maxTime, createScrapingJobRequest.maxTime) &&
        Objects.equals(this.proxies, createScrapingJobRequest.proxies);
  }

  @Override
  public int hashCode() {
    return Objects.hash(userId, orgId, tenantId, name, keywords, lang, zoom, lat, lon, fastMode, radius, depth, email, maxTime, proxies);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class CreateScrapingJobRequest {\n");
    sb.append("    userId: ").append(toIndentedString(userId)).append("\n");
    sb.append("    orgId: ").append(toIndentedString(orgId)).append("\n");
    sb.append("    tenantId: ").append(toIndentedString(tenantId)).append("\n");
    sb.append("    name: ").append(toIndentedString(name)).append("\n");
    sb.append("    keywords: ").append(toIndentedString(keywords)).append("\n");
    sb.append("    lang: ").append(toIndentedString(lang)).append("\n");
    sb.append("    zoom: ").append(toIndentedString(zoom)).append("\n");
    sb.append("    lat: ").append(toIndentedString(lat)).append("\n");
    sb.append("    lon: ").append(toIndentedString(lon)).append("\n");
    sb.append("    fastMode: ").append(toIndentedString(fastMode)).append("\n");
    sb.append("    radius: ").append(toIndentedString(radius)).append("\n");
    sb.append("    depth: ").append(toIndentedString(depth)).append("\n");
    sb.append("    email: ").append(toIndentedString(email)).append("\n");
    sb.append("    maxTime: ").append(toIndentedString(maxTime)).append("\n");
    sb.append("    proxies: ").append(toIndentedString(proxies)).append("\n");
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

