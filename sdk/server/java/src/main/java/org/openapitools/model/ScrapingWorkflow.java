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
import org.openapitools.model.OutputFormat;
import org.openapitools.model.ScrapingJob;
import org.openapitools.model.WorkflowStatus;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * ScrapingWorkflow
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-07T14:39:19.806517-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class ScrapingWorkflow {

  private String id;

  private String name;

  private String cronExpression;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime nextRunTime;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime lastRunTime;

  private WorkflowStatus status = WorkflowStatus.UNSPECIFIED;

  private Integer retryCount;

  private Integer maxRetries;

  private String alertEmails;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime deletedAt;

  @Valid
  private List<@Valid ScrapingJob> jobs = new ArrayList<>();

  private Float geoFencingRadius;

  private Double geoFencingLat;

  private Double geoFencingLon;

  private Integer geoFencingZoomMin;

  private Integer geoFencingZoomMax;

  private Boolean includeReviews;

  private Boolean includePhotos;

  private Boolean includeBusinessHours;

  private Integer maxReviewsPerBusiness;

  private OutputFormat outputFormat = OutputFormat.UNSPECIFIED;

  private String outputDestination;

  private String dataRetention;

  private Boolean anonymizePii;

  private String notificationSlackChannel;

  private String notificationEmailGroup;

  private Boolean notificationNotifyOnStart;

  private Boolean notificationNotifyOnComplete;

  private Boolean notificationNotifyOnFailure;

  @Valid
  private List<String> contentFilterAllowedCountries = new ArrayList<>();

  @Valid
  private List<String> contentFilterExcludedTypes = new ArrayList<>();

  private Float contentFilterMinimumRating;

  private Integer contentFilterMinimumReviews;

  private Integer qosMaxConcurrentRequests;

  private Integer qosMaxRetries;

  private String qosRequestTimeout;

  private Boolean qosEnableJavascript;

  private Boolean respectRobotsTxt;

  private Boolean acceptTermsOfService;

  private String userAgent;

  public ScrapingWorkflow id(String id) {
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

  public ScrapingWorkflow name(String name) {
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

  public ScrapingWorkflow cronExpression(String cronExpression) {
    this.cronExpression = cronExpression;
    return this;
  }

  /**
   * Get cronExpression
   * @return cronExpression
   */
  
  @Schema(name = "cronExpression", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("cronExpression")
  public String getCronExpression() {
    return cronExpression;
  }

  public void setCronExpression(String cronExpression) {
    this.cronExpression = cronExpression;
  }

  public ScrapingWorkflow nextRunTime(OffsetDateTime nextRunTime) {
    this.nextRunTime = nextRunTime;
    return this;
  }

  /**
   * Get nextRunTime
   * @return nextRunTime
   */
  @Valid 
  @Schema(name = "nextRunTime", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("nextRunTime")
  public OffsetDateTime getNextRunTime() {
    return nextRunTime;
  }

  public void setNextRunTime(OffsetDateTime nextRunTime) {
    this.nextRunTime = nextRunTime;
  }

  public ScrapingWorkflow lastRunTime(OffsetDateTime lastRunTime) {
    this.lastRunTime = lastRunTime;
    return this;
  }

  /**
   * Get lastRunTime
   * @return lastRunTime
   */
  @Valid 
  @Schema(name = "lastRunTime", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("lastRunTime")
  public OffsetDateTime getLastRunTime() {
    return lastRunTime;
  }

  public void setLastRunTime(OffsetDateTime lastRunTime) {
    this.lastRunTime = lastRunTime;
  }

  public ScrapingWorkflow status(WorkflowStatus status) {
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
  public WorkflowStatus getStatus() {
    return status;
  }

  public void setStatus(WorkflowStatus status) {
    this.status = status;
  }

  public ScrapingWorkflow retryCount(Integer retryCount) {
    this.retryCount = retryCount;
    return this;
  }

  /**
   * Get retryCount
   * @return retryCount
   */
  
  @Schema(name = "retryCount", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("retryCount")
  public Integer getRetryCount() {
    return retryCount;
  }

  public void setRetryCount(Integer retryCount) {
    this.retryCount = retryCount;
  }

  public ScrapingWorkflow maxRetries(Integer maxRetries) {
    this.maxRetries = maxRetries;
    return this;
  }

  /**
   * Get maxRetries
   * @return maxRetries
   */
  
  @Schema(name = "maxRetries", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("maxRetries")
  public Integer getMaxRetries() {
    return maxRetries;
  }

  public void setMaxRetries(Integer maxRetries) {
    this.maxRetries = maxRetries;
  }

  public ScrapingWorkflow alertEmails(String alertEmails) {
    this.alertEmails = alertEmails;
    return this;
  }

  /**
   * Get alertEmails
   * @return alertEmails
   */
  
  @Schema(name = "alertEmails", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("alertEmails")
  public String getAlertEmails() {
    return alertEmails;
  }

  public void setAlertEmails(String alertEmails) {
    this.alertEmails = alertEmails;
  }

  public ScrapingWorkflow createdAt(OffsetDateTime createdAt) {
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

  public ScrapingWorkflow updatedAt(OffsetDateTime updatedAt) {
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

  public ScrapingWorkflow deletedAt(OffsetDateTime deletedAt) {
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

  public ScrapingWorkflow jobs(List<@Valid ScrapingJob> jobs) {
    this.jobs = jobs;
    return this;
  }

  public ScrapingWorkflow addJobsItem(ScrapingJob jobsItem) {
    if (this.jobs == null) {
      this.jobs = new ArrayList<>();
    }
    this.jobs.add(jobsItem);
    return this;
  }

  /**
   * Get jobs
   * @return jobs
   */
  @Valid 
  @Schema(name = "jobs", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("jobs")
  public List<@Valid ScrapingJob> getJobs() {
    return jobs;
  }

  public void setJobs(List<@Valid ScrapingJob> jobs) {
    this.jobs = jobs;
  }

  public ScrapingWorkflow geoFencingRadius(Float geoFencingRadius) {
    this.geoFencingRadius = geoFencingRadius;
    return this;
  }

  /**
   * Get geoFencingRadius
   * @return geoFencingRadius
   */
  
  @Schema(name = "geoFencingRadius", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("geoFencingRadius")
  public Float getGeoFencingRadius() {
    return geoFencingRadius;
  }

  public void setGeoFencingRadius(Float geoFencingRadius) {
    this.geoFencingRadius = geoFencingRadius;
  }

  public ScrapingWorkflow geoFencingLat(Double geoFencingLat) {
    this.geoFencingLat = geoFencingLat;
    return this;
  }

  /**
   * Get geoFencingLat
   * @return geoFencingLat
   */
  
  @Schema(name = "geoFencingLat", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("geoFencingLat")
  public Double getGeoFencingLat() {
    return geoFencingLat;
  }

  public void setGeoFencingLat(Double geoFencingLat) {
    this.geoFencingLat = geoFencingLat;
  }

  public ScrapingWorkflow geoFencingLon(Double geoFencingLon) {
    this.geoFencingLon = geoFencingLon;
    return this;
  }

  /**
   * Get geoFencingLon
   * @return geoFencingLon
   */
  
  @Schema(name = "geoFencingLon", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("geoFencingLon")
  public Double getGeoFencingLon() {
    return geoFencingLon;
  }

  public void setGeoFencingLon(Double geoFencingLon) {
    this.geoFencingLon = geoFencingLon;
  }

  public ScrapingWorkflow geoFencingZoomMin(Integer geoFencingZoomMin) {
    this.geoFencingZoomMin = geoFencingZoomMin;
    return this;
  }

  /**
   * Get geoFencingZoomMin
   * @return geoFencingZoomMin
   */
  
  @Schema(name = "geoFencingZoomMin", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("geoFencingZoomMin")
  public Integer getGeoFencingZoomMin() {
    return geoFencingZoomMin;
  }

  public void setGeoFencingZoomMin(Integer geoFencingZoomMin) {
    this.geoFencingZoomMin = geoFencingZoomMin;
  }

  public ScrapingWorkflow geoFencingZoomMax(Integer geoFencingZoomMax) {
    this.geoFencingZoomMax = geoFencingZoomMax;
    return this;
  }

  /**
   * Get geoFencingZoomMax
   * @return geoFencingZoomMax
   */
  
  @Schema(name = "geoFencingZoomMax", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("geoFencingZoomMax")
  public Integer getGeoFencingZoomMax() {
    return geoFencingZoomMax;
  }

  public void setGeoFencingZoomMax(Integer geoFencingZoomMax) {
    this.geoFencingZoomMax = geoFencingZoomMax;
  }

  public ScrapingWorkflow includeReviews(Boolean includeReviews) {
    this.includeReviews = includeReviews;
    return this;
  }

  /**
   * Get includeReviews
   * @return includeReviews
   */
  
  @Schema(name = "includeReviews", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("includeReviews")
  public Boolean getIncludeReviews() {
    return includeReviews;
  }

  public void setIncludeReviews(Boolean includeReviews) {
    this.includeReviews = includeReviews;
  }

  public ScrapingWorkflow includePhotos(Boolean includePhotos) {
    this.includePhotos = includePhotos;
    return this;
  }

  /**
   * Get includePhotos
   * @return includePhotos
   */
  
  @Schema(name = "includePhotos", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("includePhotos")
  public Boolean getIncludePhotos() {
    return includePhotos;
  }

  public void setIncludePhotos(Boolean includePhotos) {
    this.includePhotos = includePhotos;
  }

  public ScrapingWorkflow includeBusinessHours(Boolean includeBusinessHours) {
    this.includeBusinessHours = includeBusinessHours;
    return this;
  }

  /**
   * Get includeBusinessHours
   * @return includeBusinessHours
   */
  
  @Schema(name = "includeBusinessHours", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("includeBusinessHours")
  public Boolean getIncludeBusinessHours() {
    return includeBusinessHours;
  }

  public void setIncludeBusinessHours(Boolean includeBusinessHours) {
    this.includeBusinessHours = includeBusinessHours;
  }

  public ScrapingWorkflow maxReviewsPerBusiness(Integer maxReviewsPerBusiness) {
    this.maxReviewsPerBusiness = maxReviewsPerBusiness;
    return this;
  }

  /**
   * Get maxReviewsPerBusiness
   * @return maxReviewsPerBusiness
   */
  
  @Schema(name = "maxReviewsPerBusiness", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("maxReviewsPerBusiness")
  public Integer getMaxReviewsPerBusiness() {
    return maxReviewsPerBusiness;
  }

  public void setMaxReviewsPerBusiness(Integer maxReviewsPerBusiness) {
    this.maxReviewsPerBusiness = maxReviewsPerBusiness;
  }

  public ScrapingWorkflow outputFormat(OutputFormat outputFormat) {
    this.outputFormat = outputFormat;
    return this;
  }

  /**
   * Get outputFormat
   * @return outputFormat
   */
  @Valid 
  @Schema(name = "outputFormat", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("outputFormat")
  public OutputFormat getOutputFormat() {
    return outputFormat;
  }

  public void setOutputFormat(OutputFormat outputFormat) {
    this.outputFormat = outputFormat;
  }

  public ScrapingWorkflow outputDestination(String outputDestination) {
    this.outputDestination = outputDestination;
    return this;
  }

  /**
   * Get outputDestination
   * @return outputDestination
   */
  
  @Schema(name = "outputDestination", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("outputDestination")
  public String getOutputDestination() {
    return outputDestination;
  }

  public void setOutputDestination(String outputDestination) {
    this.outputDestination = outputDestination;
  }

  public ScrapingWorkflow dataRetention(String dataRetention) {
    this.dataRetention = dataRetention;
    return this;
  }

  /**
   * Get dataRetention
   * @return dataRetention
   */
  
  @Schema(name = "dataRetention", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("dataRetention")
  public String getDataRetention() {
    return dataRetention;
  }

  public void setDataRetention(String dataRetention) {
    this.dataRetention = dataRetention;
  }

  public ScrapingWorkflow anonymizePii(Boolean anonymizePii) {
    this.anonymizePii = anonymizePii;
    return this;
  }

  /**
   * Get anonymizePii
   * @return anonymizePii
   */
  
  @Schema(name = "anonymizePii", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("anonymizePii")
  public Boolean getAnonymizePii() {
    return anonymizePii;
  }

  public void setAnonymizePii(Boolean anonymizePii) {
    this.anonymizePii = anonymizePii;
  }

  public ScrapingWorkflow notificationSlackChannel(String notificationSlackChannel) {
    this.notificationSlackChannel = notificationSlackChannel;
    return this;
  }

  /**
   * Get notificationSlackChannel
   * @return notificationSlackChannel
   */
  
  @Schema(name = "notificationSlackChannel", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("notificationSlackChannel")
  public String getNotificationSlackChannel() {
    return notificationSlackChannel;
  }

  public void setNotificationSlackChannel(String notificationSlackChannel) {
    this.notificationSlackChannel = notificationSlackChannel;
  }

  public ScrapingWorkflow notificationEmailGroup(String notificationEmailGroup) {
    this.notificationEmailGroup = notificationEmailGroup;
    return this;
  }

  /**
   * Get notificationEmailGroup
   * @return notificationEmailGroup
   */
  
  @Schema(name = "notificationEmailGroup", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("notificationEmailGroup")
  public String getNotificationEmailGroup() {
    return notificationEmailGroup;
  }

  public void setNotificationEmailGroup(String notificationEmailGroup) {
    this.notificationEmailGroup = notificationEmailGroup;
  }

  public ScrapingWorkflow notificationNotifyOnStart(Boolean notificationNotifyOnStart) {
    this.notificationNotifyOnStart = notificationNotifyOnStart;
    return this;
  }

  /**
   * Get notificationNotifyOnStart
   * @return notificationNotifyOnStart
   */
  
  @Schema(name = "notificationNotifyOnStart", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("notificationNotifyOnStart")
  public Boolean getNotificationNotifyOnStart() {
    return notificationNotifyOnStart;
  }

  public void setNotificationNotifyOnStart(Boolean notificationNotifyOnStart) {
    this.notificationNotifyOnStart = notificationNotifyOnStart;
  }

  public ScrapingWorkflow notificationNotifyOnComplete(Boolean notificationNotifyOnComplete) {
    this.notificationNotifyOnComplete = notificationNotifyOnComplete;
    return this;
  }

  /**
   * Get notificationNotifyOnComplete
   * @return notificationNotifyOnComplete
   */
  
  @Schema(name = "notificationNotifyOnComplete", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("notificationNotifyOnComplete")
  public Boolean getNotificationNotifyOnComplete() {
    return notificationNotifyOnComplete;
  }

  public void setNotificationNotifyOnComplete(Boolean notificationNotifyOnComplete) {
    this.notificationNotifyOnComplete = notificationNotifyOnComplete;
  }

  public ScrapingWorkflow notificationNotifyOnFailure(Boolean notificationNotifyOnFailure) {
    this.notificationNotifyOnFailure = notificationNotifyOnFailure;
    return this;
  }

  /**
   * Get notificationNotifyOnFailure
   * @return notificationNotifyOnFailure
   */
  
  @Schema(name = "notificationNotifyOnFailure", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("notificationNotifyOnFailure")
  public Boolean getNotificationNotifyOnFailure() {
    return notificationNotifyOnFailure;
  }

  public void setNotificationNotifyOnFailure(Boolean notificationNotifyOnFailure) {
    this.notificationNotifyOnFailure = notificationNotifyOnFailure;
  }

  public ScrapingWorkflow contentFilterAllowedCountries(List<String> contentFilterAllowedCountries) {
    this.contentFilterAllowedCountries = contentFilterAllowedCountries;
    return this;
  }

  public ScrapingWorkflow addContentFilterAllowedCountriesItem(String contentFilterAllowedCountriesItem) {
    if (this.contentFilterAllowedCountries == null) {
      this.contentFilterAllowedCountries = new ArrayList<>();
    }
    this.contentFilterAllowedCountries.add(contentFilterAllowedCountriesItem);
    return this;
  }

  /**
   * Get contentFilterAllowedCountries
   * @return contentFilterAllowedCountries
   */
  
  @Schema(name = "contentFilterAllowedCountries", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("contentFilterAllowedCountries")
  public List<String> getContentFilterAllowedCountries() {
    return contentFilterAllowedCountries;
  }

  public void setContentFilterAllowedCountries(List<String> contentFilterAllowedCountries) {
    this.contentFilterAllowedCountries = contentFilterAllowedCountries;
  }

  public ScrapingWorkflow contentFilterExcludedTypes(List<String> contentFilterExcludedTypes) {
    this.contentFilterExcludedTypes = contentFilterExcludedTypes;
    return this;
  }

  public ScrapingWorkflow addContentFilterExcludedTypesItem(String contentFilterExcludedTypesItem) {
    if (this.contentFilterExcludedTypes == null) {
      this.contentFilterExcludedTypes = new ArrayList<>();
    }
    this.contentFilterExcludedTypes.add(contentFilterExcludedTypesItem);
    return this;
  }

  /**
   * Get contentFilterExcludedTypes
   * @return contentFilterExcludedTypes
   */
  
  @Schema(name = "contentFilterExcludedTypes", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("contentFilterExcludedTypes")
  public List<String> getContentFilterExcludedTypes() {
    return contentFilterExcludedTypes;
  }

  public void setContentFilterExcludedTypes(List<String> contentFilterExcludedTypes) {
    this.contentFilterExcludedTypes = contentFilterExcludedTypes;
  }

  public ScrapingWorkflow contentFilterMinimumRating(Float contentFilterMinimumRating) {
    this.contentFilterMinimumRating = contentFilterMinimumRating;
    return this;
  }

  /**
   * Get contentFilterMinimumRating
   * @return contentFilterMinimumRating
   */
  
  @Schema(name = "contentFilterMinimumRating", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("contentFilterMinimumRating")
  public Float getContentFilterMinimumRating() {
    return contentFilterMinimumRating;
  }

  public void setContentFilterMinimumRating(Float contentFilterMinimumRating) {
    this.contentFilterMinimumRating = contentFilterMinimumRating;
  }

  public ScrapingWorkflow contentFilterMinimumReviews(Integer contentFilterMinimumReviews) {
    this.contentFilterMinimumReviews = contentFilterMinimumReviews;
    return this;
  }

  /**
   * Get contentFilterMinimumReviews
   * @return contentFilterMinimumReviews
   */
  
  @Schema(name = "contentFilterMinimumReviews", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("contentFilterMinimumReviews")
  public Integer getContentFilterMinimumReviews() {
    return contentFilterMinimumReviews;
  }

  public void setContentFilterMinimumReviews(Integer contentFilterMinimumReviews) {
    this.contentFilterMinimumReviews = contentFilterMinimumReviews;
  }

  public ScrapingWorkflow qosMaxConcurrentRequests(Integer qosMaxConcurrentRequests) {
    this.qosMaxConcurrentRequests = qosMaxConcurrentRequests;
    return this;
  }

  /**
   * Get qosMaxConcurrentRequests
   * @return qosMaxConcurrentRequests
   */
  
  @Schema(name = "qosMaxConcurrentRequests", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("qosMaxConcurrentRequests")
  public Integer getQosMaxConcurrentRequests() {
    return qosMaxConcurrentRequests;
  }

  public void setQosMaxConcurrentRequests(Integer qosMaxConcurrentRequests) {
    this.qosMaxConcurrentRequests = qosMaxConcurrentRequests;
  }

  public ScrapingWorkflow qosMaxRetries(Integer qosMaxRetries) {
    this.qosMaxRetries = qosMaxRetries;
    return this;
  }

  /**
   * Get qosMaxRetries
   * @return qosMaxRetries
   */
  
  @Schema(name = "qosMaxRetries", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("qosMaxRetries")
  public Integer getQosMaxRetries() {
    return qosMaxRetries;
  }

  public void setQosMaxRetries(Integer qosMaxRetries) {
    this.qosMaxRetries = qosMaxRetries;
  }

  public ScrapingWorkflow qosRequestTimeout(String qosRequestTimeout) {
    this.qosRequestTimeout = qosRequestTimeout;
    return this;
  }

  /**
   * Get qosRequestTimeout
   * @return qosRequestTimeout
   */
  
  @Schema(name = "qosRequestTimeout", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("qosRequestTimeout")
  public String getQosRequestTimeout() {
    return qosRequestTimeout;
  }

  public void setQosRequestTimeout(String qosRequestTimeout) {
    this.qosRequestTimeout = qosRequestTimeout;
  }

  public ScrapingWorkflow qosEnableJavascript(Boolean qosEnableJavascript) {
    this.qosEnableJavascript = qosEnableJavascript;
    return this;
  }

  /**
   * Get qosEnableJavascript
   * @return qosEnableJavascript
   */
  
  @Schema(name = "qosEnableJavascript", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("qosEnableJavascript")
  public Boolean getQosEnableJavascript() {
    return qosEnableJavascript;
  }

  public void setQosEnableJavascript(Boolean qosEnableJavascript) {
    this.qosEnableJavascript = qosEnableJavascript;
  }

  public ScrapingWorkflow respectRobotsTxt(Boolean respectRobotsTxt) {
    this.respectRobotsTxt = respectRobotsTxt;
    return this;
  }

  /**
   * Get respectRobotsTxt
   * @return respectRobotsTxt
   */
  
  @Schema(name = "respectRobotsTxt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("respectRobotsTxt")
  public Boolean getRespectRobotsTxt() {
    return respectRobotsTxt;
  }

  public void setRespectRobotsTxt(Boolean respectRobotsTxt) {
    this.respectRobotsTxt = respectRobotsTxt;
  }

  public ScrapingWorkflow acceptTermsOfService(Boolean acceptTermsOfService) {
    this.acceptTermsOfService = acceptTermsOfService;
    return this;
  }

  /**
   * Get acceptTermsOfService
   * @return acceptTermsOfService
   */
  
  @Schema(name = "acceptTermsOfService", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("acceptTermsOfService")
  public Boolean getAcceptTermsOfService() {
    return acceptTermsOfService;
  }

  public void setAcceptTermsOfService(Boolean acceptTermsOfService) {
    this.acceptTermsOfService = acceptTermsOfService;
  }

  public ScrapingWorkflow userAgent(String userAgent) {
    this.userAgent = userAgent;
    return this;
  }

  /**
   * Get userAgent
   * @return userAgent
   */
  
  @Schema(name = "userAgent", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("userAgent")
  public String getUserAgent() {
    return userAgent;
  }

  public void setUserAgent(String userAgent) {
    this.userAgent = userAgent;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    ScrapingWorkflow scrapingWorkflow = (ScrapingWorkflow) o;
    return Objects.equals(this.id, scrapingWorkflow.id) &&
        Objects.equals(this.name, scrapingWorkflow.name) &&
        Objects.equals(this.cronExpression, scrapingWorkflow.cronExpression) &&
        Objects.equals(this.nextRunTime, scrapingWorkflow.nextRunTime) &&
        Objects.equals(this.lastRunTime, scrapingWorkflow.lastRunTime) &&
        Objects.equals(this.status, scrapingWorkflow.status) &&
        Objects.equals(this.retryCount, scrapingWorkflow.retryCount) &&
        Objects.equals(this.maxRetries, scrapingWorkflow.maxRetries) &&
        Objects.equals(this.alertEmails, scrapingWorkflow.alertEmails) &&
        Objects.equals(this.createdAt, scrapingWorkflow.createdAt) &&
        Objects.equals(this.updatedAt, scrapingWorkflow.updatedAt) &&
        Objects.equals(this.deletedAt, scrapingWorkflow.deletedAt) &&
        Objects.equals(this.jobs, scrapingWorkflow.jobs) &&
        Objects.equals(this.geoFencingRadius, scrapingWorkflow.geoFencingRadius) &&
        Objects.equals(this.geoFencingLat, scrapingWorkflow.geoFencingLat) &&
        Objects.equals(this.geoFencingLon, scrapingWorkflow.geoFencingLon) &&
        Objects.equals(this.geoFencingZoomMin, scrapingWorkflow.geoFencingZoomMin) &&
        Objects.equals(this.geoFencingZoomMax, scrapingWorkflow.geoFencingZoomMax) &&
        Objects.equals(this.includeReviews, scrapingWorkflow.includeReviews) &&
        Objects.equals(this.includePhotos, scrapingWorkflow.includePhotos) &&
        Objects.equals(this.includeBusinessHours, scrapingWorkflow.includeBusinessHours) &&
        Objects.equals(this.maxReviewsPerBusiness, scrapingWorkflow.maxReviewsPerBusiness) &&
        Objects.equals(this.outputFormat, scrapingWorkflow.outputFormat) &&
        Objects.equals(this.outputDestination, scrapingWorkflow.outputDestination) &&
        Objects.equals(this.dataRetention, scrapingWorkflow.dataRetention) &&
        Objects.equals(this.anonymizePii, scrapingWorkflow.anonymizePii) &&
        Objects.equals(this.notificationSlackChannel, scrapingWorkflow.notificationSlackChannel) &&
        Objects.equals(this.notificationEmailGroup, scrapingWorkflow.notificationEmailGroup) &&
        Objects.equals(this.notificationNotifyOnStart, scrapingWorkflow.notificationNotifyOnStart) &&
        Objects.equals(this.notificationNotifyOnComplete, scrapingWorkflow.notificationNotifyOnComplete) &&
        Objects.equals(this.notificationNotifyOnFailure, scrapingWorkflow.notificationNotifyOnFailure) &&
        Objects.equals(this.contentFilterAllowedCountries, scrapingWorkflow.contentFilterAllowedCountries) &&
        Objects.equals(this.contentFilterExcludedTypes, scrapingWorkflow.contentFilterExcludedTypes) &&
        Objects.equals(this.contentFilterMinimumRating, scrapingWorkflow.contentFilterMinimumRating) &&
        Objects.equals(this.contentFilterMinimumReviews, scrapingWorkflow.contentFilterMinimumReviews) &&
        Objects.equals(this.qosMaxConcurrentRequests, scrapingWorkflow.qosMaxConcurrentRequests) &&
        Objects.equals(this.qosMaxRetries, scrapingWorkflow.qosMaxRetries) &&
        Objects.equals(this.qosRequestTimeout, scrapingWorkflow.qosRequestTimeout) &&
        Objects.equals(this.qosEnableJavascript, scrapingWorkflow.qosEnableJavascript) &&
        Objects.equals(this.respectRobotsTxt, scrapingWorkflow.respectRobotsTxt) &&
        Objects.equals(this.acceptTermsOfService, scrapingWorkflow.acceptTermsOfService) &&
        Objects.equals(this.userAgent, scrapingWorkflow.userAgent);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, name, cronExpression, nextRunTime, lastRunTime, status, retryCount, maxRetries, alertEmails, createdAt, updatedAt, deletedAt, jobs, geoFencingRadius, geoFencingLat, geoFencingLon, geoFencingZoomMin, geoFencingZoomMax, includeReviews, includePhotos, includeBusinessHours, maxReviewsPerBusiness, outputFormat, outputDestination, dataRetention, anonymizePii, notificationSlackChannel, notificationEmailGroup, notificationNotifyOnStart, notificationNotifyOnComplete, notificationNotifyOnFailure, contentFilterAllowedCountries, contentFilterExcludedTypes, contentFilterMinimumRating, contentFilterMinimumReviews, qosMaxConcurrentRequests, qosMaxRetries, qosRequestTimeout, qosEnableJavascript, respectRobotsTxt, acceptTermsOfService, userAgent);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ScrapingWorkflow {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    name: ").append(toIndentedString(name)).append("\n");
    sb.append("    cronExpression: ").append(toIndentedString(cronExpression)).append("\n");
    sb.append("    nextRunTime: ").append(toIndentedString(nextRunTime)).append("\n");
    sb.append("    lastRunTime: ").append(toIndentedString(lastRunTime)).append("\n");
    sb.append("    status: ").append(toIndentedString(status)).append("\n");
    sb.append("    retryCount: ").append(toIndentedString(retryCount)).append("\n");
    sb.append("    maxRetries: ").append(toIndentedString(maxRetries)).append("\n");
    sb.append("    alertEmails: ").append(toIndentedString(alertEmails)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
    sb.append("    updatedAt: ").append(toIndentedString(updatedAt)).append("\n");
    sb.append("    deletedAt: ").append(toIndentedString(deletedAt)).append("\n");
    sb.append("    jobs: ").append(toIndentedString(jobs)).append("\n");
    sb.append("    geoFencingRadius: ").append(toIndentedString(geoFencingRadius)).append("\n");
    sb.append("    geoFencingLat: ").append(toIndentedString(geoFencingLat)).append("\n");
    sb.append("    geoFencingLon: ").append(toIndentedString(geoFencingLon)).append("\n");
    sb.append("    geoFencingZoomMin: ").append(toIndentedString(geoFencingZoomMin)).append("\n");
    sb.append("    geoFencingZoomMax: ").append(toIndentedString(geoFencingZoomMax)).append("\n");
    sb.append("    includeReviews: ").append(toIndentedString(includeReviews)).append("\n");
    sb.append("    includePhotos: ").append(toIndentedString(includePhotos)).append("\n");
    sb.append("    includeBusinessHours: ").append(toIndentedString(includeBusinessHours)).append("\n");
    sb.append("    maxReviewsPerBusiness: ").append(toIndentedString(maxReviewsPerBusiness)).append("\n");
    sb.append("    outputFormat: ").append(toIndentedString(outputFormat)).append("\n");
    sb.append("    outputDestination: ").append(toIndentedString(outputDestination)).append("\n");
    sb.append("    dataRetention: ").append(toIndentedString(dataRetention)).append("\n");
    sb.append("    anonymizePii: ").append(toIndentedString(anonymizePii)).append("\n");
    sb.append("    notificationSlackChannel: ").append(toIndentedString(notificationSlackChannel)).append("\n");
    sb.append("    notificationEmailGroup: ").append(toIndentedString(notificationEmailGroup)).append("\n");
    sb.append("    notificationNotifyOnStart: ").append(toIndentedString(notificationNotifyOnStart)).append("\n");
    sb.append("    notificationNotifyOnComplete: ").append(toIndentedString(notificationNotifyOnComplete)).append("\n");
    sb.append("    notificationNotifyOnFailure: ").append(toIndentedString(notificationNotifyOnFailure)).append("\n");
    sb.append("    contentFilterAllowedCountries: ").append(toIndentedString(contentFilterAllowedCountries)).append("\n");
    sb.append("    contentFilterExcludedTypes: ").append(toIndentedString(contentFilterExcludedTypes)).append("\n");
    sb.append("    contentFilterMinimumRating: ").append(toIndentedString(contentFilterMinimumRating)).append("\n");
    sb.append("    contentFilterMinimumReviews: ").append(toIndentedString(contentFilterMinimumReviews)).append("\n");
    sb.append("    qosMaxConcurrentRequests: ").append(toIndentedString(qosMaxConcurrentRequests)).append("\n");
    sb.append("    qosMaxRetries: ").append(toIndentedString(qosMaxRetries)).append("\n");
    sb.append("    qosRequestTimeout: ").append(toIndentedString(qosRequestTimeout)).append("\n");
    sb.append("    qosEnableJavascript: ").append(toIndentedString(qosEnableJavascript)).append("\n");
    sb.append("    respectRobotsTxt: ").append(toIndentedString(respectRobotsTxt)).append("\n");
    sb.append("    acceptTermsOfService: ").append(toIndentedString(acceptTermsOfService)).append("\n");
    sb.append("    userAgent: ").append(toIndentedString(userAgent)).append("\n");
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

