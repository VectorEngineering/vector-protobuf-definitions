/**
 * User Service
 * Solomon AI User Service API
 *
 * The version of the OpenAPI document: 0.1
 * Contact: yoanyomba@solomon-ai.co
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { RequestFile } from './models';
import { APIKey } from './aPIKey';

export class CreateAPIKeyResponse {
  'apiKey'?: APIKey;
  'key'?: string;

  static discriminator: string | undefined = undefined;

  static attributeTypeMap: Array<{
    name: string;
    baseName: string;
    type: string;
  }> = [
    {
      name: 'apiKey',
      baseName: 'apiKey',
      type: 'APIKey',
    },
    {
      name: 'key',
      baseName: 'key',
      type: 'string',
    },
  ];

  static getAttributeTypeMap() {
    return CreateAPIKeyResponse.attributeTypeMap;
  }
}
