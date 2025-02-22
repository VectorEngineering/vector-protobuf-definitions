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
 * Specifies the type of entity an address is associated with.   - ADDRESSABLE_TYPE_UNSPECIFIED: Default unspecified type.  - ADDRESSABLE_TYPE_USER: Associated with a user.  - ADDRESSABLE_TYPE_BUSINESS: Associated with a business.  - ADDRESSABLE_TYPE_TEAM: Associated with a team.
 */
export enum AddressableType {
  Unspecified = <any>'ADDRESSABLE_TYPE_UNSPECIFIED',
  User = <any>'ADDRESSABLE_TYPE_USER',
  Business = <any>'ADDRESSABLE_TYPE_BUSINESS',
  Team = <any>'ADDRESSABLE_TYPE_TEAM',
}
