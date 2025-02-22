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
 *  - ORGANIZATION_MEMBER_STATUS_UNSPECIFIED: Default unspecified status.  - ORGANIZATION_MEMBER_STATUS_ACTIVE: Member is active.  - ORGANIZATION_MEMBER_STATUS_INVITED: Member has been invited but not joined.  - ORGANIZATION_MEMBER_STATUS_PENDING: Member\'s request is pending approval.  - ORGANIZATION_MEMBER_STATUS_DECLINED: Member declined the invitation.  - ORGANIZATION_MEMBER_STATUS_REMOVED: Member was removed from the organization.  - ORGANIZATION_MEMBER_STATUS_SUSPENDED: Member\'s account is temporarily suspended.  - ORGANIZATION_MEMBER_STATUS_BLOCKED: Member is blocked from the organization.  - ORGANIZATION_MEMBER_STATUS_EXPIRED: Member\'s access has expired.  - ORGANIZATION_MEMBER_STATUS_INACTIVE: Member is inactive but not removed.
 */
export enum OrganizationMemberStatus {
  Unspecified = <any>'ORGANIZATION_MEMBER_STATUS_UNSPECIFIED',
  Active = <any>'ORGANIZATION_MEMBER_STATUS_ACTIVE',
  Invited = <any>'ORGANIZATION_MEMBER_STATUS_INVITED',
  Pending = <any>'ORGANIZATION_MEMBER_STATUS_PENDING',
  Declined = <any>'ORGANIZATION_MEMBER_STATUS_DECLINED',
  Removed = <any>'ORGANIZATION_MEMBER_STATUS_REMOVED',
  Suspended = <any>'ORGANIZATION_MEMBER_STATUS_SUSPENDED',
  Blocked = <any>'ORGANIZATION_MEMBER_STATUS_BLOCKED',
  Expired = <any>'ORGANIZATION_MEMBER_STATUS_EXPIRED',
  Inactive = <any>'ORGANIZATION_MEMBER_STATUS_INACTIVE',
}
