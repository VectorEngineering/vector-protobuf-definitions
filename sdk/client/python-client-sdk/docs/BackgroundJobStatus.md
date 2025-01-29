# BackgroundJobStatus

BackgroundJobStatus represents the possible states of a background job. This enum is used to track the lifecycle of asynchronous tasks like scraping jobs.  State transitions: 1. QUEUED -> IN_PROGRESS 2. IN_PROGRESS -> COMPLETED/FAILED/CANCELLED/TIMED_OUT  Usage example: ```go job := &ScrapingJob{     Status: BackgroundJobStatus_BACKGROUND_JOB_STATUS_IN_PROGRESS, } ```   - BACKGROUND_JOB_STATUS_UNSPECIFIED: Default state, should not be used explicitly  - BACKGROUND_JOB_STATUS_QUEUED: Job is queued and waiting to be processed  - BACKGROUND_JOB_STATUS_IN_PROGRESS: Job is currently being processed  - BACKGROUND_JOB_STATUS_COMPLETED: Job has completed successfully  - BACKGROUND_JOB_STATUS_FAILED: Job encountered an error and failed  - BACKGROUND_JOB_STATUS_CANCELLED: Job was manually cancelled by user  - BACKGROUND_JOB_STATUS_TIMED_OUT: Job exceeded its maximum execution time

## Enum

* `BACKGROUND_JOB_STATUS_UNSPECIFIED` (value: `'BACKGROUND_JOB_STATUS_UNSPECIFIED'`)

* `BACKGROUND_JOB_STATUS_QUEUED` (value: `'BACKGROUND_JOB_STATUS_QUEUED'`)

* `BACKGROUND_JOB_STATUS_IN_PROGRESS` (value: `'BACKGROUND_JOB_STATUS_IN_PROGRESS'`)

* `BACKGROUND_JOB_STATUS_COMPLETED` (value: `'BACKGROUND_JOB_STATUS_COMPLETED'`)

* `BACKGROUND_JOB_STATUS_FAILED` (value: `'BACKGROUND_JOB_STATUS_FAILED'`)

* `BACKGROUND_JOB_STATUS_CANCELLED` (value: `'BACKGROUND_JOB_STATUS_CANCELLED'`)

* `BACKGROUND_JOB_STATUS_TIMED_OUT` (value: `'BACKGROUND_JOB_STATUS_TIMED_OUT'`)

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


