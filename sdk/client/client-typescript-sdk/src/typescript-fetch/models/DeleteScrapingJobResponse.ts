/* tslint:disable */
/* eslint-disable */
/**
 * Lead Scraping Service API
 * Vector Lead Scraping Service API - Manages Lead Scraping Jobs
 *
 * The version of the OpenAPI document: 1.0
 * Contact: yoanyomba@vector.ai
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { mapValues } from '../runtime';
/**
 *
 * @export
 * @interface DeleteScrapingJobResponse
 */
export interface DeleteScrapingJobResponse {
  /**
   *
   * @type {boolean}
   * @memberof DeleteScrapingJobResponse
   */
  success?: boolean;
}

/**
 * Check if a given object implements the DeleteScrapingJobResponse interface.
 */
export function instanceOfDeleteScrapingJobResponse(
  value: object
): value is DeleteScrapingJobResponse {
  return true;
}

export function DeleteScrapingJobResponseFromJSON(
  json: any
): DeleteScrapingJobResponse {
  return DeleteScrapingJobResponseFromJSONTyped(json, false);
}

export function DeleteScrapingJobResponseFromJSONTyped(
  json: any,
  ignoreDiscriminator: boolean
): DeleteScrapingJobResponse {
  if (json == null) {
    return json;
  }
  return {
    success: json['success'] == null ? undefined : json['success'],
  };
}

export function DeleteScrapingJobResponseToJSON(
  value?: DeleteScrapingJobResponse | null
): any {
  if (value == null) {
    return value;
  }
  return {
    success: value['success'],
  };
}
