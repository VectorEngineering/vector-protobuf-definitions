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
import { CommentReply } from './commentReply';

export class GetCommentRepliesResponse {
  'replies'?: Array<CommentReply>;

  static discriminator: string | undefined = undefined;

  static attributeTypeMap: Array<{
    name: string;
    baseName: string;
    type: string;
  }> = [
    {
      name: 'replies',
      baseName: 'replies',
      type: 'Array<CommentReply>',
    },
  ];

  static getAttributeTypeMap() {
    return GetCommentRepliesResponse.attributeTypeMap;
  }
}
