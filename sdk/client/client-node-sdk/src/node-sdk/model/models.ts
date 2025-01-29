import localVarRequest from 'request';

export * from './aPIInfo';
export * from './account';
export * from './any';
export * from './authContext';
export * from './authErrorCode';
export * from './authenticationErrorMessageResponse';
export * from './availabilityInfo';
export * from './backgroundJobStatus';
export * from './badGatewayErrorMessageResponse';
export * from './conflictErrorMessageResponse';
export * from './conflictInfo';
export * from './createAccountRequest';
export * from './createAccountResponse';
export * from './createScrapingJobRequest';
export * from './createScrapingJobResponse';
export * from './deleteAccountResponse';
export * from './deleteScrapingJobResponse';
export * from './dependency';
export * from './downloadScrapingResultsResponse';
export * from './errorResponse';
export * from './fieldError';
export * from './fieldViolation';
export * from './forbiddenErrorMessageResponse';
export * from './gatewayTimeoutErrorMessageResponse';
export * from './getAccountResponse';
export * from './getScrapingJobResponse';
export * from './goneErrorMessageResponse';
export * from './internalErrorCode';
export * from './internalErrorMessageResponse';
export * from './limitInfo';
export * from './listAccountsResponse';
export * from './listScrapingJobsResponse';
export * from './mFAInfo';
export * from './methodNotAllowedErrorMessageResponse';
export * from './notFoundErrorCode';
export * from './notFoundErrorMessageResponse';
export * from './notImplementedErrorMessageResponse';
export * from './operationDetails';
export * from './paymentInfo';
export * from './paymentRequiredErrorMessageResponse';
export * from './preconditionFailedErrorMessageResponse';
export * from './quotaInfo';
export * from './rateLimitContext';
export * from './rateLimitErrorMessageResponse';
export * from './resourceInfo';
export * from './resourceUtilization';
export * from './resourceValidation';
export * from './schemaValidation';
export * from './scrapingJob';
export * from './serviceStatus';
export * from './serviceUnavailableErrorMessageResponse';
export * from './sessionInfo';
export * from './status';
export * from './suggestions';
export * from './tokenInfo';
export * from './tooEarlyErrorMessageResponse';
export * from './unprocessableEntityErrorMessageResponse';
export * from './updateAccountRequest';
export * from './updateAccountResponse';
export * from './validationErrorCode';
export * from './validationErrorMessageResponse';

import * as fs from 'fs';

export interface RequestDetailedFile {
    value: Buffer;
    options?: {
        filename?: string;
        contentType?: string;
    }
}

export type RequestFile = string | Buffer | fs.ReadStream | RequestDetailedFile;


import { APIInfo } from './aPIInfo';
import { Account } from './account';
import { Any } from './any';
import { AuthContext } from './authContext';
import { AuthErrorCode } from './authErrorCode';
import { AuthenticationErrorMessageResponse } from './authenticationErrorMessageResponse';
import { AvailabilityInfo } from './availabilityInfo';
import { BackgroundJobStatus } from './backgroundJobStatus';
import { BadGatewayErrorMessageResponse } from './badGatewayErrorMessageResponse';
import { ConflictErrorMessageResponse } from './conflictErrorMessageResponse';
import { ConflictInfo } from './conflictInfo';
import { CreateAccountRequest } from './createAccountRequest';
import { CreateAccountResponse } from './createAccountResponse';
import { CreateScrapingJobRequest } from './createScrapingJobRequest';
import { CreateScrapingJobResponse } from './createScrapingJobResponse';
import { DeleteAccountResponse } from './deleteAccountResponse';
import { DeleteScrapingJobResponse } from './deleteScrapingJobResponse';
import { Dependency } from './dependency';
import { DownloadScrapingResultsResponse } from './downloadScrapingResultsResponse';
import { ErrorResponse } from './errorResponse';
import { FieldError } from './fieldError';
import { FieldViolation } from './fieldViolation';
import { ForbiddenErrorMessageResponse } from './forbiddenErrorMessageResponse';
import { GatewayTimeoutErrorMessageResponse } from './gatewayTimeoutErrorMessageResponse';
import { GetAccountResponse } from './getAccountResponse';
import { GetScrapingJobResponse } from './getScrapingJobResponse';
import { GoneErrorMessageResponse } from './goneErrorMessageResponse';
import { InternalErrorCode } from './internalErrorCode';
import { InternalErrorMessageResponse } from './internalErrorMessageResponse';
import { LimitInfo } from './limitInfo';
import { ListAccountsResponse } from './listAccountsResponse';
import { ListScrapingJobsResponse } from './listScrapingJobsResponse';
import { MFAInfo } from './mFAInfo';
import { MethodNotAllowedErrorMessageResponse } from './methodNotAllowedErrorMessageResponse';
import { NotFoundErrorCode } from './notFoundErrorCode';
import { NotFoundErrorMessageResponse } from './notFoundErrorMessageResponse';
import { NotImplementedErrorMessageResponse } from './notImplementedErrorMessageResponse';
import { OperationDetails } from './operationDetails';
import { PaymentInfo } from './paymentInfo';
import { PaymentRequiredErrorMessageResponse } from './paymentRequiredErrorMessageResponse';
import { PreconditionFailedErrorMessageResponse } from './preconditionFailedErrorMessageResponse';
import { QuotaInfo } from './quotaInfo';
import { RateLimitContext } from './rateLimitContext';
import { RateLimitErrorMessageResponse } from './rateLimitErrorMessageResponse';
import { ResourceInfo } from './resourceInfo';
import { ResourceUtilization } from './resourceUtilization';
import { ResourceValidation } from './resourceValidation';
import { SchemaValidation } from './schemaValidation';
import { ScrapingJob } from './scrapingJob';
import { ServiceStatus } from './serviceStatus';
import { ServiceUnavailableErrorMessageResponse } from './serviceUnavailableErrorMessageResponse';
import { SessionInfo } from './sessionInfo';
import { Status } from './status';
import { Suggestions } from './suggestions';
import { TokenInfo } from './tokenInfo';
import { TooEarlyErrorMessageResponse } from './tooEarlyErrorMessageResponse';
import { UnprocessableEntityErrorMessageResponse } from './unprocessableEntityErrorMessageResponse';
import { UpdateAccountRequest } from './updateAccountRequest';
import { UpdateAccountResponse } from './updateAccountResponse';
import { ValidationErrorCode } from './validationErrorCode';
import { ValidationErrorMessageResponse } from './validationErrorMessageResponse';

/* tslint:disable:no-unused-variable */
let primitives = [
                    "string",
                    "boolean",
                    "double",
                    "integer",
                    "long",
                    "float",
                    "number",
                    "any"
                 ];

let enumsMap: {[index: string]: any} = {
        "AuthErrorCode": AuthErrorCode,
        "BackgroundJobStatus": BackgroundJobStatus,
        "InternalErrorCode": InternalErrorCode,
        "NotFoundErrorCode": NotFoundErrorCode,
        "ValidationErrorCode": ValidationErrorCode,
}

let typeMap: {[index: string]: any} = {
    "APIInfo": APIInfo,
    "Account": Account,
    "Any": Any,
    "AuthContext": AuthContext,
    "AuthenticationErrorMessageResponse": AuthenticationErrorMessageResponse,
    "AvailabilityInfo": AvailabilityInfo,
    "BadGatewayErrorMessageResponse": BadGatewayErrorMessageResponse,
    "ConflictErrorMessageResponse": ConflictErrorMessageResponse,
    "ConflictInfo": ConflictInfo,
    "CreateAccountRequest": CreateAccountRequest,
    "CreateAccountResponse": CreateAccountResponse,
    "CreateScrapingJobRequest": CreateScrapingJobRequest,
    "CreateScrapingJobResponse": CreateScrapingJobResponse,
    "DeleteAccountResponse": DeleteAccountResponse,
    "DeleteScrapingJobResponse": DeleteScrapingJobResponse,
    "Dependency": Dependency,
    "DownloadScrapingResultsResponse": DownloadScrapingResultsResponse,
    "ErrorResponse": ErrorResponse,
    "FieldError": FieldError,
    "FieldViolation": FieldViolation,
    "ForbiddenErrorMessageResponse": ForbiddenErrorMessageResponse,
    "GatewayTimeoutErrorMessageResponse": GatewayTimeoutErrorMessageResponse,
    "GetAccountResponse": GetAccountResponse,
    "GetScrapingJobResponse": GetScrapingJobResponse,
    "GoneErrorMessageResponse": GoneErrorMessageResponse,
    "InternalErrorMessageResponse": InternalErrorMessageResponse,
    "LimitInfo": LimitInfo,
    "ListAccountsResponse": ListAccountsResponse,
    "ListScrapingJobsResponse": ListScrapingJobsResponse,
    "MFAInfo": MFAInfo,
    "MethodNotAllowedErrorMessageResponse": MethodNotAllowedErrorMessageResponse,
    "NotFoundErrorMessageResponse": NotFoundErrorMessageResponse,
    "NotImplementedErrorMessageResponse": NotImplementedErrorMessageResponse,
    "OperationDetails": OperationDetails,
    "PaymentInfo": PaymentInfo,
    "PaymentRequiredErrorMessageResponse": PaymentRequiredErrorMessageResponse,
    "PreconditionFailedErrorMessageResponse": PreconditionFailedErrorMessageResponse,
    "QuotaInfo": QuotaInfo,
    "RateLimitContext": RateLimitContext,
    "RateLimitErrorMessageResponse": RateLimitErrorMessageResponse,
    "ResourceInfo": ResourceInfo,
    "ResourceUtilization": ResourceUtilization,
    "ResourceValidation": ResourceValidation,
    "SchemaValidation": SchemaValidation,
    "ScrapingJob": ScrapingJob,
    "ServiceStatus": ServiceStatus,
    "ServiceUnavailableErrorMessageResponse": ServiceUnavailableErrorMessageResponse,
    "SessionInfo": SessionInfo,
    "Status": Status,
    "Suggestions": Suggestions,
    "TokenInfo": TokenInfo,
    "TooEarlyErrorMessageResponse": TooEarlyErrorMessageResponse,
    "UnprocessableEntityErrorMessageResponse": UnprocessableEntityErrorMessageResponse,
    "UpdateAccountRequest": UpdateAccountRequest,
    "UpdateAccountResponse": UpdateAccountResponse,
    "ValidationErrorMessageResponse": ValidationErrorMessageResponse,
}

export class ObjectSerializer {
    public static findCorrectType(data: any, expectedType: string) {
        if (data == undefined) {
            return expectedType;
        } else if (primitives.indexOf(expectedType.toLowerCase()) !== -1) {
            return expectedType;
        } else if (expectedType === "Date") {
            return expectedType;
        } else {
            if (enumsMap[expectedType]) {
                return expectedType;
            }

            if (!typeMap[expectedType]) {
                return expectedType; // w/e we don't know the type
            }

            // Check the discriminator
            let discriminatorProperty = typeMap[expectedType].discriminator;
            if (discriminatorProperty == null) {
                return expectedType; // the type does not have a discriminator. use it.
            } else {
                if (data[discriminatorProperty]) {
                    var discriminatorType = data[discriminatorProperty];
                    if(typeMap[discriminatorType]){
                        return discriminatorType; // use the type given in the discriminator
                    } else {
                        return expectedType; // discriminator did not map to a type
                    }
                } else {
                    return expectedType; // discriminator was not present (or an empty string)
                }
            }
        }
    }

    public static serialize(data: any, type: string) {
        if (data == undefined) {
            return data;
        } else if (primitives.indexOf(type.toLowerCase()) !== -1) {
            return data;
        } else if (type.lastIndexOf("Array<", 0) === 0) { // string.startsWith pre es6
            let subType: string = type.replace("Array<", ""); // Array<Type> => Type>
            subType = subType.substring(0, subType.length - 1); // Type> => Type
            let transformedData: any[] = [];
            for (let index = 0; index < data.length; index++) {
                let datum = data[index];
                transformedData.push(ObjectSerializer.serialize(datum, subType));
            }
            return transformedData;
        } else if (type === "Date") {
            return data.toISOString();
        } else {
            if (enumsMap[type]) {
                return data;
            }
            if (!typeMap[type]) { // in case we dont know the type
                return data;
            }

            // Get the actual type of this object
            type = this.findCorrectType(data, type);

            // get the map for the correct type.
            let attributeTypes = typeMap[type].getAttributeTypeMap();
            let instance: {[index: string]: any} = {};
            for (let index = 0; index < attributeTypes.length; index++) {
                let attributeType = attributeTypes[index];
                instance[attributeType.baseName] = ObjectSerializer.serialize(data[attributeType.name], attributeType.type);
            }
            return instance;
        }
    }

    public static deserialize(data: any, type: string) {
        // polymorphism may change the actual type.
        type = ObjectSerializer.findCorrectType(data, type);
        if (data == undefined) {
            return data;
        } else if (primitives.indexOf(type.toLowerCase()) !== -1) {
            return data;
        } else if (type.lastIndexOf("Array<", 0) === 0) { // string.startsWith pre es6
            let subType: string = type.replace("Array<", ""); // Array<Type> => Type>
            subType = subType.substring(0, subType.length - 1); // Type> => Type
            let transformedData: any[] = [];
            for (let index = 0; index < data.length; index++) {
                let datum = data[index];
                transformedData.push(ObjectSerializer.deserialize(datum, subType));
            }
            return transformedData;
        } else if (type === "Date") {
            return new Date(data);
        } else {
            if (enumsMap[type]) {// is Enum
                return data;
            }

            if (!typeMap[type]) { // dont know the type
                return data;
            }
            let instance = new typeMap[type]();
            let attributeTypes = typeMap[type].getAttributeTypeMap();
            for (let index = 0; index < attributeTypes.length; index++) {
                let attributeType = attributeTypes[index];
                instance[attributeType.name] = ObjectSerializer.deserialize(data[attributeType.baseName], attributeType.type);
            }
            return instance;
        }
    }
}

export interface Authentication {
    /**
    * Apply authentication settings to header and query params.
    */
    applyToRequest(requestOptions: localVarRequest.Options): Promise<void> | void;
}

export class HttpBasicAuth implements Authentication {
    public username: string = '';
    public password: string = '';

    applyToRequest(requestOptions: localVarRequest.Options): void {
        requestOptions.auth = {
            username: this.username, password: this.password
        }
    }
}

export class HttpBearerAuth implements Authentication {
    public accessToken: string | (() => string) = '';

    applyToRequest(requestOptions: localVarRequest.Options): void {
        if (requestOptions && requestOptions.headers) {
            const accessToken = typeof this.accessToken === 'function'
                            ? this.accessToken()
                            : this.accessToken;
            requestOptions.headers["Authorization"] = "Bearer " + accessToken;
        }
    }
}

export class ApiKeyAuth implements Authentication {
    public apiKey: string = '';

    constructor(private location: string, private paramName: string) {
    }

    applyToRequest(requestOptions: localVarRequest.Options): void {
        if (this.location == "query") {
            (<any>requestOptions.qs)[this.paramName] = this.apiKey;
        } else if (this.location == "header" && requestOptions && requestOptions.headers) {
            requestOptions.headers[this.paramName] = this.apiKey;
        } else if (this.location == 'cookie' && requestOptions && requestOptions.headers) {
            if (requestOptions.headers['Cookie']) {
                requestOptions.headers['Cookie'] += '; ' + this.paramName + '=' + encodeURIComponent(this.apiKey);
            }
            else {
                requestOptions.headers['Cookie'] = this.paramName + '=' + encodeURIComponent(this.apiKey);
            }
        }
    }
}

export class OAuth implements Authentication {
    public accessToken: string = '';

    applyToRequest(requestOptions: localVarRequest.Options): void {
        if (requestOptions && requestOptions.headers) {
            requestOptions.headers["Authorization"] = "Bearer " + this.accessToken;
        }
    }
}

export class VoidAuth implements Authentication {
    public username: string = '';
    public password: string = '';

    applyToRequest(_: localVarRequest.Options): void {
        // Do nothing
    }
}

export type Interceptor = (requestOptions: localVarRequest.Options) => (Promise<void> | void);
