# workspace_service_api

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CreateAccount**](workspace_service_api.md#CreateAccount) | **POST** /workspace-service/v1/accounts | Create a new account
[**CreateWorkspace**](workspace_service_api.md#CreateWorkspace) | **POST** /workspace-service/v1/workspaces | Create workspace
[**DeleteAccount**](workspace_service_api.md#DeleteAccount) | **DELETE** /workspace-service/v1/accounts/{id} | Delete account
[**DeleteWorkspace**](workspace_service_api.md#DeleteWorkspace) | **DELETE** /workspace-service/v1/workspaces/{id} | Delete workspace
[**GetAccount**](workspace_service_api.md#GetAccount) | **GET** /workspace-service/v1/accounts/{id} | Get account details
[**GetWorkspace**](workspace_service_api.md#GetWorkspace) | **GET** /workspace-service/v1/workspaces/{id} | Get workspace details
[**GetWorkspaceAnalytics**](workspace_service_api.md#GetWorkspaceAnalytics) | **GET** /workspace-service/v1/workspaces/analytics/{workspaceId} | Get workspace analytics
[**GetWorkspaceComplianceReport**](workspace_service_api.md#GetWorkspaceComplianceReport) | **GET** /workspace-service/v1/workspaces/compliance-report/{workspaceId} | Get compliance report
[**GetWorkspaceStorageStats**](workspace_service_api.md#GetWorkspaceStorageStats) | **GET** /workspace-service/v1/workspaces/storage-stats/{workspaceId} | Get storage statistics
[**ListWorkspaceSharings**](workspace_service_api.md#ListWorkspaceSharings) | **GET** /workspace-service/v1/workspaces/sharings/{workspaceId} | List workspace sharings
[**ListWorkspaces**](workspace_service_api.md#ListWorkspaces) | **GET** /workspace-service/v1/workspaces/list | List workspaces
[**RemoveWorkspaceSharing**](workspace_service_api.md#RemoveWorkspaceSharing) | **DELETE** /workspace-service/v1/workspace-sharings/{sharingId} | Remove workspace sharing
[**ShareWorkspace**](workspace_service_api.md#ShareWorkspace) | **POST** /workspace-service/v1/workspaces/{workspaceId}/share | Share workspace
[**UpdateAccount**](workspace_service_api.md#UpdateAccount) | **PUT** /workspace-service/v1/accounts | Update account details
[**UpdateWorkspace**](workspace_service_api.md#UpdateWorkspace) | **PUT** /workspace-service/v1/workspaces | Update workspace
[**UpdateWorkspaceSharing**](workspace_service_api.md#UpdateWorkspaceSharing) | **PUT** /workspace-service/v1/workspace-sharings | Update workspace sharing


<a name="CreateAccount"></a>
# **CreateAccount**
> CreateAccountResponse CreateAccount(createAccountRequest)

Create a new account

Creates a new user account with initial workspace
<a name="CreateWorkspace"></a>
# **CreateWorkspace**
> CreateWorkspaceResponse CreateWorkspace(createWorkspaceRequest)

Create workspace
<a name="DeleteAccount"></a>
# **DeleteAccount**
> DeleteAccountResponse DeleteAccount(Id_)

Delete account
<a name="DeleteWorkspace"></a>
# **DeleteWorkspace**
> DeleteWorkspaceResponse DeleteWorkspace(Id_)

Delete workspace
<a name="GetAccount"></a>
# **GetAccount**
> GetAccountResponse GetAccount(Id_)

Get account details
<a name="GetWorkspace"></a>
# **GetWorkspace**
> GetWorkspaceResponse GetWorkspace(Id_)

Get workspace details
<a name="GetWorkspaceAnalytics"></a>
# **GetWorkspaceAnalytics**
> GetWorkspaceAnalyticsResponse GetWorkspaceAnalytics(workspaceId, startTime, endTime)

Get workspace analytics
<a name="GetWorkspaceComplianceReport"></a>
# **GetWorkspaceComplianceReport**
> GetWorkspaceComplianceReportResponse GetWorkspaceComplianceReport(workspaceId, complianceType)

Get compliance report
<a name="GetWorkspaceStorageStats"></a>
# **GetWorkspaceStorageStats**
> GetWorkspaceStorageStatsResponse GetWorkspaceStorageStats(workspaceId)

Get storage statistics
<a name="ListWorkspaceSharings"></a>
# **ListWorkspaceSharings**
> ListWorkspaceSharingsResponse ListWorkspaceSharings(workspaceId, pageSize, pageToken)

List workspace sharings
<a name="ListWorkspaces"></a>
# **ListWorkspaces**
> ListWorkspacesResponse ListWorkspaces(accountId, pageSize, pageToken, filter)

List workspaces
<a name="RemoveWorkspaceSharing"></a>
# **RemoveWorkspaceSharing**
> RemoveWorkspaceSharingResponse RemoveWorkspaceSharing(sharingId)

Remove workspace sharing
<a name="ShareWorkspace"></a>
# **ShareWorkspace**
> ShareWorkspaceResponse ShareWorkspace(workspaceId, shareWorkspaceBody)

Share workspace
<a name="UpdateAccount"></a>
# **UpdateAccount**
> UpdateAccountResponse UpdateAccount(updateAccountRequest)

Update account details
<a name="UpdateWorkspace"></a>
# **UpdateWorkspace**
> UpdateWorkspaceResponse UpdateWorkspace(updateWorkspaceRequest)

Update workspace
<a name="UpdateWorkspaceSharing"></a>
# **UpdateWorkspaceSharing**
> UpdateWorkspaceSharingResponse UpdateWorkspaceSharing(updateWorkspaceSharingRequest)

Update workspace sharing
