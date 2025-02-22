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
import type { FieldError } from './FieldError';
import {
  FieldErrorFromJSON,
  FieldErrorFromJSONTyped,
  FieldErrorToJSON,
} from './FieldError';
import type { ErrorResponse } from './ErrorResponse';
import {
  ErrorResponseFromJSON,
  ErrorResponseFromJSONTyped,
  ErrorResponseToJSON,
} from './ErrorResponse';

/**
 * Represents errors when the server understands the content type but cannot process the instructions
 * @export
 * @interface UnprocessableEntityErrorMessageResponse
 */
export interface UnprocessableEntityErrorMessageResponse {
  /**
   *
   * @type {number}
   * @memberof UnprocessableEntityErrorMessageResponse
   */
  code?: number;
  /**
   *
   * @type {string}
   * @memberof UnprocessableEntityErrorMessageResponse
   */
  message?: string;
  /**
   *
   * @type {Array<FieldError>}
   * @memberof UnprocessableEntityErrorMessageResponse
   */
  errors?: Array<FieldError>;
  /**
   *
   * @type {ErrorResponse}
   * @memberof UnprocessableEntityErrorMessageResponse
   */
  errorResponse?: ErrorResponse;
}

/**
 * Check if a given object implements the UnprocessableEntityErrorMessageResponse interface.
 */
export function instanceOfUnprocessableEntityErrorMessageResponse(
  value: object
): value is UnprocessableEntityErrorMessageResponse {
  return true;
}

export function UnprocessableEntityErrorMessageResponseFromJSON(
  json: any
): UnprocessableEntityErrorMessageResponse {
  return UnprocessableEntityErrorMessageResponseFromJSONTyped(json, false);
}

export function UnprocessableEntityErrorMessageResponseFromJSONTyped(
  json: any,
  ignoreDiscriminator: boolean
): UnprocessableEntityErrorMessageResponse {
  if (json == null) {
    return json;
  }
  return {
    code: json['code'] == null ? undefined : json['code'],
    message: json['message'] == null ? undefined : json['message'],
    errors:
      json['errors'] == null
        ? undefined
        : (json['errors'] as Array<any>).map(FieldErrorFromJSON),
    errorResponse:
      json['errorResponse'] == null
        ? undefined
        : ErrorResponseFromJSON(json['errorResponse']),
  };
}

export function UnprocessableEntityErrorMessageResponseToJSON(
  value?: UnprocessableEntityErrorMessageResponse | null
): any {
  if (value == null) {
    return value;
  }
  return {
    code: value['code'],
    message: value['message'],
    errors:
      value['errors'] == null
        ? undefined
        : (value['errors'] as Array<any>).map(FieldErrorToJSON),
    errorResponse: ErrorResponseToJSON(value['errorResponse']),
  };
}
