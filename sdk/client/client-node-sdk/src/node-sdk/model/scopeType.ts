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
 *  - SCOPE_TYPE_UNSPECIFIED: Default unspecified scope.  - SCOPE_TYPE_READ: Read-only access to resources.  - SCOPE_TYPE_WRITE: Write access to resources.  - SCOPE_TYPE_DELETE: Delete access to resources.  - SCOPE_TYPE_ADMIN: Full administrative access.  - SCOPE_TYPE_USER_READ: Read-only access to user resources.  - SCOPE_TYPE_USER_WRITE: Write access to user resources.  - SCOPE_TYPE_TEAM_READ: Read-only access to team resources.  - SCOPE_TYPE_TEAM_WRITE: Write access to team resources.  - SCOPE_TYPE_BILLING: Access to billing and payment resources.  - SCOPE_TYPE_ANALYTICS: Access to analytics and reporting.
 */
export enum ScopeType {
  Unspecified = <any>'SCOPE_TYPE_UNSPECIFIED',
  Read = <any>'SCOPE_TYPE_READ',
  Write = <any>'SCOPE_TYPE_WRITE',
  Delete = <any>'SCOPE_TYPE_DELETE',
  Admin = <any>'SCOPE_TYPE_ADMIN',
  UserRead = <any>'SCOPE_TYPE_USER_READ',
  UserWrite = <any>'SCOPE_TYPE_USER_WRITE',
  TeamRead = <any>'SCOPE_TYPE_TEAM_READ',
  TeamWrite = <any>'SCOPE_TYPE_TEAM_WRITE',
  Billing = <any>'SCOPE_TYPE_BILLING',
  Analytics = <any>'SCOPE_TYPE_ANALYTICS',
}
