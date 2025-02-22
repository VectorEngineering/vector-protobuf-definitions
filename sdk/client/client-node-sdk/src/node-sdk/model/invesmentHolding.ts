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

export class InvesmentHolding {
  'id'?: string;
  'name'?: string;
  'plaidAccountId'?: string;
  'costBasis'?: number;
  'institutionPrice'?: number;
  'institutionPriceAsOf'?: string;
  'institutionPriceDatetime'?: string;
  'institutionValue'?: number;
  'isoCurrencyCode'?: string;
  'quantity'?: number;
  'securityId'?: string;
  'unofficialCurrencyCode'?: string;
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
      name: 'name',
      baseName: 'name',
      type: 'string',
    },
    {
      name: 'plaidAccountId',
      baseName: 'plaidAccountId',
      type: 'string',
    },
    {
      name: 'costBasis',
      baseName: 'costBasis',
      type: 'number',
    },
    {
      name: 'institutionPrice',
      baseName: 'institutionPrice',
      type: 'number',
    },
    {
      name: 'institutionPriceAsOf',
      baseName: 'institutionPriceAsOf',
      type: 'string',
    },
    {
      name: 'institutionPriceDatetime',
      baseName: 'institutionPriceDatetime',
      type: 'string',
    },
    {
      name: 'institutionValue',
      baseName: 'institutionValue',
      type: 'number',
    },
    {
      name: 'isoCurrencyCode',
      baseName: 'isoCurrencyCode',
      type: 'string',
    },
    {
      name: 'quantity',
      baseName: 'quantity',
      type: 'number',
    },
    {
      name: 'securityId',
      baseName: 'securityId',
      type: 'string',
    },
    {
      name: 'unofficialCurrencyCode',
      baseName: 'unofficialCurrencyCode',
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
    return InvesmentHolding.attributeTypeMap;
  }
}
