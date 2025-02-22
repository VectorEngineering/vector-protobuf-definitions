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

export class CreateAccountRequest {
  'authPlatformUserId': string;
  'orgId': string;
  'tenantId': string;
  'email': string;

  static discriminator: string | undefined = undefined;

  static attributeTypeMap: Array<{
    name: string;
    baseName: string;
    type: string;
  }> = [
    {
      name: 'authPlatformUserId',
      baseName: 'authPlatformUserId',
      type: 'string',
    },
    {
      name: 'orgId',
      baseName: 'orgId',
      type: 'string',
    },
    {
      name: 'tenantId',
      baseName: 'tenantId',
      type: 'string',
    },
    {
      name: 'email',
      baseName: 'email',
      type: 'string',
    },
  ];

  static getAttributeTypeMap() {
    return CreateAccountRequest.attributeTypeMap;
  }
}
