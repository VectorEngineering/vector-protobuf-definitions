# Documentation for Lead Scraping Service API

<a name="documentation-for-api-endpoints"></a>
## Documentation for API Endpoints

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

| Class | Method | HTTP request | Description |
|------------ | ------------- | ------------- | -------------|
| *LeadScraperServiceApi* | [**createOrganization**](Apis/LeadScraperServiceApi.md#createorganization) | **POST** /lead-scraper-microservice/api/v1/organization | Create a new organization |
*LeadScraperServiceApi* | [**createTenant**](Apis/LeadScraperServiceApi.md#createtenant) | **POST** /lead-scraper-microservice/api/v1/organizations/{organizationId}/tenants | Create a new tenant |
*LeadScraperServiceApi* | [**deleteOrganization**](Apis/LeadScraperServiceApi.md#deleteorganization) | **DELETE** /lead-scraper-microservice/api/v1/organization/{id} | Delete an organization |
*LeadScraperServiceApi* | [**deleteTenant**](Apis/LeadScraperServiceApi.md#deletetenant) | **DELETE** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId}/{tenantId} | Delete a tenant |
*LeadScraperServiceApi* | [**getOrganization**](Apis/LeadScraperServiceApi.md#getorganization) | **GET** /lead-scraper-microservice/api/v1/organization/{id} | Get organization details |
*LeadScraperServiceApi* | [**getTenant**](Apis/LeadScraperServiceApi.md#gettenant) | **GET** /lead-scraper-microservice/api/v1/organizations/tenants/{organizationId}/{tenantId} | Get tenant details |
*LeadScraperServiceApi* | [**listOrganizations**](Apis/LeadScraperServiceApi.md#listorganizations) | **GET** /lead-scraper-microservice/api/v1/organization | List all organizations |
*LeadScraperServiceApi* | [**listTenants**](Apis/LeadScraperServiceApi.md#listtenants) | **GET** /lead-scraper-microservice/api/v1/organization/tenants/{organizationId} | List all tenants |
*LeadScraperServiceApi* | [**updateOrganization**](Apis/LeadScraperServiceApi.md#updateorganization) | **PUT** /lead-scraper-microservice/api/v1/organization | Update organization details |
*LeadScraperServiceApi* | [**updateTenant**](Apis/LeadScraperServiceApi.md#updatetenant) | **PUT** /lead-scraper-microservice/api/v1/organizations/tenants | Update tenant details |
| *WorkspaceServiceApi* | [**createAccount**](Apis/WorkspaceServiceApi.md#createaccount) | **POST** /workspace-service/v1/accounts | Create a new account |
*WorkspaceServiceApi* | [**createWorkspace**](Apis/WorkspaceServiceApi.md#createworkspace) | **POST** /workspace-service/v1/workspaces | Create workspace |
*WorkspaceServiceApi* | [**deleteAccount**](Apis/WorkspaceServiceApi.md#deleteaccount) | **DELETE** /workspace-service/v1/accounts/{id} | Delete account |
*WorkspaceServiceApi* | [**deleteWorkspace**](Apis/WorkspaceServiceApi.md#deleteworkspace) | **DELETE** /workspace-service/v1/workspaces/{id} | Delete workspace |
*WorkspaceServiceApi* | [**getAccount**](Apis/WorkspaceServiceApi.md#getaccount) | **GET** /workspace-service/v1/accounts/{id} | Get account details |
*WorkspaceServiceApi* | [**getWorkspace**](Apis/WorkspaceServiceApi.md#getworkspace) | **GET** /workspace-service/v1/workspaces/{id} | Get workspace details |
*WorkspaceServiceApi* | [**getWorkspaceAnalytics**](Apis/WorkspaceServiceApi.md#getworkspaceanalytics) | **GET** /workspace-service/v1/workspaces/analytics/{workspaceId} | Get workspace analytics |
*WorkspaceServiceApi* | [**getWorkspaceComplianceReport**](Apis/WorkspaceServiceApi.md#getworkspacecompliancereport) | **GET** /workspace-service/v1/workspaces/compliance-report/{workspaceId} | Get compliance report |
*WorkspaceServiceApi* | [**getWorkspaceStorageStats**](Apis/WorkspaceServiceApi.md#getworkspacestoragestats) | **GET** /workspace-service/v1/workspaces/storage-stats/{workspaceId} | Get storage statistics |
*WorkspaceServiceApi* | [**listWorkspaceSharings**](Apis/WorkspaceServiceApi.md#listworkspacesharings) | **GET** /workspace-service/v1/workspaces/sharings/{workspaceId} | List workspace sharings |
*WorkspaceServiceApi* | [**listWorkspaces**](Apis/WorkspaceServiceApi.md#listworkspaces) | **GET** /workspace-service/v1/workspaces/list | List workspaces |
*WorkspaceServiceApi* | [**removeWorkspaceSharing**](Apis/WorkspaceServiceApi.md#removeworkspacesharing) | **DELETE** /workspace-service/v1/workspace-sharings/{sharingId} | Remove workspace sharing |
*WorkspaceServiceApi* | [**shareWorkspace**](Apis/WorkspaceServiceApi.md#shareworkspace) | **POST** /workspace-service/v1/workspaces/{workspaceId}/share | Share workspace |
*WorkspaceServiceApi* | [**updateAccount**](Apis/WorkspaceServiceApi.md#updateaccount) | **PUT** /workspace-service/v1/accounts | Update account details |
*WorkspaceServiceApi* | [**updateWorkspace**](Apis/WorkspaceServiceApi.md#updateworkspace) | **PUT** /workspace-service/v1/workspaces | Update workspace |
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
 - [CreateAccountRequest](./Models/CreateAccountRequest.md)
 - [CreateAccountResponse](./Models/CreateAccountResponse.md)
 - [CreateOrganizationRequest](./Models/CreateOrganizationRequest.md)
 - [CreateOrganizationResponse](./Models/CreateOrganizationResponse.md)
 - [CreateTenantBody](./Models/CreateTenantBody.md)
 - [CreateTenantResponse](./Models/CreateTenantResponse.md)
 - [CreateWorkspaceRequest](./Models/CreateWorkspaceRequest.md)
 - [CreateWorkspaceResponse](./Models/CreateWorkspaceResponse.md)
 - [DataProfile](./Models/DataProfile.md)
 - [DayOfWeek](./Models/DayOfWeek.md)
 - [DeleteAccountResponse](./Models/DeleteAccountResponse.md)
 - [DeleteOrganizationResponse](./Models/DeleteOrganizationResponse.md)
 - [DeleteTenantResponse](./Models/DeleteTenantResponse.md)
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
 - [GetAccountResponse](./Models/GetAccountResponse.md)
 - [GetOrganizationResponse](./Models/GetOrganizationResponse.md)
 - [GetTenantResponse](./Models/GetTenantResponse.md)
 - [GetWorkspaceAnalyticsResponse](./Models/GetWorkspaceAnalyticsResponse.md)
 - [GetWorkspaceComplianceReportResponse](./Models/GetWorkspaceComplianceReportResponse.md)
 - [GetWorkspaceResponse](./Models/GetWorkspaceResponse.md)
 - [GetWorkspaceStorageStatsResponse](./Models/GetWorkspaceStorageStatsResponse.md)
 - [GoneErrorMessageResponse](./Models/GoneErrorMessageResponse.md)
 - [IncludedField](./Models/IncludedField.md)
 - [InternalErrorCode](./Models/InternalErrorCode.md)
 - [InternalErrorMessageResponse](./Models/InternalErrorMessageResponse.md)
 - [Interval](./Models/Interval.md)
 - [Language](./Models/Language.md)
 - [Lead](./Models/Lead.md)
 - [LimitInfo](./Models/LimitInfo.md)
 - [ListAccountsResponse](./Models/ListAccountsResponse.md)
 - [ListOrganizationsResponse](./Models/ListOrganizationsResponse.md)
 - [ListTenantsResponse](./Models/ListTenantsResponse.md)
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
 - [UnprocessableEntityErrorMessageResponse](./Models/UnprocessableEntityErrorMessageResponse.md)
 - [UpdateAccountRequest](./Models/UpdateAccountRequest.md)
 - [UpdateAccountResponse](./Models/UpdateAccountResponse.md)
 - [UpdateOrganizationRequest](./Models/UpdateOrganizationRequest.md)
 - [UpdateOrganizationResponse](./Models/UpdateOrganizationResponse.md)
 - [UpdateTenantRequest](./Models/UpdateTenantRequest.md)
 - [UpdateTenantResponse](./Models/UpdateTenantResponse.md)
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

