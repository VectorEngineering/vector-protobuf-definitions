# workspace_service_api

All URIs are relative to *http://lead-scraping-microservice.vector.svc.cluster.local:9896*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CreateAccount1**](workspace_service_api.md#CreateAccount1) | **POST** /workspace-service/v1/accounts | Create a new account
[**CreateWorkspace1**](workspace_service_api.md#CreateWorkspace1) | **POST** /workspace-service/v1/workspaces | Create workspace
[**DeleteAccount1**](workspace_service_api.md#DeleteAccount1) | **DELETE** /workspace-service/v1/accounts/{id} | Delete account
[**DeleteWorkspace1**](workspace_service_api.md#DeleteWorkspace1) | **DELETE** /workspace-service/v1/workspaces/{id} | Delete workspace
[**GetAccount1**](workspace_service_api.md#GetAccount1) | **GET** /workspace-service/v1/accounts/{id} | Get account details
[**GetWorkspace1**](workspace_service_api.md#GetWorkspace1) | **GET** /workspace-service/v1/workspaces/{id} | Get workspace details
[**GetWorkspaceAnalytics1**](workspace_service_api.md#GetWorkspaceAnalytics1) | **GET** /workspace-service/v1/workspaces/analytics/{workspaceId} | Get workspace analytics
[**GetWorkspaceComplianceReport**](workspace_service_api.md#GetWorkspaceComplianceReport) | **GET** /workspace-service/v1/workspaces/compliance-report/{workspaceId} | Get compliance report
[**GetWorkspaceStorageStats**](workspace_service_api.md#GetWorkspaceStorageStats) | **GET** /workspace-service/v1/workspaces/storage-stats/{workspaceId} | Get storage statistics
[**ListWorkspaceSharings**](workspace_service_api.md#ListWorkspaceSharings) | **GET** /workspace-service/v1/workspaces/sharings/{workspaceId} | List workspace sharings
[**ListWorkspaces1**](workspace_service_api.md#ListWorkspaces1) | **GET** /workspace-service/v1/workspaces/list | List workspaces
[**RemoveWorkspaceSharing**](workspace_service_api.md#RemoveWorkspaceSharing) | **DELETE** /workspace-service/v1/workspace-sharings/{sharingId} | Remove workspace sharing
[**ShareWorkspace**](workspace_service_api.md#ShareWorkspace) | **POST** /workspace-service/v1/workspaces/{workspaceId}/share | Share workspace
[**UpdateAccount1**](workspace_service_api.md#UpdateAccount1) | **PUT** /workspace-service/v1/accounts | Update account details
[**UpdateWorkspace1**](workspace_service_api.md#UpdateWorkspace1) | **PUT** /workspace-service/v1/workspaces | Update workspace
[**UpdateWorkspaceSharing**](workspace_service_api.md#UpdateWorkspaceSharing) | **PUT** /workspace-service/v1/workspace-sharings | Update workspace sharing


<a name="CreateAccount1"></a>
# **CreateAccount1**
> CreateAccountResponse1 CreateAccount1(createAccountRequest1)

Create a new account

Creates a new user account with initial workspace
<a name="CreateWorkspace1"></a>
# **CreateWorkspace1**
> CreateWorkspaceResponse1 CreateWorkspace1(createWorkspaceRequest1)

Create workspace
<a name="DeleteAccount1"></a>
# **DeleteAccount1**
> DeleteAccountResponse DeleteAccount1(Id_)

Delete account
<a name="DeleteWorkspace1"></a>
# **DeleteWorkspace1**
> DeleteWorkspaceResponse DeleteWorkspace1(Id_)

Delete workspace
<a name="GetAccount1"></a>
# **GetAccount1**
> GetAccountResponse1 GetAccount1(Id_)

Get account details
<a name="GetWorkspace1"></a>
# **GetWorkspace1**
> GetWorkspaceResponse GetWorkspace1(Id_)

Get workspace details
<a name="GetWorkspaceAnalytics1"></a>
# **GetWorkspaceAnalytics1**
> GetWorkspaceAnalyticsResponse1 GetWorkspaceAnalytics1(workspaceId, startTime, endTime)

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
<a name="ListWorkspaces1"></a>
# **ListWorkspaces1**
> ListWorkspacesResponse1 ListWorkspaces1(accountId, pageSize, pageToken, filter)

List workspaces
<a name="RemoveWorkspaceSharing"></a>
# **RemoveWorkspaceSharing**
> RemoveWorkspaceSharingResponse RemoveWorkspaceSharing(sharingId)

Remove workspace sharing
<a name="ShareWorkspace"></a>
# **ShareWorkspace**
> ShareWorkspaceResponse ShareWorkspace(workspaceId, shareWorkspaceBody)

Share workspace
<a name="UpdateAccount1"></a>
# **UpdateAccount1**
> UpdateAccountResponse UpdateAccount1(updateAccountRequest1)

Update account details
<a name="UpdateWorkspace1"></a>
# **UpdateWorkspace1**
> UpdateWorkspaceResponse UpdateWorkspace1(updateWorkspaceRequest)

Update workspace
<a name="UpdateWorkspaceSharing"></a>
# **UpdateWorkspaceSharing**
> UpdateWorkspaceSharingResponse UpdateWorkspaceSharing(updateWorkspaceSharingRequest)

Update workspace sharing
