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
 * BackgroundJobStatus represents the possible states of a background job. This enum is used to track the lifecycle of asynchronous tasks like scraping jobs.  State transitions: 1. QUEUED -> IN_PROGRESS 2. IN_PROGRESS -> COMPLETED/FAILED/CANCELLED/TIMED_OUT  Usage example: ```go job := &ScrapingJob{     Status: BackgroundJobStatus_BACKGROUND_JOB_STATUS_IN_PROGRESS, } ```   - BACKGROUND_JOB_STATUS_UNSPECIFIED: Default state, should not be used explicitly  - BACKGROUND_JOB_STATUS_QUEUED: Job is queued and waiting to be processed  - BACKGROUND_JOB_STATUS_IN_PROGRESS: Job is currently being processed  - BACKGROUND_JOB_STATUS_COMPLETED: Job has completed successfully  - BACKGROUND_JOB_STATUS_FAILED: Job encountered an error and failed  - BACKGROUND_JOB_STATUS_CANCELLED: Job was manually cancelled by user  - BACKGROUND_JOB_STATUS_TIMED_OUT: Job exceeded its maximum execution time
 */

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-09T22:48:58.302614-05:00[America/New_York]", comments = "Generator version: 7.7.0")
public enum BackgroundJobStatus {
  
  UNSPECIFIED("BACKGROUND_JOB_STATUS_UNSPECIFIED"),
  
  QUEUED("BACKGROUND_JOB_STATUS_QUEUED"),
  
  IN_PROGRESS("BACKGROUND_JOB_STATUS_IN_PROGRESS"),
  
  COMPLETED("BACKGROUND_JOB_STATUS_COMPLETED"),
  
  FAILED("BACKGROUND_JOB_STATUS_FAILED"),
  
  CANCELLED("BACKGROUND_JOB_STATUS_CANCELLED"),
  
  TIMED_OUT("BACKGROUND_JOB_STATUS_TIMED_OUT");

  private String value;

  BackgroundJobStatus(String value) {
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
  public static BackgroundJobStatus fromValue(String value) {
    for (BackgroundJobStatus b : BackgroundJobStatus.values()) {
      if (b.value.equals(value)) {
        return b;
      }
    }
    throw new IllegalArgumentException("Unexpected value '" + value + "'");
  }
}

