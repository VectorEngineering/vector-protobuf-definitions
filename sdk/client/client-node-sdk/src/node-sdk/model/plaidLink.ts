/**
 * User Service API
 * Solomon AI User Service API - Manages user profiles and authentication
 *
 * The version of the OpenAPI document: 1.0
 * Contact: yoanyomba@solomon-ai.co
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { RequestFile } from './models';

export class PlaidLink {
  'id'?: string;
  'products'?: Array<string>;
  'webhookUrl'?: string;
  'institutionId'?: string;
  'institutionName'?: string;
  'usePlaidSync'?: boolean;
  'itemId'?: string;
  'createdAt'?: Date;
  'deletedAt'?: Date;

  static discriminator: string | undefined = undefined;

  static attributeTypeMap: Array<{
    name: string;
    baseName: string;
    type: string;
  }> = [
    {
      name: 'id',
      baseName: 'id',
      type: 'string',
    },
    {
      name: 'products',
      baseName: 'products',
      type: 'Array<string>',
    },
    {
      name: 'webhookUrl',
      baseName: 'webhookUrl',
      type: 'string',
    },
    {
      name: 'institutionId',
      baseName: 'institutionId',
      type: 'string',
    },
    {
      name: 'institutionName',
      baseName: 'institutionName',
      type: 'string',
    },
    {
      name: 'usePlaidSync',
      baseName: 'usePlaidSync',
      type: 'boolean',
    },
    {
      name: 'itemId',
      baseName: 'itemId',
      type: 'string',
    },
    {
      name: 'createdAt',
      baseName: 'createdAt',
      type: 'Date',
    },
    {
      name: 'deletedAt',
      baseName: 'deletedAt',
      type: 'Date',
    },
  ];

  static getAttributeTypeMap() {
    return PlaidLink.attributeTypeMap;
  }
}
