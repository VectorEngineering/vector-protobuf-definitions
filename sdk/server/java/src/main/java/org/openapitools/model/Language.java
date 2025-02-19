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
 * - LANGUAGE_ENGLISH: en  - LANGUAGE_SPANISH: es  - LANGUAGE_FRENCH: fr  - LANGUAGE_GERMAN: de  - LANGUAGE_ITALIAN: it  - LANGUAGE_PORTUGUESE: pt  - LANGUAGE_DUTCH: nl  - LANGUAGE_RUSSIAN: ru  - LANGUAGE_CHINESE: zh  - LANGUAGE_JAPANESE: ja  - LANGUAGE_KOREAN: ko  - LANGUAGE_ARABIC: ar  - LANGUAGE_HINDI: hi  - LANGUAGE_GREEK: el  - LANGUAGE_TURKISH: tr
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-19T02:14:00.124734-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum Language {
  
  UNSPECIFIED("LANGUAGE_UNSPECIFIED"),
  
  ENGLISH("LANGUAGE_ENGLISH"),
  
  SPANISH("LANGUAGE_SPANISH"),
  
  FRENCH("LANGUAGE_FRENCH"),
  
  GERMAN("LANGUAGE_GERMAN"),
  
  ITALIAN("LANGUAGE_ITALIAN"),
  
  PORTUGUESE("LANGUAGE_PORTUGUESE"),
  
  DUTCH("LANGUAGE_DUTCH"),
  
  RUSSIAN("LANGUAGE_RUSSIAN"),
  
  CHINESE("LANGUAGE_CHINESE"),
  
  JAPANESE("LANGUAGE_JAPANESE"),
  
  KOREAN("LANGUAGE_KOREAN"),
  
  ARABIC("LANGUAGE_ARABIC"),
  
  HINDI("LANGUAGE_HINDI"),
  
  GREEK("LANGUAGE_GREEK"),
  
  TURKISH("LANGUAGE_TURKISH");

  private String value;

  Language(String value) {
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
  public static Language fromValue(String value) {
    for (Language b : Language.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

