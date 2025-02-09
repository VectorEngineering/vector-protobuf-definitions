package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import java.time.OffsetDateTime;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import org.openapitools.model.AppAnalytics;
import org.openapitools.model.AppCategory;
import org.openapitools.model.AppDevelopmentInfo;
import org.openapitools.model.AppInstallation;
import org.openapitools.model.AppPermission;
import org.openapitools.model.AppReview;
import org.openapitools.model.AppVersion;
import org.openapitools.model.AppWebhook;
import org.openapitools.model.PricingModel;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * MarketplaceApp
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-09T12:56:25.550372-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class MarketplaceApp {

  private String id;

  private String name;

  private String description;

  private String developerId;

  private AppCategory category = AppCategory.UNSPECIFIED;

  @Valid
  private List<String> supportedContractTypes = new ArrayList<>();

  @Valid
  private List<String> features = new ArrayList<>();

  private PricingModel pricingModel = PricingModel.UNSPECIFIED;

  @Valid
  private Map<String, Float> pricingTiers = new HashMap<>();

  @Valid
  private List<String> supportedIntegrations = new ArrayList<>();

  private Boolean isVerified;

  private Float rating;

  private Integer installationCount;

  @Valid
  private List<String> requiredPermissions = new ArrayList<>();

  @Valid
  private Map<String, String> metadata = new HashMap<>();

  private String documentationUrl;

  private String supportEmail;

  @Valid
  private List<String> screenshots = new ArrayList<>();

  private String privacyPolicyUrl;

  private String termsUrl;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime publishedAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime lastUpdated;

  @Valid
  private List<@Valid AppVersion> versions = new ArrayList<>();

  @Valid
  private List<@Valid AppInstallation> installations = new ArrayList<>();

  @Valid
  private List<@Valid AppAnalytics> analytics = new ArrayList<>();

  @Valid
  private List<@Valid AppReview> reviews = new ArrayList<>();

  private AppDevelopmentInfo info;

  @Valid
  private List<@Valid AppWebhook> webhooks = new ArrayList<>();

  @Valid
  private List<@Valid AppPermission> permissions = new ArrayList<>();

  public MarketplaceApp id(String id) {
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

  public MarketplaceApp name(String name) {
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

  public MarketplaceApp description(String description) {
    this.description = description;
    return this;
  }

  /**
   * Get description
   * @return description
   */
  
  @Schema(name = "description", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("description")
  public String getDescription() {
    return description;
  }

  public void setDescription(String description) {
    this.description = description;
  }

  public MarketplaceApp developerId(String developerId) {
    this.developerId = developerId;
    return this;
  }

  /**
   * Get developerId
   * @return developerId
   */
  
  @Schema(name = "developerId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("developerId")
  public String getDeveloperId() {
    return developerId;
  }

  public void setDeveloperId(String developerId) {
    this.developerId = developerId;
  }

  public MarketplaceApp category(AppCategory category) {
    this.category = category;
    return this;
  }

  /**
   * Get category
   * @return category
   */
  @Valid 
  @Schema(name = "category", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("category")
  public AppCategory getCategory() {
    return category;
  }

  public void setCategory(AppCategory category) {
    this.category = category;
  }

  public MarketplaceApp supportedContractTypes(List<String> supportedContractTypes) {
    this.supportedContractTypes = supportedContractTypes;
    return this;
  }

  public MarketplaceApp addSupportedContractTypesItem(String supportedContractTypesItem) {
    if (this.supportedContractTypes == null) {
      this.supportedContractTypes = new ArrayList<>();
    }
    this.supportedContractTypes.add(supportedContractTypesItem);
    return this;
  }

  /**
   * Get supportedContractTypes
   * @return supportedContractTypes
   */
  
  @Schema(name = "supportedContractTypes", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("supportedContractTypes")
  public List<String> getSupportedContractTypes() {
    return supportedContractTypes;
  }

  public void setSupportedContractTypes(List<String> supportedContractTypes) {
    this.supportedContractTypes = supportedContractTypes;
  }

  public MarketplaceApp features(List<String> features) {
    this.features = features;
    return this;
  }

  public MarketplaceApp addFeaturesItem(String featuresItem) {
    if (this.features == null) {
      this.features = new ArrayList<>();
    }
    this.features.add(featuresItem);
    return this;
  }

  /**
   * Get features
   * @return features
   */
  
  @Schema(name = "features", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("features")
  public List<String> getFeatures() {
    return features;
  }

  public void setFeatures(List<String> features) {
    this.features = features;
  }

  public MarketplaceApp pricingModel(PricingModel pricingModel) {
    this.pricingModel = pricingModel;
    return this;
  }

  /**
   * Get pricingModel
   * @return pricingModel
   */
  @Valid 
  @Schema(name = "pricingModel", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("pricingModel")
  public PricingModel getPricingModel() {
    return pricingModel;
  }

  public void setPricingModel(PricingModel pricingModel) {
    this.pricingModel = pricingModel;
  }

  public MarketplaceApp pricingTiers(Map<String, Float> pricingTiers) {
    this.pricingTiers = pricingTiers;
    return this;
  }

  public MarketplaceApp putPricingTiersItem(String key, Float pricingTiersItem) {
    if (this.pricingTiers == null) {
      this.pricingTiers = new HashMap<>();
    }
    this.pricingTiers.put(key, pricingTiersItem);
    return this;
  }

  /**
   * Get pricingTiers
   * @return pricingTiers
   */
  
  @Schema(name = "pricingTiers", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("pricingTiers")
  public Map<String, Float> getPricingTiers() {
    return pricingTiers;
  }

  public void setPricingTiers(Map<String, Float> pricingTiers) {
    this.pricingTiers = pricingTiers;
  }

  public MarketplaceApp supportedIntegrations(List<String> supportedIntegrations) {
    this.supportedIntegrations = supportedIntegrations;
    return this;
  }

  public MarketplaceApp addSupportedIntegrationsItem(String supportedIntegrationsItem) {
    if (this.supportedIntegrations == null) {
      this.supportedIntegrations = new ArrayList<>();
    }
    this.supportedIntegrations.add(supportedIntegrationsItem);
    return this;
  }

  /**
   * Get supportedIntegrations
   * @return supportedIntegrations
   */
  
  @Schema(name = "supportedIntegrations", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("supportedIntegrations")
  public List<String> getSupportedIntegrations() {
    return supportedIntegrations;
  }

  public void setSupportedIntegrations(List<String> supportedIntegrations) {
    this.supportedIntegrations = supportedIntegrations;
  }

  public MarketplaceApp isVerified(Boolean isVerified) {
    this.isVerified = isVerified;
    return this;
  }

  /**
   * Get isVerified
   * @return isVerified
   */
  
  @Schema(name = "isVerified", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("isVerified")
  public Boolean getIsVerified() {
    return isVerified;
  }

  public void setIsVerified(Boolean isVerified) {
    this.isVerified = isVerified;
  }

  public MarketplaceApp rating(Float rating) {
    this.rating = rating;
    return this;
  }

  /**
   * Get rating
   * @return rating
   */
  
  @Schema(name = "rating", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("rating")
  public Float getRating() {
    return rating;
  }

  public void setRating(Float rating) {
    this.rating = rating;
  }

  public MarketplaceApp installationCount(Integer installationCount) {
    this.installationCount = installationCount;
    return this;
  }

  /**
   * Get installationCount
   * @return installationCount
   */
  
  @Schema(name = "installationCount", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("installationCount")
  public Integer getInstallationCount() {
    return installationCount;
  }

  public void setInstallationCount(Integer installationCount) {
    this.installationCount = installationCount;
  }

  public MarketplaceApp requiredPermissions(List<String> requiredPermissions) {
    this.requiredPermissions = requiredPermissions;
    return this;
  }

  public MarketplaceApp addRequiredPermissionsItem(String requiredPermissionsItem) {
    if (this.requiredPermissions == null) {
      this.requiredPermissions = new ArrayList<>();
    }
    this.requiredPermissions.add(requiredPermissionsItem);
    return this;
  }

  /**
   * Get requiredPermissions
   * @return requiredPermissions
   */
  
  @Schema(name = "requiredPermissions", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("requiredPermissions")
  public List<String> getRequiredPermissions() {
    return requiredPermissions;
  }

  public void setRequiredPermissions(List<String> requiredPermissions) {
    this.requiredPermissions = requiredPermissions;
  }

  public MarketplaceApp metadata(Map<String, String> metadata) {
    this.metadata = metadata;
    return this;
  }

  public MarketplaceApp putMetadataItem(String key, String metadataItem) {
    if (this.metadata == null) {
      this.metadata = new HashMap<>();
    }
    this.metadata.put(key, metadataItem);
    return this;
  }

  /**
   * Get metadata
   * @return metadata
   */
  
  @Schema(name = "metadata", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("metadata")
  public Map<String, String> getMetadata() {
    return metadata;
  }

  public void setMetadata(Map<String, String> metadata) {
    this.metadata = metadata;
  }

  public MarketplaceApp documentationUrl(String documentationUrl) {
    this.documentationUrl = documentationUrl;
    return this;
  }

  /**
   * Get documentationUrl
   * @return documentationUrl
   */
  
  @Schema(name = "documentationUrl", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("documentationUrl")
  public String getDocumentationUrl() {
    return documentationUrl;
  }

  public void setDocumentationUrl(String documentationUrl) {
    this.documentationUrl = documentationUrl;
  }

  public MarketplaceApp supportEmail(String supportEmail) {
    this.supportEmail = supportEmail;
    return this;
  }

  /**
   * Get supportEmail
   * @return supportEmail
   */
  
  @Schema(name = "supportEmail", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("supportEmail")
  public String getSupportEmail() {
    return supportEmail;
  }

  public void setSupportEmail(String supportEmail) {
    this.supportEmail = supportEmail;
  }

  public MarketplaceApp screenshots(List<String> screenshots) {
    this.screenshots = screenshots;
    return this;
  }

  public MarketplaceApp addScreenshotsItem(String screenshotsItem) {
    if (this.screenshots == null) {
      this.screenshots = new ArrayList<>();
    }
    this.screenshots.add(screenshotsItem);
    return this;
  }

  /**
   * Get screenshots
   * @return screenshots
   */
  
  @Schema(name = "screenshots", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("screenshots")
  public List<String> getScreenshots() {
    return screenshots;
  }

  public void setScreenshots(List<String> screenshots) {
    this.screenshots = screenshots;
  }

  public MarketplaceApp privacyPolicyUrl(String privacyPolicyUrl) {
    this.privacyPolicyUrl = privacyPolicyUrl;
    return this;
  }

  /**
   * Get privacyPolicyUrl
   * @return privacyPolicyUrl
   */
  
  @Schema(name = "privacyPolicyUrl", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("privacyPolicyUrl")
  public String getPrivacyPolicyUrl() {
    return privacyPolicyUrl;
  }

  public void setPrivacyPolicyUrl(String privacyPolicyUrl) {
    this.privacyPolicyUrl = privacyPolicyUrl;
  }

  public MarketplaceApp termsUrl(String termsUrl) {
    this.termsUrl = termsUrl;
    return this;
  }

  /**
   * Get termsUrl
   * @return termsUrl
   */
  
  @Schema(name = "termsUrl", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("termsUrl")
  public String getTermsUrl() {
    return termsUrl;
  }

  public void setTermsUrl(String termsUrl) {
    this.termsUrl = termsUrl;
  }

  public MarketplaceApp publishedAt(OffsetDateTime publishedAt) {
    this.publishedAt = publishedAt;
    return this;
  }

  /**
   * Get publishedAt
   * @return publishedAt
   */
  @Valid 
  @Schema(name = "publishedAt", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("publishedAt")
  public OffsetDateTime getPublishedAt() {
    return publishedAt;
  }

  public void setPublishedAt(OffsetDateTime publishedAt) {
    this.publishedAt = publishedAt;
  }

  public MarketplaceApp lastUpdated(OffsetDateTime lastUpdated) {
    this.lastUpdated = lastUpdated;
    return this;
  }

  /**
   * Get lastUpdated
   * @return lastUpdated
   */
  @Valid 
  @Schema(name = "lastUpdated", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("lastUpdated")
  public OffsetDateTime getLastUpdated() {
    return lastUpdated;
  }

  public void setLastUpdated(OffsetDateTime lastUpdated) {
    this.lastUpdated = lastUpdated;
  }

  public MarketplaceApp versions(List<@Valid AppVersion> versions) {
    this.versions = versions;
    return this;
  }

  public MarketplaceApp addVersionsItem(AppVersion versionsItem) {
    if (this.versions == null) {
      this.versions = new ArrayList<>();
    }
    this.versions.add(versionsItem);
    return this;
  }

  /**
   * Get versions
   * @return versions
   */
  @Valid 
  @Schema(name = "versions", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("versions")
  public List<@Valid AppVersion> getVersions() {
    return versions;
  }

  public void setVersions(List<@Valid AppVersion> versions) {
    this.versions = versions;
  }

  public MarketplaceApp installations(List<@Valid AppInstallation> installations) {
    this.installations = installations;
    return this;
  }

  public MarketplaceApp addInstallationsItem(AppInstallation installationsItem) {
    if (this.installations == null) {
      this.installations = new ArrayList<>();
    }
    this.installations.add(installationsItem);
    return this;
  }

  /**
   * Get installations
   * @return installations
   */
  @Valid 
  @Schema(name = "installations", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("installations")
  public List<@Valid AppInstallation> getInstallations() {
    return installations;
  }

  public void setInstallations(List<@Valid AppInstallation> installations) {
    this.installations = installations;
  }

  public MarketplaceApp analytics(List<@Valid AppAnalytics> analytics) {
    this.analytics = analytics;
    return this;
  }

  public MarketplaceApp addAnalyticsItem(AppAnalytics analyticsItem) {
    if (this.analytics == null) {
      this.analytics = new ArrayList<>();
    }
    this.analytics.add(analyticsItem);
    return this;
  }

  /**
   * Get analytics
   * @return analytics
   */
  @Valid 
  @Schema(name = "analytics", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("analytics")
  public List<@Valid AppAnalytics> getAnalytics() {
    return analytics;
  }

  public void setAnalytics(List<@Valid AppAnalytics> analytics) {
    this.analytics = analytics;
  }

  public MarketplaceApp reviews(List<@Valid AppReview> reviews) {
    this.reviews = reviews;
    return this;
  }

  public MarketplaceApp addReviewsItem(AppReview reviewsItem) {
    if (this.reviews == null) {
      this.reviews = new ArrayList<>();
    }
    this.reviews.add(reviewsItem);
    return this;
  }

  /**
   * Get reviews
   * @return reviews
   */
  @Valid 
  @Schema(name = "reviews", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("reviews")
  public List<@Valid AppReview> getReviews() {
    return reviews;
  }

  public void setReviews(List<@Valid AppReview> reviews) {
    this.reviews = reviews;
  }

  public MarketplaceApp info(AppDevelopmentInfo info) {
    this.info = info;
    return this;
  }

  /**
   * Get info
   * @return info
   */
  @Valid 
  @Schema(name = "info", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("info")
  public AppDevelopmentInfo getInfo() {
    return info;
  }

  public void setInfo(AppDevelopmentInfo info) {
    this.info = info;
  }

  public MarketplaceApp webhooks(List<@Valid AppWebhook> webhooks) {
    this.webhooks = webhooks;
    return this;
  }

  public MarketplaceApp addWebhooksItem(AppWebhook webhooksItem) {
    if (this.webhooks == null) {
      this.webhooks = new ArrayList<>();
    }
    this.webhooks.add(webhooksItem);
    return this;
  }

  /**
   * Get webhooks
   * @return webhooks
   */
  @Valid 
  @Schema(name = "webhooks", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("webhooks")
  public List<@Valid AppWebhook> getWebhooks() {
    return webhooks;
  }

  public void setWebhooks(List<@Valid AppWebhook> webhooks) {
    this.webhooks = webhooks;
  }

  public MarketplaceApp permissions(List<@Valid AppPermission> permissions) {
    this.permissions = permissions;
    return this;
  }

  public MarketplaceApp addPermissionsItem(AppPermission permissionsItem) {
    if (this.permissions == null) {
      this.permissions = new ArrayList<>();
    }
    this.permissions.add(permissionsItem);
    return this;
  }

  /**
   * Get permissions
   * @return permissions
   */
  @Valid 
  @Schema(name = "permissions", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("permissions")
  public List<@Valid AppPermission> getPermissions() {
    return permissions;
  }

  public void setPermissions(List<@Valid AppPermission> permissions) {
    this.permissions = permissions;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    MarketplaceApp marketplaceApp = (MarketplaceApp) o;
    return Objects.equals(this.id, marketplaceApp.id) &&
        Objects.equals(this.name, marketplaceApp.name) &&
        Objects.equals(this.description, marketplaceApp.description) &&
        Objects.equals(this.developerId, marketplaceApp.developerId) &&
        Objects.equals(this.category, marketplaceApp.category) &&
        Objects.equals(this.supportedContractTypes, marketplaceApp.supportedContractTypes) &&
        Objects.equals(this.features, marketplaceApp.features) &&
        Objects.equals(this.pricingModel, marketplaceApp.pricingModel) &&
        Objects.equals(this.pricingTiers, marketplaceApp.pricingTiers) &&
        Objects.equals(this.supportedIntegrations, marketplaceApp.supportedIntegrations) &&
        Objects.equals(this.isVerified, marketplaceApp.isVerified) &&
        Objects.equals(this.rating, marketplaceApp.rating) &&
        Objects.equals(this.installationCount, marketplaceApp.installationCount) &&
        Objects.equals(this.requiredPermissions, marketplaceApp.requiredPermissions) &&
        Objects.equals(this.metadata, marketplaceApp.metadata) &&
        Objects.equals(this.documentationUrl, marketplaceApp.documentationUrl) &&
        Objects.equals(this.supportEmail, marketplaceApp.supportEmail) &&
        Objects.equals(this.screenshots, marketplaceApp.screenshots) &&
        Objects.equals(this.privacyPolicyUrl, marketplaceApp.privacyPolicyUrl) &&
        Objects.equals(this.termsUrl, marketplaceApp.termsUrl) &&
        Objects.equals(this.publishedAt, marketplaceApp.publishedAt) &&
        Objects.equals(this.lastUpdated, marketplaceApp.lastUpdated) &&
        Objects.equals(this.versions, marketplaceApp.versions) &&
        Objects.equals(this.installations, marketplaceApp.installations) &&
        Objects.equals(this.analytics, marketplaceApp.analytics) &&
        Objects.equals(this.reviews, marketplaceApp.reviews) &&
        Objects.equals(this.info, marketplaceApp.info) &&
        Objects.equals(this.webhooks, marketplaceApp.webhooks) &&
        Objects.equals(this.permissions, marketplaceApp.permissions);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, name, description, developerId, category, supportedContractTypes, features, pricingModel, pricingTiers, supportedIntegrations, isVerified, rating, installationCount, requiredPermissions, metadata, documentationUrl, supportEmail, screenshots, privacyPolicyUrl, termsUrl, publishedAt, lastUpdated, versions, installations, analytics, reviews, info, webhooks, permissions);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class MarketplaceApp {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    name: ").append(toIndentedString(name)).append("\n");
    sb.append("    description: ").append(toIndentedString(description)).append("\n");
    sb.append("    developerId: ").append(toIndentedString(developerId)).append("\n");
    sb.append("    category: ").append(toIndentedString(category)).append("\n");
    sb.append("    supportedContractTypes: ").append(toIndentedString(supportedContractTypes)).append("\n");
    sb.append("    features: ").append(toIndentedString(features)).append("\n");
    sb.append("    pricingModel: ").append(toIndentedString(pricingModel)).append("\n");
    sb.append("    pricingTiers: ").append(toIndentedString(pricingTiers)).append("\n");
    sb.append("    supportedIntegrations: ").append(toIndentedString(supportedIntegrations)).append("\n");
    sb.append("    isVerified: ").append(toIndentedString(isVerified)).append("\n");
    sb.append("    rating: ").append(toIndentedString(rating)).append("\n");
    sb.append("    installationCount: ").append(toIndentedString(installationCount)).append("\n");
    sb.append("    requiredPermissions: ").append(toIndentedString(requiredPermissions)).append("\n");
    sb.append("    metadata: ").append(toIndentedString(metadata)).append("\n");
    sb.append("    documentationUrl: ").append(toIndentedString(documentationUrl)).append("\n");
    sb.append("    supportEmail: ").append(toIndentedString(supportEmail)).append("\n");
    sb.append("    screenshots: ").append(toIndentedString(screenshots)).append("\n");
    sb.append("    privacyPolicyUrl: ").append(toIndentedString(privacyPolicyUrl)).append("\n");
    sb.append("    termsUrl: ").append(toIndentedString(termsUrl)).append("\n");
    sb.append("    publishedAt: ").append(toIndentedString(publishedAt)).append("\n");
    sb.append("    lastUpdated: ").append(toIndentedString(lastUpdated)).append("\n");
    sb.append("    versions: ").append(toIndentedString(versions)).append("\n");
    sb.append("    installations: ").append(toIndentedString(installations)).append("\n");
    sb.append("    analytics: ").append(toIndentedString(analytics)).append("\n");
    sb.append("    reviews: ").append(toIndentedString(reviews)).append("\n");
    sb.append("    info: ").append(toIndentedString(info)).append("\n");
    sb.append("    webhooks: ").append(toIndentedString(webhooks)).append("\n");
    sb.append("    permissions: ").append(toIndentedString(permissions)).append("\n");
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

