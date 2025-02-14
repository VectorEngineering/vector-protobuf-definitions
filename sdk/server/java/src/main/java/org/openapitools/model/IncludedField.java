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
 * - INCLUDED_FIELD_NAME: Core business information  - INCLUDED_FIELD_LOCATION: city, state, country  - INCLUDED_FIELD_COORDINATES: Geographic data  latitude, longitude  - INCLUDED_FIELD_GOOGLE_RATING: Ratings and reviews  - INCLUDED_FIELD_BUSINESS_HOURS: Business details  - INCLUDED_FIELD_PHOTOS: Rich media  - INCLUDED_FIELD_BUSINESS_TYPES: Categories and attributes  - INCLUDED_FIELD_SOCIAL_PROFILES: Social media  - INCLUDED_FIELD_EMPLOYEE_COUNT: Enhanced business data  - INCLUDED_FIELD_NAICS_CODE: Industry classification  - INCLUDED_FIELD_SCRAPING_METADATA: Metadata  timestamps, session info  - INCLUDED_FIELD_COMPLIANCE_INFO: sustainability, certifications  - INCLUDED_FIELD_ALTERNATE_PHONES: Contact information
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-14T00:28:49.378974-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum IncludedField {
  
  UNSPECIFIED("INCLUDED_FIELD_UNSPECIFIED"),
  
  NAME("INCLUDED_FIELD_NAME"),
  
  WEBSITE("INCLUDED_FIELD_WEBSITE"),
  
  PHONE("INCLUDED_FIELD_PHONE"),
  
  ADDRESS("INCLUDED_FIELD_ADDRESS"),
  
  LOCATION("INCLUDED_FIELD_LOCATION"),
  
  COORDINATES("INCLUDED_FIELD_COORDINATES"),
  
  GOOGLE_RATING("INCLUDED_FIELD_GOOGLE_RATING"),
  
  REVIEW_COUNT("INCLUDED_FIELD_REVIEW_COUNT"),
  
  REVIEWS("INCLUDED_FIELD_REVIEWS"),
  
  BUSINESS_HOURS("INCLUDED_FIELD_BUSINESS_HOURS"),
  
  BUSINESS_STATUS("INCLUDED_FIELD_BUSINESS_STATUS"),
  
  PLACE_ID("INCLUDED_FIELD_PLACE_ID"),
  
  GOOGLE_MAPS_URL("INCLUDED_FIELD_GOOGLE_MAPS_URL"),
  
  PHOTOS("INCLUDED_FIELD_PHOTOS"),
  
  MAIN_PHOTO("INCLUDED_FIELD_MAIN_PHOTO"),
  
  BUSINESS_TYPES("INCLUDED_FIELD_BUSINESS_TYPES"),
  
  AMENITIES("INCLUDED_FIELD_AMENITIES"),
  
  PAYMENT_METHODS("INCLUDED_FIELD_PAYMENT_METHODS"),
  
  SOCIAL_PROFILES("INCLUDED_FIELD_SOCIAL_PROFILES"),
  
  EMPLOYEE_COUNT("INCLUDED_FIELD_EMPLOYEE_COUNT"),
  
  REVENUE_INFO("INCLUDED_FIELD_REVENUE_INFO"),
  
  FOUNDED_YEAR("INCLUDED_FIELD_FOUNDED_YEAR"),
  
  CERTIFICATIONS("INCLUDED_FIELD_CERTIFICATIONS"),
  
  NAICS_CODE("INCLUDED_FIELD_NAICS_CODE"),
  
  SIC_CODE("INCLUDED_FIELD_SIC_CODE"),
  
  SCRAPING_METADATA("INCLUDED_FIELD_SCRAPING_METADATA"),
  
  COMPLIANCE_INFO("INCLUDED_FIELD_COMPLIANCE_INFO"),
  
  ALTERNATE_PHONES("INCLUDED_FIELD_ALTERNATE_PHONES"),
  
  CONTACT_PERSON("INCLUDED_FIELD_CONTACT_PERSON"),
  
  CONTACT_EMAIL("INCLUDED_FIELD_CONTACT_EMAIL");

  private String value;

  IncludedField(String value) {
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
  public static IncludedField fromValue(String value) {
    for (IncludedField b : IncludedField.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

