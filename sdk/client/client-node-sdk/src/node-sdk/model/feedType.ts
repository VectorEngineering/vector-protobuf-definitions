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
 * FeedType encompasses the various types of feeds a profile can have (in conjuction) with getstream.   - FEED_TYPE_PERSONAL: UserFeed is a profile\'s personal feed  - FEED_TYPE_NEWS: NewsFeed is a profile\'s timeline  - FEED_TYPE_NOTIFICATION: NotificationFeed encompasses a profile\'s notification feed
 */
export enum FeedType {
  Unspecified = <any>'FEED_TYPE_UNSPECIFIED',
  Personal = <any>'FEED_TYPE_PERSONAL',
  News = <any>'FEED_TYPE_NEWS',
  Notification = <any>'FEED_TYPE_NOTIFICATION',
}
