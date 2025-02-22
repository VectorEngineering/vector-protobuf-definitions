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
 * DeleteFileResponse represents the response to a file deletion request. This message indicates whether the file was successfully deleted from the system.
 */
export class DeleteFileResponse {
  'isDeleted'?: boolean;

  static discriminator: string | undefined = undefined;

  static attributeTypeMap: Array<{
    name: string;
    baseName: string;
    type: string;
  }> = [
    {
      name: 'isDeleted',
      baseName: 'isDeleted',
      type: 'boolean',
    },
  ];

  static getAttributeTypeMap() {
    return DeleteFileResponse.attributeTypeMap;
  }
}
