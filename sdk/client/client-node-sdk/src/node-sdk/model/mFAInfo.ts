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

import { RequestFile } from './models';

export class MFAInfo {
  'mfaRequired'?: boolean;
  'allowedMethods'?: Array<string>;
  'failedReason'?: string;

  static discriminator: string | undefined = undefined;

  static attributeTypeMap: Array<{
    name: string;
    baseName: string;
    type: string;
  }> = [
    {
      name: 'mfaRequired',
      baseName: 'mfaRequired',
      type: 'boolean',
    },
    {
      name: 'allowedMethods',
      baseName: 'allowedMethods',
      type: 'Array<string>',
    },
    {
      name: 'failedReason',
      baseName: 'failedReason',
      type: 'string',
    },
  ];

  static getAttributeTypeMap() {
    return MFAInfo.attributeTypeMap;
  }
}
