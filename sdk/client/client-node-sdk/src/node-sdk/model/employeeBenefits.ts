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
 * The Benefit object is used to represent a benefit that an employee has enrolled in.
 */
export class EmployeeBenefits {
  'id'?: string;
  /**
   * The third-party API ID of the matching object.
   */
  'remoteId'?: string;
  /**
   * The name of the benefit provider.
   */
  'providerName'?: string;
  /**
   * The ID of the employee.
   */
  'employeeMergeAccountId'?: string;
  /**
   * The ID of the benefit plan.
   */
  'benefitPlanMergeAccountId'?: string;
  /**
   * The employee\'s contribution.
   */
  'employeeContribution'?: string;
  /**
   * The company\'s contribution.
   */
  'companyContribution'?: string;
  /**
   * The day and time the benefit started.
   */
  'startDate'?: Date;
  /**
   * The day and time the benefit ended.
   */
  'endDate'?: Date;
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
      name: 'providerName',
      baseName: 'providerName',
      type: 'string',
    },
    {
      name: 'employeeMergeAccountId',
      baseName: 'employeeMergeAccountId',
      type: 'string',
    },
    {
      name: 'benefitPlanMergeAccountId',
      baseName: 'benefitPlanMergeAccountId',
      type: 'string',
    },
    {
      name: 'employeeContribution',
      baseName: 'employeeContribution',
      type: 'string',
    },
    {
      name: 'companyContribution',
      baseName: 'companyContribution',
      type: 'string',
    },
    {
      name: 'startDate',
      baseName: 'startDate',
      type: 'Date',
    },
    {
      name: 'endDate',
      baseName: 'endDate',
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
    return EmployeeBenefits.attributeTypeMap;
  }
}
