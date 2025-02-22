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
import { Category1 } from './category1';

/**
 * The Budgets table stores information about each budget created by the user, including the name of the budget, the start and end dates, and the user ID.
 */
export class Budget {
  'id'?: string;
  'name'?: string;
  'description'?: string;
  'startDate'?: string;
  'endDate'?: string;
  'category'?: Category1;
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
      name: 'description',
      baseName: 'description',
      type: 'string',
    },
    {
      name: 'startDate',
      baseName: 'startDate',
      type: 'string',
    },
    {
      name: 'endDate',
      baseName: 'endDate',
      type: 'string',
    },
    {
      name: 'category',
      baseName: 'category',
      type: 'Category1',
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
    return Budget.attributeTypeMap;
  }
}
