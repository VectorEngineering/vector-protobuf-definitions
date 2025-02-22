/* tslint:disable */
/* eslint-disable */
/**
 * Lead Scraping Service API
 * Vector Lead Scraping Service API - Manages Lead Scraping Jobs
 *
 * The version of the OpenAPI document: 1.0
 * Contact: yoanyomba@vector.ai
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { mapValues } from '../runtime';
import type { OperationDetails } from './OperationDetails';
import {
  OperationDetailsFromJSON,
  OperationDetailsFromJSONTyped,
  OperationDetailsToJSON,
} from './OperationDetails';
import type { InternalErrorCode } from './InternalErrorCode';
import {
  InternalErrorCodeFromJSON,
  InternalErrorCodeFromJSONTyped,
  InternalErrorCodeToJSON,
} from './InternalErrorCode';
import type { ServiceStatus } from './ServiceStatus';
import {
  ServiceStatusFromJSON,
  ServiceStatusFromJSONTyped,
  ServiceStatusToJSON,
} from './ServiceStatus';
import type { ResourceUtilization } from './ResourceUtilization';
import {
  ResourceUtilizationFromJSON,
  ResourceUtilizationFromJSONTyped,
  ResourceUtilizationToJSON,
} from './ResourceUtilization';
import type { ErrorResponse } from './ErrorResponse';
import {
  ErrorResponseFromJSON,
  ErrorResponseFromJSONTyped,
  ErrorResponseToJSON,
} from './ErrorResponse';

/**
 * Represents internal server and system-level errors
 * @export
 * @interface InternalErrorMessageResponse
 */
export interface InternalErrorMessageResponse {
  /**
   *
   * @type {InternalErrorCode}
   * @memberof InternalErrorMessageResponse
   */
  code?: InternalErrorCode;
  /**
   *
   * @type {string}
   * @memberof InternalErrorMessageResponse
   */
  message?: string;
  /**
   *
   * @type {string}
   * @memberof InternalErrorMessageResponse
   */
  incidentId?: string;
  /**
   *
   * @type {ServiceStatus}
   * @memberof InternalErrorMessageResponse
   */
  serviceStatus?: ServiceStatus;
  /**
   *
   * @type {ResourceUtilization}
   * @memberof InternalErrorMessageResponse
   */
  resourceUtilization?: ResourceUtilization;
  /**
   *
   * @type {OperationDetails}
   * @memberof InternalErrorMessageResponse
   */
  operationDetails?: OperationDetails;
  /**
   *
   * @type {ErrorResponse}
   * @memberof InternalErrorMessageResponse
   */
  errorResponse?: ErrorResponse;
}

/**
 * Check if a given object implements the InternalErrorMessageResponse interface.
 */
export function instanceOfInternalErrorMessageResponse(
  value: object
): value is InternalErrorMessageResponse {
  return true;
}

export function InternalErrorMessageResponseFromJSON(
  json: any
): InternalErrorMessageResponse {
  return InternalErrorMessageResponseFromJSONTyped(json, false);
}

export function InternalErrorMessageResponseFromJSONTyped(
  json: any,
  ignoreDiscriminator: boolean
): InternalErrorMessageResponse {
  if (json == null) {
    return json;
  }
  return {
    code:
      json['code'] == null
        ? undefined
        : InternalErrorCodeFromJSON(json['code']),
    message: json['message'] == null ? undefined : json['message'],
    incidentId: json['incidentId'] == null ? undefined : json['incidentId'],
    serviceStatus:
      json['serviceStatus'] == null
        ? undefined
        : ServiceStatusFromJSON(json['serviceStatus']),
    resourceUtilization:
      json['resourceUtilization'] == null
        ? undefined
        : ResourceUtilizationFromJSON(json['resourceUtilization']),
    operationDetails:
      json['operationDetails'] == null
        ? undefined
        : OperationDetailsFromJSON(json['operationDetails']),
    errorResponse:
      json['errorResponse'] == null
        ? undefined
        : ErrorResponseFromJSON(json['errorResponse']),
  };
}

export function InternalErrorMessageResponseToJSON(
  value?: InternalErrorMessageResponse | null
): any {
  if (value == null) {
    return value;
  }
  return {
    code: InternalErrorCodeToJSON(value['code']),
    message: value['message'],
    incidentId: value['incidentId'],
    serviceStatus: ServiceStatusToJSON(value['serviceStatus']),
    resourceUtilization: ResourceUtilizationToJSON(
      value['resourceUtilization']
    ),
    operationDetails: OperationDetailsToJSON(value['operationDetails']),
    errorResponse: ErrorResponseToJSON(value['errorResponse']),
  };
}
