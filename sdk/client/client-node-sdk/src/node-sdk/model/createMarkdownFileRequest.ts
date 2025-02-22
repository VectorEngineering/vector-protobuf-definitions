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
 * CreateMarkdownFileRequest represents a request to create a new Markdown file in the workspace. This message contains all necessary information to create and store a Markdown file, including content, location details, and organizational context.
 */
export class CreateMarkdownFileRequest {
  'workspaceId': string;
  'folderId': string;
  'filename': string;
  'userId': string;
  'content': string;
  'orgId': string;
  'tenantId': string;

  static discriminator: string | undefined = undefined;

  static attributeTypeMap: Array<{
    name: string;
    baseName: string;
    type: string;
  }> = [
    {
      name: 'workspaceId',
      baseName: 'workspaceId',
      type: 'string',
    },
    {
      name: 'folderId',
      baseName: 'folderId',
      type: 'string',
    },
    {
      name: 'filename',
      baseName: 'filename',
      type: 'string',
    },
    {
      name: 'userId',
      baseName: 'userId',
      type: 'string',
    },
    {
      name: 'content',
      baseName: 'content',
      type: 'string',
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
    return CreateMarkdownFileRequest.attributeTypeMap;
  }
}
