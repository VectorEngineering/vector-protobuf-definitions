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
import { FinancialUserProfileType } from './financialUserProfileType';

export class PlaidInitiateTokenExchangeRequest {
  'userId': string;
  /**
   * The user\'s full legal name. This is an optional field used in the [returning user experience](https://plaid.com/docs/link/returning-user) to associate Items to the user.
   */
  'fullName': string;
  /**
   * The user\'s email address. This field is optional, but required to enable the [pre-authenticated returning user flow](https://plaid.com/docs/link/returning-user/#enabling-the-returning-user-experience).
   */
  'email': string;
  /**
   * The user\'s phone number in [E.164](https://en.wikipedia.org/wiki/E.164) format. This field is optional, but required to enable the [returning user experience](https://plaid.com/docs/link/returning-user).
   */
  'phoneNumber': string;
  'profileType': FinancialUserProfileType;
  /**
   * Organization identifier for multi-org support
   */
  'orgId': string;
  /**
   * Tenant identifier for multi-tenancy support
   */
  'tenantId': string;

  static discriminator: string | undefined = undefined;

  static attributeTypeMap: Array<{
    name: string;
    baseName: string;
    type: string;
  }> = [
    {
      name: 'userId',
      baseName: 'userId',
      type: 'string',
    },
    {
      name: 'fullName',
      baseName: 'fullName',
      type: 'string',
    },
    {
      name: 'email',
      baseName: 'email',
      type: 'string',
    },
    {
      name: 'phoneNumber',
      baseName: 'phoneNumber',
      type: 'string',
    },
    {
      name: 'profileType',
      baseName: 'profileType',
      type: 'FinancialUserProfileType',
    },
    {
      name: 'orgId',
      baseName: 'orgId',
      type: 'string',
    },
    {
      name: 'tenantId',
      baseName: 'tenantId',
      type: 'string',
    },
  ];

  static getAttributeTypeMap() {
    return PlaidInitiateTokenExchangeRequest.attributeTypeMap;
  }
}

export namespace PlaidInitiateTokenExchangeRequest {}
