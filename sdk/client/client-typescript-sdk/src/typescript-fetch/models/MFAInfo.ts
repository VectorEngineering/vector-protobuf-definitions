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
 * @interface MFAInfo
 */
export interface MFAInfo {
  /**
   *
   * @type {boolean}
   * @memberof MFAInfo
   */
  mfaRequired?: boolean;
  /**
   *
   * @type {Array<string>}
   * @memberof MFAInfo
   */
  allowedMethods?: Array<string>;
  /**
   *
   * @type {string}
   * @memberof MFAInfo
   */
  failedReason?: string;
}

/**
 * Check if a given object implements the MFAInfo interface.
 */
export function instanceOfMFAInfo(value: object): value is MFAInfo {
  return true;
}

export function MFAInfoFromJSON(json: any): MFAInfo {
  return MFAInfoFromJSONTyped(json, false);
}

export function MFAInfoFromJSONTyped(
  json: any,
  ignoreDiscriminator: boolean
): MFAInfo {
  if (json == null) {
    return json;
  }
  return {
    mfaRequired: json['mfaRequired'] == null ? undefined : json['mfaRequired'],
    allowedMethods:
      json['allowedMethods'] == null ? undefined : json['allowedMethods'],
    failedReason:
      json['failedReason'] == null ? undefined : json['failedReason'],
  };
}

export function MFAInfoToJSON(value?: MFAInfo | null): any {
  if (value == null) {
    return value;
  }
  return {
    mfaRequired: value['mfaRequired'],
    allowedMethods: value['allowedMethods'],
    failedReason: value['failedReason'],
  };
}
