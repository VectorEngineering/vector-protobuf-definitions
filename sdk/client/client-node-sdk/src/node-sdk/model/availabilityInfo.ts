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

export class AvailabilityInfo {
  'retryAfter'?: number;
  'maintenanceWindow'?: string;
  'affectedServices'?: Array<string>;
  'statusPageUrl'?: string;

  static discriminator: string | undefined = undefined;

  static attributeTypeMap: Array<{
    name: string;
    baseName: string;
    type: string;
  }> = [
    {
      name: 'retryAfter',
      baseName: 'retryAfter',
      type: 'number',
    },
    {
      name: 'maintenanceWindow',
      baseName: 'maintenanceWindow',
      type: 'string',
    },
    {
      name: 'affectedServices',
      baseName: 'affectedServices',
      type: 'Array<string>',
    },
    {
      name: 'statusPageUrl',
      baseName: 'statusPageUrl',
      type: 'string',
    },
  ];

  static getAttributeTypeMap() {
    return AvailabilityInfo.attributeTypeMap;
  }
}
