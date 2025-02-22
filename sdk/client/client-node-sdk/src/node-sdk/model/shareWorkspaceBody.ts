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
import { SharePermission } from './sharePermission';

export class ShareWorkspaceBody {
  'userId': string;
  'sharedWithUserId': string;
  'permissionLevel': SharePermission;
  'expiresAt'?: Date;
  'orgId': string;
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
      name: 'sharedWithUserId',
      baseName: 'sharedWithUserId',
      type: 'string',
    },
    {
      name: 'permissionLevel',
      baseName: 'permissionLevel',
      type: 'SharePermission',
    },
    {
      name: 'expiresAt',
      baseName: 'expiresAt',
      type: 'Date',
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
    return ShareWorkspaceBody.attributeTypeMap;
  }
}

export namespace ShareWorkspaceBody {}
