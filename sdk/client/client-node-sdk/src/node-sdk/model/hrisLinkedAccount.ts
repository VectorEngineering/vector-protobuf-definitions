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
import { CompanyProfile } from './companyProfile';
import { Employee } from './employee';
import { EmployerBenefits } from './employerBenefits';
import { EmployerPayrollRun } from './employerPayrollRun';

export class HrisLinkedAccount {
  'id'?: string;
  'profile'?: CompanyProfile;
  'benefitPlans'?: Array<EmployerBenefits>;
  'employees'?: Array<Employee>;
  'payrollRuns'?: Array<EmployerPayrollRun>;
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
      name: 'profile',
      baseName: 'profile',
      type: 'CompanyProfile',
    },
    {
      name: 'benefitPlans',
      baseName: 'benefitPlans',
      type: 'Array<EmployerBenefits>',
    },
    {
      name: 'employees',
      baseName: 'employees',
      type: 'Array<Employee>',
    },
    {
      name: 'payrollRuns',
      baseName: 'payrollRuns',
      type: 'Array<EmployerPayrollRun>',
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
    return HrisLinkedAccount.attributeTypeMap;
  }
}
