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
import { FlsaStatus } from './flsaStatus';
import { PayFrequency } from './payFrequency';
import { PayPeriod } from './payPeriod';

/**
 * The Employment object is used to represent a job position at a company. NOTE: When there is a change in pay or title, integrations with historical data will create new Employment objects while integrations without historical data will update existing ones.
 */
export class EmployeeJobPositionAtCompany {
  'id'?: string;
  /**
   * The third-party API ID of the matching object.
   */
  'remoteId'?: string;
  'jobTitle'?: string;
  'payRate'?: number;
  'payPeriod'?: PayPeriod;
  'payFrequency'?: PayFrequency;
  'payCurrency'?: string;
  'flsaStatus'?: FlsaStatus;
  'effectiveDate'?: Date;
  /**
   * Indicates whether or not this object has been deleted in the third party platform.
   */
  'remoteWasDeleted'?: boolean;
  'createdAt'?: Date;
  'modifiedAt'?: Date;
  'mergeAccountId'?: string;
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
      name: 'jobTitle',
      baseName: 'jobTitle',
      type: 'string',
    },
    {
      name: 'payRate',
      baseName: 'payRate',
      type: 'number',
    },
    {
      name: 'payPeriod',
      baseName: 'payPeriod',
      type: 'PayPeriod',
    },
    {
      name: 'payFrequency',
      baseName: 'payFrequency',
      type: 'PayFrequency',
    },
    {
      name: 'payCurrency',
      baseName: 'payCurrency',
      type: 'string',
    },
    {
      name: 'flsaStatus',
      baseName: 'flsaStatus',
      type: 'FlsaStatus',
    },
    {
      name: 'effectiveDate',
      baseName: 'effectiveDate',
      type: 'Date',
    },
    {
      name: 'remoteWasDeleted',
      baseName: 'remoteWasDeleted',
      type: 'boolean',
    },
    {
      name: 'createdAt',
      baseName: 'createdAt',
      type: 'Date',
    },
    {
      name: 'modifiedAt',
      baseName: 'modifiedAt',
      type: 'Date',
    },
    {
      name: 'mergeAccountId',
      baseName: 'mergeAccountId',
      type: 'string',
    },
    {
      name: 'deletedAt',
      baseName: 'deletedAt',
      type: 'Date',
    },
  ];

  static getAttributeTypeMap() {
    return EmployeeJobPositionAtCompany.attributeTypeMap;
  }
}

export namespace EmployeeJobPositionAtCompany {}
