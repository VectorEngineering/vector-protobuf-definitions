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
import { Comment1 } from './comment1';
import { CommentEdit } from './commentEdit';

export class GetCommentResponse {
  'comment'?: Comment1;
  'reactions'?: { [key: string]: number | undefined };
  'editHistory'?: Array<CommentEdit>;

  static discriminator: string | undefined = undefined;

  static attributeTypeMap: Array<{
    name: string;
    baseName: string;
    type: string;
  }> = [
    {
      name: 'comment',
      baseName: 'comment',
      type: 'Comment1',
    },
    {
      name: 'reactions',
      baseName: 'reactions',
      type: '{ [key: string]: number | undefined; }',
    },
    {
      name: 'editHistory',
      baseName: 'editHistory',
      type: 'Array<CommentEdit>',
    },
  ];

  static getAttributeTypeMap() {
    return GetCommentResponse.attributeTypeMap;
  }
}
