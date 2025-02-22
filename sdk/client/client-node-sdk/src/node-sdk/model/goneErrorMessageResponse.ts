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
import { ErrorResponse } from './errorResponse';

/**
 * Indicates that the resource requested is no longer available and will not be available again
 */
export class GoneErrorMessageResponse {
  'code'?: number;
  'message'?: string;
  'resource'?: string;
  'errorResponse'?: ErrorResponse;

  static discriminator: string | undefined = undefined;

  static attributeTypeMap: Array<{
    name: string;
    baseName: string;
    type: string;
  }> = [
    {
      name: 'code',
      baseName: 'code',
      type: 'number',
    },
    {
      name: 'message',
      baseName: 'message',
      type: 'string',
    },
    {
      name: 'resource',
      baseName: 'resource',
      type: 'string',
    },
    {
      name: 'errorResponse',
      baseName: 'errorResponse',
      type: 'ErrorResponse',
    },
  ];

  static getAttributeTypeMap() {
    return GoneErrorMessageResponse.attributeTypeMap;
  }
}
