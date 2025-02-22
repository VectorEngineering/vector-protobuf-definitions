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
 * @interface CreateScrapingJobRequest
 */
export interface CreateScrapingJobRequest {
  /**
   *
   * @type {string}
   * @memberof CreateScrapingJobRequest
   */
  userId: string;
  /**
   *
   * @type {string}
   * @memberof CreateScrapingJobRequest
   */
  orgId: string;
  /**
   *
   * @type {string}
   * @memberof CreateScrapingJobRequest
   */
  tenantId: string;
  /**
   *
   * @type {string}
   * @memberof CreateScrapingJobRequest
   */
  name?: string;
  /**
   *
   * @type {Array<string>}
   * @memberof CreateScrapingJobRequest
   */
  keywords?: Array<string>;
  /**
   *
   * @type {string}
   * @memberof CreateScrapingJobRequest
   */
  lang?: string;
  /**
   *
   * @type {number}
   * @memberof CreateScrapingJobRequest
   */
  zoom?: number;
  /**
   *
   * @type {string}
   * @memberof CreateScrapingJobRequest
   */
  lat?: string;
  /**
   *
   * @type {string}
   * @memberof CreateScrapingJobRequest
   */
  lon?: string;
  /**
   *
   * @type {boolean}
   * @memberof CreateScrapingJobRequest
   */
  fastMode?: boolean;
  /**
   *
   * @type {number}
   * @memberof CreateScrapingJobRequest
   */
  radius?: number;
  /**
   *
   * @type {number}
   * @memberof CreateScrapingJobRequest
   */
  depth?: number;
  /**
   *
   * @type {boolean}
   * @memberof CreateScrapingJobRequest
   */
  email?: boolean;
  /**
   *
   * @type {number}
   * @memberof CreateScrapingJobRequest
   */
  maxTime?: number;
  /**
   *
   * @type {Array<string>}
   * @memberof CreateScrapingJobRequest
   */
  proxies?: Array<string>;
}

/**
 * Check if a given object implements the CreateScrapingJobRequest interface.
 */
export function instanceOfCreateScrapingJobRequest(
  value: object
): value is CreateScrapingJobRequest {
  if (!('userId' in value) || value['userId'] === undefined) return false;
  if (!('orgId' in value) || value['orgId'] === undefined) return false;
  if (!('tenantId' in value) || value['tenantId'] === undefined) return false;
  return true;
}

export function CreateScrapingJobRequestFromJSON(
  json: any
): CreateScrapingJobRequest {
  return CreateScrapingJobRequestFromJSONTyped(json, false);
}

export function CreateScrapingJobRequestFromJSONTyped(
  json: any,
  ignoreDiscriminator: boolean
): CreateScrapingJobRequest {
  if (json == null) {
    return json;
  }
  return {
    userId: json['userId'],
    orgId: json['orgId'],
    tenantId: json['tenantId'],
    name: json['name'] == null ? undefined : json['name'],
    keywords: json['keywords'] == null ? undefined : json['keywords'],
    lang: json['lang'] == null ? undefined : json['lang'],
    zoom: json['zoom'] == null ? undefined : json['zoom'],
    lat: json['lat'] == null ? undefined : json['lat'],
    lon: json['lon'] == null ? undefined : json['lon'],
    fastMode: json['fastMode'] == null ? undefined : json['fastMode'],
    radius: json['radius'] == null ? undefined : json['radius'],
    depth: json['depth'] == null ? undefined : json['depth'],
    email: json['email'] == null ? undefined : json['email'],
    maxTime: json['maxTime'] == null ? undefined : json['maxTime'],
    proxies: json['proxies'] == null ? undefined : json['proxies'],
  };
}

export function CreateScrapingJobRequestToJSON(
  value?: CreateScrapingJobRequest | null
): any {
  if (value == null) {
    return value;
  }
  return {
    userId: value['userId'],
    orgId: value['orgId'],
    tenantId: value['tenantId'],
    name: value['name'],
    keywords: value['keywords'],
    lang: value['lang'],
    zoom: value['zoom'],
    lat: value['lat'],
    lon: value['lon'],
    fastMode: value['fastMode'],
    radius: value['radius'],
    depth: value['depth'],
    email: value['email'],
    maxTime: value['maxTime'],
    proxies: value['proxies'],
  };
}
