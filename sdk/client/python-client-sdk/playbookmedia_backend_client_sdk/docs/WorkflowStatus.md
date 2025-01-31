# WorkflowStatus

WorkflowStatus represents the current state of a scraping workflow. This enum tracks the lifecycle of automated, recurring scraping configurations.  State transitions: 1. DRAFT -> ACTIVE/ARCHIVED 2. ACTIVE -> PAUSED/FAILED/COMPLETED/ARCHIVED 3. PAUSED -> ACTIVE/ARCHIVED 4. FAILED -> ACTIVE/ARCHIVED   - WORKFLOW_STATUS_UNSPECIFIED: Default state, should not be used explicitly  - WORKFLOW_STATUS_DRAFT: Initial draft state before activation  - WORKFLOW_STATUS_ACTIVE: Workflow is actively running on schedule  - WORKFLOW_STATUS_PAUSED: Workflow is temporarily suspended  - WORKFLOW_STATUS_FAILED: Workflow encountered an error  - WORKFLOW_STATUS_COMPLETED: Workflow completed all scheduled runs  - WORKFLOW_STATUS_ARCHIVED: Workflow is archived (soft-deleted)  - WORKFLOW_STATUS_PENDING_APPROVAL: Workflow is pending approval  - WORKFLOW_STATUS_VALIDATING: Workflow is being validated  - WORKFLOW_STATUS_QUOTA_EXCEEDED: Workflow exceeded resource quotas  - WORKFLOW_STATUS_WARNING: Workflow is running but with warnings

## Enum

* `WORKFLOW_STATUS_UNSPECIFIED` (value: `'WORKFLOW_STATUS_UNSPECIFIED'`)

* `WORKFLOW_STATUS_DRAFT` (value: `'WORKFLOW_STATUS_DRAFT'`)

* `WORKFLOW_STATUS_ACTIVE` (value: `'WORKFLOW_STATUS_ACTIVE'`)

* `WORKFLOW_STATUS_PAUSED` (value: `'WORKFLOW_STATUS_PAUSED'`)

* `WORKFLOW_STATUS_FAILED` (value: `'WORKFLOW_STATUS_FAILED'`)

* `WORKFLOW_STATUS_COMPLETED` (value: `'WORKFLOW_STATUS_COMPLETED'`)

* `WORKFLOW_STATUS_ARCHIVED` (value: `'WORKFLOW_STATUS_ARCHIVED'`)

* `WORKFLOW_STATUS_PENDING_APPROVAL` (value: `'WORKFLOW_STATUS_PENDING_APPROVAL'`)

* `WORKFLOW_STATUS_VALIDATING` (value: `'WORKFLOW_STATUS_VALIDATING'`)

* `WORKFLOW_STATUS_QUOTA_EXCEEDED` (value: `'WORKFLOW_STATUS_QUOTA_EXCEEDED'`)

* `WORKFLOW_STATUS_WARNING` (value: `'WORKFLOW_STATUS_WARNING'`)

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


