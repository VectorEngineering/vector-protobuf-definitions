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
import { Bookmark } from './bookmark';
import { UserTags } from './userTags';

export class UserProfile {
  'id'?: string;
  'tags': Array<UserTags>;
  'name': string;
  '_private': boolean;
  'followers': string;
  'following': string;
  'notificationFeedTimelineId': string;
  'personalFeedTimelineId': string;
  'newsFeedTimelineId': string;
  'profileImageUrl': string;
  'bookmarks': Bookmark;
  'algoliaId': string;

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
      name: 'tags',
      baseName: 'tags',
      type: 'Array<UserTags>',
    },
    {
      name: 'name',
      baseName: 'name',
      type: 'string',
    },
    {
      name: '_private',
      baseName: 'private',
      type: 'boolean',
    },
    {
      name: 'followers',
      baseName: 'followers',
      type: 'string',
    },
    {
      name: 'following',
      baseName: 'following',
      type: 'string',
    },
    {
      name: 'notificationFeedTimelineId',
      baseName: 'notificationFeedTimelineId',
      type: 'string',
    },
    {
      name: 'personalFeedTimelineId',
      baseName: 'personalFeedTimelineId',
      type: 'string',
    },
    {
      name: 'newsFeedTimelineId',
      baseName: 'newsFeedTimelineId',
      type: 'string',
    },
    {
      name: 'profileImageUrl',
      baseName: 'profileImageUrl',
      type: 'string',
    },
    {
      name: 'bookmarks',
      baseName: 'bookmarks',
      type: 'Bookmark',
    },
    {
      name: 'algoliaId',
      baseName: 'algoliaId',
      type: 'string',
    },
  ];

  static getAttributeTypeMap() {
    return UserProfile.attributeTypeMap;
  }
}
