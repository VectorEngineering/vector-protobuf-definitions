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
import { WorkspaceservicehttpErrorDetails } from './workspaceservicehttpErrorDetails';

export class WorkspaceservicehttpTusError {
  /**
   * HTTP status code
   */
  'code'?: number;
  'details'?: WorkspaceservicehttpErrorDetails;
  /**
   * Error message
   */
  'message'?: string;
  /**
   * Error reference code
   */
  'reference'?: string;

  static discriminator: string | undefined = undefined;

  static attributeTypeMap: Array<{
    name: string;
    baseName: string;
    type: string;
  }> = [
    {
      name: 'code',
      baseName: 'code',
      type: 'number',
    },
    {
      name: 'details',
      baseName: 'details',
      type: 'WorkspaceservicehttpErrorDetails',
    },
    {
      name: 'message',
      baseName: 'message',
      type: 'string',
    },
    {
      name: 'reference',
      baseName: 'reference',
      type: 'string',
    },
  ];

  static getAttributeTypeMap() {
    return WorkspaceservicehttpTusError.attributeTypeMap;
  }
}
