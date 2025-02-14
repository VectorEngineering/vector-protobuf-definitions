package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import java.time.OffsetDateTime;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.model.BackgroundJobStatus;
import org.openapitools.model.Language;
import org.openapitools.model.Lead;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * ScrapingJob represents a Google Maps scraping task. This message defines both the configuration and current state of a scraping operation.  Key components: - Basic metadata (id, name, timestamps) - Job status tracking - Search configuration parameters - Geographic settings - Performance options - Multi-tenant context  Database considerations: - Stored in \&quot;gmaps_jobs\&quot; table - Uses GORM for ORM mapping - Includes foreign key to Account - Supports soft deletes  Usage example: &#x60;&#x60;&#x60;go job :&#x3D; &amp;ScrapingJob{     Name: \&quot;Athens Cafes\&quot;,     Status: BackgroundJobStatus_BACKGROUND_JOB_STATUS_QUEUED,     Keywords: []string{\&quot;cafe\&quot;, \&quot;coffee\&quot;},     Lang: \&quot;el\&quot;,     Zoom: 15,     FastMode: true,     MaxTime: 3600, } &#x60;&#x60;&#x60;
 */

@Schema(name = "ScrapingJob", description = "ScrapingJob represents a Google Maps scraping task. This message defines both the configuration and current state of a scraping operation.  Key components: - Basic metadata (id, name, timestamps) - Job status tracking - Search configuration parameters - Geographic settings - Performance options - Multi-tenant context  Database considerations: - Stored in \"gmaps_jobs\" table - Uses GORM for ORM mapping - Includes foreign key to Account - Supports soft deletes  Usage example: ```go job := &ScrapingJob{     Name: \"Athens Cafes\",     Status: BackgroundJobStatus_BACKGROUND_JOB_STATUS_QUEUED,     Keywords: []string{\"cafe\", \"coffee\"},     Lang: \"el\",     Zoom: 15,     FastMode: true,     MaxTime: 3600, } ```")
@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T08:15:02.658504-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ScrapingJob {

  private String id;

  private Integer priority;

  private String payloadType;

  private byte[] payload;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  private BackgroundJobStatus status = BackgroundJobStatus.UNSPECIFIED;

  private String name;

  @Valid
  private List<String> keywords = new ArrayList<>();

  private Language lang = Language.UNSPECIFIED;

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

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime deletedAt;

  @Valid
  private List<@Valid Lead> leads = new ArrayList<>();

  private String url;

  public ScrapingJob id(String id) {
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

  public ScrapingJob priority(Integer priority) {
    this.priority = priority;
    return this;
  }

  /**
   * Get priority
   * @return priority
   */
  
  @Schema(name = "priority", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("priority")
  public Integer getPriority() {
    return priority;
  }

  public void setPriority(Integer priority) {
    this.priority = priority;
  }

  public ScrapingJob payloadType(String payloadType) {
    this.payloadType = payloadType;
    return this;
  }

  /**
   * Get payloadType
   * @return payloadType
   */
  
  @Schema(name = "payloadType", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("payloadType")
  public String getPayloadType() {
    return payloadType;
  }

  public void setPayloadType(String payloadType) {
    this.payloadType = payloadType;
  }

  public ScrapingJob payload(byte[] payload) {
    this.payload = payload;
    return this;
  }

  /**
   * Get payload
   * @return payload
   */
  
  @Schema(name = "payload", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("payload")
  public byte[] getPayload() {
    return payload;
  }

  public void setPayload(byte[] payload) {
    this.payload = payload;
  }

  public ScrapingJob createdAt(OffsetDateTime createdAt) {
    this.createdAt = createdAt;
    return this;
  }

  /**
   * Get createdAt
   * @return createdAt
   */
  @Valid 
  @Schema(name = "createdAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("createdAt")
  public OffsetDateTime getCreatedAt() {
    return createdAt;
  }

  public void setCreatedAt(OffsetDateTime createdAt) {
    this.createdAt = createdAt;
  }

  public ScrapingJob status(BackgroundJobStatus status) {
    this.status = status;
    return this;
  }

  /**
   * Get status
   * @return status
   */
  @Valid 
  @Schema(name = "status", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("status")
  public BackgroundJobStatus getStatus() {
    return status;
  }

  public void setStatus(BackgroundJobStatus status) {
    this.status = status;
  }

  public ScrapingJob name(String name) {
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

  public ScrapingJob keywords(List<String> keywords) {
    this.keywords = keywords;
    return this;
  }

  public ScrapingJob addKeywordsItem(String keywordsItem) {
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

  public ScrapingJob lang(Language lang) {
    this.lang = lang;
    return this;
  }

  /**
   * Get lang
   * @return lang
   */
  @Valid 
  @Schema(name = "lang", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("lang")
  public Language getLang() {
    return lang;
  }

  public void setLang(Language lang) {
    this.lang = lang;
  }

  public ScrapingJob zoom(Integer zoom) {
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

  public ScrapingJob lat(String lat) {
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

  public ScrapingJob lon(String lon) {
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

  public ScrapingJob fastMode(Boolean fastMode) {
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

  public ScrapingJob radius(Integer radius) {
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

  public ScrapingJob depth(Integer depth) {
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

  public ScrapingJob email(Boolean email) {
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

  public ScrapingJob maxTime(Integer maxTime) {
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

  public ScrapingJob proxies(List<String> proxies) {
    this.proxies = proxies;
    return this;
  }

  public ScrapingJob addProxiesItem(String proxiesItem) {
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

  public ScrapingJob updatedAt(OffsetDateTime updatedAt) {
    this.updatedAt = updatedAt;
    return this;
  }

  /**
   * Get updatedAt
   * @return updatedAt
   */
  @Valid 
  @Schema(name = "updatedAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("updatedAt")
  public OffsetDateTime getUpdatedAt() {
    return updatedAt;
  }

  public void setUpdatedAt(OffsetDateTime updatedAt) {
    this.updatedAt = updatedAt;
  }

  public ScrapingJob deletedAt(OffsetDateTime deletedAt) {
    this.deletedAt = deletedAt;
    return this;
  }

  /**
   * Get deletedAt
   * @return deletedAt
   */
  @Valid 
  @Schema(name = "deletedAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("deletedAt")
  public OffsetDateTime getDeletedAt() {
    return deletedAt;
  }

  public void setDeletedAt(OffsetDateTime deletedAt) {
    this.deletedAt = deletedAt;
  }

  public ScrapingJob leads(List<@Valid Lead> leads) {
    this.leads = leads;
    return this;
  }

  public ScrapingJob addLeadsItem(Lead leadsItem) {
    if (this.leads == null) {
      this.leads = new ArrayList<>();
    }
    this.leads.add(leadsItem);
    return this;
  }

  /**
   * Get leads
   * @return leads
   */
  @Valid 
  @Schema(name = "leads", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("leads")
  public List<@Valid Lead> getLeads() {
    return leads;
  }

  public void setLeads(List<@Valid Lead> leads) {
    this.leads = leads;
  }

  public ScrapingJob url(String url) {
    this.url = url;
    return this;
  }

  /**
   * Get url
   * @return url
   */
  
  @Schema(name = "url", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("url")
  public String getUrl() {
    return url;
  }

  public void setUrl(String url) {
    this.url = url;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    ScrapingJob scrapingJob = (ScrapingJob) o;
    return Objects.equals(this.id, scrapingJob.id) &&
        Objects.equals(this.priority, scrapingJob.priority) &&
        Objects.equals(this.payloadType, scrapingJob.payloadType) &&
        Arrays.equals(this.payload, scrapingJob.payload) &&
        Objects.equals(this.createdAt, scrapingJob.createdAt) &&
        Objects.equals(this.status, scrapingJob.status) &&
        Objects.equals(this.name, scrapingJob.name) &&
        Objects.equals(this.keywords, scrapingJob.keywords) &&
        Objects.equals(this.lang, scrapingJob.lang) &&
        Objects.equals(this.zoom, scrapingJob.zoom) &&
        Objects.equals(this.lat, scrapingJob.lat) &&
        Objects.equals(this.lon, scrapingJob.lon) &&
        Objects.equals(this.fastMode, scrapingJob.fastMode) &&
        Objects.equals(this.radius, scrapingJob.radius) &&
        Objects.equals(this.depth, scrapingJob.depth) &&
        Objects.equals(this.email, scrapingJob.email) &&
        Objects.equals(this.maxTime, scrapingJob.maxTime) &&
        Objects.equals(this.proxies, scrapingJob.proxies) &&
        Objects.equals(this.updatedAt, scrapingJob.updatedAt) &&
        Objects.equals(this.deletedAt, scrapingJob.deletedAt) &&
        Objects.equals(this.leads, scrapingJob.leads) &&
        Objects.equals(this.url, scrapingJob.url);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, priority, payloadType, Arrays.hashCode(payload), createdAt, status, name, keywords, lang, zoom, lat, lon, fastMode, radius, depth, email, maxTime, proxies, updatedAt, deletedAt, leads, url);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ScrapingJob {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    priority: ").append(toIndentedString(priority)).append("\n");
    sb.append("    payloadType: ").append(toIndentedString(payloadType)).append("\n");
    sb.append("    payload: ").append(toIndentedString(payload)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
    sb.append("    status: ").append(toIndentedString(status)).append("\n");
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
    sb.append("    updatedAt: ").append(toIndentedString(updatedAt)).append("\n");
    sb.append("    deletedAt: ").append(toIndentedString(deletedAt)).append("\n");
    sb.append("    leads: ").append(toIndentedString(leads)).append("\n");
    sb.append("    url: ").append(toIndentedString(url)).append("\n");
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

