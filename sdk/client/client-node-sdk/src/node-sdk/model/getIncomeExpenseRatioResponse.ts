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
import { IncomeExpenseRatio } from './incomeExpenseRatio';

export class GetIncomeExpenseRatioResponse {
  'incomeExpenseRatios'?: Array<IncomeExpenseRatio>;
  'nextPageNumber'?: string;

  static discriminator: string | undefined = undefined;

  static attributeTypeMap: Array<{
    name: string;
    baseName: string;
    type: string;
  }> = [
    {
      name: 'incomeExpenseRatios',
      baseName: 'incomeExpenseRatios',
      type: 'Array<IncomeExpenseRatio>',
    },
    {
      name: 'nextPageNumber',
      baseName: 'nextPageNumber',
      type: 'string',
    },
  ];

  static getAttributeTypeMap() {
    return GetIncomeExpenseRatioResponse.attributeTypeMap;
  }
}
