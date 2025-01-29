# Workspace Service Documentation

## Overview

This service provides a hierarchical workspace service for the platform. It supports a variety of workspace features, including workspaces, folders, files, comments, and more. It also supports a variety of compliance features, including GDPR and HIPAA compliance.

## Core Entities

```mermaid
classDiagram
    %% Core Entities
    Account "1" --> "*" Workspace : has_many
    Workspace "1" --> "*" FolderMetadata : has_many
    Workspace "1" --> "*" WorkspaceSharing : has_many
    Workspace "1" --> "*" WorkspaceActivity : has_many
    Workspace "1" --> "*" WorkspaceCompliance : has_many

    %% Folder and File Structure
    FolderMetadata "1" --> "*" FolderMetadata : has_many child_folders
    FolderMetadata "1" --> "*" FileMetadata : has_many

    %% File Related Entities
    FileMetadata "1" --> "1" FileEmbeddings : has_one
    FileMetadata "1" --> "*" FileVersions : has_many
    FileMetadata "1" --> "*" CommentThread : has_many
    FileMetadata "1" --> "*" FileSharing : has_many
    FileMetadata "1" --> "*" DocumentSnapshot : has_many

    %% Comment System
    CommentThread "1" --> "1" Comment : has_one root_comment
    CommentThread "1" --> "*" Comment : has_many replies
    Comment "1" --> "*" CommentEdit : has_many

    class Account {
        +uint64 id
        +string auth0_user_id
        +string base_directory
        +string bucket_name
        +string region
        +uint64 org_id
        +uint64 tenant_id
        +string email
    }

    class Workspace {
        +uint64 id
        +string name
        +string unique_identifier
        +string s3_bucket_name
        +string s3_folder_path
        +int64 storage_quota
        +int64 used_storage
        +bool allow_public_sharing
        +bool require_approval
    }

    class FolderMetadata {
        +uint64 id
        +string name
        +string s3_bucket_name
        +string s3_folder_path
        +bool is_deleted
    }

    class FileMetadata {
        +uint64 id
        +string name
        +int64 size
        +string s3_key
        +string s3_bucket_name
        +bool is_deleted
        +int32 version
    }

    class CommentThread {
        +uint64 id
        +uint64 file_id
        +string status
        +int32 from
        +int32 to
    }

    class Comment {
        +uint64 id
        +uint64 thread_id
        +string content
        +uint64 parent_comment_id
        +string status
    }

    class FileVersions {
        +uint64 id
        +uint64 file_id
        +int32 version_number
        +string s3_version_id
        +int64 size
    }

    class WorkspaceSharing {
        +uint64 id
        +uint64 shared_with_tenant_id
        +string permission_level
        +bool is_active
    }

    class WorkspaceActivity {
        +uint64 id
        +uint64 tenant_id
        +string action_type
        +string ip_address
    }

    class WorkspaceCompliance {
        +uint64 id
        +uint64 tenant_id
        +string compliance_level
        +bool gdpr_compliant
        +bool hipaa_compliant
    }

    class FileEmbeddings {
        +uint64 id
        +uint64 file_id
        +int32 chunk_index
        +string chunk_text
    }
```
