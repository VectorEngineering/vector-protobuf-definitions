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
import { ReportItem } from './reportItem';

/**
 * The CashFlowStatement object shows operating activities, investing activities, and financing activities over a period of time (month, quarter, or year).
 */
export class CashFlowStatement {
  'id'?: string;
  /**
   * The third-party API ID of the matching object.
   */
  'remoteId'?: string;
  /**
   * The cash flow statement\'s name.
   */
  'name'?: string;
  'currency'?: string;
  /**
   * The company the cash flow statement belongs to.
   */
  'company'?: string;
  /**
   * The cash flow statement\'s start period.  Consider using google.protobuf.Timestamp
   */
  'startPeriod'?: Date;
  /**
   * The cash flow statement\'s end period.  Consider using google.protobuf.Timestamp
   */
  'endPeriod'?: Date;
  /**
   * Cash and cash equivalents at the beginning of the cash flow statement\'s period.
   */
  'cashAtBeginningOfPeriod'?: number;
  /**
   * Cash and cash equivalents at the beginning of the cash flow statement\'s period.
   */
  'cashAtEndOfPeriod'?: number;
  'operatingActivities'?: Array<ReportItem>;
  'investingActivities'?: Array<ReportItem>;
  'financingActivities'?: Array<ReportItem>;
  /**
   * The time that cash flow statement was generated by the accounting system.  Consider using google.protobuf.Timestamp
   */
  'remoteGeneratedAt'?: Date;
  /**
   * Indicates whether or not this object has been deleted by third party webhooks.
   */
  'remoteWasDeleted'?: boolean;
  /**
   * Consider using google.protobuf.Timestamp
   */
  'modifiedAt'?: Date;
  'mergeRecordId'?: string;
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
      name: 'remoteId',
      baseName: 'remoteId',
      type: 'string',
    },
    {
      name: 'name',
      baseName: 'name',
      type: 'string',
    },
    {
      name: 'currency',
      baseName: 'currency',
      type: 'string',
    },
    {
      name: 'company',
      baseName: 'company',
      type: 'string',
    },
    {
      name: 'startPeriod',
      baseName: 'startPeriod',
      type: 'Date',
    },
    {
      name: 'endPeriod',
      baseName: 'endPeriod',
      type: 'Date',
    },
    {
      name: 'cashAtBeginningOfPeriod',
      baseName: 'cashAtBeginningOfPeriod',
      type: 'number',
    },
    {
      name: 'cashAtEndOfPeriod',
      baseName: 'cashAtEndOfPeriod',
      type: 'number',
    },
    {
      name: 'operatingActivities',
      baseName: 'operatingActivities',
      type: 'Array<ReportItem>',
    },
    {
      name: 'investingActivities',
      baseName: 'investingActivities',
      type: 'Array<ReportItem>',
    },
    {
      name: 'financingActivities',
      baseName: 'financingActivities',
      type: 'Array<ReportItem>',
    },
    {
      name: 'remoteGeneratedAt',
      baseName: 'remoteGeneratedAt',
      type: 'Date',
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
      name: 'mergeRecordId',
      baseName: 'mergeRecordId',
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
    return CashFlowStatement.attributeTypeMap;
  }
}
