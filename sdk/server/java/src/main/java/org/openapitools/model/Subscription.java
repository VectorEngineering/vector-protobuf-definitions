package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import java.time.OffsetDateTime;
import org.openapitools.model.BillingMode;
import org.openapitools.model.Interval;
import org.openapitools.model.PaymentStatus;
import org.openapitools.model.PlanTier;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * Key features: - Stripe integration - Usage-based billing - Metered pricing - Feature flags  Database considerations: - Syncs with Stripe webhooks - Tracks usage for metered billing - Maintains subscription lifecycle
 */

@Schema(name = "Subscription", description = "Key features: - Stripe integration - Usage-based billing - Metered pricing - Feature flags  Database considerations: - Syncs with Stripe webhooks - Tracks usage for metered billing - Maintains subscription lifecycle")
@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-13T19:56:39.276659-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class Subscription {

  private String id;

  private String stripeCustomerId;

  private String stripeSubscriptionId;

  private String stripePriceId;

  private String stripeProductId;

  private PlanTier planTier = PlanTier.UNSPECIFIED;

  private BillingMode billingMode = BillingMode.UNSPECIFIED;

  private Integer includedJobs;

  private Float perJobRate;

  private Integer maxConcurrentJobs;

  private String includedStorage;

  private Float perGbRate;

  private Boolean advancedFilteringEnabled;

  private Boolean prioritySupportEnabled;

  private Boolean customExportsEnabled;

  private Boolean apiAccessEnabled;

  private Boolean customProxiesEnabled;

  private Boolean advancedAnalyticsEnabled;

  private Integer retentionDays;

  private Integer maxResultsPerJob;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime currentPeriodStart;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime currentPeriodEnd;

  private Float currentPeriodUsage;

  private String currency;

  private Float basePrice;

  private Interval billingInterval = Interval.UNSPECIFIED;

  private Boolean autoRenew;

  private PaymentStatus paymentStatus = PaymentStatus.UNSPECIFIED;

  private Boolean isTrial;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime trialStart;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime trialEnd;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime canceledAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime endedAt;

  public Subscription id(String id) {
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

  public Subscription stripeCustomerId(String stripeCustomerId) {
    this.stripeCustomerId = stripeCustomerId;
    return this;
  }

  /**
   * Get stripeCustomerId
   * @return stripeCustomerId
   */
  
  @Schema(name = "stripeCustomerId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("stripeCustomerId")
  public String getStripeCustomerId() {
    return stripeCustomerId;
  }

  public void setStripeCustomerId(String stripeCustomerId) {
    this.stripeCustomerId = stripeCustomerId;
  }

  public Subscription stripeSubscriptionId(String stripeSubscriptionId) {
    this.stripeSubscriptionId = stripeSubscriptionId;
    return this;
  }

  /**
   * Get stripeSubscriptionId
   * @return stripeSubscriptionId
   */
  
  @Schema(name = "stripeSubscriptionId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("stripeSubscriptionId")
  public String getStripeSubscriptionId() {
    return stripeSubscriptionId;
  }

  public void setStripeSubscriptionId(String stripeSubscriptionId) {
    this.stripeSubscriptionId = stripeSubscriptionId;
  }

  public Subscription stripePriceId(String stripePriceId) {
    this.stripePriceId = stripePriceId;
    return this;
  }

  /**
   * Get stripePriceId
   * @return stripePriceId
   */
  
  @Schema(name = "stripePriceId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("stripePriceId")
  public String getStripePriceId() {
    return stripePriceId;
  }

  public void setStripePriceId(String stripePriceId) {
    this.stripePriceId = stripePriceId;
  }

  public Subscription stripeProductId(String stripeProductId) {
    this.stripeProductId = stripeProductId;
    return this;
  }

  /**
   * Get stripeProductId
   * @return stripeProductId
   */
  
  @Schema(name = "stripeProductId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("stripeProductId")
  public String getStripeProductId() {
    return stripeProductId;
  }

  public void setStripeProductId(String stripeProductId) {
    this.stripeProductId = stripeProductId;
  }

  public Subscription planTier(PlanTier planTier) {
    this.planTier = planTier;
    return this;
  }

  /**
   * Get planTier
   * @return planTier
   */
  @Valid 
  @Schema(name = "planTier", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("planTier")
  public PlanTier getPlanTier() {
    return planTier;
  }

  public void setPlanTier(PlanTier planTier) {
    this.planTier = planTier;
  }

  public Subscription billingMode(BillingMode billingMode) {
    this.billingMode = billingMode;
    return this;
  }

  /**
   * Get billingMode
   * @return billingMode
   */
  @Valid 
  @Schema(name = "billingMode", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("billingMode")
  public BillingMode getBillingMode() {
    return billingMode;
  }

  public void setBillingMode(BillingMode billingMode) {
    this.billingMode = billingMode;
  }

  public Subscription includedJobs(Integer includedJobs) {
    this.includedJobs = includedJobs;
    return this;
  }

  /**
   * Jobs included in base price
   * @return includedJobs
   */
  
  @Schema(name = "includedJobs", description = "Jobs included in base price", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("includedJobs")
  public Integer getIncludedJobs() {
    return includedJobs;
  }

  public void setIncludedJobs(Integer includedJobs) {
    this.includedJobs = includedJobs;
  }

  public Subscription perJobRate(Float perJobRate) {
    this.perJobRate = perJobRate;
    return this;
  }

  /**
   * Get perJobRate
   * @return perJobRate
   */
  
  @Schema(name = "perJobRate", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("perJobRate")
  public Float getPerJobRate() {
    return perJobRate;
  }

  public void setPerJobRate(Float perJobRate) {
    this.perJobRate = perJobRate;
  }

  public Subscription maxConcurrentJobs(Integer maxConcurrentJobs) {
    this.maxConcurrentJobs = maxConcurrentJobs;
    return this;
  }

  /**
   * Get maxConcurrentJobs
   * @return maxConcurrentJobs
   */
  
  @Schema(name = "maxConcurrentJobs", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("maxConcurrentJobs")
  public Integer getMaxConcurrentJobs() {
    return maxConcurrentJobs;
  }

  public void setMaxConcurrentJobs(Integer maxConcurrentJobs) {
    this.maxConcurrentJobs = maxConcurrentJobs;
  }

  public Subscription includedStorage(String includedStorage) {
    this.includedStorage = includedStorage;
    return this;
  }

  /**
   * Get includedStorage
   * @return includedStorage
   */
  
  @Schema(name = "includedStorage", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("includedStorage")
  public String getIncludedStorage() {
    return includedStorage;
  }

  public void setIncludedStorage(String includedStorage) {
    this.includedStorage = includedStorage;
  }

  public Subscription perGbRate(Float perGbRate) {
    this.perGbRate = perGbRate;
    return this;
  }

  /**
   * Get perGbRate
   * @return perGbRate
   */
  
  @Schema(name = "perGbRate", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("perGbRate")
  public Float getPerGbRate() {
    return perGbRate;
  }

  public void setPerGbRate(Float perGbRate) {
    this.perGbRate = perGbRate;
  }

  public Subscription advancedFilteringEnabled(Boolean advancedFilteringEnabled) {
    this.advancedFilteringEnabled = advancedFilteringEnabled;
    return this;
  }

  /**
   * Get advancedFilteringEnabled
   * @return advancedFilteringEnabled
   */
  
  @Schema(name = "advancedFilteringEnabled", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("advancedFilteringEnabled")
  public Boolean getAdvancedFilteringEnabled() {
    return advancedFilteringEnabled;
  }

  public void setAdvancedFilteringEnabled(Boolean advancedFilteringEnabled) {
    this.advancedFilteringEnabled = advancedFilteringEnabled;
  }

  public Subscription prioritySupportEnabled(Boolean prioritySupportEnabled) {
    this.prioritySupportEnabled = prioritySupportEnabled;
    return this;
  }

  /**
   * Get prioritySupportEnabled
   * @return prioritySupportEnabled
   */
  
  @Schema(name = "prioritySupportEnabled", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("prioritySupportEnabled")
  public Boolean getPrioritySupportEnabled() {
    return prioritySupportEnabled;
  }

  public void setPrioritySupportEnabled(Boolean prioritySupportEnabled) {
    this.prioritySupportEnabled = prioritySupportEnabled;
  }

  public Subscription customExportsEnabled(Boolean customExportsEnabled) {
    this.customExportsEnabled = customExportsEnabled;
    return this;
  }

  /**
   * Get customExportsEnabled
   * @return customExportsEnabled
   */
  
  @Schema(name = "customExportsEnabled", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("customExportsEnabled")
  public Boolean getCustomExportsEnabled() {
    return customExportsEnabled;
  }

  public void setCustomExportsEnabled(Boolean customExportsEnabled) {
    this.customExportsEnabled = customExportsEnabled;
  }

  public Subscription apiAccessEnabled(Boolean apiAccessEnabled) {
    this.apiAccessEnabled = apiAccessEnabled;
    return this;
  }

  /**
   * Get apiAccessEnabled
   * @return apiAccessEnabled
   */
  
  @Schema(name = "apiAccessEnabled", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("apiAccessEnabled")
  public Boolean getApiAccessEnabled() {
    return apiAccessEnabled;
  }

  public void setApiAccessEnabled(Boolean apiAccessEnabled) {
    this.apiAccessEnabled = apiAccessEnabled;
  }

  public Subscription customProxiesEnabled(Boolean customProxiesEnabled) {
    this.customProxiesEnabled = customProxiesEnabled;
    return this;
  }

  /**
   * Get customProxiesEnabled
   * @return customProxiesEnabled
   */
  
  @Schema(name = "customProxiesEnabled", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("customProxiesEnabled")
  public Boolean getCustomProxiesEnabled() {
    return customProxiesEnabled;
  }

  public void setCustomProxiesEnabled(Boolean customProxiesEnabled) {
    this.customProxiesEnabled = customProxiesEnabled;
  }

  public Subscription advancedAnalyticsEnabled(Boolean advancedAnalyticsEnabled) {
    this.advancedAnalyticsEnabled = advancedAnalyticsEnabled;
    return this;
  }

  /**
   * Get advancedAnalyticsEnabled
   * @return advancedAnalyticsEnabled
   */
  
  @Schema(name = "advancedAnalyticsEnabled", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("advancedAnalyticsEnabled")
  public Boolean getAdvancedAnalyticsEnabled() {
    return advancedAnalyticsEnabled;
  }

  public void setAdvancedAnalyticsEnabled(Boolean advancedAnalyticsEnabled) {
    this.advancedAnalyticsEnabled = advancedAnalyticsEnabled;
  }

  public Subscription retentionDays(Integer retentionDays) {
    this.retentionDays = retentionDays;
    return this;
  }

  /**
   * Get retentionDays
   * @return retentionDays
   */
  
  @Schema(name = "retentionDays", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("retentionDays")
  public Integer getRetentionDays() {
    return retentionDays;
  }

  public void setRetentionDays(Integer retentionDays) {
    this.retentionDays = retentionDays;
  }

  public Subscription maxResultsPerJob(Integer maxResultsPerJob) {
    this.maxResultsPerJob = maxResultsPerJob;
    return this;
  }

  /**
   * Get maxResultsPerJob
   * @return maxResultsPerJob
   */
  
  @Schema(name = "maxResultsPerJob", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("maxResultsPerJob")
  public Integer getMaxResultsPerJob() {
    return maxResultsPerJob;
  }

  public void setMaxResultsPerJob(Integer maxResultsPerJob) {
    this.maxResultsPerJob = maxResultsPerJob;
  }

  public Subscription currentPeriodStart(OffsetDateTime currentPeriodStart) {
    this.currentPeriodStart = currentPeriodStart;
    return this;
  }

  /**
   * Get currentPeriodStart
   * @return currentPeriodStart
   */
  @Valid 
  @Schema(name = "currentPeriodStart", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("currentPeriodStart")
  public OffsetDateTime getCurrentPeriodStart() {
    return currentPeriodStart;
  }

  public void setCurrentPeriodStart(OffsetDateTime currentPeriodStart) {
    this.currentPeriodStart = currentPeriodStart;
  }

  public Subscription currentPeriodEnd(OffsetDateTime currentPeriodEnd) {
    this.currentPeriodEnd = currentPeriodEnd;
    return this;
  }

  /**
   * Get currentPeriodEnd
   * @return currentPeriodEnd
   */
  @Valid 
  @Schema(name = "currentPeriodEnd", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("currentPeriodEnd")
  public OffsetDateTime getCurrentPeriodEnd() {
    return currentPeriodEnd;
  }

  public void setCurrentPeriodEnd(OffsetDateTime currentPeriodEnd) {
    this.currentPeriodEnd = currentPeriodEnd;
  }

  public Subscription currentPeriodUsage(Float currentPeriodUsage) {
    this.currentPeriodUsage = currentPeriodUsage;
    return this;
  }

  /**
   * Get currentPeriodUsage
   * @return currentPeriodUsage
   */
  
  @Schema(name = "currentPeriodUsage", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("currentPeriodUsage")
  public Float getCurrentPeriodUsage() {
    return currentPeriodUsage;
  }

  public void setCurrentPeriodUsage(Float currentPeriodUsage) {
    this.currentPeriodUsage = currentPeriodUsage;
  }

  public Subscription currency(String currency) {
    this.currency = currency;
    return this;
  }

  /**
   * e.g., \"usd\"
   * @return currency
   */
  
  @Schema(name = "currency", description = "e.g., \"usd\"", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("currency")
  public String getCurrency() {
    return currency;
  }

  public void setCurrency(String currency) {
    this.currency = currency;
  }

  public Subscription basePrice(Float basePrice) {
    this.basePrice = basePrice;
    return this;
  }

  /**
   * Get basePrice
   * @return basePrice
   */
  
  @Schema(name = "basePrice", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("basePrice")
  public Float getBasePrice() {
    return basePrice;
  }

  public void setBasePrice(Float basePrice) {
    this.basePrice = basePrice;
  }

  public Subscription billingInterval(Interval billingInterval) {
    this.billingInterval = billingInterval;
    return this;
  }

  /**
   * Get billingInterval
   * @return billingInterval
   */
  @Valid 
  @Schema(name = "billingInterval", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("billingInterval")
  public Interval getBillingInterval() {
    return billingInterval;
  }

  public void setBillingInterval(Interval billingInterval) {
    this.billingInterval = billingInterval;
  }

  public Subscription autoRenew(Boolean autoRenew) {
    this.autoRenew = autoRenew;
    return this;
  }

  /**
   * Get autoRenew
   * @return autoRenew
   */
  
  @Schema(name = "autoRenew", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("autoRenew")
  public Boolean getAutoRenew() {
    return autoRenew;
  }

  public void setAutoRenew(Boolean autoRenew) {
    this.autoRenew = autoRenew;
  }

  public Subscription paymentStatus(PaymentStatus paymentStatus) {
    this.paymentStatus = paymentStatus;
    return this;
  }

  /**
   * Get paymentStatus
   * @return paymentStatus
   */
  @Valid 
  @Schema(name = "paymentStatus", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("paymentStatus")
  public PaymentStatus getPaymentStatus() {
    return paymentStatus;
  }

  public void setPaymentStatus(PaymentStatus paymentStatus) {
    this.paymentStatus = paymentStatus;
  }

  public Subscription isTrial(Boolean isTrial) {
    this.isTrial = isTrial;
    return this;
  }

  /**
   * Get isTrial
   * @return isTrial
   */
  
  @Schema(name = "isTrial", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("isTrial")
  public Boolean getIsTrial() {
    return isTrial;
  }

  public void setIsTrial(Boolean isTrial) {
    this.isTrial = isTrial;
  }

  public Subscription trialStart(OffsetDateTime trialStart) {
    this.trialStart = trialStart;
    return this;
  }

  /**
   * Get trialStart
   * @return trialStart
   */
  @Valid 
  @Schema(name = "trialStart", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("trialStart")
  public OffsetDateTime getTrialStart() {
    return trialStart;
  }

  public void setTrialStart(OffsetDateTime trialStart) {
    this.trialStart = trialStart;
  }

  public Subscription trialEnd(OffsetDateTime trialEnd) {
    this.trialEnd = trialEnd;
    return this;
  }

  /**
   * Get trialEnd
   * @return trialEnd
   */
  @Valid 
  @Schema(name = "trialEnd", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("trialEnd")
  public OffsetDateTime getTrialEnd() {
    return trialEnd;
  }

  public void setTrialEnd(OffsetDateTime trialEnd) {
    this.trialEnd = trialEnd;
  }

  public Subscription createdAt(OffsetDateTime createdAt) {
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

  public Subscription updatedAt(OffsetDateTime updatedAt) {
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

  public Subscription canceledAt(OffsetDateTime canceledAt) {
    this.canceledAt = canceledAt;
    return this;
  }

  /**
   * Get canceledAt
   * @return canceledAt
   */
  @Valid 
  @Schema(name = "canceledAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("canceledAt")
  public OffsetDateTime getCanceledAt() {
    return canceledAt;
  }

  public void setCanceledAt(OffsetDateTime canceledAt) {
    this.canceledAt = canceledAt;
  }

  public Subscription endedAt(OffsetDateTime endedAt) {
    this.endedAt = endedAt;
    return this;
  }

  /**
   * Get endedAt
   * @return endedAt
   */
  @Valid 
  @Schema(name = "endedAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("endedAt")
  public OffsetDateTime getEndedAt() {
    return endedAt;
  }

  public void setEndedAt(OffsetDateTime endedAt) {
    this.endedAt = endedAt;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    Subscription subscription = (Subscription) o;
    return Objects.equals(this.id, subscription.id) &&
        Objects.equals(this.stripeCustomerId, subscription.stripeCustomerId) &&
        Objects.equals(this.stripeSubscriptionId, subscription.stripeSubscriptionId) &&
        Objects.equals(this.stripePriceId, subscription.stripePriceId) &&
        Objects.equals(this.stripeProductId, subscription.stripeProductId) &&
        Objects.equals(this.planTier, subscription.planTier) &&
        Objects.equals(this.billingMode, subscription.billingMode) &&
        Objects.equals(this.includedJobs, subscription.includedJobs) &&
        Objects.equals(this.perJobRate, subscription.perJobRate) &&
        Objects.equals(this.maxConcurrentJobs, subscription.maxConcurrentJobs) &&
        Objects.equals(this.includedStorage, subscription.includedStorage) &&
        Objects.equals(this.perGbRate, subscription.perGbRate) &&
        Objects.equals(this.advancedFilteringEnabled, subscription.advancedFilteringEnabled) &&
        Objects.equals(this.prioritySupportEnabled, subscription.prioritySupportEnabled) &&
        Objects.equals(this.customExportsEnabled, subscription.customExportsEnabled) &&
        Objects.equals(this.apiAccessEnabled, subscription.apiAccessEnabled) &&
        Objects.equals(this.customProxiesEnabled, subscription.customProxiesEnabled) &&
        Objects.equals(this.advancedAnalyticsEnabled, subscription.advancedAnalyticsEnabled) &&
        Objects.equals(this.retentionDays, subscription.retentionDays) &&
        Objects.equals(this.maxResultsPerJob, subscription.maxResultsPerJob) &&
        Objects.equals(this.currentPeriodStart, subscription.currentPeriodStart) &&
        Objects.equals(this.currentPeriodEnd, subscription.currentPeriodEnd) &&
        Objects.equals(this.currentPeriodUsage, subscription.currentPeriodUsage) &&
        Objects.equals(this.currency, subscription.currency) &&
        Objects.equals(this.basePrice, subscription.basePrice) &&
        Objects.equals(this.billingInterval, subscription.billingInterval) &&
        Objects.equals(this.autoRenew, subscription.autoRenew) &&
        Objects.equals(this.paymentStatus, subscription.paymentStatus) &&
        Objects.equals(this.isTrial, subscription.isTrial) &&
        Objects.equals(this.trialStart, subscription.trialStart) &&
        Objects.equals(this.trialEnd, subscription.trialEnd) &&
        Objects.equals(this.createdAt, subscription.createdAt) &&
        Objects.equals(this.updatedAt, subscription.updatedAt) &&
        Objects.equals(this.canceledAt, subscription.canceledAt) &&
        Objects.equals(this.endedAt, subscription.endedAt);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, stripeCustomerId, stripeSubscriptionId, stripePriceId, stripeProductId, planTier, billingMode, includedJobs, perJobRate, maxConcurrentJobs, includedStorage, perGbRate, advancedFilteringEnabled, prioritySupportEnabled, customExportsEnabled, apiAccessEnabled, customProxiesEnabled, advancedAnalyticsEnabled, retentionDays, maxResultsPerJob, currentPeriodStart, currentPeriodEnd, currentPeriodUsage, currency, basePrice, billingInterval, autoRenew, paymentStatus, isTrial, trialStart, trialEnd, createdAt, updatedAt, canceledAt, endedAt);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class Subscription {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    stripeCustomerId: ").append(toIndentedString(stripeCustomerId)).append("\n");
    sb.append("    stripeSubscriptionId: ").append(toIndentedString(stripeSubscriptionId)).append("\n");
    sb.append("    stripePriceId: ").append(toIndentedString(stripePriceId)).append("\n");
    sb.append("    stripeProductId: ").append(toIndentedString(stripeProductId)).append("\n");
    sb.append("    planTier: ").append(toIndentedString(planTier)).append("\n");
    sb.append("    billingMode: ").append(toIndentedString(billingMode)).append("\n");
    sb.append("    includedJobs: ").append(toIndentedString(includedJobs)).append("\n");
    sb.append("    perJobRate: ").append(toIndentedString(perJobRate)).append("\n");
    sb.append("    maxConcurrentJobs: ").append(toIndentedString(maxConcurrentJobs)).append("\n");
    sb.append("    includedStorage: ").append(toIndentedString(includedStorage)).append("\n");
    sb.append("    perGbRate: ").append(toIndentedString(perGbRate)).append("\n");
    sb.append("    advancedFilteringEnabled: ").append(toIndentedString(advancedFilteringEnabled)).append("\n");
    sb.append("    prioritySupportEnabled: ").append(toIndentedString(prioritySupportEnabled)).append("\n");
    sb.append("    customExportsEnabled: ").append(toIndentedString(customExportsEnabled)).append("\n");
    sb.append("    apiAccessEnabled: ").append(toIndentedString(apiAccessEnabled)).append("\n");
    sb.append("    customProxiesEnabled: ").append(toIndentedString(customProxiesEnabled)).append("\n");
    sb.append("    advancedAnalyticsEnabled: ").append(toIndentedString(advancedAnalyticsEnabled)).append("\n");
    sb.append("    retentionDays: ").append(toIndentedString(retentionDays)).append("\n");
    sb.append("    maxResultsPerJob: ").append(toIndentedString(maxResultsPerJob)).append("\n");
    sb.append("    currentPeriodStart: ").append(toIndentedString(currentPeriodStart)).append("\n");
    sb.append("    currentPeriodEnd: ").append(toIndentedString(currentPeriodEnd)).append("\n");
    sb.append("    currentPeriodUsage: ").append(toIndentedString(currentPeriodUsage)).append("\n");
    sb.append("    currency: ").append(toIndentedString(currency)).append("\n");
    sb.append("    basePrice: ").append(toIndentedString(basePrice)).append("\n");
    sb.append("    billingInterval: ").append(toIndentedString(billingInterval)).append("\n");
    sb.append("    autoRenew: ").append(toIndentedString(autoRenew)).append("\n");
    sb.append("    paymentStatus: ").append(toIndentedString(paymentStatus)).append("\n");
    sb.append("    isTrial: ").append(toIndentedString(isTrial)).append("\n");
    sb.append("    trialStart: ").append(toIndentedString(trialStart)).append("\n");
    sb.append("    trialEnd: ").append(toIndentedString(trialEnd)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
    sb.append("    updatedAt: ").append(toIndentedString(updatedAt)).append("\n");
    sb.append("    canceledAt: ").append(toIndentedString(canceledAt)).append("\n");
    sb.append("    endedAt: ").append(toIndentedString(endedAt)).append("\n");
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

