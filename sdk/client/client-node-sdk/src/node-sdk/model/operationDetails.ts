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

export class OperationDetails {
  'operationId'?: string;
  'startTime'?: Date;
  'endTime'?: Date;
  'stackTrace'?: Array<string>;
  'parameters'?: { [key: string]: string | undefined };

  static discriminator: string | undefined = undefined;

  static attributeTypeMap: Array<{
    name: string;
    baseName: string;
    type: string;
  }> = [
    {
      name: 'operationId',
      baseName: 'operationId',
      type: 'string',
    },
    {
      name: 'startTime',
      baseName: 'startTime',
      type: 'Date',
    },
    {
      name: 'endTime',
      baseName: 'endTime',
      type: 'Date',
    },
    {
      name: 'stackTrace',
      baseName: 'stackTrace',
      type: 'Array<string>',
    },
    {
      name: 'parameters',
      baseName: 'parameters',
      type: '{ [key: string]: string | undefined; }',
    },
  ];

  static getAttributeTypeMap() {
    return OperationDetails.attributeTypeMap;
  }
}
