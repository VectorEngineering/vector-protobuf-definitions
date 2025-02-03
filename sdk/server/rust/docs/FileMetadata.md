# FileMetadata

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [optional] [default to None]
**name** | **String** |  | [optional] [default to None]
**size** | **String** |  | [optional] [default to None]
**s3_key** | **String** |  | [optional] [default to None]
**s3_bucket_name** | **String** |  | [optional] [default to None]
**is_deleted** | **bool** |  | [optional] [default to None]
**version** | **i32** |  | [optional] [default to None]
**created_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**updated_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**deleted_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) |  | [optional] [default to None]
**embeddings** | [***models::FileEmbeddings**](FileEmbeddings.md) |  | [optional] [default to None]
**versions** | [**Vec<models::FileVersion>**](FileVersion.md) |  | [optional] [default to None]
**comments** | [**Vec<models::CommentThread>**](CommentThread.md) |  | [optional] [default to None]
**sharing** | [**Vec<models::FileSharing>**](FileSharing.md) |  | [optional] [default to None]
**snapshots** | [**Vec<models::DocumentSnapshot>**](DocumentSnapshot.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


