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

/**
 * The PurchaseOrderLineItem object is used to represent a purchase order\'s line item.
 */
export class PurchaseOrderLineItem {
  'id'?: string;
  'remoteId'?: string;
  'description'?: string;
  'quantity'?: number;
  /**
   * Assuming string, but might be better as float or double.
   */
  'unitPrice'?: string;
  'item'?: string;
  'account'?: string;
  'trackingCategory'?: string;
  'trackingCategories'?: Array<string>;
  'taxAmount'?: string;
  'totalLineAmount'?: string;
  'currency'?: string;
  /**
   * Assuming string, but might be better as float or double.
   */
  'exchangeRate'?: string;
  'company'?: string;
  'modifiedAt'?: Date;
  'createdAt'?: Date;
  'remoteWasDeleted'?: boolean;
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
      name: 'remoteId',
      baseName: 'remoteId',
      type: 'string',
    },
    {
      name: 'description',
      baseName: 'description',
      type: 'string',
    },
    {
      name: 'quantity',
      baseName: 'quantity',
      type: 'number',
    },
    {
      name: 'unitPrice',
      baseName: 'unitPrice',
      type: 'string',
    },
    {
      name: 'item',
      baseName: 'item',
      type: 'string',
    },
    {
      name: 'account',
      baseName: 'account',
      type: 'string',
    },
    {
      name: 'trackingCategory',
      baseName: 'trackingCategory',
      type: 'string',
    },
    {
      name: 'trackingCategories',
      baseName: 'trackingCategories',
      type: 'Array<string>',
    },
    {
      name: 'taxAmount',
      baseName: 'taxAmount',
      type: 'string',
    },
    {
      name: 'totalLineAmount',
      baseName: 'totalLineAmount',
      type: 'string',
    },
    {
      name: 'currency',
      baseName: 'currency',
      type: 'string',
    },
    {
      name: 'exchangeRate',
      baseName: 'exchangeRate',
      type: 'string',
    },
    {
      name: 'company',
      baseName: 'company',
      type: 'string',
    },
    {
      name: 'modifiedAt',
      baseName: 'modifiedAt',
      type: 'Date',
    },
    {
      name: 'createdAt',
      baseName: 'createdAt',
      type: 'Date',
    },
    {
      name: 'remoteWasDeleted',
      baseName: 'remoteWasDeleted',
      type: 'boolean',
    },
    {
      name: 'deletedAt',
      baseName: 'deletedAt',
      type: 'Date',
    },
  ];

  static getAttributeTypeMap() {
    return PurchaseOrderLineItem.attributeTypeMap;
  }
}
