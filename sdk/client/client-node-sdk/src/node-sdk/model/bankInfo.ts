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
import { BankAccountType1 } from './bankAccountType1';

/**
 * The BankInfo object is used to represent the Bank Account information for an Employee.
 */
export class BankInfo {
  'id'?: string;
  /**
   * The third-party API ID of the matching object.
   */
  'remoteId'?: string;
  'accountNumber'?: string;
  'routingNumber'?: string;
  'bankName'?: string;
  'accountType'?: BankAccountType1;
  /**
   * When the matching bank object was created in the third party system.
   */
  'createdAt'?: Date;
  'mergeAccountId'?: string;
  /**
   * Indicates whether or not this object has been deleted in the third party platform.
   */
  'remoteWasDeleted'?: boolean;
  'modifiedAt'?: Date;
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
      name: 'accountNumber',
      baseName: 'accountNumber',
      type: 'string',
    },
    {
      name: 'routingNumber',
      baseName: 'routingNumber',
      type: 'string',
    },
    {
      name: 'bankName',
      baseName: 'bankName',
      type: 'string',
    },
    {
      name: 'accountType',
      baseName: 'accountType',
      type: 'BankAccountType1',
    },
    {
      name: 'createdAt',
      baseName: 'createdAt',
      type: 'Date',
    },
    {
      name: 'mergeAccountId',
      baseName: 'mergeAccountId',
      type: 'string',
    },
    {
      name: 'remoteWasDeleted',
      baseName: 'remoteWasDeleted',
      type: 'boolean',
    },
    {
      name: 'modifiedAt',
      baseName: 'modifiedAt',
      type: 'Date',
    },
    {
      name: 'deletedAt',
      baseName: 'deletedAt',
      type: 'Date',
    },
  ];

  static getAttributeTypeMap() {
    return BankInfo.attributeTypeMap;
  }
}

export namespace BankInfo {}
