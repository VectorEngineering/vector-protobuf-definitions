# Workspace1

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [optional] [default to None]
**name** | **String** |  | [optional] [default to None]
**unique_identifier** | **String** |  | [optional] [default to None]
**s3_bucket_name** | **String** |  | [optional] [default to None]
**s3_folder_path** | **String** |  | [optional] [default to None]
**storage_quota** | **String** |  | [optional] [default to None]
**used_storage** | **String** |  | [optional] [default to None]
**allow_public_sharing** | **bool** |  | [optional] [default to None]
**require_approval** | **bool** |  | [optional] [default to None]
**gdpr_compliant** | **bool** |  | [optional] [default to None]
**hipaa_compliant** | **bool** |  | [optional] [default to None]
**created_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**updated_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**deleted_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**folders** | [**Vec<models::FolderMetadata>**](FolderMetadata.md) |  | [optional] [default to None]
**templates** | [**Vec<models::DocumentTemplate>**](DocumentTemplate.md) |  | [optional] [default to None]
**sharing** | [**Vec<models::WorkspaceSharing>**](WorkspaceSharing.md) |  | [optional] [default to None]
**activities** | [**Vec<models::WorkspaceActivity>**](WorkspaceActivity.md) |  | [optional] [default to None]
**compliance** | [**Vec<models::WorkspaceCompliance>**](WorkspaceCompliance.md) |  | [optional] [default to None]
**installed_apps** | [**Vec<models::MarketplaceApp>**](MarketplaceApp.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


