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
 * The position\'s pay frequency. Possible values include: WEEKLY, BIWEEKLY, MONTHLY, QUARTERLY, SEMIANNUALLY, ANNUALLY, THIRTEEN-MONTHLY, PRO_RATA, SEMIMONTHLY. In cases where there is no clear mapping, the original value passed through will be returned.
 */
export enum PayFrequency {
  Unspecified = <any>'PAY_FREQUENCY_UNSPECIFIED',
  Weekly = <any>'PAY_FREQUENCY_WEEKLY',
  Biweekly = <any>'PAY_FREQUENCY_BIWEEKLY',
  Monthly = <any>'PAY_FREQUENCY_MONTHLY',
  Quarterly = <any>'PAY_FREQUENCY_QUARTERLY',
  Semiannually = <any>'PAY_FREQUENCY_SEMIANNUALLY',
  Annually = <any>'PAY_FREQUENCY_ANNUALLY',
  ThirteenMonthly = <any>'PAY_FREQUENCY_THIRTEEN_MONTHLY',
  ProRata = <any>'PAY_FREQUENCY_PRO_RATA',
  Semimonthly = <any>'PAY_FREQUENCY_SEMIMONTHLY',
}
