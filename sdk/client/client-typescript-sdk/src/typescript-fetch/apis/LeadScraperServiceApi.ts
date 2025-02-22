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

import * as runtime from '../runtime';
import type {
  AuthenticationErrorMessageResponse,
  BadGatewayErrorMessageResponse,
  ConflictErrorMessageResponse,
  CreateAccountRequest,
  CreateAccountResponse,
  CreateScrapingJobRequest,
  CreateScrapingJobResponse,
  DeleteAccountResponse,
  DeleteScrapingJobResponse,
  DownloadScrapingResultsResponse,
  ForbiddenErrorMessageResponse,
  GatewayTimeoutErrorMessageResponse,
  GetAccountResponse,
  GetScrapingJobResponse,
  GoneErrorMessageResponse,
  InternalErrorMessageResponse,
  ListAccountsResponse,
  ListScrapingJobsResponse,
  MethodNotAllowedErrorMessageResponse,
  NotFoundErrorMessageResponse,
  NotImplementedErrorMessageResponse,
  PaymentRequiredErrorMessageResponse,
  PreconditionFailedErrorMessageResponse,
  RateLimitErrorMessageResponse,
  ServiceUnavailableErrorMessageResponse,
  Status,
  TooEarlyErrorMessageResponse,
  UnprocessableEntityErrorMessageResponse,
  UpdateAccountRequest,
  UpdateAccountResponse,
  ValidationErrorMessageResponse,
} from '../models/index';
import {
  AuthenticationErrorMessageResponseFromJSON,
  AuthenticationErrorMessageResponseToJSON,
  BadGatewayErrorMessageResponseFromJSON,
  BadGatewayErrorMessageResponseToJSON,
  ConflictErrorMessageResponseFromJSON,
  ConflictErrorMessageResponseToJSON,
  CreateAccountRequestFromJSON,
  CreateAccountRequestToJSON,
  CreateAccountResponseFromJSON,
  CreateAccountResponseToJSON,
  CreateScrapingJobRequestFromJSON,
  CreateScrapingJobRequestToJSON,
  CreateScrapingJobResponseFromJSON,
  CreateScrapingJobResponseToJSON,
  DeleteAccountResponseFromJSON,
  DeleteAccountResponseToJSON,
  DeleteScrapingJobResponseFromJSON,
  DeleteScrapingJobResponseToJSON,
  DownloadScrapingResultsResponseFromJSON,
  DownloadScrapingResultsResponseToJSON,
  ForbiddenErrorMessageResponseFromJSON,
  ForbiddenErrorMessageResponseToJSON,
  GatewayTimeoutErrorMessageResponseFromJSON,
  GatewayTimeoutErrorMessageResponseToJSON,
  GetAccountResponseFromJSON,
  GetAccountResponseToJSON,
  GetScrapingJobResponseFromJSON,
  GetScrapingJobResponseToJSON,
  GoneErrorMessageResponseFromJSON,
  GoneErrorMessageResponseToJSON,
  InternalErrorMessageResponseFromJSON,
  InternalErrorMessageResponseToJSON,
  ListAccountsResponseFromJSON,
  ListAccountsResponseToJSON,
  ListScrapingJobsResponseFromJSON,
  ListScrapingJobsResponseToJSON,
  MethodNotAllowedErrorMessageResponseFromJSON,
  MethodNotAllowedErrorMessageResponseToJSON,
  NotFoundErrorMessageResponseFromJSON,
  NotFoundErrorMessageResponseToJSON,
  NotImplementedErrorMessageResponseFromJSON,
  NotImplementedErrorMessageResponseToJSON,
  PaymentRequiredErrorMessageResponseFromJSON,
  PaymentRequiredErrorMessageResponseToJSON,
  PreconditionFailedErrorMessageResponseFromJSON,
  PreconditionFailedErrorMessageResponseToJSON,
  RateLimitErrorMessageResponseFromJSON,
  RateLimitErrorMessageResponseToJSON,
  ServiceUnavailableErrorMessageResponseFromJSON,
  ServiceUnavailableErrorMessageResponseToJSON,
  StatusFromJSON,
  StatusToJSON,
  TooEarlyErrorMessageResponseFromJSON,
  TooEarlyErrorMessageResponseToJSON,
  UnprocessableEntityErrorMessageResponseFromJSON,
  UnprocessableEntityErrorMessageResponseToJSON,
  UpdateAccountRequestFromJSON,
  UpdateAccountRequestToJSON,
  UpdateAccountResponseFromJSON,
  UpdateAccountResponseToJSON,
  ValidationErrorMessageResponseFromJSON,
  ValidationErrorMessageResponseToJSON,
} from '../models/index';

export interface CreateAccountOperationRequest {
  createAccountRequest: CreateAccountRequest;
}

export interface CreateScrapingJobOperationRequest {
  createScrapingJobRequest: CreateScrapingJobRequest;
}

export interface DeleteAccountRequest {
  accountId: string;
  orgId: string;
  tenantId: string;
}

export interface DeleteScrapingJobRequest {
  jobId: string;
  userId: string;
  orgId: string;
  tenantId: string;
}

export interface DownloadScrapingResultsRequest {
  jobId: string;
  userId: string;
  orgId: string;
  tenantId: string;
}

export interface GetAccountRequest {
  accountId: string;
  orgId: string;
  tenantId: string;
}

export interface GetScrapingJobRequest {
  jobId: string;
  userId: string;
  orgId: string;
  tenantId: string;
}

export interface ListAccountsRequest {
  orgId: string;
  tenantId: string;
  offset?: number;
  limit?: number;
}

export interface ListScrapingJobsRequest {
  userId: string;
  orgId: string;
  tenantId: string;
}

export interface UpdateAccountOperationRequest {
  updateAccountRequest: UpdateAccountRequest;
}

/**
 *
 */
export class LeadScraperServiceApi extends runtime.BaseAPI {
  /**
   * Creates a new user account in the workspace service
   * Create a new account
   */
  async createAccountRaw(
    requestParameters: CreateAccountOperationRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction
  ): Promise<runtime.ApiResponse<CreateAccountResponse>> {
    if (requestParameters['createAccountRequest'] == null) {
      throw new runtime.RequiredError(
        'createAccountRequest',
        'Required parameter "createAccountRequest" was null or undefined when calling createAccount().'
      );
    }

    const queryParameters: any = {};

    const headerParameters: runtime.HTTPHeaders = {};

    headerParameters['Content-Type'] = 'application/json';

    const response = await this.request(
      {
        path: `/lead-scraper-microservice/api/v1/accounts`,
        method: 'POST',
        headers: headerParameters,
        query: queryParameters,
        body: CreateAccountRequestToJSON(
          requestParameters['createAccountRequest']
        ),
      },
      initOverrides
    );

    return new runtime.JSONApiResponse(response, jsonValue =>
      CreateAccountResponseFromJSON(jsonValue)
    );
  }

  /**
   * Creates a new user account in the workspace service
   * Create a new account
   */
  async createAccount(
    requestParameters: CreateAccountOperationRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction
  ): Promise<CreateAccountResponse> {
    const response = await this.createAccountRaw(
      requestParameters,
      initOverrides
    );
    return await response.value();
  }

  /**
   * This endpoint creates a new Google Maps scraping job
   * Create a new job scraping task
   */
  async createScrapingJobRaw(
    requestParameters: CreateScrapingJobOperationRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction
  ): Promise<runtime.ApiResponse<CreateScrapingJobResponse>> {
    if (requestParameters['createScrapingJobRequest'] == null) {
      throw new runtime.RequiredError(
        'createScrapingJobRequest',
        'Required parameter "createScrapingJobRequest" was null or undefined when calling createScrapingJob().'
      );
    }

    const queryParameters: any = {};

    const headerParameters: runtime.HTTPHeaders = {};

    headerParameters['Content-Type'] = 'application/json';

    const response = await this.request(
      {
        path: `/lead-scraper-microservice/api/v1/jobs`,
        method: 'POST',
        headers: headerParameters,
        query: queryParameters,
        body: CreateScrapingJobRequestToJSON(
          requestParameters['createScrapingJobRequest']
        ),
      },
      initOverrides
    );

    return new runtime.JSONApiResponse(response, jsonValue =>
      CreateScrapingJobResponseFromJSON(jsonValue)
    );
  }

  /**
   * This endpoint creates a new Google Maps scraping job
   * Create a new job scraping task
   */
  async createScrapingJob(
    requestParameters: CreateScrapingJobOperationRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction
  ): Promise<CreateScrapingJobResponse> {
    const response = await this.createScrapingJobRaw(
      requestParameters,
      initOverrides
    );
    return await response.value();
  }

  /**
   * Permanently deletes an account and associated resources
   * Delete account
   */
  async deleteAccountRaw(
    requestParameters: DeleteAccountRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction
  ): Promise<runtime.ApiResponse<DeleteAccountResponse>> {
    if (requestParameters['accountId'] == null) {
      throw new runtime.RequiredError(
        'accountId',
        'Required parameter "accountId" was null or undefined when calling deleteAccount().'
      );
    }

    if (requestParameters['orgId'] == null) {
      throw new runtime.RequiredError(
        'orgId',
        'Required parameter "orgId" was null or undefined when calling deleteAccount().'
      );
    }

    if (requestParameters['tenantId'] == null) {
      throw new runtime.RequiredError(
        'tenantId',
        'Required parameter "tenantId" was null or undefined when calling deleteAccount().'
      );
    }

    const queryParameters: any = {};

    if (requestParameters['orgId'] != null) {
      queryParameters['orgId'] = requestParameters['orgId'];
    }

    if (requestParameters['tenantId'] != null) {
      queryParameters['tenantId'] = requestParameters['tenantId'];
    }

    const headerParameters: runtime.HTTPHeaders = {};

    const response = await this.request(
      {
        path: `/lead-scraper-microservice/api/v1/accounts/{accountId}`.replace(
          `{${'accountId'}}`,
          encodeURIComponent(String(requestParameters['accountId']))
        ),
        method: 'DELETE',
        headers: headerParameters,
        query: queryParameters,
      },
      initOverrides
    );

    return new runtime.JSONApiResponse(response, jsonValue =>
      DeleteAccountResponseFromJSON(jsonValue)
    );
  }

  /**
   * Permanently deletes an account and associated resources
   * Delete account
   */
  async deleteAccount(
    requestParameters: DeleteAccountRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction
  ): Promise<DeleteAccountResponse> {
    const response = await this.deleteAccountRaw(
      requestParameters,
      initOverrides
    );
    return await response.value();
  }

  /**
   * This endpoint deletes a specific Google Maps scraping job
   * Delete a specific job
   */
  async deleteScrapingJobRaw(
    requestParameters: DeleteScrapingJobRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction
  ): Promise<runtime.ApiResponse<DeleteScrapingJobResponse>> {
    if (requestParameters['jobId'] == null) {
      throw new runtime.RequiredError(
        'jobId',
        'Required parameter "jobId" was null or undefined when calling deleteScrapingJob().'
      );
    }

    if (requestParameters['userId'] == null) {
      throw new runtime.RequiredError(
        'userId',
        'Required parameter "userId" was null or undefined when calling deleteScrapingJob().'
      );
    }

    if (requestParameters['orgId'] == null) {
      throw new runtime.RequiredError(
        'orgId',
        'Required parameter "orgId" was null or undefined when calling deleteScrapingJob().'
      );
    }

    if (requestParameters['tenantId'] == null) {
      throw new runtime.RequiredError(
        'tenantId',
        'Required parameter "tenantId" was null or undefined when calling deleteScrapingJob().'
      );
    }

    const queryParameters: any = {};

    if (requestParameters['userId'] != null) {
      queryParameters['userId'] = requestParameters['userId'];
    }

    if (requestParameters['orgId'] != null) {
      queryParameters['orgId'] = requestParameters['orgId'];
    }

    if (requestParameters['tenantId'] != null) {
      queryParameters['tenantId'] = requestParameters['tenantId'];
    }

    const headerParameters: runtime.HTTPHeaders = {};

    const response = await this.request(
      {
        path: `/lead-scraper-microservice/api/v1/jobs/{jobId}`.replace(
          `{${'jobId'}}`,
          encodeURIComponent(String(requestParameters['jobId']))
        ),
        method: 'DELETE',
        headers: headerParameters,
        query: queryParameters,
      },
      initOverrides
    );

    return new runtime.JSONApiResponse(response, jsonValue =>
      DeleteScrapingJobResponseFromJSON(jsonValue)
    );
  }

  /**
   * This endpoint deletes a specific Google Maps scraping job
   * Delete a specific job
   */
  async deleteScrapingJob(
    requestParameters: DeleteScrapingJobRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction
  ): Promise<DeleteScrapingJobResponse> {
    const response = await this.deleteScrapingJobRaw(
      requestParameters,
      initOverrides
    );
    return await response.value();
  }

  /**
   * This endpoint downloads the results of a Google Maps scraping job as CSV
   * Download job results as CSV
   */
  async downloadScrapingResultsRaw(
    requestParameters: DownloadScrapingResultsRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction
  ): Promise<runtime.ApiResponse<DownloadScrapingResultsResponse>> {
    if (requestParameters['jobId'] == null) {
      throw new runtime.RequiredError(
        'jobId',
        'Required parameter "jobId" was null or undefined when calling downloadScrapingResults().'
      );
    }

    if (requestParameters['userId'] == null) {
      throw new runtime.RequiredError(
        'userId',
        'Required parameter "userId" was null or undefined when calling downloadScrapingResults().'
      );
    }

    if (requestParameters['orgId'] == null) {
      throw new runtime.RequiredError(
        'orgId',
        'Required parameter "orgId" was null or undefined when calling downloadScrapingResults().'
      );
    }

    if (requestParameters['tenantId'] == null) {
      throw new runtime.RequiredError(
        'tenantId',
        'Required parameter "tenantId" was null or undefined when calling downloadScrapingResults().'
      );
    }

    const queryParameters: any = {};

    if (requestParameters['userId'] != null) {
      queryParameters['userId'] = requestParameters['userId'];
    }

    if (requestParameters['orgId'] != null) {
      queryParameters['orgId'] = requestParameters['orgId'];
    }

    if (requestParameters['tenantId'] != null) {
      queryParameters['tenantId'] = requestParameters['tenantId'];
    }

    const headerParameters: runtime.HTTPHeaders = {};

    const response = await this.request(
      {
        path: `/lead-scraper-microservice/api/v1/jobs/{jobId}/download`.replace(
          `{${'jobId'}}`,
          encodeURIComponent(String(requestParameters['jobId']))
        ),
        method: 'GET',
        headers: headerParameters,
        query: queryParameters,
      },
      initOverrides
    );

    return new runtime.JSONApiResponse(response, jsonValue =>
      DownloadScrapingResultsResponseFromJSON(jsonValue)
    );
  }

  /**
   * This endpoint downloads the results of a Google Maps scraping job as CSV
   * Download job results as CSV
   */
  async downloadScrapingResults(
    requestParameters: DownloadScrapingResultsRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction
  ): Promise<DownloadScrapingResultsResponse> {
    const response = await this.downloadScrapingResultsRaw(
      requestParameters,
      initOverrides
    );
    return await response.value();
  }

  /**
   * Retrieves details of a specific account
   * Get account details
   */
  async getAccountRaw(
    requestParameters: GetAccountRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction
  ): Promise<runtime.ApiResponse<GetAccountResponse>> {
    if (requestParameters['accountId'] == null) {
      throw new runtime.RequiredError(
        'accountId',
        'Required parameter "accountId" was null or undefined when calling getAccount().'
      );
    }

    if (requestParameters['orgId'] == null) {
      throw new runtime.RequiredError(
        'orgId',
        'Required parameter "orgId" was null or undefined when calling getAccount().'
      );
    }

    if (requestParameters['tenantId'] == null) {
      throw new runtime.RequiredError(
        'tenantId',
        'Required parameter "tenantId" was null or undefined when calling getAccount().'
      );
    }

    const queryParameters: any = {};

    if (requestParameters['orgId'] != null) {
      queryParameters['orgId'] = requestParameters['orgId'];
    }

    if (requestParameters['tenantId'] != null) {
      queryParameters['tenantId'] = requestParameters['tenantId'];
    }

    const headerParameters: runtime.HTTPHeaders = {};

    const response = await this.request(
      {
        path: `/lead-scraper-microservice/api/v1/accounts/{accountId}`.replace(
          `{${'accountId'}}`,
          encodeURIComponent(String(requestParameters['accountId']))
        ),
        method: 'GET',
        headers: headerParameters,
        query: queryParameters,
      },
      initOverrides
    );

    return new runtime.JSONApiResponse(response, jsonValue =>
      GetAccountResponseFromJSON(jsonValue)
    );
  }

  /**
   * Retrieves details of a specific account
   * Get account details
   */
  async getAccount(
    requestParameters: GetAccountRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction
  ): Promise<GetAccountResponse> {
    const response = await this.getAccountRaw(requestParameters, initOverrides);
    return await response.value();
  }

  /**
   * This endpoint retrieves a specific Google Maps scraping job
   * Get a specific job
   */
  async getScrapingJobRaw(
    requestParameters: GetScrapingJobRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction
  ): Promise<runtime.ApiResponse<GetScrapingJobResponse>> {
    if (requestParameters['jobId'] == null) {
      throw new runtime.RequiredError(
        'jobId',
        'Required parameter "jobId" was null or undefined when calling getScrapingJob().'
      );
    }

    if (requestParameters['userId'] == null) {
      throw new runtime.RequiredError(
        'userId',
        'Required parameter "userId" was null or undefined when calling getScrapingJob().'
      );
    }

    if (requestParameters['orgId'] == null) {
      throw new runtime.RequiredError(
        'orgId',
        'Required parameter "orgId" was null or undefined when calling getScrapingJob().'
      );
    }

    if (requestParameters['tenantId'] == null) {
      throw new runtime.RequiredError(
        'tenantId',
        'Required parameter "tenantId" was null or undefined when calling getScrapingJob().'
      );
    }

    const queryParameters: any = {};

    if (requestParameters['userId'] != null) {
      queryParameters['userId'] = requestParameters['userId'];
    }

    if (requestParameters['orgId'] != null) {
      queryParameters['orgId'] = requestParameters['orgId'];
    }

    if (requestParameters['tenantId'] != null) {
      queryParameters['tenantId'] = requestParameters['tenantId'];
    }

    const headerParameters: runtime.HTTPHeaders = {};

    const response = await this.request(
      {
        path: `/lead-scraper-microservice/api/v1/jobs/{jobId}`.replace(
          `{${'jobId'}}`,
          encodeURIComponent(String(requestParameters['jobId']))
        ),
        method: 'GET',
        headers: headerParameters,
        query: queryParameters,
      },
      initOverrides
    );

    return new runtime.JSONApiResponse(response, jsonValue =>
      GetScrapingJobResponseFromJSON(jsonValue)
    );
  }

  /**
   * This endpoint retrieves a specific Google Maps scraping job
   * Get a specific job
   */
  async getScrapingJob(
    requestParameters: GetScrapingJobRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction
  ): Promise<GetScrapingJobResponse> {
    const response = await this.getScrapingJobRaw(
      requestParameters,
      initOverrides
    );
    return await response.value();
  }

  /**
   * Retrieves a list of accounts with optional filtering
   * List accounts
   */
  async listAccountsRaw(
    requestParameters: ListAccountsRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction
  ): Promise<runtime.ApiResponse<ListAccountsResponse>> {
    if (requestParameters['orgId'] == null) {
      throw new runtime.RequiredError(
        'orgId',
        'Required parameter "orgId" was null or undefined when calling listAccounts().'
      );
    }

    if (requestParameters['tenantId'] == null) {
      throw new runtime.RequiredError(
        'tenantId',
        'Required parameter "tenantId" was null or undefined when calling listAccounts().'
      );
    }

    const queryParameters: any = {};

    if (requestParameters['orgId'] != null) {
      queryParameters['orgId'] = requestParameters['orgId'];
    }

    if (requestParameters['tenantId'] != null) {
      queryParameters['tenantId'] = requestParameters['tenantId'];
    }

    if (requestParameters['offset'] != null) {
      queryParameters['offset'] = requestParameters['offset'];
    }

    if (requestParameters['limit'] != null) {
      queryParameters['limit'] = requestParameters['limit'];
    }

    const headerParameters: runtime.HTTPHeaders = {};

    const response = await this.request(
      {
        path: `/lead-scraper-microservice/api/v1/accounts`,
        method: 'GET',
        headers: headerParameters,
        query: queryParameters,
      },
      initOverrides
    );

    return new runtime.JSONApiResponse(response, jsonValue =>
      ListAccountsResponseFromJSON(jsonValue)
    );
  }

  /**
   * Retrieves a list of accounts with optional filtering
   * List accounts
   */
  async listAccounts(
    requestParameters: ListAccountsRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction
  ): Promise<ListAccountsResponse> {
    const response = await this.listAccountsRaw(
      requestParameters,
      initOverrides
    );
    return await response.value();
  }

  /**
   * This endpoint retrieves all Google Maps scraping jobs
   * Get all jobs
   */
  async listScrapingJobsRaw(
    requestParameters: ListScrapingJobsRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction
  ): Promise<runtime.ApiResponse<ListScrapingJobsResponse>> {
    if (requestParameters['userId'] == null) {
      throw new runtime.RequiredError(
        'userId',
        'Required parameter "userId" was null or undefined when calling listScrapingJobs().'
      );
    }

    if (requestParameters['orgId'] == null) {
      throw new runtime.RequiredError(
        'orgId',
        'Required parameter "orgId" was null or undefined when calling listScrapingJobs().'
      );
    }

    if (requestParameters['tenantId'] == null) {
      throw new runtime.RequiredError(
        'tenantId',
        'Required parameter "tenantId" was null or undefined when calling listScrapingJobs().'
      );
    }

    const queryParameters: any = {};

    if (requestParameters['userId'] != null) {
      queryParameters['userId'] = requestParameters['userId'];
    }

    if (requestParameters['orgId'] != null) {
      queryParameters['orgId'] = requestParameters['orgId'];
    }

    if (requestParameters['tenantId'] != null) {
      queryParameters['tenantId'] = requestParameters['tenantId'];
    }

    const headerParameters: runtime.HTTPHeaders = {};

    const response = await this.request(
      {
        path: `/lead-scraper-microservice/api/v1/jobs`,
        method: 'GET',
        headers: headerParameters,
        query: queryParameters,
      },
      initOverrides
    );

    return new runtime.JSONApiResponse(response, jsonValue =>
      ListScrapingJobsResponseFromJSON(jsonValue)
    );
  }

  /**
   * This endpoint retrieves all Google Maps scraping jobs
   * Get all jobs
   */
  async listScrapingJobs(
    requestParameters: ListScrapingJobsRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction
  ): Promise<ListScrapingJobsResponse> {
    const response = await this.listScrapingJobsRaw(
      requestParameters,
      initOverrides
    );
    return await response.value();
  }

  /**
   * Updates specified fields of an existing account
   * Update account details
   */
  async updateAccountRaw(
    requestParameters: UpdateAccountOperationRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction
  ): Promise<runtime.ApiResponse<UpdateAccountResponse>> {
    if (requestParameters['updateAccountRequest'] == null) {
      throw new runtime.RequiredError(
        'updateAccountRequest',
        'Required parameter "updateAccountRequest" was null or undefined when calling updateAccount().'
      );
    }

    const queryParameters: any = {};

    const headerParameters: runtime.HTTPHeaders = {};

    headerParameters['Content-Type'] = 'application/json';

    const response = await this.request(
      {
        path: `/lead-scraper-microservice/api/v1/accounts`,
        method: 'PATCH',
        headers: headerParameters,
        query: queryParameters,
        body: UpdateAccountRequestToJSON(
          requestParameters['updateAccountRequest']
        ),
      },
      initOverrides
    );

    return new runtime.JSONApiResponse(response, jsonValue =>
      UpdateAccountResponseFromJSON(jsonValue)
    );
  }

  /**
   * Updates specified fields of an existing account
   * Update account details
   */
  async updateAccount(
    requestParameters: UpdateAccountOperationRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction
  ): Promise<UpdateAccountResponse> {
    const response = await this.updateAccountRaw(
      requestParameters,
      initOverrides
    );
    return await response.value();
  }
}
