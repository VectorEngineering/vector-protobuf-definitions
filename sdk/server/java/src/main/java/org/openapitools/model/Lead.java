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
import org.openapitools.model.BusinessHours;
import org.openapitools.model.EmployeeBenefit;
import org.openapitools.model.RevenueRange;
import org.openapitools.model.Review;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;
import io.swagger.v3.oas.annotations.media.Schema;


import java.util.*;
import javax.annotation.Generated;

/**
 * Lead
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T00:28:49.378974-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public class Lead {

  private String id;

  private String name;

  private String website;

  private String phone;

  private String address;

  private String city;

  private String state;

  private String country;

  private Double latitude;

  private Double longitude;

  private Float googleRating;

  private Integer reviewCount;

  private String industry;

  private Integer employeeCount;

  private String estimatedRevenue;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime createdAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime updatedAt;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime deletedAt;

  private String placeId;

  private String googleMapsUrl;

  private String businessStatus;

  @Valid
  private List<@Valid BusinessHours> regularHours = new ArrayList<>();

  @Valid
  private List<@Valid BusinessHours> specialHours = new ArrayList<>();

  @Valid
  private List<String> photoReferences = new ArrayList<>();

  private String mainPhotoUrl;

  @Valid
  private List<@Valid Review> reviews = new ArrayList<>();

  @Valid
  private List<String> types = new ArrayList<>();

  @Valid
  private List<String> amenities = new ArrayList<>();

  private Boolean servesVegetarianFood;

  private Boolean outdoorSeating;

  @Valid
  private List<String> paymentMethods = new ArrayList<>();

  private Boolean wheelchairAccessible;

  private Boolean parkingAvailable;

  @Valid
  private Map<String, String> socialMedia = new HashMap<>();

  private String ratingCategory;

  private Float rating;

  private Integer count;

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime lastUpdated;

  private String dataSourceVersion;

  private String scrapingSessionId;

  @Valid
  private List<String> alternatePhones = new ArrayList<>();

  private String contactPersonName;

  private String contactPersonTitle;

  private String contactEmail;

  private Integer foundedYear;

  private String businessType;

  @Valid
  private List<String> certifications = new ArrayList<>();

  private String licenseNumber;

  private RevenueRange revenueRange = RevenueRange.UNSPECIFIED;

  private String fundingStage;

  private Boolean isPublicCompany;

  private Float websiteLoadSpeed;

  private Boolean hasSslCertificate;

  private String cmsUsed;

  @Valid
  private List<String> ecommercePlatforms = new ArrayList<>();

  private String timezone;

  private String neighborhood;

  @Valid
  private List<String> nearbyLandmarks = new ArrayList<>();

  private String transportationAccess;

  @Valid
  private List<EmployeeBenefit> employeeBenefits = new ArrayList<>();

  private String parentCompany;

  @Valid
  private List<String> subsidiaries = new ArrayList<>();

  private Boolean isFranchise;

  @Valid
  private List<String> seoKeywords = new ArrayList<>();

  private Boolean usesGoogleAds;

  private String googleMyBusinessCategory;

  private String naicsCode;

  private String sicCode;

  private String unspscCode;

  private Boolean isGreenCertified;

  @Valid
  private List<String> energySources = new ArrayList<>();

  private String sustainabilityRating;

  @Valid
  private List<String> recentAnnouncements = new ArrayList<>();

  @DateTimeFormat(iso = DateTimeFormat.ISO.DATE_TIME)
  private OffsetDateTime lastProductLaunch;

  private Boolean hasLitigationHistory;

  private String exportControlStatus;

  public Lead id(String id) {
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

  public Lead name(String name) {
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

  public Lead website(String website) {
    this.website = website;
    return this;
  }

  /**
   * Get website
   * @return website
   */
  
  @Schema(name = "website", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("website")
  public String getWebsite() {
    return website;
  }

  public void setWebsite(String website) {
    this.website = website;
  }

  public Lead phone(String phone) {
    this.phone = phone;
    return this;
  }

  /**
   * Get phone
   * @return phone
   */
  
  @Schema(name = "phone", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("phone")
  public String getPhone() {
    return phone;
  }

  public void setPhone(String phone) {
    this.phone = phone;
  }

  public Lead address(String address) {
    this.address = address;
    return this;
  }

  /**
   * Get address
   * @return address
   */
  
  @Schema(name = "address", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("address")
  public String getAddress() {
    return address;
  }

  public void setAddress(String address) {
    this.address = address;
  }

  public Lead city(String city) {
    this.city = city;
    return this;
  }

  /**
   * Get city
   * @return city
   */
  
  @Schema(name = "city", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("city")
  public String getCity() {
    return city;
  }

  public void setCity(String city) {
    this.city = city;
  }

  public Lead state(String state) {
    this.state = state;
    return this;
  }

  /**
   * Get state
   * @return state
   */
  
  @Schema(name = "state", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("state")
  public String getState() {
    return state;
  }

  public void setState(String state) {
    this.state = state;
  }

  public Lead country(String country) {
    this.country = country;
    return this;
  }

  /**
   * Get country
   * @return country
   */
  
  @Schema(name = "country", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("country")
  public String getCountry() {
    return country;
  }

  public void setCountry(String country) {
    this.country = country;
  }

  public Lead latitude(Double latitude) {
    this.latitude = latitude;
    return this;
  }

  /**
   * Get latitude
   * @return latitude
   */
  
  @Schema(name = "latitude", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("latitude")
  public Double getLatitude() {
    return latitude;
  }

  public void setLatitude(Double latitude) {
    this.latitude = latitude;
  }

  public Lead longitude(Double longitude) {
    this.longitude = longitude;
    return this;
  }

  /**
   * Get longitude
   * @return longitude
   */
  
  @Schema(name = "longitude", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("longitude")
  public Double getLongitude() {
    return longitude;
  }

  public void setLongitude(Double longitude) {
    this.longitude = longitude;
  }

  public Lead googleRating(Float googleRating) {
    this.googleRating = googleRating;
    return this;
  }

  /**
   * Get googleRating
   * @return googleRating
   */
  
  @Schema(name = "googleRating", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("googleRating")
  public Float getGoogleRating() {
    return googleRating;
  }

  public void setGoogleRating(Float googleRating) {
    this.googleRating = googleRating;
  }

  public Lead reviewCount(Integer reviewCount) {
    this.reviewCount = reviewCount;
    return this;
  }

  /**
   * Get reviewCount
   * @return reviewCount
   */
  
  @Schema(name = "reviewCount", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("reviewCount")
  public Integer getReviewCount() {
    return reviewCount;
  }

  public void setReviewCount(Integer reviewCount) {
    this.reviewCount = reviewCount;
  }

  public Lead industry(String industry) {
    this.industry = industry;
    return this;
  }

  /**
   * Get industry
   * @return industry
   */
  
  @Schema(name = "industry", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("industry")
  public String getIndustry() {
    return industry;
  }

  public void setIndustry(String industry) {
    this.industry = industry;
  }

  public Lead employeeCount(Integer employeeCount) {
    this.employeeCount = employeeCount;
    return this;
  }

  /**
   * Get employeeCount
   * @return employeeCount
   */
  
  @Schema(name = "employeeCount", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("employeeCount")
  public Integer getEmployeeCount() {
    return employeeCount;
  }

  public void setEmployeeCount(Integer employeeCount) {
    this.employeeCount = employeeCount;
  }

  public Lead estimatedRevenue(String estimatedRevenue) {
    this.estimatedRevenue = estimatedRevenue;
    return this;
  }

  /**
   * Get estimatedRevenue
   * @return estimatedRevenue
   */
  
  @Schema(name = "estimatedRevenue", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("estimatedRevenue")
  public String getEstimatedRevenue() {
    return estimatedRevenue;
  }

  public void setEstimatedRevenue(String estimatedRevenue) {
    this.estimatedRevenue = estimatedRevenue;
  }

  public Lead createdAt(OffsetDateTime createdAt) {
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

  public Lead updatedAt(OffsetDateTime updatedAt) {
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

  public Lead deletedAt(OffsetDateTime deletedAt) {
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

  public Lead placeId(String placeId) {
    this.placeId = placeId;
    return this;
  }

  /**
   * Get placeId
   * @return placeId
   */
  
  @Schema(name = "placeId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("placeId")
  public String getPlaceId() {
    return placeId;
  }

  public void setPlaceId(String placeId) {
    this.placeId = placeId;
  }

  public Lead googleMapsUrl(String googleMapsUrl) {
    this.googleMapsUrl = googleMapsUrl;
    return this;
  }

  /**
   * Get googleMapsUrl
   * @return googleMapsUrl
   */
  
  @Schema(name = "googleMapsUrl", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("googleMapsUrl")
  public String getGoogleMapsUrl() {
    return googleMapsUrl;
  }

  public void setGoogleMapsUrl(String googleMapsUrl) {
    this.googleMapsUrl = googleMapsUrl;
  }

  public Lead businessStatus(String businessStatus) {
    this.businessStatus = businessStatus;
    return this;
  }

  /**
   * Get businessStatus
   * @return businessStatus
   */
  
  @Schema(name = "businessStatus", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("businessStatus")
  public String getBusinessStatus() {
    return businessStatus;
  }

  public void setBusinessStatus(String businessStatus) {
    this.businessStatus = businessStatus;
  }

  public Lead regularHours(List<@Valid BusinessHours> regularHours) {
    this.regularHours = regularHours;
    return this;
  }

  public Lead addRegularHoursItem(BusinessHours regularHoursItem) {
    if (this.regularHours == null) {
      this.regularHours = new ArrayList<>();
    }
    this.regularHours.add(regularHoursItem);
    return this;
  }

  /**
   * Get regularHours
   * @return regularHours
   */
  @Valid 
  @Schema(name = "regularHours", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("regularHours")
  public List<@Valid BusinessHours> getRegularHours() {
    return regularHours;
  }

  public void setRegularHours(List<@Valid BusinessHours> regularHours) {
    this.regularHours = regularHours;
  }

  public Lead specialHours(List<@Valid BusinessHours> specialHours) {
    this.specialHours = specialHours;
    return this;
  }

  public Lead addSpecialHoursItem(BusinessHours specialHoursItem) {
    if (this.specialHours == null) {
      this.specialHours = new ArrayList<>();
    }
    this.specialHours.add(specialHoursItem);
    return this;
  }

  /**
   * Get specialHours
   * @return specialHours
   */
  @Valid 
  @Schema(name = "specialHours", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("specialHours")
  public List<@Valid BusinessHours> getSpecialHours() {
    return specialHours;
  }

  public void setSpecialHours(List<@Valid BusinessHours> specialHours) {
    this.specialHours = specialHours;
  }

  public Lead photoReferences(List<String> photoReferences) {
    this.photoReferences = photoReferences;
    return this;
  }

  public Lead addPhotoReferencesItem(String photoReferencesItem) {
    if (this.photoReferences == null) {
      this.photoReferences = new ArrayList<>();
    }
    this.photoReferences.add(photoReferencesItem);
    return this;
  }

  /**
   * Google Places photo IDs
   * @return photoReferences
   */
  
  @Schema(name = "photoReferences", description = "Google Places photo IDs", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("photoReferences")
  public List<String> getPhotoReferences() {
    return photoReferences;
  }

  public void setPhotoReferences(List<String> photoReferences) {
    this.photoReferences = photoReferences;
  }

  public Lead mainPhotoUrl(String mainPhotoUrl) {
    this.mainPhotoUrl = mainPhotoUrl;
    return this;
  }

  /**
   * Get mainPhotoUrl
   * @return mainPhotoUrl
   */
  
  @Schema(name = "mainPhotoUrl", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("mainPhotoUrl")
  public String getMainPhotoUrl() {
    return mainPhotoUrl;
  }

  public void setMainPhotoUrl(String mainPhotoUrl) {
    this.mainPhotoUrl = mainPhotoUrl;
  }

  public Lead reviews(List<@Valid Review> reviews) {
    this.reviews = reviews;
    return this;
  }

  public Lead addReviewsItem(Review reviewsItem) {
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
  public List<@Valid Review> getReviews() {
    return reviews;
  }

  public void setReviews(List<@Valid Review> reviews) {
    this.reviews = reviews;
  }

  public Lead types(List<String> types) {
    this.types = types;
    return this;
  }

  public Lead addTypesItem(String typesItem) {
    if (this.types == null) {
      this.types = new ArrayList<>();
    }
    this.types.add(typesItem);
    return this;
  }

  /**
   * Google Places types
   * @return types
   */
  
  @Schema(name = "types", description = "Google Places types", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("types")
  public List<String> getTypes() {
    return types;
  }

  public void setTypes(List<String> types) {
    this.types = types;
  }

  public Lead amenities(List<String> amenities) {
    this.amenities = amenities;
    return this;
  }

  public Lead addAmenitiesItem(String amenitiesItem) {
    if (this.amenities == null) {
      this.amenities = new ArrayList<>();
    }
    this.amenities.add(amenitiesItem);
    return this;
  }

  /**
   * Get amenities
   * @return amenities
   */
  
  @Schema(name = "amenities", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("amenities")
  public List<String> getAmenities() {
    return amenities;
  }

  public void setAmenities(List<String> amenities) {
    this.amenities = amenities;
  }

  public Lead servesVegetarianFood(Boolean servesVegetarianFood) {
    this.servesVegetarianFood = servesVegetarianFood;
    return this;
  }

  /**
   * Get servesVegetarianFood
   * @return servesVegetarianFood
   */
  
  @Schema(name = "servesVegetarianFood", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("servesVegetarianFood")
  public Boolean getServesVegetarianFood() {
    return servesVegetarianFood;
  }

  public void setServesVegetarianFood(Boolean servesVegetarianFood) {
    this.servesVegetarianFood = servesVegetarianFood;
  }

  public Lead outdoorSeating(Boolean outdoorSeating) {
    this.outdoorSeating = outdoorSeating;
    return this;
  }

  /**
   * Get outdoorSeating
   * @return outdoorSeating
   */
  
  @Schema(name = "outdoorSeating", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("outdoorSeating")
  public Boolean getOutdoorSeating() {
    return outdoorSeating;
  }

  public void setOutdoorSeating(Boolean outdoorSeating) {
    this.outdoorSeating = outdoorSeating;
  }

  public Lead paymentMethods(List<String> paymentMethods) {
    this.paymentMethods = paymentMethods;
    return this;
  }

  public Lead addPaymentMethodsItem(String paymentMethodsItem) {
    if (this.paymentMethods == null) {
      this.paymentMethods = new ArrayList<>();
    }
    this.paymentMethods.add(paymentMethodsItem);
    return this;
  }

  /**
   * visa, amex, cash, etc
   * @return paymentMethods
   */
  
  @Schema(name = "paymentMethods", description = "visa, amex, cash, etc", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("paymentMethods")
  public List<String> getPaymentMethods() {
    return paymentMethods;
  }

  public void setPaymentMethods(List<String> paymentMethods) {
    this.paymentMethods = paymentMethods;
  }

  public Lead wheelchairAccessible(Boolean wheelchairAccessible) {
    this.wheelchairAccessible = wheelchairAccessible;
    return this;
  }

  /**
   * Get wheelchairAccessible
   * @return wheelchairAccessible
   */
  
  @Schema(name = "wheelchairAccessible", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("wheelchairAccessible")
  public Boolean getWheelchairAccessible() {
    return wheelchairAccessible;
  }

  public void setWheelchairAccessible(Boolean wheelchairAccessible) {
    this.wheelchairAccessible = wheelchairAccessible;
  }

  public Lead parkingAvailable(Boolean parkingAvailable) {
    this.parkingAvailable = parkingAvailable;
    return this;
  }

  /**
   * Get parkingAvailable
   * @return parkingAvailable
   */
  
  @Schema(name = "parkingAvailable", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("parkingAvailable")
  public Boolean getParkingAvailable() {
    return parkingAvailable;
  }

  public void setParkingAvailable(Boolean parkingAvailable) {
    this.parkingAvailable = parkingAvailable;
  }

  public Lead socialMedia(Map<String, String> socialMedia) {
    this.socialMedia = socialMedia;
    return this;
  }

  public Lead putSocialMediaItem(String key, String socialMediaItem) {
    if (this.socialMedia == null) {
      this.socialMedia = new HashMap<>();
    }
    this.socialMedia.put(key, socialMediaItem);
    return this;
  }

  /**
   * platform -> url
   * @return socialMedia
   */
  
  @Schema(name = "socialMedia", description = "platform -> url", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("socialMedia")
  public Map<String, String> getSocialMedia() {
    return socialMedia;
  }

  public void setSocialMedia(Map<String, String> socialMedia) {
    this.socialMedia = socialMedia;
  }

  public Lead ratingCategory(String ratingCategory) {
    this.ratingCategory = ratingCategory;
    return this;
  }

  /**
   * Get ratingCategory
   * @return ratingCategory
   */
  
  @Schema(name = "ratingCategory", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("ratingCategory")
  public String getRatingCategory() {
    return ratingCategory;
  }

  public void setRatingCategory(String ratingCategory) {
    this.ratingCategory = ratingCategory;
  }

  public Lead rating(Float rating) {
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

  public Lead count(Integer count) {
    this.count = count;
    return this;
  }

  /**
   * Get count
   * @return count
   */
  
  @Schema(name = "count", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("count")
  public Integer getCount() {
    return count;
  }

  public void setCount(Integer count) {
    this.count = count;
  }

  public Lead lastUpdated(OffsetDateTime lastUpdated) {
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

  public Lead dataSourceVersion(String dataSourceVersion) {
    this.dataSourceVersion = dataSourceVersion;
    return this;
  }

  /**
   * Get dataSourceVersion
   * @return dataSourceVersion
   */
  
  @Schema(name = "dataSourceVersion", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("dataSourceVersion")
  public String getDataSourceVersion() {
    return dataSourceVersion;
  }

  public void setDataSourceVersion(String dataSourceVersion) {
    this.dataSourceVersion = dataSourceVersion;
  }

  public Lead scrapingSessionId(String scrapingSessionId) {
    this.scrapingSessionId = scrapingSessionId;
    return this;
  }

  /**
   * Get scrapingSessionId
   * @return scrapingSessionId
   */
  
  @Schema(name = "scrapingSessionId", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("scrapingSessionId")
  public String getScrapingSessionId() {
    return scrapingSessionId;
  }

  public void setScrapingSessionId(String scrapingSessionId) {
    this.scrapingSessionId = scrapingSessionId;
  }

  public Lead alternatePhones(List<String> alternatePhones) {
    this.alternatePhones = alternatePhones;
    return this;
  }

  public Lead addAlternatePhonesItem(String alternatePhonesItem) {
    if (this.alternatePhones == null) {
      this.alternatePhones = new ArrayList<>();
    }
    this.alternatePhones.add(alternatePhonesItem);
    return this;
  }

  /**
   * Get alternatePhones
   * @return alternatePhones
   */
  
  @Schema(name = "alternatePhones", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("alternatePhones")
  public List<String> getAlternatePhones() {
    return alternatePhones;
  }

  public void setAlternatePhones(List<String> alternatePhones) {
    this.alternatePhones = alternatePhones;
  }

  public Lead contactPersonName(String contactPersonName) {
    this.contactPersonName = contactPersonName;
    return this;
  }

  /**
   * Get contactPersonName
   * @return contactPersonName
   */
  
  @Schema(name = "contactPersonName", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("contactPersonName")
  public String getContactPersonName() {
    return contactPersonName;
  }

  public void setContactPersonName(String contactPersonName) {
    this.contactPersonName = contactPersonName;
  }

  public Lead contactPersonTitle(String contactPersonTitle) {
    this.contactPersonTitle = contactPersonTitle;
    return this;
  }

  /**
   * Get contactPersonTitle
   * @return contactPersonTitle
   */
  
  @Schema(name = "contactPersonTitle", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("contactPersonTitle")
  public String getContactPersonTitle() {
    return contactPersonTitle;
  }

  public void setContactPersonTitle(String contactPersonTitle) {
    this.contactPersonTitle = contactPersonTitle;
  }

  public Lead contactEmail(String contactEmail) {
    this.contactEmail = contactEmail;
    return this;
  }

  /**
   * Get contactEmail
   * @return contactEmail
   */
  
  @Schema(name = "contactEmail", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("contactEmail")
  public String getContactEmail() {
    return contactEmail;
  }

  public void setContactEmail(String contactEmail) {
    this.contactEmail = contactEmail;
  }

  public Lead foundedYear(Integer foundedYear) {
    this.foundedYear = foundedYear;
    return this;
  }

  /**
   * Get foundedYear
   * @return foundedYear
   */
  
  @Schema(name = "foundedYear", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("foundedYear")
  public Integer getFoundedYear() {
    return foundedYear;
  }

  public void setFoundedYear(Integer foundedYear) {
    this.foundedYear = foundedYear;
  }

  public Lead businessType(String businessType) {
    this.businessType = businessType;
    return this;
  }

  /**
   * Get businessType
   * @return businessType
   */
  
  @Schema(name = "businessType", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("businessType")
  public String getBusinessType() {
    return businessType;
  }

  public void setBusinessType(String businessType) {
    this.businessType = businessType;
  }

  public Lead certifications(List<String> certifications) {
    this.certifications = certifications;
    return this;
  }

  public Lead addCertificationsItem(String certificationsItem) {
    if (this.certifications == null) {
      this.certifications = new ArrayList<>();
    }
    this.certifications.add(certificationsItem);
    return this;
  }

  /**
   * Get certifications
   * @return certifications
   */
  
  @Schema(name = "certifications", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("certifications")
  public List<String> getCertifications() {
    return certifications;
  }

  public void setCertifications(List<String> certifications) {
    this.certifications = certifications;
  }

  public Lead licenseNumber(String licenseNumber) {
    this.licenseNumber = licenseNumber;
    return this;
  }

  /**
   * Get licenseNumber
   * @return licenseNumber
   */
  
  @Schema(name = "licenseNumber", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("licenseNumber")
  public String getLicenseNumber() {
    return licenseNumber;
  }

  public void setLicenseNumber(String licenseNumber) {
    this.licenseNumber = licenseNumber;
  }

  public Lead revenueRange(RevenueRange revenueRange) {
    this.revenueRange = revenueRange;
    return this;
  }

  /**
   * Get revenueRange
   * @return revenueRange
   */
  @Valid 
  @Schema(name = "revenueRange", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("revenueRange")
  public RevenueRange getRevenueRange() {
    return revenueRange;
  }

  public void setRevenueRange(RevenueRange revenueRange) {
    this.revenueRange = revenueRange;
  }

  public Lead fundingStage(String fundingStage) {
    this.fundingStage = fundingStage;
    return this;
  }

  /**
   * Get fundingStage
   * @return fundingStage
   */
  
  @Schema(name = "fundingStage", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("fundingStage")
  public String getFundingStage() {
    return fundingStage;
  }

  public void setFundingStage(String fundingStage) {
    this.fundingStage = fundingStage;
  }

  public Lead isPublicCompany(Boolean isPublicCompany) {
    this.isPublicCompany = isPublicCompany;
    return this;
  }

  /**
   * Get isPublicCompany
   * @return isPublicCompany
   */
  
  @Schema(name = "isPublicCompany", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("isPublicCompany")
  public Boolean getIsPublicCompany() {
    return isPublicCompany;
  }

  public void setIsPublicCompany(Boolean isPublicCompany) {
    this.isPublicCompany = isPublicCompany;
  }

  public Lead websiteLoadSpeed(Float websiteLoadSpeed) {
    this.websiteLoadSpeed = websiteLoadSpeed;
    return this;
  }

  /**
   * In seconds
   * @return websiteLoadSpeed
   */
  
  @Schema(name = "websiteLoadSpeed", description = "In seconds", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("websiteLoadSpeed")
  public Float getWebsiteLoadSpeed() {
    return websiteLoadSpeed;
  }

  public void setWebsiteLoadSpeed(Float websiteLoadSpeed) {
    this.websiteLoadSpeed = websiteLoadSpeed;
  }

  public Lead hasSslCertificate(Boolean hasSslCertificate) {
    this.hasSslCertificate = hasSslCertificate;
    return this;
  }

  /**
   * Get hasSslCertificate
   * @return hasSslCertificate
   */
  
  @Schema(name = "hasSslCertificate", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("hasSslCertificate")
  public Boolean getHasSslCertificate() {
    return hasSslCertificate;
  }

  public void setHasSslCertificate(Boolean hasSslCertificate) {
    this.hasSslCertificate = hasSslCertificate;
  }

  public Lead cmsUsed(String cmsUsed) {
    this.cmsUsed = cmsUsed;
    return this;
  }

  /**
   * Get cmsUsed
   * @return cmsUsed
   */
  
  @Schema(name = "cmsUsed", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("cmsUsed")
  public String getCmsUsed() {
    return cmsUsed;
  }

  public void setCmsUsed(String cmsUsed) {
    this.cmsUsed = cmsUsed;
  }

  public Lead ecommercePlatforms(List<String> ecommercePlatforms) {
    this.ecommercePlatforms = ecommercePlatforms;
    return this;
  }

  public Lead addEcommercePlatformsItem(String ecommercePlatformsItem) {
    if (this.ecommercePlatforms == null) {
      this.ecommercePlatforms = new ArrayList<>();
    }
    this.ecommercePlatforms.add(ecommercePlatformsItem);
    return this;
  }

  /**
   * Get ecommercePlatforms
   * @return ecommercePlatforms
   */
  
  @Schema(name = "ecommercePlatforms", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("ecommercePlatforms")
  public List<String> getEcommercePlatforms() {
    return ecommercePlatforms;
  }

  public void setEcommercePlatforms(List<String> ecommercePlatforms) {
    this.ecommercePlatforms = ecommercePlatforms;
  }

  public Lead timezone(String timezone) {
    this.timezone = timezone;
    return this;
  }

  /**
   * Get timezone
   * @return timezone
   */
  
  @Schema(name = "timezone", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("timezone")
  public String getTimezone() {
    return timezone;
  }

  public void setTimezone(String timezone) {
    this.timezone = timezone;
  }

  public Lead neighborhood(String neighborhood) {
    this.neighborhood = neighborhood;
    return this;
  }

  /**
   * Get neighborhood
   * @return neighborhood
   */
  
  @Schema(name = "neighborhood", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("neighborhood")
  public String getNeighborhood() {
    return neighborhood;
  }

  public void setNeighborhood(String neighborhood) {
    this.neighborhood = neighborhood;
  }

  public Lead nearbyLandmarks(List<String> nearbyLandmarks) {
    this.nearbyLandmarks = nearbyLandmarks;
    return this;
  }

  public Lead addNearbyLandmarksItem(String nearbyLandmarksItem) {
    if (this.nearbyLandmarks == null) {
      this.nearbyLandmarks = new ArrayList<>();
    }
    this.nearbyLandmarks.add(nearbyLandmarksItem);
    return this;
  }

  /**
   * Get nearbyLandmarks
   * @return nearbyLandmarks
   */
  
  @Schema(name = "nearbyLandmarks", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("nearbyLandmarks")
  public List<String> getNearbyLandmarks() {
    return nearbyLandmarks;
  }

  public void setNearbyLandmarks(List<String> nearbyLandmarks) {
    this.nearbyLandmarks = nearbyLandmarks;
  }

  public Lead transportationAccess(String transportationAccess) {
    this.transportationAccess = transportationAccess;
    return this;
  }

  /**
   * Get transportationAccess
   * @return transportationAccess
   */
  
  @Schema(name = "transportationAccess", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("transportationAccess")
  public String getTransportationAccess() {
    return transportationAccess;
  }

  public void setTransportationAccess(String transportationAccess) {
    this.transportationAccess = transportationAccess;
  }

  public Lead employeeBenefits(List<EmployeeBenefit> employeeBenefits) {
    this.employeeBenefits = employeeBenefits;
    return this;
  }

  public Lead addEmployeeBenefitsItem(EmployeeBenefit employeeBenefitsItem) {
    if (this.employeeBenefits == null) {
      this.employeeBenefits = new ArrayList<>();
    }
    this.employeeBenefits.add(employeeBenefitsItem);
    return this;
  }

  /**
   * Get employeeBenefits
   * @return employeeBenefits
   */
  @Valid 
  @Schema(name = "employeeBenefits", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("employeeBenefits")
  public List<EmployeeBenefit> getEmployeeBenefits() {
    return employeeBenefits;
  }

  public void setEmployeeBenefits(List<EmployeeBenefit> employeeBenefits) {
    this.employeeBenefits = employeeBenefits;
  }

  public Lead parentCompany(String parentCompany) {
    this.parentCompany = parentCompany;
    return this;
  }

  /**
   * Get parentCompany
   * @return parentCompany
   */
  
  @Schema(name = "parentCompany", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("parentCompany")
  public String getParentCompany() {
    return parentCompany;
  }

  public void setParentCompany(String parentCompany) {
    this.parentCompany = parentCompany;
  }

  public Lead subsidiaries(List<String> subsidiaries) {
    this.subsidiaries = subsidiaries;
    return this;
  }

  public Lead addSubsidiariesItem(String subsidiariesItem) {
    if (this.subsidiaries == null) {
      this.subsidiaries = new ArrayList<>();
    }
    this.subsidiaries.add(subsidiariesItem);
    return this;
  }

  /**
   * Get subsidiaries
   * @return subsidiaries
   */
  
  @Schema(name = "subsidiaries", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("subsidiaries")
  public List<String> getSubsidiaries() {
    return subsidiaries;
  }

  public void setSubsidiaries(List<String> subsidiaries) {
    this.subsidiaries = subsidiaries;
  }

  public Lead isFranchise(Boolean isFranchise) {
    this.isFranchise = isFranchise;
    return this;
  }

  /**
   * Get isFranchise
   * @return isFranchise
   */
  
  @Schema(name = "isFranchise", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("isFranchise")
  public Boolean getIsFranchise() {
    return isFranchise;
  }

  public void setIsFranchise(Boolean isFranchise) {
    this.isFranchise = isFranchise;
  }

  public Lead seoKeywords(List<String> seoKeywords) {
    this.seoKeywords = seoKeywords;
    return this;
  }

  public Lead addSeoKeywordsItem(String seoKeywordsItem) {
    if (this.seoKeywords == null) {
      this.seoKeywords = new ArrayList<>();
    }
    this.seoKeywords.add(seoKeywordsItem);
    return this;
  }

  /**
   * Get seoKeywords
   * @return seoKeywords
   */
  
  @Schema(name = "seoKeywords", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("seoKeywords")
  public List<String> getSeoKeywords() {
    return seoKeywords;
  }

  public void setSeoKeywords(List<String> seoKeywords) {
    this.seoKeywords = seoKeywords;
  }

  public Lead usesGoogleAds(Boolean usesGoogleAds) {
    this.usesGoogleAds = usesGoogleAds;
    return this;
  }

  /**
   * Get usesGoogleAds
   * @return usesGoogleAds
   */
  
  @Schema(name = "usesGoogleAds", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("usesGoogleAds")
  public Boolean getUsesGoogleAds() {
    return usesGoogleAds;
  }

  public void setUsesGoogleAds(Boolean usesGoogleAds) {
    this.usesGoogleAds = usesGoogleAds;
  }

  public Lead googleMyBusinessCategory(String googleMyBusinessCategory) {
    this.googleMyBusinessCategory = googleMyBusinessCategory;
    return this;
  }

  /**
   * Get googleMyBusinessCategory
   * @return googleMyBusinessCategory
   */
  
  @Schema(name = "googleMyBusinessCategory", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("googleMyBusinessCategory")
  public String getGoogleMyBusinessCategory() {
    return googleMyBusinessCategory;
  }

  public void setGoogleMyBusinessCategory(String googleMyBusinessCategory) {
    this.googleMyBusinessCategory = googleMyBusinessCategory;
  }

  public Lead naicsCode(String naicsCode) {
    this.naicsCode = naicsCode;
    return this;
  }

  /**
   * Get naicsCode
   * @return naicsCode
   */
  
  @Schema(name = "naicsCode", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("naicsCode")
  public String getNaicsCode() {
    return naicsCode;
  }

  public void setNaicsCode(String naicsCode) {
    this.naicsCode = naicsCode;
  }

  public Lead sicCode(String sicCode) {
    this.sicCode = sicCode;
    return this;
  }

  /**
   * Get sicCode
   * @return sicCode
   */
  
  @Schema(name = "sicCode", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("sicCode")
  public String getSicCode() {
    return sicCode;
  }

  public void setSicCode(String sicCode) {
    this.sicCode = sicCode;
  }

  public Lead unspscCode(String unspscCode) {
    this.unspscCode = unspscCode;
    return this;
  }

  /**
   * Get unspscCode
   * @return unspscCode
   */
  
  @Schema(name = "unspscCode", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("unspscCode")
  public String getUnspscCode() {
    return unspscCode;
  }

  public void setUnspscCode(String unspscCode) {
    this.unspscCode = unspscCode;
  }

  public Lead isGreenCertified(Boolean isGreenCertified) {
    this.isGreenCertified = isGreenCertified;
    return this;
  }

  /**
   * Get isGreenCertified
   * @return isGreenCertified
   */
  
  @Schema(name = "isGreenCertified", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("isGreenCertified")
  public Boolean getIsGreenCertified() {
    return isGreenCertified;
  }

  public void setIsGreenCertified(Boolean isGreenCertified) {
    this.isGreenCertified = isGreenCertified;
  }

  public Lead energySources(List<String> energySources) {
    this.energySources = energySources;
    return this;
  }

  public Lead addEnergySourcesItem(String energySourcesItem) {
    if (this.energySources == null) {
      this.energySources = new ArrayList<>();
    }
    this.energySources.add(energySourcesItem);
    return this;
  }

  /**
   * Get energySources
   * @return energySources
   */
  
  @Schema(name = "energySources", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("energySources")
  public List<String> getEnergySources() {
    return energySources;
  }

  public void setEnergySources(List<String> energySources) {
    this.energySources = energySources;
  }

  public Lead sustainabilityRating(String sustainabilityRating) {
    this.sustainabilityRating = sustainabilityRating;
    return this;
  }

  /**
   * Get sustainabilityRating
   * @return sustainabilityRating
   */
  
  @Schema(name = "sustainabilityRating", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("sustainabilityRating")
  public String getSustainabilityRating() {
    return sustainabilityRating;
  }

  public void setSustainabilityRating(String sustainabilityRating) {
    this.sustainabilityRating = sustainabilityRating;
  }

  public Lead recentAnnouncements(List<String> recentAnnouncements) {
    this.recentAnnouncements = recentAnnouncements;
    return this;
  }

  public Lead addRecentAnnouncementsItem(String recentAnnouncementsItem) {
    if (this.recentAnnouncements == null) {
      this.recentAnnouncements = new ArrayList<>();
    }
    this.recentAnnouncements.add(recentAnnouncementsItem);
    return this;
  }

  /**
   * Get recentAnnouncements
   * @return recentAnnouncements
   */
  
  @Schema(name = "recentAnnouncements", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("recentAnnouncements")
  public List<String> getRecentAnnouncements() {
    return recentAnnouncements;
  }

  public void setRecentAnnouncements(List<String> recentAnnouncements) {
    this.recentAnnouncements = recentAnnouncements;
  }

  public Lead lastProductLaunch(OffsetDateTime lastProductLaunch) {
    this.lastProductLaunch = lastProductLaunch;
    return this;
  }

  /**
   * Get lastProductLaunch
   * @return lastProductLaunch
   */
  @Valid 
  @Schema(name = "lastProductLaunch", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("lastProductLaunch")
  public OffsetDateTime getLastProductLaunch() {
    return lastProductLaunch;
  }

  public void setLastProductLaunch(OffsetDateTime lastProductLaunch) {
    this.lastProductLaunch = lastProductLaunch;
  }

  public Lead hasLitigationHistory(Boolean hasLitigationHistory) {
    this.hasLitigationHistory = hasLitigationHistory;
    return this;
  }

  /**
   * Get hasLitigationHistory
   * @return hasLitigationHistory
   */
  
  @Schema(name = "hasLitigationHistory", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("hasLitigationHistory")
  public Boolean getHasLitigationHistory() {
    return hasLitigationHistory;
  }

  public void setHasLitigationHistory(Boolean hasLitigationHistory) {
    this.hasLitigationHistory = hasLitigationHistory;
  }

  public Lead exportControlStatus(String exportControlStatus) {
    this.exportControlStatus = exportControlStatus;
    return this;
  }

  /**
   * Get exportControlStatus
   * @return exportControlStatus
   */
  
  @Schema(name = "exportControlStatus", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
  @JsonProperty("exportControlStatus")
  public String getExportControlStatus() {
    return exportControlStatus;
  }

  public void setExportControlStatus(String exportControlStatus) {
    this.exportControlStatus = exportControlStatus;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    Lead lead = (Lead) o;
    return Objects.equals(this.id, lead.id) &&
        Objects.equals(this.name, lead.name) &&
        Objects.equals(this.website, lead.website) &&
        Objects.equals(this.phone, lead.phone) &&
        Objects.equals(this.address, lead.address) &&
        Objects.equals(this.city, lead.city) &&
        Objects.equals(this.state, lead.state) &&
        Objects.equals(this.country, lead.country) &&
        Objects.equals(this.latitude, lead.latitude) &&
        Objects.equals(this.longitude, lead.longitude) &&
        Objects.equals(this.googleRating, lead.googleRating) &&
        Objects.equals(this.reviewCount, lead.reviewCount) &&
        Objects.equals(this.industry, lead.industry) &&
        Objects.equals(this.employeeCount, lead.employeeCount) &&
        Objects.equals(this.estimatedRevenue, lead.estimatedRevenue) &&
        Objects.equals(this.createdAt, lead.createdAt) &&
        Objects.equals(this.updatedAt, lead.updatedAt) &&
        Objects.equals(this.deletedAt, lead.deletedAt) &&
        Objects.equals(this.placeId, lead.placeId) &&
        Objects.equals(this.googleMapsUrl, lead.googleMapsUrl) &&
        Objects.equals(this.businessStatus, lead.businessStatus) &&
        Objects.equals(this.regularHours, lead.regularHours) &&
        Objects.equals(this.specialHours, lead.specialHours) &&
        Objects.equals(this.photoReferences, lead.photoReferences) &&
        Objects.equals(this.mainPhotoUrl, lead.mainPhotoUrl) &&
        Objects.equals(this.reviews, lead.reviews) &&
        Objects.equals(this.types, lead.types) &&
        Objects.equals(this.amenities, lead.amenities) &&
        Objects.equals(this.servesVegetarianFood, lead.servesVegetarianFood) &&
        Objects.equals(this.outdoorSeating, lead.outdoorSeating) &&
        Objects.equals(this.paymentMethods, lead.paymentMethods) &&
        Objects.equals(this.wheelchairAccessible, lead.wheelchairAccessible) &&
        Objects.equals(this.parkingAvailable, lead.parkingAvailable) &&
        Objects.equals(this.socialMedia, lead.socialMedia) &&
        Objects.equals(this.ratingCategory, lead.ratingCategory) &&
        Objects.equals(this.rating, lead.rating) &&
        Objects.equals(this.count, lead.count) &&
        Objects.equals(this.lastUpdated, lead.lastUpdated) &&
        Objects.equals(this.dataSourceVersion, lead.dataSourceVersion) &&
        Objects.equals(this.scrapingSessionId, lead.scrapingSessionId) &&
        Objects.equals(this.alternatePhones, lead.alternatePhones) &&
        Objects.equals(this.contactPersonName, lead.contactPersonName) &&
        Objects.equals(this.contactPersonTitle, lead.contactPersonTitle) &&
        Objects.equals(this.contactEmail, lead.contactEmail) &&
        Objects.equals(this.foundedYear, lead.foundedYear) &&
        Objects.equals(this.businessType, lead.businessType) &&
        Objects.equals(this.certifications, lead.certifications) &&
        Objects.equals(this.licenseNumber, lead.licenseNumber) &&
        Objects.equals(this.revenueRange, lead.revenueRange) &&
        Objects.equals(this.fundingStage, lead.fundingStage) &&
        Objects.equals(this.isPublicCompany, lead.isPublicCompany) &&
        Objects.equals(this.websiteLoadSpeed, lead.websiteLoadSpeed) &&
        Objects.equals(this.hasSslCertificate, lead.hasSslCertificate) &&
        Objects.equals(this.cmsUsed, lead.cmsUsed) &&
        Objects.equals(this.ecommercePlatforms, lead.ecommercePlatforms) &&
        Objects.equals(this.timezone, lead.timezone) &&
        Objects.equals(this.neighborhood, lead.neighborhood) &&
        Objects.equals(this.nearbyLandmarks, lead.nearbyLandmarks) &&
        Objects.equals(this.transportationAccess, lead.transportationAccess) &&
        Objects.equals(this.employeeBenefits, lead.employeeBenefits) &&
        Objects.equals(this.parentCompany, lead.parentCompany) &&
        Objects.equals(this.subsidiaries, lead.subsidiaries) &&
        Objects.equals(this.isFranchise, lead.isFranchise) &&
        Objects.equals(this.seoKeywords, lead.seoKeywords) &&
        Objects.equals(this.usesGoogleAds, lead.usesGoogleAds) &&
        Objects.equals(this.googleMyBusinessCategory, lead.googleMyBusinessCategory) &&
        Objects.equals(this.naicsCode, lead.naicsCode) &&
        Objects.equals(this.sicCode, lead.sicCode) &&
        Objects.equals(this.unspscCode, lead.unspscCode) &&
        Objects.equals(this.isGreenCertified, lead.isGreenCertified) &&
        Objects.equals(this.energySources, lead.energySources) &&
        Objects.equals(this.sustainabilityRating, lead.sustainabilityRating) &&
        Objects.equals(this.recentAnnouncements, lead.recentAnnouncements) &&
        Objects.equals(this.lastProductLaunch, lead.lastProductLaunch) &&
        Objects.equals(this.hasLitigationHistory, lead.hasLitigationHistory) &&
        Objects.equals(this.exportControlStatus, lead.exportControlStatus);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, name, website, phone, address, city, state, country, latitude, longitude, googleRating, reviewCount, industry, employeeCount, estimatedRevenue, createdAt, updatedAt, deletedAt, placeId, googleMapsUrl, businessStatus, regularHours, specialHours, photoReferences, mainPhotoUrl, reviews, types, amenities, servesVegetarianFood, outdoorSeating, paymentMethods, wheelchairAccessible, parkingAvailable, socialMedia, ratingCategory, rating, count, lastUpdated, dataSourceVersion, scrapingSessionId, alternatePhones, contactPersonName, contactPersonTitle, contactEmail, foundedYear, businessType, certifications, licenseNumber, revenueRange, fundingStage, isPublicCompany, websiteLoadSpeed, hasSslCertificate, cmsUsed, ecommercePlatforms, timezone, neighborhood, nearbyLandmarks, transportationAccess, employeeBenefits, parentCompany, subsidiaries, isFranchise, seoKeywords, usesGoogleAds, googleMyBusinessCategory, naicsCode, sicCode, unspscCode, isGreenCertified, energySources, sustainabilityRating, recentAnnouncements, lastProductLaunch, hasLitigationHistory, exportControlStatus);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class Lead {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    name: ").append(toIndentedString(name)).append("\n");
    sb.append("    website: ").append(toIndentedString(website)).append("\n");
    sb.append("    phone: ").append(toIndentedString(phone)).append("\n");
    sb.append("    address: ").append(toIndentedString(address)).append("\n");
    sb.append("    city: ").append(toIndentedString(city)).append("\n");
    sb.append("    state: ").append(toIndentedString(state)).append("\n");
    sb.append("    country: ").append(toIndentedString(country)).append("\n");
    sb.append("    latitude: ").append(toIndentedString(latitude)).append("\n");
    sb.append("    longitude: ").append(toIndentedString(longitude)).append("\n");
    sb.append("    googleRating: ").append(toIndentedString(googleRating)).append("\n");
    sb.append("    reviewCount: ").append(toIndentedString(reviewCount)).append("\n");
    sb.append("    industry: ").append(toIndentedString(industry)).append("\n");
    sb.append("    employeeCount: ").append(toIndentedString(employeeCount)).append("\n");
    sb.append("    estimatedRevenue: ").append(toIndentedString(estimatedRevenue)).append("\n");
    sb.append("    createdAt: ").append(toIndentedString(createdAt)).append("\n");
    sb.append("    updatedAt: ").append(toIndentedString(updatedAt)).append("\n");
    sb.append("    deletedAt: ").append(toIndentedString(deletedAt)).append("\n");
    sb.append("    placeId: ").append(toIndentedString(placeId)).append("\n");
    sb.append("    googleMapsUrl: ").append(toIndentedString(googleMapsUrl)).append("\n");
    sb.append("    businessStatus: ").append(toIndentedString(businessStatus)).append("\n");
    sb.append("    regularHours: ").append(toIndentedString(regularHours)).append("\n");
    sb.append("    specialHours: ").append(toIndentedString(specialHours)).append("\n");
    sb.append("    photoReferences: ").append(toIndentedString(photoReferences)).append("\n");
    sb.append("    mainPhotoUrl: ").append(toIndentedString(mainPhotoUrl)).append("\n");
    sb.append("    reviews: ").append(toIndentedString(reviews)).append("\n");
    sb.append("    types: ").append(toIndentedString(types)).append("\n");
    sb.append("    amenities: ").append(toIndentedString(amenities)).append("\n");
    sb.append("    servesVegetarianFood: ").append(toIndentedString(servesVegetarianFood)).append("\n");
    sb.append("    outdoorSeating: ").append(toIndentedString(outdoorSeating)).append("\n");
    sb.append("    paymentMethods: ").append(toIndentedString(paymentMethods)).append("\n");
    sb.append("    wheelchairAccessible: ").append(toIndentedString(wheelchairAccessible)).append("\n");
    sb.append("    parkingAvailable: ").append(toIndentedString(parkingAvailable)).append("\n");
    sb.append("    socialMedia: ").append(toIndentedString(socialMedia)).append("\n");
    sb.append("    ratingCategory: ").append(toIndentedString(ratingCategory)).append("\n");
    sb.append("    rating: ").append(toIndentedString(rating)).append("\n");
    sb.append("    count: ").append(toIndentedString(count)).append("\n");
    sb.append("    lastUpdated: ").append(toIndentedString(lastUpdated)).append("\n");
    sb.append("    dataSourceVersion: ").append(toIndentedString(dataSourceVersion)).append("\n");
    sb.append("    scrapingSessionId: ").append(toIndentedString(scrapingSessionId)).append("\n");
    sb.append("    alternatePhones: ").append(toIndentedString(alternatePhones)).append("\n");
    sb.append("    contactPersonName: ").append(toIndentedString(contactPersonName)).append("\n");
    sb.append("    contactPersonTitle: ").append(toIndentedString(contactPersonTitle)).append("\n");
    sb.append("    contactEmail: ").append(toIndentedString(contactEmail)).append("\n");
    sb.append("    foundedYear: ").append(toIndentedString(foundedYear)).append("\n");
    sb.append("    businessType: ").append(toIndentedString(businessType)).append("\n");
    sb.append("    certifications: ").append(toIndentedString(certifications)).append("\n");
    sb.append("    licenseNumber: ").append(toIndentedString(licenseNumber)).append("\n");
    sb.append("    revenueRange: ").append(toIndentedString(revenueRange)).append("\n");
    sb.append("    fundingStage: ").append(toIndentedString(fundingStage)).append("\n");
    sb.append("    isPublicCompany: ").append(toIndentedString(isPublicCompany)).append("\n");
    sb.append("    websiteLoadSpeed: ").append(toIndentedString(websiteLoadSpeed)).append("\n");
    sb.append("    hasSslCertificate: ").append(toIndentedString(hasSslCertificate)).append("\n");
    sb.append("    cmsUsed: ").append(toIndentedString(cmsUsed)).append("\n");
    sb.append("    ecommercePlatforms: ").append(toIndentedString(ecommercePlatforms)).append("\n");
    sb.append("    timezone: ").append(toIndentedString(timezone)).append("\n");
    sb.append("    neighborhood: ").append(toIndentedString(neighborhood)).append("\n");
    sb.append("    nearbyLandmarks: ").append(toIndentedString(nearbyLandmarks)).append("\n");
    sb.append("    transportationAccess: ").append(toIndentedString(transportationAccess)).append("\n");
    sb.append("    employeeBenefits: ").append(toIndentedString(employeeBenefits)).append("\n");
    sb.append("    parentCompany: ").append(toIndentedString(parentCompany)).append("\n");
    sb.append("    subsidiaries: ").append(toIndentedString(subsidiaries)).append("\n");
    sb.append("    isFranchise: ").append(toIndentedString(isFranchise)).append("\n");
    sb.append("    seoKeywords: ").append(toIndentedString(seoKeywords)).append("\n");
    sb.append("    usesGoogleAds: ").append(toIndentedString(usesGoogleAds)).append("\n");
    sb.append("    googleMyBusinessCategory: ").append(toIndentedString(googleMyBusinessCategory)).append("\n");
    sb.append("    naicsCode: ").append(toIndentedString(naicsCode)).append("\n");
    sb.append("    sicCode: ").append(toIndentedString(sicCode)).append("\n");
    sb.append("    unspscCode: ").append(toIndentedString(unspscCode)).append("\n");
    sb.append("    isGreenCertified: ").append(toIndentedString(isGreenCertified)).append("\n");
    sb.append("    energySources: ").append(toIndentedString(energySources)).append("\n");
    sb.append("    sustainabilityRating: ").append(toIndentedString(sustainabilityRating)).append("\n");
    sb.append("    recentAnnouncements: ").append(toIndentedString(recentAnnouncements)).append("\n");
    sb.append("    lastProductLaunch: ").append(toIndentedString(lastProductLaunch)).append("\n");
    sb.append("    hasLitigationHistory: ").append(toIndentedString(hasLitigationHistory)).append("\n");
    sb.append("    exportControlStatus: ").append(toIndentedString(exportControlStatus)).append("\n");
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

