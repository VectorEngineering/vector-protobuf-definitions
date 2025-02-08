# Documentation for Lead Scraping Service API

<a name="documentation-for-api-endpoints"></a>
## Documentation for API Endpoints

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

| Class | Method | HTTP request | Description |
|------------ | ------------- | ------------- | -------------|
| *LeadScraperServiceApi* | [**createAPIKey**](Apis/LeadScraperServiceApi.md#createapikey) | **POST** /lead-scraper-microservice/api/v1/api-keys | Create a new API key |
*LeadScraperServiceApi* | [**createOrganization**](Apis/LeadScraperServiceApi.md#createorganization) | **POST** /lead-scraper-microservice/api/v1/organization | Create a new organization |
*LeadScraperServiceApi* | [**createTenant**](Apis/LeadScraperServiceApi.md#createtenant) | **POST** /lead-scraper-microservice/api/v1/organizations/{organizationId}/tenants | Create a new tenant |
*LeadScraperServiceApi* | [**createTenantAPIKey**](Apis/LeadScraperServiceApi.md#createtenantapikey) | **POST** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys | Create a new tenant API key |
*LeadScraperServiceApi* | [**createWebhook**](Apis/LeadScraperServiceApi.md#createwebhook) | **POST** /lead-scraper-microservice/api/v1/webhooks | Create webhook |
*LeadScraperServiceApi* | [**createWorkflow**](Apis/LeadScraperServiceApi.md#createworkflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows | Create a new workflow |
*LeadScraperServiceApi* | [**deleteAPIKey**](Apis/LeadScraperServiceApi.md#deleteapikey) | **DELETE** /lead-scraper-microservice/api/v1/api-keys/{keyId} | Delete API key |
*LeadScraperServiceApi* | [**deleteOrganization**](Apis/LeadScraperServiceApi.md#deleteorganization) | **DELETE** /lead-scraper-microservice/api/v1/organization/{id} | Delete an organization |
*LeadScraperServiceApi* | [**deleteTenant**](Apis/LeadScraperServiceApi.md#deletetenant) | **DELETE** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId}/{tenantId} | Delete a tenant |
*LeadScraperServiceApi* | [**deleteTenantAPIKey**](Apis/LeadScraperServiceApi.md#deletetenantapikey) | **DELETE** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/{keyId} | Delete tenant API key |
*LeadScraperServiceApi* | [**deleteWebhook**](Apis/LeadScraperServiceApi.md#deletewebhook) | **DELETE** /lead-scraper-microservice/api/v1/webhooks/{webhookId} | Delete webhook |
*LeadScraperServiceApi* | [**deleteWorkflow**](Apis/LeadScraperServiceApi.md#deleteworkflow) | **DELETE** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id} | Delete workflow |
*LeadScraperServiceApi* | [**deleteWorkspace**](Apis/LeadScraperServiceApi.md#deleteworkspace) | **DELETE** /lead-scraper-microservice/api/v1/workspace/{id} | Delete a workspace |
*LeadScraperServiceApi* | [**getAPIKey**](Apis/LeadScraperServiceApi.md#getapikey) | **GET** /lead-scraper-microservice/api/v1/api-keys/{keyId} | Get API key details |
*LeadScraperServiceApi* | [**getLead**](Apis/LeadScraperServiceApi.md#getlead) | **GET** /lead-scraper-microservice/api/v1/leads/{leadId} | Get lead details |
*LeadScraperServiceApi* | [**getOrganization**](Apis/LeadScraperServiceApi.md#getorganization) | **GET** /lead-scraper-microservice/api/v1/organization/{id} | Get organization details |
*LeadScraperServiceApi* | [**getTenant**](Apis/LeadScraperServiceApi.md#gettenant) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/{organizationId}/{tenantId} | Get tenant details |
*LeadScraperServiceApi* | [**getTenantAPIKey**](Apis/LeadScraperServiceApi.md#gettenantapikey) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/{keyId} | Get tenant API key details |
*LeadScraperServiceApi* | [**getWebhook**](Apis/LeadScraperServiceApi.md#getwebhook) | **GET** /lead-scraper-microservice/api/v1/webhooks/{webhookId} | Get webhook |
*LeadScraperServiceApi* | [**getWorkflow**](Apis/LeadScraperServiceApi.md#getworkflow) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id} | Get workflow details |
*LeadScraperServiceApi* | [**getWorkspace**](Apis/LeadScraperServiceApi.md#getworkspace) | **GET** /lead-scraper-microservice/api/v1/workspace/{id} | Get workspace details |
*LeadScraperServiceApi* | [**getWorkspaceAnalytics**](Apis/LeadScraperServiceApi.md#getworkspaceanalytics) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/analytics | Get workspace analytics |
*LeadScraperServiceApi* | [**listAPIKeys**](Apis/LeadScraperServiceApi.md#listapikeys) | **GET** /lead-scraper-microservice/api/v1/api-keys/list | List API keys |
*LeadScraperServiceApi* | [**listLeads**](Apis/LeadScraperServiceApi.md#listleads) | **GET** /lead-scraper-microservice/api/v1/leads | List leads |
*LeadScraperServiceApi* | [**listOrganizations**](Apis/LeadScraperServiceApi.md#listorganizations) | **GET** /lead-scraper-microservice/api/v1/organization | List all organizations |
*LeadScraperServiceApi* | [**listTenantAPIKeys**](Apis/LeadScraperServiceApi.md#listtenantapikeys) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/list | List tenant API keys |
*LeadScraperServiceApi* | [**listTenants**](Apis/LeadScraperServiceApi.md#listtenants) | **GET** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId} | List all tenants |
*LeadScraperServiceApi* | [**listWebhooks**](Apis/LeadScraperServiceApi.md#listwebhooks) | **GET** /lead-scraper-microservice/api/v1/webhooks | List webhooks |
*LeadScraperServiceApi* | [**listWorkflows**](Apis/LeadScraperServiceApi.md#listworkflows) | **GET** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows | List workflows |
*LeadScraperServiceApi* | [**pauseWorkflow**](Apis/LeadScraperServiceApi.md#pauseworkflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id}/pause | Pause workflow execution |
*LeadScraperServiceApi* | [**rotateAPIKey**](Apis/LeadScraperServiceApi.md#rotateapikey) | **POST** /lead-scraper-microservice/api/v1/api-keys/rotate | Rotate API key |
*LeadScraperServiceApi* | [**rotateTenantAPIKey**](Apis/LeadScraperServiceApi.md#rotatetenantapikey) | **POST** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/rotate | Rotate tenant API key |
*LeadScraperServiceApi* | [**triggerWorkflow**](Apis/LeadScraperServiceApi.md#triggerworkflow) | **POST** /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id}/trigger | Trigger workflow execution |
*LeadScraperServiceApi* | [**updateAPIKey**](Apis/LeadScraperServiceApi.md#updateapikey) | **PUT** /lead-scraper-microservice/api/v1/api-keys | Update API key |
*LeadScraperServiceApi* | [**updateOrganization**](Apis/LeadScraperServiceApi.md#updateorganization) | **PUT** /lead-scraper-microservice/api/v1/organization | Update organization details |
*LeadScraperServiceApi* | [**updateTenant**](Apis/LeadScraperServiceApi.md#updatetenant) | **PUT** /lead-scraper-microservice/api/v1/organizations/tenants | Update tenant details |
*LeadScraperServiceApi* | [**updateTenantAPIKey**](Apis/LeadScraperServiceApi.md#updatetenantapikey) | **PUT** /lead-scraper-microservice/api/v1/organizations/tenants/api-keys | Update tenant API key |
*LeadScraperServiceApi* | [**updateWebhook**](Apis/LeadScraperServiceApi.md#updatewebhook) | **PUT** /lead-scraper-microservice/api/v1/webhooks | Update webhook |
*LeadScraperServiceApi* | [**updateWorkflow**](Apis/LeadScraperServiceApi.md#updateworkflow) | **PUT** /lead-scraper-microservice/api/v1/workspaces/workflow | Update workflow details |
*LeadScraperServiceApi* | [**updateWorkspace**](Apis/LeadScraperServiceApi.md#updateworkspace) | **PUT** /lead-scraper-microservice/api/v1/workspace | Update workspace details |
| *WorkspaceServiceApi* | [**createAccount**](Apis/WorkspaceServiceApi.md#createaccount) | **POST** /workspace-service/v1/accounts | Create a new account |
*WorkspaceServiceApi* | [**createWorkspace**](Apis/WorkspaceServiceApi.md#createworkspace) | **POST** /workspace-service/v1/workspaces | Create workspace |
*WorkspaceServiceApi* | [**deleteAccount**](Apis/WorkspaceServiceApi.md#deleteaccount) | **DELETE** /workspace-service/v1/accounts/{id} | Delete account |
*WorkspaceServiceApi* | [**deleteWorkspace1**](Apis/WorkspaceServiceApi.md#deleteworkspace1) | **DELETE** /workspace-service/v1/workspaces/{id} | Delete workspace |
*WorkspaceServiceApi* | [**getAccount**](Apis/WorkspaceServiceApi.md#getaccount) | **GET** /workspace-service/v1/accounts/{id} | Get account details |
*WorkspaceServiceApi* | [**getWorkspace1**](Apis/WorkspaceServiceApi.md#getworkspace1) | **GET** /workspace-service/v1/workspaces/{id} | Get workspace details |
*WorkspaceServiceApi* | [**getWorkspaceAnalytics1**](Apis/WorkspaceServiceApi.md#getworkspaceanalytics1) | **GET** /workspace-service/v1/workspaces/analytics/{workspaceId} | Get workspace analytics |
*WorkspaceServiceApi* | [**getWorkspaceComplianceReport**](Apis/WorkspaceServiceApi.md#getworkspacecompliancereport) | **GET** /workspace-service/v1/workspaces/compliance-report/{workspaceId} | Get compliance report |
*WorkspaceServiceApi* | [**getWorkspaceStorageStats**](Apis/WorkspaceServiceApi.md#getworkspacestoragestats) | **GET** /workspace-service/v1/workspaces/storage-stats/{workspaceId} | Get storage statistics |
*WorkspaceServiceApi* | [**listWorkspaceSharings**](Apis/WorkspaceServiceApi.md#listworkspacesharings) | **GET** /workspace-service/v1/workspaces/sharings/{workspaceId} | List workspace sharings |
*WorkspaceServiceApi* | [**listWorkspaces**](Apis/WorkspaceServiceApi.md#listworkspaces) | **GET** /workspace-service/v1/workspaces/list | List workspaces |
*WorkspaceServiceApi* | [**removeWorkspaceSharing**](Apis/WorkspaceServiceApi.md#removeworkspacesharing) | **DELETE** /workspace-service/v1/workspace-sharings/{sharingId} | Remove workspace sharing |
*WorkspaceServiceApi* | [**shareWorkspace**](Apis/WorkspaceServiceApi.md#shareworkspace) | **POST** /workspace-service/v1/workspaces/{workspaceId}/share | Share workspace |
*WorkspaceServiceApi* | [**updateAccount**](Apis/WorkspaceServiceApi.md#updateaccount) | **PUT** /workspace-service/v1/accounts | Update account details |
*WorkspaceServiceApi* | [**updateWorkspace1**](Apis/WorkspaceServiceApi.md#updateworkspace1) | **PUT** /workspace-service/v1/workspaces | Update workspace |
*WorkspaceServiceApi* | [**updateWorkspaceSharing**](Apis/WorkspaceServiceApi.md#updateworkspacesharing) | **PUT** /workspace-service/v1/workspace-sharings | Update workspace sharing |


<a name="documentation-for-models"></a>
## Documentation for Models

 - [AIAssistanceLog](./Models/AIAssistanceLog.md)
 - [APIInfo](./Models/APIInfo.md)
 - [APIKey](./Models/APIKey.md)
 - [Account](./Models/Account.md)
 - [Account1](./Models/Account1.md)
 - [AccountSettings](./Models/AccountSettings.md)
 - [AccountStatus](./Models/AccountStatus.md)
 - [ActivityMetrics](./Models/ActivityMetrics.md)
 - [Any](./Models/Any.md)
 - [AppAnalytics](./Models/AppAnalytics.md)
 - [AppCategory](./Models/AppCategory.md)
 - [AppDevelopmentInfo](./Models/AppDevelopmentInfo.md)
 - [AppInstallation](./Models/AppInstallation.md)
 - [AppPermission](./Models/AppPermission.md)
 - [AppReview](./Models/AppReview.md)
 - [AppVersion](./Models/AppVersion.md)
 - [AppWebhook](./Models/AppWebhook.md)
 - [AuthContext](./Models/AuthContext.md)
 - [AuthErrorCode](./Models/AuthErrorCode.md)
 - [AuthenticationErrorMessageResponse](./Models/AuthenticationErrorMessageResponse.md)
 - [AuthenticationErrorMessageResponse1](./Models/AuthenticationErrorMessageResponse1.md)
 - [AvailabilityInfo](./Models/AvailabilityInfo.md)
 - [BackgroundJobStatus](./Models/BackgroundJobStatus.md)
 - [BadGatewayErrorMessageResponse](./Models/BadGatewayErrorMessageResponse.md)
 - [BillingMode](./Models/BillingMode.md)
 - [BillingPlan](./Models/BillingPlan.md)
 - [BranchMerge](./Models/BranchMerge.md)
 - [BranchPolicy](./Models/BranchPolicy.md)
 - [BusinessHours](./Models/BusinessHours.md)
 - [ChangeSet](./Models/ChangeSet.md)
 - [CommentThread](./Models/CommentThread.md)
 - [ComplianceCheck](./Models/ComplianceCheck.md)
 - [ComplianceLevel](./Models/ComplianceLevel.md)
 - [ComplianceMetrics](./Models/ComplianceMetrics.md)
 - [ComplianceScore](./Models/ComplianceScore.md)
 - [ComplianceViolation](./Models/ComplianceViolation.md)
 - [ConflictErrorMessageResponse](./Models/ConflictErrorMessageResponse.md)
 - [ConflictInfo](./Models/ConflictInfo.md)
 - [ContextualSummary](./Models/ContextualSummary.md)
 - [ContractIntelligence](./Models/ContractIntelligence.md)
 - [CreateAPIKeyRequest](./Models/CreateAPIKeyRequest.md)
 - [CreateAPIKeyResponse](./Models/CreateAPIKeyResponse.md)
 - [CreateAccountRequest](./Models/CreateAccountRequest.md)
 - [CreateAccountResponse](./Models/CreateAccountResponse.md)
 - [CreateOrganizationRequest](./Models/CreateOrganizationRequest.md)
 - [CreateOrganizationResponse](./Models/CreateOrganizationResponse.md)
 - [CreateTenantAPIKeyRequest](./Models/CreateTenantAPIKeyRequest.md)
 - [CreateTenantAPIKeyResponse](./Models/CreateTenantAPIKeyResponse.md)
 - [CreateTenantBody](./Models/CreateTenantBody.md)
 - [CreateTenantResponse](./Models/CreateTenantResponse.md)
 - [CreateWebhookRequest](./Models/CreateWebhookRequest.md)
 - [CreateWebhookResponse](./Models/CreateWebhookResponse.md)
 - [CreateWorkflowBody](./Models/CreateWorkflowBody.md)
 - [CreateWorkflowResponse](./Models/CreateWorkflowResponse.md)
 - [CreateWorkspaceRequest](./Models/CreateWorkspaceRequest.md)
 - [CreateWorkspaceResponse](./Models/CreateWorkspaceResponse.md)
 - [DataProfile](./Models/DataProfile.md)
 - [DayOfWeek](./Models/DayOfWeek.md)
 - [DeleteAPIKeyResponse](./Models/DeleteAPIKeyResponse.md)
 - [DeleteAccountResponse](./Models/DeleteAccountResponse.md)
 - [DeleteOrganizationResponse](./Models/DeleteOrganizationResponse.md)
 - [DeleteTenantAPIKeyResponse](./Models/DeleteTenantAPIKeyResponse.md)
 - [DeleteTenantResponse](./Models/DeleteTenantResponse.md)
 - [DeleteWebhookResponse](./Models/DeleteWebhookResponse.md)
 - [DeleteWorkflowResponse](./Models/DeleteWorkflowResponse.md)
 - [DeleteWorkspaceResponse](./Models/DeleteWorkspaceResponse.md)
 - [Dependency](./Models/Dependency.md)
 - [DocumentBranch](./Models/DocumentBranch.md)
 - [DocumentInstance](./Models/DocumentInstance.md)
 - [DocumentSnapshot](./Models/DocumentSnapshot.md)
 - [DocumentStatus](./Models/DocumentStatus.md)
 - [DocumentTemplate](./Models/DocumentTemplate.md)
 - [DocumentVersion](./Models/DocumentVersion.md)
 - [EmployeeBenefit](./Models/EmployeeBenefit.md)
 - [ErrorResponse](./Models/ErrorResponse.md)
 - [ExplanationBlock](./Models/ExplanationBlock.md)
 - [FieldError](./Models/FieldError.md)
 - [FieldViolation](./Models/FieldViolation.md)
 - [FileEmbeddings](./Models/FileEmbeddings.md)
 - [FileMetadata](./Models/FileMetadata.md)
 - [FileSharing](./Models/FileSharing.md)
 - [FileVersion](./Models/FileVersion.md)
 - [FolderMetadata](./Models/FolderMetadata.md)
 - [ForbiddenErrorMessageResponse](./Models/ForbiddenErrorMessageResponse.md)
 - [GatewayTimeoutErrorMessageResponse](./Models/GatewayTimeoutErrorMessageResponse.md)
 - [GetAPIKeyResponse](./Models/GetAPIKeyResponse.md)
 - [GetAccountResponse](./Models/GetAccountResponse.md)
 - [GetLeadResponse](./Models/GetLeadResponse.md)
 - [GetOrganizationResponse](./Models/GetOrganizationResponse.md)
 - [GetTenantAPIKeyResponse](./Models/GetTenantAPIKeyResponse.md)
 - [GetTenantResponse](./Models/GetTenantResponse.md)
 - [GetWebhookResponse](./Models/GetWebhookResponse.md)
 - [GetWorkflowResponse](./Models/GetWorkflowResponse.md)
 - [GetWorkspaceAnalyticsResponse](./Models/GetWorkspaceAnalyticsResponse.md)
 - [GetWorkspaceAnalyticsResponse1](./Models/GetWorkspaceAnalyticsResponse1.md)
 - [GetWorkspaceComplianceReportResponse](./Models/GetWorkspaceComplianceReportResponse.md)
 - [GetWorkspaceResponse](./Models/GetWorkspaceResponse.md)
 - [GetWorkspaceResponse1](./Models/GetWorkspaceResponse1.md)
 - [GetWorkspaceStorageStatsResponse](./Models/GetWorkspaceStorageStatsResponse.md)
 - [GoneErrorMessageResponse](./Models/GoneErrorMessageResponse.md)
 - [IncludedField](./Models/IncludedField.md)
 - [InternalErrorCode](./Models/InternalErrorCode.md)
 - [InternalErrorMessageResponse](./Models/InternalErrorMessageResponse.md)
 - [Interval](./Models/Interval.md)
 - [JobSuccessRate](./Models/JobSuccessRate.md)
 - [Language](./Models/Language.md)
 - [Lead](./Models/Lead.md)
 - [LimitInfo](./Models/LimitInfo.md)
 - [ListAPIKeysResponse](./Models/ListAPIKeysResponse.md)
 - [ListAccountsResponse](./Models/ListAccountsResponse.md)
 - [ListLeadsResponse](./Models/ListLeadsResponse.md)
 - [ListOrganizationsResponse](./Models/ListOrganizationsResponse.md)
 - [ListTenantAPIKeysResponse](./Models/ListTenantAPIKeysResponse.md)
 - [ListTenantsResponse](./Models/ListTenantsResponse.md)
 - [ListWebhooksResponse](./Models/ListWebhooksResponse.md)
 - [ListWorkflowsResponse](./Models/ListWorkflowsResponse.md)
 - [ListWorkspaceSharingsResponse](./Models/ListWorkspaceSharingsResponse.md)
 - [ListWorkspacesResponse](./Models/ListWorkspacesResponse.md)
 - [MFAInfo](./Models/MFAInfo.md)
 - [MarketplaceApp](./Models/MarketplaceApp.md)
 - [MergeRequest](./Models/MergeRequest.md)
 - [MethodNotAllowedErrorMessageResponse](./Models/MethodNotAllowedErrorMessageResponse.md)
 - [NegotiationHistory](./Models/NegotiationHistory.md)
 - [NegotiationRound](./Models/NegotiationRound.md)
 - [NotFoundErrorCode](./Models/NotFoundErrorCode.md)
 - [NotFoundErrorMessageResponse](./Models/NotFoundErrorMessageResponse.md)
 - [NotImplementedErrorMessageResponse](./Models/NotImplementedErrorMessageResponse.md)
 - [NullValue](./Models/NullValue.md)
 - [OperationDetails](./Models/OperationDetails.md)
 - [Organization](./Models/Organization.md)
 - [OutputFormat](./Models/OutputFormat.md)
 - [PauseWorkflowBody](./Models/PauseWorkflowBody.md)
 - [PauseWorkflowResponse](./Models/PauseWorkflowResponse.md)
 - [PayloadFormat](./Models/PayloadFormat.md)
 - [PaymentInfo](./Models/PaymentInfo.md)
 - [PaymentRequiredErrorMessageResponse](./Models/PaymentRequiredErrorMessageResponse.md)
 - [PaymentStatus](./Models/PaymentStatus.md)
 - [Permission](./Models/Permission.md)
 - [PlanTier](./Models/PlanTier.md)
 - [PreconditionFailedErrorMessageResponse](./Models/PreconditionFailedErrorMessageResponse.md)
 - [PricingModel](./Models/PricingModel.md)
 - [QuotaInfo](./Models/QuotaInfo.md)
 - [RateLimitContext](./Models/RateLimitContext.md)
 - [RateLimitErrorMessageResponse](./Models/RateLimitErrorMessageResponse.md)
 - [RemoveWorkspaceSharingResponse](./Models/RemoveWorkspaceSharingResponse.md)
 - [ResourceInfo](./Models/ResourceInfo.md)
 - [ResourceUtilization](./Models/ResourceUtilization.md)
 - [ResourceValidation](./Models/ResourceValidation.md)
 - [RevenueRange](./Models/RevenueRange.md)
 - [Review](./Models/Review.md)
 - [RiskAssessment](./Models/RiskAssessment.md)
 - [Role](./Models/Role.md)
 - [RotateAPIKeyRequest](./Models/RotateAPIKeyRequest.md)
 - [RotateAPIKeyResponse](./Models/RotateAPIKeyResponse.md)
 - [RotateTenantAPIKeyRequest](./Models/RotateTenantAPIKeyRequest.md)
 - [RotateTenantAPIKeyResponse](./Models/RotateTenantAPIKeyResponse.md)
 - [SchemaValidation](./Models/SchemaValidation.md)
 - [ScrapingJob](./Models/ScrapingJob.md)
 - [ScrapingWorkflow](./Models/ScrapingWorkflow.md)
 - [ServiceStatus](./Models/ServiceStatus.md)
 - [ServiceUnavailableErrorMessageResponse](./Models/ServiceUnavailableErrorMessageResponse.md)
 - [SessionInfo](./Models/SessionInfo.md)
 - [ShareWorkspaceBody](./Models/ShareWorkspaceBody.md)
 - [ShareWorkspaceResponse](./Models/ShareWorkspaceResponse.md)
 - [SignatureBlock](./Models/SignatureBlock.md)
 - [SignatureRequest](./Models/SignatureRequest.md)
 - [SignatureStatus](./Models/SignatureStatus.md)
 - [SignatureWorkflow](./Models/SignatureWorkflow.md)
 - [Status](./Models/Status.md)
 - [StorageBreakdown](./Models/StorageBreakdown.md)
 - [Subscription](./Models/Subscription.md)
 - [Suggestions](./Models/Suggestions.md)
 - [TemplateType](./Models/TemplateType.md)
 - [TemplateVariable](./Models/TemplateVariable.md)
 - [TemplateVersion](./Models/TemplateVersion.md)
 - [Tenant](./Models/Tenant.md)
 - [TenantAPIKey](./Models/TenantAPIKey.md)
 - [TenantAPIKeyScope](./Models/TenantAPIKeyScope.md)
 - [Timezone](./Models/Timezone.md)
 - [TokenInfo](./Models/TokenInfo.md)
 - [TooEarlyErrorMessageResponse](./Models/TooEarlyErrorMessageResponse.md)
 - [TriggerEvent](./Models/TriggerEvent.md)
 - [TriggerWorkflowBody](./Models/TriggerWorkflowBody.md)
 - [TriggerWorkflowResponse](./Models/TriggerWorkflowResponse.md)
 - [UnprocessableEntityErrorMessageResponse](./Models/UnprocessableEntityErrorMessageResponse.md)
 - [UpdateAPIKeyRequest](./Models/UpdateAPIKeyRequest.md)
 - [UpdateAPIKeyResponse](./Models/UpdateAPIKeyResponse.md)
 - [UpdateAccountRequest](./Models/UpdateAccountRequest.md)
 - [UpdateAccountResponse](./Models/UpdateAccountResponse.md)
 - [UpdateOrganizationRequest](./Models/UpdateOrganizationRequest.md)
 - [UpdateOrganizationResponse](./Models/UpdateOrganizationResponse.md)
 - [UpdateTenantAPIKeyRequest](./Models/UpdateTenantAPIKeyRequest.md)
 - [UpdateTenantAPIKeyResponse](./Models/UpdateTenantAPIKeyResponse.md)
 - [UpdateTenantRequest](./Models/UpdateTenantRequest.md)
 - [UpdateTenantResponse](./Models/UpdateTenantResponse.md)
 - [UpdateWebhookRequest](./Models/UpdateWebhookRequest.md)
 - [UpdateWebhookResponse](./Models/UpdateWebhookResponse.md)
 - [UpdateWorkflowRequest](./Models/UpdateWorkflowRequest.md)
 - [UpdateWorkflowResponse](./Models/UpdateWorkflowResponse.md)
 - [UpdateWorkspaceRequest](./Models/UpdateWorkspaceRequest.md)
 - [UpdateWorkspaceResponse](./Models/UpdateWorkspaceResponse.md)
 - [UpdateWorkspaceSharingRequest](./Models/UpdateWorkspaceSharingRequest.md)
 - [UpdateWorkspaceSharingResponse](./Models/UpdateWorkspaceSharingResponse.md)
 - [UserActivity](./Models/UserActivity.md)
 - [ValidationErrorCode](./Models/ValidationErrorCode.md)
 - [ValidationErrorMessageResponse](./Models/ValidationErrorMessageResponse.md)
 - [WebhookConfig](./Models/WebhookConfig.md)
 - [WorkflowStatus](./Models/WorkflowStatus.md)
 - [Workspace](./Models/Workspace.md)
 - [Workspace1](./Models/Workspace1.md)
 - [WorkspaceActivity](./Models/WorkspaceActivity.md)
 - [WorkspaceCompliance](./Models/WorkspaceCompliance.md)
 - [WorkspaceSharing](./Models/WorkspaceSharing.md)
 - [rpc.Status](./Models/rpc.Status.md)
 - [v1.Status](./Models/v1.Status.md)


<a name="documentation-for-authorization"></a>
## Documentation for Authorization

<a name="ApiKeyAuth"></a>
### ApiKeyAuth

- **Type**: API key
- **API key parameter name**: X-API-Key
- **Location**: HTTP header

