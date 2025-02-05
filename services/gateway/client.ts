// Generated TypeScript API Client
import type { ZodiosInstance } from "@zodios/core";

// Generated schemas
import { makeApi, Zodios, type ZodiosOptions } from "@zodios/core";
import { z } from "zod";

type APIKey = Partial<{
  id: string;
  name: string;
  keyHash: string;
  keyPrefix: string;
  orgId: string;
  tenantId: string;
  scopes: Array<string>;
  allowedIps: Array<string>;
  allowedDomains: Array<string>;
  allowedEnvironments: Array<string>;
  isTestKey: boolean;
  requestsPerSecond: number;
  requestsPerDay: number;
  concurrentRequests: number;
  monthlyRequestQuota: string;
  costPerRequest: number;
  billingTier: string;
  totalRequests: string;
  totalErrors: string;
  lastUsedAt: string;
  averageResponseTime: number;
  endpointUsageJson: string;
  errorRatesJson: string;
  recentErrors: string;
  successfulRequestsCount: number;
  successRate: number;
  status: v1_Status;
  createdAt: string;
  updatedAt: string;
  expiresAt: string;
  deletedAt: string;
  lastRotatedAt: string;
  lastSecurityReviewAt: string;
  requiresClientSecret: boolean;
  clientSecretHash: string;
  enforceHttps: boolean;
  enforceSigning: boolean;
  allowedSignatureAlgorithms: Array<string>;
  enforceMutualTls: boolean;
  clientCertificateHash: string;
  requireRequestSigning: boolean;
  description: string;
  metadataJson: string;
  tags: Array<string>;
  apiVersion: string;
  supportedFeatures: Array<string>;
  documentationUrl: string;
  supportContact: string;
  account: Account;
  workspace: Workspace;
  logAllRequests: boolean;
  lastRotationReason: string;
  lastRotationDate: string;
  rotationFrequencyDays: number;
  complianceStandards: Array<string>;
  requiresAuditLogging: boolean;
  dataResidency: string;
  approvedIntegrations: Array<string>;
  alertEmails: Array<string>;
  webhookUrl: string;
  alertOnQuotaThreshold: boolean;
  quotaAlertThreshold: number;
  alertOnErrorSpike: boolean;
  errorAlertThreshold: number;
  monitoringIntegrations: Array<string>;
  encrypted: boolean;
  dataClassification: string;
}>;
type v1_Status =
  | "STATUS_UNSPECIFIED"
  | "STATUS_ACTIVE"
  | "STATUS_SUSPENDED"
  | "STATUS_PENDING_VERIFICATION"
  | "STATUS_REVOKED"
  | "STATUS_EXPIRED"
  | "STATUS_RATE_LIMITED"
  | "STATUS_PENDING_REVIEW"
  | "STATUS_DEPRECATED"
  | "STATUS_MAINTENANCE";
type AccountStatus =
  | "ACCOUNT_STATUS_UNSPECIFIED"
  | "ACCOUNT_STATUS_ACTIVE"
  | "ACCOUNT_STATUS_SUSPENDED"
  | "ACCOUNT_STATUS_PENDING_VERIFICATION";
type Role =
  | "ROLE_UNSPECIFIED"
  | "ROLE_ADMIN"
  | "ROLE_USER"
  | "ROLE_VIEWER"
  | "ROLE_MANAGER";
type Permission =
  | "PERMISSION_UNSPECIFIED"
  | "PERMISSION_READ"
  | "PERMISSION_WRITE"
  | "PERMISSION_DELETE"
  | "PERMISSION_MANAGE_USERS"
  | "PERMISSION_MANAGE_BILLING"
  | "PERMISSION_VIEW_ANALYTICS"
  | "PERMISSION_MANAGE_WORKFLOWS";
type Timezone =
  | "TIMEZONE_UNSPECIFIED"
  | "TIMEZONE_UTC"
  | "TIMEZONE_EST"
  | "TIMEZONE_CST"
  | "TIMEZONE_MST"
  | "TIMEZONE_PST"
  | "TIMEZONE_GMT"
  | "TIMEZONE_CET"
  | "TIMEZONE_IST"
  | "TIMEZONE_JST"
  | "TIMEZONE_AEST";
type WorkflowStatus =
  | "WORKFLOW_STATUS_UNSPECIFIED"
  | "WORKFLOW_STATUS_DRAFT"
  | "WORKFLOW_STATUS_ACTIVE"
  | "WORKFLOW_STATUS_PAUSED"
  | "WORKFLOW_STATUS_FAILED"
  | "WORKFLOW_STATUS_COMPLETED"
  | "WORKFLOW_STATUS_ARCHIVED"
  | "WORKFLOW_STATUS_PENDING_APPROVAL"
  | "WORKFLOW_STATUS_VALIDATING"
  | "WORKFLOW_STATUS_QUOTA_EXCEEDED"
  | "WORKFLOW_STATUS_WARNING";
type BackgroundJobStatus =
  | "BACKGROUND_JOB_STATUS_UNSPECIFIED"
  | "BACKGROUND_JOB_STATUS_QUEUED"
  | "BACKGROUND_JOB_STATUS_IN_PROGRESS"
  | "BACKGROUND_JOB_STATUS_COMPLETED"
  | "BACKGROUND_JOB_STATUS_FAILED"
  | "BACKGROUND_JOB_STATUS_CANCELLED"
  | "BACKGROUND_JOB_STATUS_TIMED_OUT";
type Language =
  | "LANGUAGE_UNSPECIFIED"
  | "LANGUAGE_ENGLISH"
  | "LANGUAGE_SPANISH"
  | "LANGUAGE_FRENCH"
  | "LANGUAGE_GERMAN"
  | "LANGUAGE_ITALIAN"
  | "LANGUAGE_PORTUGUESE"
  | "LANGUAGE_DUTCH"
  | "LANGUAGE_RUSSIAN"
  | "LANGUAGE_CHINESE"
  | "LANGUAGE_JAPANESE"
  | "LANGUAGE_KOREAN"
  | "LANGUAGE_ARABIC"
  | "LANGUAGE_HINDI"
  | "LANGUAGE_GREEK"
  | "LANGUAGE_TURKISH";
type BusinessHours = Partial<{
  id: string;
  day: DayOfWeek;
  openTime: string;
  closeTime: string;
  closed: boolean;
  createdAt: string;
  updatedAt: string;
  deletedAt: string;
}>;
type DayOfWeek =
  | "DAY_OF_WEEK_UNSPECIFIED"
  | "DAY_OF_WEEK_MONDAY"
  | "DAY_OF_WEEK_TUESDAY"
  | "DAY_OF_WEEK_WEDNESDAY"
  | "DAY_OF_WEEK_THURSDAY"
  | "DAY_OF_WEEK_FRIDAY"
  | "DAY_OF_WEEK_SATURDAY"
  | "DAY_OF_WEEK_SUNDAY";
type Review = Partial<{
  id: string;
  author: string;
  rating: number;
  text: string;
  time: string;
  language: string;
  profilePhotoUrl: string;
  reviewCount: number;
  createdAt: string;
  updatedAt: string;
  deletedAt: string;
}>;
type RevenueRange =
  | "REVENUE_RANGE_UNSPECIFIED"
  | "REVENUE_RANGE_UNDER_100K"
  | "REVENUE_RANGE_100K_TO_1M"
  | "REVENUE_RANGE_1M_TO_10M"
  | "REVENUE_RANGE_10M_TO_50M"
  | "REVENUE_RANGE_OVER_50M";
type EmployeeBenefit =
  | "EMPLOYEE_BENEFIT_UNSPECIFIED"
  | "EMPLOYEE_BENEFIT_HEALTH_INSURANCE"
  | "EMPLOYEE_BENEFIT_RETIREMENT_PLAN"
  | "EMPLOYEE_BENEFIT_PAID_TIME_OFF"
  | "EMPLOYEE_BENEFIT_REMOTE_WORK";
type OutputFormat =
  | "OUTPUT_FORMAT_UNSPECIFIED"
  | "OUTPUT_FORMAT_JSON"
  | "OUTPUT_FORMAT_CSV"
  | "OUTPUT_FORMAT_BIGQUERY"
  | "OUTPUT_FORMAT_POSTGRES";
type WebhookConfig = Partial<{
  id: string;
  url: string;
  authType: string;
  authToken: string;
  customHeaders: {};
  maxRetries: number;
  retryInterval: string;
  triggerEvents: Array<TriggerEvent>;
  includedFields: Array<IncludedField>;
  includeFullResults: boolean;
  payloadFormat: PayloadFormat;
  verifySsl: boolean;
  signingSecret: string;
  rateLimit: number;
  rateLimitInterval: string;
  createdAt: string;
  updatedAt: string;
  lastTriggeredAt: string;
  successfulCalls: number;
  failedCalls: number;
  metadata: {};
  webhookName: string;
}>;
type TriggerEvent =
  | "TRIGGER_EVENT_UNSPECIFIED"
  | "TRIGGER_EVENT_JOB_STARTED"
  | "TRIGGER_EVENT_JOB_COMPLETED"
  | "TRIGGER_EVENT_JOB_FAILED"
  | "TRIGGER_EVENT_LEAD_FOUND"
  | "TRIGGER_EVENT_QUOTA_EXCEEDED"
  | "TRIGGER_EVENT_ERROR_THRESHOLD_REACHED"
  | "TRIGGER_EVENT_RATE_LIMIT_REACHED"
  | "TRIGGER_EVENT_DATA_VALIDATION_FAILED"
  | "TRIGGER_EVENT_NEW_PROXY_NEEDED"
  | "TRIGGER_EVENT_SCHEDULED_MAINTENANCE";
type IncludedField =
  | "INCLUDED_FIELD_UNSPECIFIED"
  | "INCLUDED_FIELD_NAME"
  | "INCLUDED_FIELD_WEBSITE"
  | "INCLUDED_FIELD_PHONE"
  | "INCLUDED_FIELD_ADDRESS"
  | "INCLUDED_FIELD_LOCATION"
  | "INCLUDED_FIELD_COORDINATES"
  | "INCLUDED_FIELD_GOOGLE_RATING"
  | "INCLUDED_FIELD_REVIEW_COUNT"
  | "INCLUDED_FIELD_REVIEWS"
  | "INCLUDED_FIELD_BUSINESS_HOURS"
  | "INCLUDED_FIELD_BUSINESS_STATUS"
  | "INCLUDED_FIELD_PLACE_ID"
  | "INCLUDED_FIELD_GOOGLE_MAPS_URL"
  | "INCLUDED_FIELD_PHOTOS"
  | "INCLUDED_FIELD_MAIN_PHOTO"
  | "INCLUDED_FIELD_BUSINESS_TYPES"
  | "INCLUDED_FIELD_AMENITIES"
  | "INCLUDED_FIELD_PAYMENT_METHODS"
  | "INCLUDED_FIELD_SOCIAL_PROFILES"
  | "INCLUDED_FIELD_EMPLOYEE_COUNT"
  | "INCLUDED_FIELD_REVENUE_INFO"
  | "INCLUDED_FIELD_FOUNDED_YEAR"
  | "INCLUDED_FIELD_CERTIFICATIONS"
  | "INCLUDED_FIELD_NAICS_CODE"
  | "INCLUDED_FIELD_SIC_CODE"
  | "INCLUDED_FIELD_SCRAPING_METADATA"
  | "INCLUDED_FIELD_COMPLIANCE_INFO"
  | "INCLUDED_FIELD_ALTERNATE_PHONES"
  | "INCLUDED_FIELD_CONTACT_PERSON"
  | "INCLUDED_FIELD_CONTACT_EMAIL";
type PayloadFormat =
  | "PAYLOAD_FORMAT_UNSPECIFIED"
  | "PAYLOAD_FORMAT_JSON"
  | "PAYLOAD_FORMAT_XML"
  | "PAYLOAD_FORMAT_FORM_DATA"
  | "PAYLOAD_FORMAT_PROTOBUF"
  | "PAYLOAD_FORMAT_YAML";
type AccountSettings = Partial<{
  id: string;
  emailNotifications: boolean;
  slackNotifications: boolean;
  defaultDataRetention: string;
  autoPurgeEnabled: boolean;
  require2fa: boolean;
  sessionTimeout: string;
  createdAt: string;
  updatedAt: string;
  deletedAt: string;
}>;
type Account = Partial<{
  id: string;
  authPlatformUserId: string;
  email: string;
  deletedAt: string;
  createdAt: string;
  accountStatus: AccountStatus;
  roles: Array<Role>;
  permissions: Array<Permission>;
  mfaEnabled: boolean;
  lastLoginAt: string;
  timezone: Timezone;
  totalJobsRun: number;
  monthlyJobLimit: number;
  concurrentJobLimit: number;
  workspaces: Array<Workspace>;
  settings: AccountSettings;
}>;
type Lead = Partial<{
  id: string;
  name: string;
  website: string;
  phone: string;
  address: string;
  city: string;
  state: string;
  country: string;
  latitude: number;
  longitude: number;
  googleRating: number;
  reviewCount: number;
  industry: string;
  employeeCount: number;
  estimatedRevenue: string;
  orgId: string;
  tenantId: string;
  createdAt: string;
  updatedAt: string;
  deletedAt: string;
  job: ScrapingJob;
  workspace: Workspace;
  placeId: string;
  googleMapsUrl: string;
  businessStatus: string;
  regularHours: Array<BusinessHours>;
  specialHours: Array<BusinessHours>;
  photoReferences: Array<string>;
  mainPhotoUrl: string;
  reviews: Array<Review>;
  types: Array<string>;
  amenities: Array<string>;
  servesVegetarianFood: boolean;
  outdoorSeating: boolean;
  paymentMethods: Array<string>;
  wheelchairAccessible: boolean;
  parkingAvailable: boolean;
  socialMedia: {};
  ratingCategory: string;
  rating: number;
  count: number;
  lastUpdated: string;
  dataSourceVersion: string;
  scrapingSessionId: string;
  alternatePhones: Array<string>;
  contactPersonName: string;
  contactPersonTitle: string;
  contactEmail: string;
  foundedYear: number;
  businessType: string;
  certifications: Array<string>;
  licenseNumber: string;
  revenueRange: RevenueRange;
  fundingStage: string;
  isPublicCompany: boolean;
  websiteLoadSpeed: number;
  hasSslCertificate: boolean;
  cmsUsed: string;
  ecommercePlatforms: Array<string>;
  timezone: string;
  neighborhood: string;
  nearbyLandmarks: Array<string>;
  transportationAccess: string;
  employeeBenefits: Array<EmployeeBenefit>;
  parentCompany: string;
  subsidiaries: Array<string>;
  isFranchise: boolean;
  seoKeywords: Array<string>;
  usesGoogleAds: boolean;
  googleMyBusinessCategory: string;
  naicsCode: string;
  sicCode: string;
  unspscCode: string;
  isGreenCertified: boolean;
  energySources: Array<string>;
  sustainabilityRating: string;
  recentAnnouncements: Array<string>;
  lastProductLaunch: string;
  hasLitigationHistory: boolean;
  exportControlStatus: string;
}>;
type Organization = Partial<{
  id: string;
  name: string;
  displayName: string;
  website: string;
  description: string;
  billingEmail: string;
  technicalEmail: string;
  phone: string;
  address: string;
  ssoEnabled: boolean;
  ssoProvider: string;
  ssoDomain: string;
  allowedDomains: Array<string>;
  enforce2fa: boolean;
  billingPlan: BillingPlan;
  billingCurrency: string;
  autoBilling: boolean;
  taxId: string;
  complianceFrameworks: Array<string>;
  dataProcessingAgreement: boolean;
  dataRegion: string;
  maxTenants: number;
  totalStorageLimit: string;
  maxApiKeys: number;
  maxUsers: number;
  tenants: Array<Tenant>;
  subscriptions: Subscription;
  createdAt: string;
  updatedAt: string;
  deletedAt: string;
  createdBy: string;
  status: v1_Status;
}>;
type BillingPlan =
  | "BILLING_PLAN_UNSPECIFIED"
  | "BILLING_PLAN_STARTUP"
  | "BILLING_PLAN_BUSINESS"
  | "BILLING_PLAN_ENTERPRISE";
type TenantAPIKey = Partial<{
  id: string;
  keyHash: string;
  keyPrefix: string;
  name: string;
  description: string;
  status: v1_Status;
  scopes: Array<TenantAPIKeyScope>;
  maxUses: number;
  allowedIps: Array<string>;
  useCount: number;
  expiresAt: string;
  createdAt: string;
  updatedAt: string;
  deletedAt: string;
}>;
type TenantAPIKeyScope =
  | "TENANT_API_KEY_SCOPE_UNSPECIFIED"
  | "TENANT_API_KEY_SCOPE_READ_JOBS"
  | "TENANT_API_KEY_SCOPE_READ_LEADS"
  | "TENANT_API_KEY_SCOPE_READ_WORKFLOWS"
  | "TENANT_API_KEY_SCOPE_READ_ANALYTICS"
  | "TENANT_API_KEY_SCOPE_READ_SETTINGS"
  | "TENANT_API_KEY_SCOPE_WRITE_JOBS"
  | "TENANT_API_KEY_SCOPE_WRITE_LEADS"
  | "TENANT_API_KEY_SCOPE_WRITE_WORKFLOWS"
  | "TENANT_API_KEY_SCOPE_WRITE_SETTINGS"
  | "TENANT_API_KEY_SCOPE_DELETE_JOBS"
  | "TENANT_API_KEY_SCOPE_DELETE_LEADS"
  | "TENANT_API_KEY_SCOPE_DELETE_WORKFLOWS"
  | "TENANT_API_KEY_SCOPE_EXPORT_DATA"
  | "TENANT_API_KEY_SCOPE_MANAGE_KEYS"
  | "TENANT_API_KEY_SCOPE_BILLING_READ"
  | "TENANT_API_KEY_SCOPE_BILLING_WRITE"
  | "TENANT_API_KEY_SCOPE_ADMIN";
type Subscription = Partial<{
  id: string;
  stripeCustomerId: string;
  stripeSubscriptionId: string;
  stripePriceId: string;
  stripeProductId: string;
  planTier: PlanTier;
  billingMode: BillingMode;
  includedJobs: number;
  perJobRate: number;
  maxConcurrentJobs: number;
  includedStorage: string;
  perGbRate: number;
  advancedFilteringEnabled: boolean;
  prioritySupportEnabled: boolean;
  customExportsEnabled: boolean;
  apiAccessEnabled: boolean;
  customProxiesEnabled: boolean;
  advancedAnalyticsEnabled: boolean;
  retentionDays: number;
  maxResultsPerJob: number;
  currentPeriodStart: string;
  currentPeriodEnd: string;
  currentPeriodUsage: number;
  currency: string;
  basePrice: number;
  billingInterval: Interval;
  autoRenew: boolean;
  paymentStatus: PaymentStatus;
  isTrial: boolean;
  trialStart: string;
  trialEnd: string;
  createdAt: string;
  updatedAt: string;
  canceledAt: string;
  endedAt: string;
}>;
type PlanTier =
  | "PLAN_TIER_UNSPECIFIED"
  | "PLAN_TIER_FREE"
  | "PLAN_TIER_STARTER"
  | "PLAN_TIER_PROFESSIONAL"
  | "PLAN_TIER_ENTERPRISE";
type BillingMode =
  | "BILLING_MODE_UNSPECIFIED"
  | "BILLING_MODE_LICENSED"
  | "BILLING_MODE_METERED"
  | "BILLING_MODE_HYBRID";
type Interval = "INTERVAL_UNSPECIFIED" | "INTERVAL_MONTHLY" | "INTERVAL_YEARLY";
type PaymentStatus =
  | "PAYMENT_STATUS_UNSPECIFIED"
  | "PAYMENT_STATUS_PAID"
  | "PAYMENT_STATUS_PAST_DUE"
  | "PAYMENT_STATUS_FAILED"
  | "PAYMENT_STATUS_CANCELED";
type ScrapingJob = Partial<{
  id: string;
  priority: number;
  payloadType: string;
  payload: string;
  createdAt: string;
  status: BackgroundJobStatus;
  name: string;
  keywords: Array<string>;
  lang: Language;
  zoom: number;
  lat: string;
  lon: string;
  fastMode: boolean;
  radius: number;
  depth: number;
  email: boolean;
  maxTime: number;
  proxies: Array<string>;
  updatedAt: string;
  deletedAt: string;
  leads: Array<Lead>;
}>;
type ScrapingWorkflow = Partial<{
  id: string;
  name: string;
  cronExpression: string;
  nextRunTime: string;
  lastRunTime: string;
  status: WorkflowStatus;
  retryCount: number;
  maxRetries: number;
  alertEmails: string;
  createdAt: string;
  updatedAt: string;
  deletedAt: string;
  jobs: Array<ScrapingJob>;
  workspace: Workspace;
  geoFencingRadius: number;
  geoFencingLat: number;
  geoFencingLon: number;
  geoFencingZoomMin: number;
  geoFencingZoomMax: number;
  includeReviews: boolean;
  includePhotos: boolean;
  includeBusinessHours: boolean;
  maxReviewsPerBusiness: number;
  outputFormat: OutputFormat;
  outputDestination: string;
  dataRetention: string;
  anonymizePii: boolean;
  notificationSlackChannel: string;
  notificationEmailGroup: string;
  notificationNotifyOnStart: boolean;
  notificationNotifyOnComplete: boolean;
  notificationNotifyOnFailure: boolean;
  contentFilterAllowedCountries: Array<string>;
  contentFilterExcludedTypes: Array<string>;
  contentFilterMinimumRating: number;
  contentFilterMinimumReviews: number;
  qosMaxConcurrentRequests: number;
  qosMaxRetries: number;
  qosRequestTimeout: string;
  qosEnableJavascript: boolean;
  respectRobotsTxt: boolean;
  acceptTermsOfService: boolean;
  userAgent: string;
}>;
type Tenant = Partial<{
  id: string;
  name: string;
  displayName: string;
  description: string;
  organization: Organization;
  apiBaseUrl: string;
  environmentVariables: {};
  allowedOrigins: Array<string>;
  storageQuota: string;
  monthlyRequestLimit: string;
  maxConcurrentJobs: number;
  enableCaching: boolean;
  enableRateLimiting: boolean;
  enableRequestLogging: boolean;
  accounts: Array<Account>;
  apiKeys: Array<TenantAPIKey>;
  totalRequests: string;
  totalStorageUsed: string;
  averageResponseTime: number;
  createdAt: string;
  updatedAt: string;
  deletedAt: string;
  status: v1_Status;
}>;
type Workspace = Partial<{
  id: string;
  name: string;
  industry: string;
  domain: string;
  gdprCompliant: boolean;
  hipaaCompliant: boolean;
  soc2Compliant: boolean;
  storageQuota: string;
  usedStorage: string;
  createdAt: string;
  updatedAt: string;
  deletedAt: string;
  workflows: Array<ScrapingWorkflow>;
  jobsRunThisMonth: number;
  workspaceJobLimit: number;
  dailyJobQuota: number;
  activeScrapers: number;
  totalLeadsCollected: number;
  lastJobRun: string;
  scrapingJobs: Array<ScrapingJob>;
  apiKeys: Array<APIKey>;
  webhooks: Array<WebhookConfig>;
}>;
type FolderMetadata = Partial<{
  id: string;
  name: string;
  s3BucketName: string;
  s3FolderPath: string;
  isDeleted: boolean;
  parentFolderId: string;
  createdAt: string;
  updatedAt: string;
  deletedAt: string;
  childFolders: Array<FolderMetadata>;
  files: Array<FileMetadata>;
}>;
type FileMetadata = Partial<{
  id: string;
  name: string;
  size: string;
  s3Key: string;
  s3BucketName: string;
  isDeleted: boolean;
  version: number;
  createdAt: string;
  updatedAt: string;
  deletedAt: string;
  embeddings: FileEmbeddings;
  versions: Array<FileVersion>;
  comments: Array<CommentThread>;
  sharing: Array<FileSharing>;
  snapshots: Array<DocumentSnapshot>;
}>;
type FileEmbeddings = Partial<{
  id: string;
  embeddings: Array<number>;
  modelVersion: string;
  createdAt: string;
  updatedAt: string;
}>;
type FileVersion = Partial<{
  id: string;
  fileId: string;
  versionNumber: number;
  s3Key: string;
  size: string;
  commitMessage: string;
  authorId: string;
  createdAt: string;
}>;
type CommentThread = Partial<{
  id: string;
  authorId: string;
  content: string;
  startPosition: number;
  endPosition: number;
  resolved: boolean;
  createdAt: string;
  updatedAt: string;
}>;
type FileSharing = Partial<{
  id: string;
  sharedWithEmail: string;
  permissionLevel: string;
  expiresAt: string;
  createdAt: string;
  updatedAt: string;
}>;
type DocumentSnapshot = Partial<{
  id: string;
  fileId: string;
  snapshotHash: string;
  content: string;
  authorId: string;
  reason: string;
  createdAt: string;
}>;

const AccountStatus = z.enum([
  "ACCOUNT_STATUS_UNSPECIFIED",
  "ACCOUNT_STATUS_ACTIVE",
  "ACCOUNT_STATUS_SUSPENDED",
  "ACCOUNT_STATUS_PENDING_VERIFICATION",
]);
const Role = z.enum([
  "ROLE_UNSPECIFIED",
  "ROLE_ADMIN",
  "ROLE_USER",
  "ROLE_VIEWER",
  "ROLE_MANAGER",
]);
const Permission = z.enum([
  "PERMISSION_UNSPECIFIED",
  "PERMISSION_READ",
  "PERMISSION_WRITE",
  "PERMISSION_DELETE",
  "PERMISSION_MANAGE_USERS",
  "PERMISSION_MANAGE_BILLING",
  "PERMISSION_VIEW_ANALYTICS",
  "PERMISSION_MANAGE_WORKFLOWS",
]);
const Timezone = z.enum([
  "TIMEZONE_UNSPECIFIED",
  "TIMEZONE_UTC",
  "TIMEZONE_EST",
  "TIMEZONE_CST",
  "TIMEZONE_MST",
  "TIMEZONE_PST",
  "TIMEZONE_GMT",
  "TIMEZONE_CET",
  "TIMEZONE_IST",
  "TIMEZONE_JST",
  "TIMEZONE_AEST",
]);
const WorkflowStatus = z.enum([
  "WORKFLOW_STATUS_UNSPECIFIED",
  "WORKFLOW_STATUS_DRAFT",
  "WORKFLOW_STATUS_ACTIVE",
  "WORKFLOW_STATUS_PAUSED",
  "WORKFLOW_STATUS_FAILED",
  "WORKFLOW_STATUS_COMPLETED",
  "WORKFLOW_STATUS_ARCHIVED",
  "WORKFLOW_STATUS_PENDING_APPROVAL",
  "WORKFLOW_STATUS_VALIDATING",
  "WORKFLOW_STATUS_QUOTA_EXCEEDED",
  "WORKFLOW_STATUS_WARNING",
]);
const BackgroundJobStatus = z.enum([
  "BACKGROUND_JOB_STATUS_UNSPECIFIED",
  "BACKGROUND_JOB_STATUS_QUEUED",
  "BACKGROUND_JOB_STATUS_IN_PROGRESS",
  "BACKGROUND_JOB_STATUS_COMPLETED",
  "BACKGROUND_JOB_STATUS_FAILED",
  "BACKGROUND_JOB_STATUS_CANCELLED",
  "BACKGROUND_JOB_STATUS_TIMED_OUT",
]);
const Language = z.enum([
  "LANGUAGE_UNSPECIFIED",
  "LANGUAGE_ENGLISH",
  "LANGUAGE_SPANISH",
  "LANGUAGE_FRENCH",
  "LANGUAGE_GERMAN",
  "LANGUAGE_ITALIAN",
  "LANGUAGE_PORTUGUESE",
  "LANGUAGE_DUTCH",
  "LANGUAGE_RUSSIAN",
  "LANGUAGE_CHINESE",
  "LANGUAGE_JAPANESE",
  "LANGUAGE_KOREAN",
  "LANGUAGE_ARABIC",
  "LANGUAGE_HINDI",
  "LANGUAGE_GREEK",
  "LANGUAGE_TURKISH",
]);
const DayOfWeek = z.enum([
  "DAY_OF_WEEK_UNSPECIFIED",
  "DAY_OF_WEEK_MONDAY",
  "DAY_OF_WEEK_TUESDAY",
  "DAY_OF_WEEK_WEDNESDAY",
  "DAY_OF_WEEK_THURSDAY",
  "DAY_OF_WEEK_FRIDAY",
  "DAY_OF_WEEK_SATURDAY",
  "DAY_OF_WEEK_SUNDAY",
]);
const BusinessHours = z
  .object({
    id: z.string(),
    day: DayOfWeek.default("DAY_OF_WEEK_UNSPECIFIED"),
    openTime: z.string(),
    closeTime: z.string(),
    closed: z.boolean(),
    createdAt: z.string().datetime({ offset: true }),
    updatedAt: z.string().datetime({ offset: true }),
    deletedAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const Review = z
  .object({
    id: z.string(),
    author: z.string(),
    rating: z.number(),
    text: z.string(),
    time: z.string().datetime({ offset: true }),
    language: z.string(),
    profilePhotoUrl: z.string(),
    reviewCount: z.number().int(),
    createdAt: z.string().datetime({ offset: true }),
    updatedAt: z.string().datetime({ offset: true }),
    deletedAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const RevenueRange = z.enum([
  "REVENUE_RANGE_UNSPECIFIED",
  "REVENUE_RANGE_UNDER_100K",
  "REVENUE_RANGE_100K_TO_1M",
  "REVENUE_RANGE_1M_TO_10M",
  "REVENUE_RANGE_10M_TO_50M",
  "REVENUE_RANGE_OVER_50M",
]);
const EmployeeBenefit = z.enum([
  "EMPLOYEE_BENEFIT_UNSPECIFIED",
  "EMPLOYEE_BENEFIT_HEALTH_INSURANCE",
  "EMPLOYEE_BENEFIT_RETIREMENT_PLAN",
  "EMPLOYEE_BENEFIT_PAID_TIME_OFF",
  "EMPLOYEE_BENEFIT_REMOTE_WORK",
]);
const Lead: z.ZodType<Lead> = z.lazy(() =>
  z
    .object({
      id: z.string(),
      name: z.string(),
      website: z.string(),
      phone: z.string(),
      address: z.string(),
      city: z.string(),
      state: z.string(),
      country: z.string(),
      latitude: z.number(),
      longitude: z.number(),
      googleRating: z.number(),
      reviewCount: z.number().int(),
      industry: z.string(),
      employeeCount: z.number().int(),
      estimatedRevenue: z.string(),
      orgId: z.string(),
      tenantId: z.string(),
      createdAt: z.string().datetime({ offset: true }),
      updatedAt: z.string().datetime({ offset: true }),
      deletedAt: z.string().datetime({ offset: true }),
      job: ScrapingJob,
      workspace: Workspace,
      placeId: z.string(),
      googleMapsUrl: z.string(),
      businessStatus: z.string(),
      regularHours: z.array(BusinessHours),
      specialHours: z.array(BusinessHours),
      photoReferences: z.array(z.string()),
      mainPhotoUrl: z.string(),
      reviews: z.array(Review),
      types: z.array(z.string()),
      amenities: z.array(z.string()),
      servesVegetarianFood: z.boolean(),
      outdoorSeating: z.boolean(),
      paymentMethods: z.array(z.string()),
      wheelchairAccessible: z.boolean(),
      parkingAvailable: z.boolean(),
      socialMedia: z.record(z.string()),
      ratingCategory: z.string(),
      rating: z.number(),
      count: z.number().int(),
      lastUpdated: z.string().datetime({ offset: true }),
      dataSourceVersion: z.string(),
      scrapingSessionId: z.string(),
      alternatePhones: z.array(z.string()),
      contactPersonName: z.string(),
      contactPersonTitle: z.string(),
      contactEmail: z.string(),
      foundedYear: z.number().int(),
      businessType: z.string(),
      certifications: z.array(z.string()),
      licenseNumber: z.string(),
      revenueRange: RevenueRange.default("REVENUE_RANGE_UNSPECIFIED"),
      fundingStage: z.string(),
      isPublicCompany: z.boolean(),
      websiteLoadSpeed: z.number(),
      hasSslCertificate: z.boolean(),
      cmsUsed: z.string(),
      ecommercePlatforms: z.array(z.string()),
      timezone: z.string(),
      neighborhood: z.string(),
      nearbyLandmarks: z.array(z.string()),
      transportationAccess: z.string(),
      employeeBenefits: z.array(EmployeeBenefit),
      parentCompany: z.string(),
      subsidiaries: z.array(z.string()),
      isFranchise: z.boolean(),
      seoKeywords: z.array(z.string()),
      usesGoogleAds: z.boolean(),
      googleMyBusinessCategory: z.string(),
      naicsCode: z.string(),
      sicCode: z.string(),
      unspscCode: z.string(),
      isGreenCertified: z.boolean(),
      energySources: z.array(z.string()),
      sustainabilityRating: z.string(),
      recentAnnouncements: z.array(z.string()),
      lastProductLaunch: z.string().datetime({ offset: true }),
      hasLitigationHistory: z.boolean(),
      exportControlStatus: z.string(),
    })
    .partial()
    .passthrough(),
);
const ScrapingJob: z.ZodType<ScrapingJob> = z.lazy(() =>
  z
    .object({
      id: z.string(),
      priority: z.number().int(),
      payloadType: z.string(),
      payload: z.string(),
      createdAt: z.string().datetime({ offset: true }),
      status: BackgroundJobStatus.default("BACKGROUND_JOB_STATUS_UNSPECIFIED"),
      name: z.string(),
      keywords: z.array(z.string()),
      lang: Language.default("LANGUAGE_UNSPECIFIED"),
      zoom: z.number().int(),
      lat: z.string(),
      lon: z.string(),
      fastMode: z.boolean(),
      radius: z.number().int(),
      depth: z.number().int(),
      email: z.boolean(),
      maxTime: z.number().int(),
      proxies: z.array(z.string()),
      updatedAt: z.string().datetime({ offset: true }),
      deletedAt: z.string().datetime({ offset: true }),
      leads: z.array(Lead),
    })
    .partial()
    .passthrough(),
);
const OutputFormat = z.enum([
  "OUTPUT_FORMAT_UNSPECIFIED",
  "OUTPUT_FORMAT_JSON",
  "OUTPUT_FORMAT_CSV",
  "OUTPUT_FORMAT_BIGQUERY",
  "OUTPUT_FORMAT_POSTGRES",
]);
const ScrapingWorkflow: z.ZodType<ScrapingWorkflow> = z.lazy(() =>
  z
    .object({
      id: z.string(),
      name: z.string(),
      cronExpression: z.string(),
      nextRunTime: z.string().datetime({ offset: true }),
      lastRunTime: z.string().datetime({ offset: true }),
      status: WorkflowStatus.default("WORKFLOW_STATUS_UNSPECIFIED"),
      retryCount: z.number().int(),
      maxRetries: z.number().int(),
      alertEmails: z.string(),
      createdAt: z.string().datetime({ offset: true }),
      updatedAt: z.string().datetime({ offset: true }),
      deletedAt: z.string().datetime({ offset: true }),
      jobs: z.array(ScrapingJob),
      workspace: Workspace,
      geoFencingRadius: z.number(),
      geoFencingLat: z.number(),
      geoFencingLon: z.number(),
      geoFencingZoomMin: z.number().int(),
      geoFencingZoomMax: z.number().int(),
      includeReviews: z.boolean(),
      includePhotos: z.boolean(),
      includeBusinessHours: z.boolean(),
      maxReviewsPerBusiness: z.number().int(),
      outputFormat: OutputFormat.default("OUTPUT_FORMAT_UNSPECIFIED"),
      outputDestination: z.string(),
      dataRetention: z.string(),
      anonymizePii: z.boolean(),
      notificationSlackChannel: z.string(),
      notificationEmailGroup: z.string(),
      notificationNotifyOnStart: z.boolean(),
      notificationNotifyOnComplete: z.boolean(),
      notificationNotifyOnFailure: z.boolean(),
      contentFilterAllowedCountries: z.array(z.string()),
      contentFilterExcludedTypes: z.array(z.string()),
      contentFilterMinimumRating: z.number(),
      contentFilterMinimumReviews: z.number().int(),
      qosMaxConcurrentRequests: z.number().int(),
      qosMaxRetries: z.number().int(),
      qosRequestTimeout: z.string(),
      qosEnableJavascript: z.boolean(),
      respectRobotsTxt: z.boolean(),
      acceptTermsOfService: z.boolean(),
      userAgent: z.string(),
    })
    .partial()
    .passthrough(),
);
const v1_Status = z.enum([
  "STATUS_UNSPECIFIED",
  "STATUS_ACTIVE",
  "STATUS_SUSPENDED",
  "STATUS_PENDING_VERIFICATION",
  "STATUS_REVOKED",
  "STATUS_EXPIRED",
  "STATUS_RATE_LIMITED",
  "STATUS_PENDING_REVIEW",
  "STATUS_DEPRECATED",
  "STATUS_MAINTENANCE",
]);
const APIKey: z.ZodType<APIKey> = z.lazy(() =>
  z
    .object({
      id: z.string(),
      name: z.string(),
      keyHash: z.string(),
      keyPrefix: z.string(),
      orgId: z.string(),
      tenantId: z.string(),
      scopes: z.array(z.string()),
      allowedIps: z.array(z.string()),
      allowedDomains: z.array(z.string()),
      allowedEnvironments: z.array(z.string()),
      isTestKey: z.boolean(),
      requestsPerSecond: z.number().int(),
      requestsPerDay: z.number().int(),
      concurrentRequests: z.number().int(),
      monthlyRequestQuota: z.string(),
      costPerRequest: z.number(),
      billingTier: z.string(),
      totalRequests: z.string(),
      totalErrors: z.string(),
      lastUsedAt: z.string().datetime({ offset: true }),
      averageResponseTime: z.number(),
      endpointUsageJson: z.string(),
      errorRatesJson: z.string(),
      recentErrors: z.string(),
      successfulRequestsCount: z.number().int(),
      successRate: z.number(),
      status: v1_Status.default("STATUS_UNSPECIFIED"),
      createdAt: z.string().datetime({ offset: true }),
      updatedAt: z.string().datetime({ offset: true }),
      expiresAt: z.string().datetime({ offset: true }),
      deletedAt: z.string().datetime({ offset: true }),
      lastRotatedAt: z.string().datetime({ offset: true }),
      lastSecurityReviewAt: z.string().datetime({ offset: true }),
      requiresClientSecret: z.boolean(),
      clientSecretHash: z.string(),
      enforceHttps: z.boolean(),
      enforceSigning: z.boolean(),
      allowedSignatureAlgorithms: z.array(z.string()),
      enforceMutualTls: z.boolean(),
      clientCertificateHash: z.string(),
      requireRequestSigning: z.boolean(),
      description: z.string(),
      metadataJson: z.string(),
      tags: z.array(z.string()),
      apiVersion: z.string(),
      supportedFeatures: z.array(z.string()),
      documentationUrl: z.string(),
      supportContact: z.string(),
      account: Account,
      workspace: Workspace,
      logAllRequests: z.boolean(),
      lastRotationReason: z.string(),
      lastRotationDate: z.string().datetime({ offset: true }),
      rotationFrequencyDays: z.number().int(),
      complianceStandards: z.array(z.string()),
      requiresAuditLogging: z.boolean(),
      dataResidency: z.string(),
      approvedIntegrations: z.array(z.string()),
      alertEmails: z.array(z.string()),
      webhookUrl: z.string(),
      alertOnQuotaThreshold: z.boolean(),
      quotaAlertThreshold: z.number(),
      alertOnErrorSpike: z.boolean(),
      errorAlertThreshold: z.number(),
      monitoringIntegrations: z.array(z.string()),
      encrypted: z.boolean(),
      dataClassification: z.string(),
    })
    .partial()
    .passthrough(),
);
const TriggerEvent = z.enum([
  "TRIGGER_EVENT_UNSPECIFIED",
  "TRIGGER_EVENT_JOB_STARTED",
  "TRIGGER_EVENT_JOB_COMPLETED",
  "TRIGGER_EVENT_JOB_FAILED",
  "TRIGGER_EVENT_LEAD_FOUND",
  "TRIGGER_EVENT_QUOTA_EXCEEDED",
  "TRIGGER_EVENT_ERROR_THRESHOLD_REACHED",
  "TRIGGER_EVENT_RATE_LIMIT_REACHED",
  "TRIGGER_EVENT_DATA_VALIDATION_FAILED",
  "TRIGGER_EVENT_NEW_PROXY_NEEDED",
  "TRIGGER_EVENT_SCHEDULED_MAINTENANCE",
]);
const IncludedField = z.enum([
  "INCLUDED_FIELD_UNSPECIFIED",
  "INCLUDED_FIELD_NAME",
  "INCLUDED_FIELD_WEBSITE",
  "INCLUDED_FIELD_PHONE",
  "INCLUDED_FIELD_ADDRESS",
  "INCLUDED_FIELD_LOCATION",
  "INCLUDED_FIELD_COORDINATES",
  "INCLUDED_FIELD_GOOGLE_RATING",
  "INCLUDED_FIELD_REVIEW_COUNT",
  "INCLUDED_FIELD_REVIEWS",
  "INCLUDED_FIELD_BUSINESS_HOURS",
  "INCLUDED_FIELD_BUSINESS_STATUS",
  "INCLUDED_FIELD_PLACE_ID",
  "INCLUDED_FIELD_GOOGLE_MAPS_URL",
  "INCLUDED_FIELD_PHOTOS",
  "INCLUDED_FIELD_MAIN_PHOTO",
  "INCLUDED_FIELD_BUSINESS_TYPES",
  "INCLUDED_FIELD_AMENITIES",
  "INCLUDED_FIELD_PAYMENT_METHODS",
  "INCLUDED_FIELD_SOCIAL_PROFILES",
  "INCLUDED_FIELD_EMPLOYEE_COUNT",
  "INCLUDED_FIELD_REVENUE_INFO",
  "INCLUDED_FIELD_FOUNDED_YEAR",
  "INCLUDED_FIELD_CERTIFICATIONS",
  "INCLUDED_FIELD_NAICS_CODE",
  "INCLUDED_FIELD_SIC_CODE",
  "INCLUDED_FIELD_SCRAPING_METADATA",
  "INCLUDED_FIELD_COMPLIANCE_INFO",
  "INCLUDED_FIELD_ALTERNATE_PHONES",
  "INCLUDED_FIELD_CONTACT_PERSON",
  "INCLUDED_FIELD_CONTACT_EMAIL",
]);
const PayloadFormat = z.enum([
  "PAYLOAD_FORMAT_UNSPECIFIED",
  "PAYLOAD_FORMAT_JSON",
  "PAYLOAD_FORMAT_XML",
  "PAYLOAD_FORMAT_FORM_DATA",
  "PAYLOAD_FORMAT_PROTOBUF",
  "PAYLOAD_FORMAT_YAML",
]);
const WebhookConfig = z
  .object({
    id: z.string(),
    url: z.string(),
    authType: z.string(),
    authToken: z.string(),
    customHeaders: z.record(z.string()),
    maxRetries: z.number().int(),
    retryInterval: z.string(),
    triggerEvents: z.array(TriggerEvent),
    includedFields: z.array(IncludedField),
    includeFullResults: z.boolean(),
    payloadFormat: PayloadFormat.default("PAYLOAD_FORMAT_UNSPECIFIED"),
    verifySsl: z.boolean(),
    signingSecret: z.string(),
    rateLimit: z.number().int(),
    rateLimitInterval: z.string(),
    createdAt: z.string().datetime({ offset: true }),
    updatedAt: z.string().datetime({ offset: true }),
    lastTriggeredAt: z.string().datetime({ offset: true }),
    successfulCalls: z.number().int(),
    failedCalls: z.number().int(),
    metadata: z.object({}).partial().passthrough(),
    webhookName: z.string(),
  })
  .partial()
  .passthrough();
const Workspace: z.ZodType<Workspace> = z.lazy(() =>
  z
    .object({
      id: z.string(),
      name: z.string(),
      industry: z.string(),
      domain: z.string(),
      gdprCompliant: z.boolean(),
      hipaaCompliant: z.boolean(),
      soc2Compliant: z.boolean(),
      storageQuota: z.string(),
      usedStorage: z.string(),
      createdAt: z.string().datetime({ offset: true }),
      updatedAt: z.string().datetime({ offset: true }),
      deletedAt: z.string().datetime({ offset: true }),
      workflows: z.array(ScrapingWorkflow),
      jobsRunThisMonth: z.number().int(),
      workspaceJobLimit: z.number().int(),
      dailyJobQuota: z.number().int(),
      activeScrapers: z.number().int(),
      totalLeadsCollected: z.number().int(),
      lastJobRun: z.string().datetime({ offset: true }),
      scrapingJobs: z.array(ScrapingJob),
      apiKeys: z.array(APIKey),
      webhooks: z.array(WebhookConfig),
    })
    .partial()
    .passthrough(),
);
const AccountSettings = z
  .object({
    id: z.string(),
    emailNotifications: z.boolean(),
    slackNotifications: z.boolean(),
    defaultDataRetention: z.string(),
    autoPurgeEnabled: z.boolean(),
    require2fa: z.boolean(),
    sessionTimeout: z.string(),
    createdAt: z.string().datetime({ offset: true }),
    updatedAt: z.string().datetime({ offset: true }),
    deletedAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const Account: z.ZodType<Account> = z.lazy(() =>
  z
    .object({
      id: z.string(),
      authPlatformUserId: z.string(),
      email: z.string(),
      deletedAt: z.string().datetime({ offset: true }),
      createdAt: z.string().datetime({ offset: true }),
      accountStatus: AccountStatus.default("ACCOUNT_STATUS_UNSPECIFIED"),
      roles: z.array(Role),
      permissions: z.array(Permission),
      mfaEnabled: z.boolean(),
      lastLoginAt: z.string().datetime({ offset: true }),
      timezone: Timezone.default("TIMEZONE_UNSPECIFIED"),
      totalJobsRun: z.number().int(),
      monthlyJobLimit: z.number().int(),
      concurrentJobLimit: z.number().int(),
      workspaces: z.array(Workspace),
      settings: AccountSettings,
    })
    .partial()
    .passthrough(),
);
const ListAccountsResponse = z
  .object({ accounts: z.array(Account), nextPageNumber: z.number().int() })
  .partial()
  .passthrough();
const ValidationErrorCode = z.enum([
  "NO_ERROR",
  "VALIDATION_ERROR",
  "INVALID_WRITE_INPUT",
  "INVALID_CHECK_INPUT",
  "INVALID_EXPAND_INPUT",
  "INVALID_OBJECT_FORMAT",
  "INVALID_USER",
  "INVALID_TUPLE",
  "AUTHORIZATION_MODEL_NOT_FOUND",
  "TYPE_NOT_FOUND",
  "RELATION_NOT_FOUND",
  "STORE_ID_INVALID_LENGTH",
  "ID_TOO_LONG",
  "INVALID_CONTINUATION_TOKEN",
  "PAGE_SIZE_INVALID",
  "PARAM_MISSING_VALUE",
  "EXCEEDED_ENTITY_LIMIT",
  "DUPLICATE_ENTRY",
  "CANNOT_ALLOW_DUPLICATE_TUPLES",
  "CANNOT_ALLOW_DUPLICATE_TYPES",
  "INVALID_SCHEMA_VERSION",
  "INVALID_TYPE_PATTERN",
  "INVALID_RELATION_PATTERN",
  "INVALID_OBJECT_PATTERN",
  "INVALID_FORMAT",
  "INVALID_EMAIL_FORMAT",
  "INVALID_PASSWORD_STRENGTH",
  "FIELD_TOO_SHORT",
  "FIELD_TOO_LONG",
  "UNSUPPORTED_MEDIA_TYPE",
]);
const FieldViolation = z
  .object({
    field: z.string(),
    validation: z.string(),
    message: z.string(),
    expected: z.string(),
    actual: z.string(),
  })
  .partial()
  .passthrough();
const SchemaValidation = z
  .object({
    schemaVersion: z.string(),
    invalidPatterns: z.array(z.string()),
    typeMismatches: z.array(z.string()),
  })
  .partial()
  .passthrough();
const ResourceValidation = z
  .object({
    missingResources: z.array(z.string()),
    invalidReferences: z.array(z.string()),
    idViolations: z.record(z.string()),
  })
  .partial()
  .passthrough();
const Any = z.object({ "@type": z.string() }).partial().passthrough();
const Status = z
  .object({
    code: z.number().int(),
    message: z.string(),
    details: z.array(Any),
  })
  .partial()
  .passthrough();
const ErrorResponse = z.object({ status: Status }).partial().passthrough();
const ValidationErrorMessageResponse = z
  .object({
    code: ValidationErrorCode.default("NO_ERROR"),
    message: z.string(),
    fieldViolations: z.array(FieldViolation),
    schemaValidation: SchemaValidation,
    resourceValidation: ResourceValidation,
    errorResponse: ErrorResponse,
  })
  .partial()
  .passthrough();
const AuthErrorCode = z.enum([
  "NO_AUTH_ERROR",
  "AUTH_FAILED_INVALID_BEARER_TOKEN",
  "AUTH_FAILED_INVALID_SUBJECT",
  "AUTH_FAILED_INVALID_AUDIENCE",
  "AUTH_FAILED_INVALID_ISSUER",
  "BEARER_TOKEN_MISSING",
  "TOKEN_EXPIRED",
  "TOKEN_NOT_ACTIVE",
  "TOKEN_REVOKED",
  "INVALID_CLAIMS",
  "MISSING_REQUIRED_CLAIMS",
  "INVALID_SCOPE",
  "INVALID_PERMISSIONS",
  "SESSION_EXPIRED",
  "SESSION_INVALID",
  "SESSION_REVOKED",
  "UNAUTHENTICATED",
  "MULTI_FACTOR_REQUIRED",
  "MULTI_FACTOR_FAILED",
  "ACCOUNT_LOCKED",
  "ACCOUNT_DISABLED",
  "PASSWORD_EXPIRED",
  "PASSWORD_RESET_REQUIRED",
  "UNRECOGNIZED_DEVICE",
]);
const TokenInfo = z
  .object({
    tokenId: z.string(),
    tokenType: z.string(),
    expiry: z.string().datetime({ offset: true }),
    issuer: z.string(),
    audiences: z.array(z.string()),
  })
  .partial()
  .passthrough();
const AuthContext = z
  .object({
    requiredScopes: z.array(z.string()),
    providedScopes: z.array(z.string()),
    missingPermissions: z.array(z.string()),
    tenantId: z.string(),
  })
  .partial()
  .passthrough();
const SessionInfo = z
  .object({
    sessionId: z.string(),
    lastActive: z.string().datetime({ offset: true }),
    deviceId: z.string(),
    ipAddress: z.string(),
  })
  .partial()
  .passthrough();
const MFAInfo = z
  .object({
    mfaRequired: z.boolean(),
    allowedMethods: z.array(z.string()),
    failedReason: z.string(),
  })
  .partial()
  .passthrough();
const AuthenticationErrorMessageResponse = z
  .object({
    code: AuthErrorCode.default("NO_AUTH_ERROR"),
    message: z.string(),
    tokenInfo: TokenInfo,
    authContext: AuthContext,
    sessionInfo: SessionInfo,
    mfaInfo: MFAInfo,
    errorResponse: ErrorResponse,
  })
  .partial()
  .passthrough();
const PaymentInfo = z
  .object({
    amountDue: z.number(),
    currency: z.string(),
    dueDate: z.string().datetime({ offset: true }),
    paymentUrl: z.string(),
  })
  .partial()
  .passthrough();
const PaymentRequiredErrorMessageResponse = z
  .object({
    code: z.number().int(),
    message: z.string(),
    paymentInfo: PaymentInfo,
    errorResponse: ErrorResponse,
  })
  .partial()
  .passthrough();
const ForbiddenErrorMessageResponse = z
  .object({
    code: z.number().int(),
    message: z.string(),
    reason: z.string(),
    requiredPermissions: z.array(z.string()),
    errorResponse: ErrorResponse,
  })
  .partial()
  .passthrough();
const NotFoundErrorCode = z.enum([
  "NO_NOT_FOUND_ERROR",
  "UNDEFINED_ENDPOINT",
  "UNIMPLEMENTED",
  "STORE_ID_NOT_FOUND",
  "USER_NOT_FOUND",
  "RESOURCE_NOT_FOUND",
  "TENANT_NOT_FOUND",
]);
const ResourceInfo = z
  .object({
    type: z.string(),
    id: z.string(),
    path: z.string(),
    tenantId: z.string(),
    scopes: z.array(z.string()),
  })
  .partial()
  .passthrough();
const Suggestions = z
  .object({
    similarResources: z.array(z.string()),
    alternativePaths: z.array(z.string()),
    documentationUrl: z.string(),
    hints: z.record(z.string()),
  })
  .partial()
  .passthrough();
const APIInfo = z
  .object({
    version: z.string(),
    supportedVersions: z.array(z.string()),
    isDeprecated: z.boolean(),
    sunsetDate: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const NotFoundErrorMessageResponse = z
  .object({
    code: NotFoundErrorCode.default("NO_NOT_FOUND_ERROR"),
    message: z.string(),
    resourceInfo: ResourceInfo,
    suggestions: Suggestions,
    apiInfo: APIInfo,
    errorResponse: ErrorResponse,
  })
  .partial()
  .passthrough();
const MethodNotAllowedErrorMessageResponse = z
  .object({
    code: z.number().int(),
    message: z.string(),
    allowedMethods: z.array(z.string()),
    errorResponse: ErrorResponse,
  })
  .partial()
  .passthrough();
const ConflictInfo = z
  .object({
    resourceType: z.string(),
    identifier: z.string(),
    conflictReason: z.string(),
    createdAt: z.string().datetime({ offset: true }),
    createdBy: z.string(),
  })
  .partial()
  .passthrough();
const ConflictErrorMessageResponse = z
  .object({
    code: ValidationErrorCode.default("NO_ERROR"),
    message: z.string(),
    conflictInfo: ConflictInfo,
    errorResponse: ErrorResponse,
  })
  .partial()
  .passthrough();
const GoneErrorMessageResponse = z
  .object({
    code: z.number().int(),
    message: z.string(),
    resource: z.string(),
    errorResponse: ErrorResponse,
  })
  .partial()
  .passthrough();
const PreconditionFailedErrorMessageResponse = z
  .object({
    code: z.number().int(),
    message: z.string(),
    failedConditions: z.array(z.string()),
    errorResponse: ErrorResponse,
  })
  .partial()
  .passthrough();
const FieldError = z
  .object({ field: z.string(), error: z.string() })
  .partial()
  .passthrough();
const UnprocessableEntityErrorMessageResponse = z
  .object({
    code: z.number().int(),
    message: z.string(),
    errors: z.array(FieldError),
    errorResponse: ErrorResponse,
  })
  .partial()
  .passthrough();
const TooEarlyErrorMessageResponse = z
  .object({
    code: z.number().int(),
    message: z.string(),
    errorResponse: ErrorResponse,
  })
  .partial()
  .passthrough();
const InternalErrorCode = z.enum([
  "NO_INTERNAL_ERROR",
  "INTERNAL_ERROR",
  "DATA_CORRUPTION",
  "DATABASE_ERROR",
  "CACHE_ERROR",
  "RESOURCE_EXHAUSTED",
  "QUOTA_EXCEEDED",
  "RATE_LIMIT_EXCEEDED",
  "CANCELLED",
  "DEADLINE_EXCEEDED",
  "ALREADY_EXISTS",
  "FAILED_PRECONDITION",
  "ABORTED",
  "OUT_OF_RANGE",
  "UNAVAILABLE",
  "DATA_LOSS",
  "SERVICE_DEGRADED",
  "DEPENDENCY_FAILURE",
  "CONFIGURATION_ERROR",
  "SECURITY_BREACH_DETECTED",
]);
const LimitInfo = z
  .object({
    current: z.number().int(),
    limit: z.number().int(),
    resetTime: z.string().datetime({ offset: true }),
    windowSize: z.string(),
  })
  .partial()
  .passthrough();
const QuotaInfo = z
  .object({
    resource: z.string(),
    used: z.string(),
    limit: z.string(),
    resetTime: z.string().datetime({ offset: true }),
    resourceUsage: z.record(z.string()),
  })
  .partial()
  .passthrough();
const RateLimitContext = z
  .object({
    userId: z.string(),
    ipAddress: z.string(),
    affectedEndpoints: z.array(z.string()),
    headers: z.record(z.string()),
  })
  .partial()
  .passthrough();
const RateLimitErrorMessageResponse = z
  .object({
    code: InternalErrorCode.default("NO_INTERNAL_ERROR"),
    message: z.string(),
    limitInfo: LimitInfo,
    quotaInfo: QuotaInfo,
    context: RateLimitContext,
    errorResponse: ErrorResponse,
  })
  .partial()
  .passthrough();
const Dependency = z
  .object({
    name: z.string(),
    status: z.string(),
    error: z.string(),
    latency: z.number().int(),
  })
  .partial()
  .passthrough();
const ServiceStatus = z
  .object({
    name: z.string(),
    status: z.string(),
    metrics: z.record(z.string()),
    dependencies: z.array(Dependency),
  })
  .partial()
  .passthrough();
const ResourceUtilization = z
  .object({
    cpuUsage: z.number(),
    memoryUsage: z.number(),
    activeConnections: z.number().int(),
    quotas: z.record(z.number()),
  })
  .partial()
  .passthrough();
const OperationDetails = z
  .object({
    operationId: z.string(),
    startTime: z.string().datetime({ offset: true }),
    endTime: z.string().datetime({ offset: true }),
    stackTrace: z.array(z.string()),
    parameters: z.record(z.string()),
  })
  .partial()
  .passthrough();
const InternalErrorMessageResponse = z
  .object({
    code: InternalErrorCode.default("NO_INTERNAL_ERROR"),
    message: z.string(),
    incidentId: z.string(),
    serviceStatus: ServiceStatus,
    resourceUtilization: ResourceUtilization,
    operationDetails: OperationDetails,
    errorResponse: ErrorResponse,
  })
  .partial()
  .passthrough();
const NotImplementedErrorMessageResponse = z
  .object({
    code: z.number().int(),
    message: z.string(),
    feature: z.string(),
    errorResponse: ErrorResponse,
  })
  .partial()
  .passthrough();
const BadGatewayErrorMessageResponse = z
  .object({
    code: z.number().int(),
    message: z.string(),
    upstreamService: z.string(),
    errorResponse: ErrorResponse,
  })
  .partial()
  .passthrough();
const AvailabilityInfo = z
  .object({
    retryAfter: z.number().int(),
    maintenanceWindow: z.string(),
    affectedServices: z.array(z.string()),
    statusPageUrl: z.string(),
  })
  .partial()
  .passthrough();
const ServiceUnavailableErrorMessageResponse = z
  .object({
    code: InternalErrorCode.default("NO_INTERNAL_ERROR"),
    message: z.string(),
    availabilityInfo: AvailabilityInfo,
    errorResponse: ErrorResponse,
  })
  .partial()
  .passthrough();
const GatewayTimeoutErrorMessageResponse = z
  .object({
    code: z.number().int(),
    message: z.string(),
    upstreamService: z.string(),
    errorResponse: ErrorResponse,
  })
  .partial()
  .passthrough();
const rpc_Status = z
  .object({
    code: z.number().int(),
    message: z.string(),
    details: z.array(Any),
  })
  .partial()
  .passthrough();
const CreateAccountRequest = z
  .object({
    account: Account,
    initialWorkspaceName: z.string(),
    organizationId: z.string(),
    tenantId: z.string(),
  })
  .partial()
  .passthrough();
const CreateAccountResponse = z
  .object({ account: Account, initialWorkspaceId: z.string() })
  .partial()
  .passthrough();
const ComplianceLevel = z.enum([
  "COMPLIANCE_LEVEL_UNSPECIFIED",
  "COMPLIANCE_LEVEL_NONE",
  "COMPLIANCE_LEVEL_BASIC",
  "COMPLIANCE_LEVEL_ADVANCED",
  "COMPLIANCE_LEVEL_ENTERPRISE",
]);
const FileEmbeddings = z
  .object({
    id: z.string(),
    embeddings: z.array(z.number()),
    modelVersion: z.string(),
    createdAt: z.string().datetime({ offset: true }),
    updatedAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const FileVersion = z
  .object({
    id: z.string(),
    fileId: z.string(),
    versionNumber: z.number().int(),
    s3Key: z.string(),
    size: z.string(),
    commitMessage: z.string(),
    authorId: z.string(),
    createdAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const CommentThread = z
  .object({
    id: z.string(),
    authorId: z.string(),
    content: z.string(),
    startPosition: z.number().int(),
    endPosition: z.number().int(),
    resolved: z.boolean(),
    createdAt: z.string().datetime({ offset: true }),
    updatedAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const FileSharing = z
  .object({
    id: z.string(),
    sharedWithEmail: z.string(),
    permissionLevel: z.string(),
    expiresAt: z.string().datetime({ offset: true }),
    createdAt: z.string().datetime({ offset: true }),
    updatedAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const DocumentSnapshot = z
  .object({
    id: z.string(),
    fileId: z.string(),
    snapshotHash: z.string(),
    content: z.string(),
    authorId: z.string(),
    reason: z.string(),
    createdAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const FileMetadata = z
  .object({
    id: z.string(),
    name: z.string(),
    size: z.string(),
    s3Key: z.string(),
    s3BucketName: z.string(),
    isDeleted: z.boolean(),
    version: z.number().int(),
    createdAt: z.string().datetime({ offset: true }),
    updatedAt: z.string().datetime({ offset: true }),
    deletedAt: z.string().datetime({ offset: true }),
    embeddings: FileEmbeddings,
    versions: z.array(FileVersion),
    comments: z.array(CommentThread),
    sharing: z.array(FileSharing),
    snapshots: z.array(DocumentSnapshot),
  })
  .partial()
  .passthrough();
const FolderMetadata: z.ZodType<FolderMetadata> = z.lazy(() =>
  z
    .object({
      id: z.string(),
      name: z.string(),
      s3BucketName: z.string(),
      s3FolderPath: z.string(),
      isDeleted: z.boolean(),
      parentFolderId: z.string(),
      createdAt: z.string().datetime({ offset: true }),
      updatedAt: z.string().datetime({ offset: true }),
      deletedAt: z.string().datetime({ offset: true }),
      childFolders: z.array(FolderMetadata),
      files: z.array(FileMetadata),
    })
    .partial()
    .passthrough(),
);
const TemplateType = z.enum([
  "TEMPLATE_TYPE_UNSPECIFIED",
  "TEMPLATE_TYPE_STANDARD",
  "TEMPLATE_TYPE_SMART",
  "TEMPLATE_TYPE_ADAPTIVE",
  "TEMPLATE_TYPE_AI_GENERATED",
]);
const TemplateVersion = z
  .object({
    id: z.string(),
    templateId: z.string(),
    version: z.string(),
    baseContent: z.string(),
    metadata: z.record(z.string()),
    authorId: z.string(),
    changeDescription: z.string(),
    createdAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const TemplateVariable = z
  .object({
    id: z.string(),
    name: z.string(),
    description: z.string(),
    variableType: z.string(),
    defaultValue: z.string(),
    isRequired: z.boolean(),
    validationRules: z.string(),
    dataSource: z.string(),
    aiExtractionRules: z.string(),
    alternatives: z.array(z.string()),
    templateId: z.string(),
  })
  .partial()
  .passthrough();
const DocumentStatus = z.enum([
  "DOCUMENT_STATUS_UNSPECIFIED",
  "DOCUMENT_STATUS_DRAFT",
  "DOCUMENT_STATUS_IN_REVIEW",
  "DOCUMENT_STATUS_APPROVED",
  "DOCUMENT_STATUS_REJECTED",
  "DOCUMENT_STATUS_EXPIRED",
  "DOCUMENT_STATUS_ARCHIVED",
]);
const SignatureStatus = z.enum([
  "SIGNATURE_STATUS_UNSPECIFIED",
  "SIGNATURE_STATUS_PENDING",
  "SIGNATURE_STATUS_SIGNED",
  "SIGNATURE_STATUS_REJECTED",
  "SIGNATURE_STATUS_EXPIRED",
  "SIGNATURE_STATUS_REVOKED",
]);
const SignatureBlock = z
  .object({
    id: z.string(),
    requestId: z.string(),
    blockType: z.string(),
    pageNumber: z.number().int(),
    xPosition: z.number(),
    yPosition: z.number(),
    width: z.number(),
    height: z.number(),
    isRequired: z.boolean(),
    signatureData: z.string(),
    signedAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const SignatureWorkflow = z
  .object({
    id: z.string(),
    requestId: z.string(),
    signingOrder: z.array(z.string()),
    currentSigner: z.string(),
    workflowStatus: z.string(),
    parallelSigning: z.boolean(),
    requireAllSignatures: z.boolean(),
    deadline: z.string().datetime({ offset: true }),
    createdAt: z.string().datetime({ offset: true }),
    updatedAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const SignatureRequest = z
  .object({
    id: z.string(),
    status: SignatureStatus.default("SIGNATURE_STATUS_UNSPECIFIED"),
    signerEmail: z.string(),
    signerName: z.string(),
    role: z.string(),
    authenticationMethod: z.string(),
    expiresAt: z.string().datetime({ offset: true }),
    requiresMfa: z.boolean(),
    signatureType: z.string(),
    reminderSchedule: z.array(z.string()),
    documentId: z.string(),
    signatureBlocks: z.array(SignatureBlock),
    workflow: SignatureWorkflow,
  })
  .partial()
  .passthrough();
const AIAssistanceLog = z
  .object({
    id: z.string(),
    documentId: z.string(),
    interactionType: z.string(),
    userQuery: z.string(),
    aiResponse: z.string(),
    context: z.record(z.string()),
    createdAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const NegotiationRound = z
  .object({
    id: z.string(),
    historyId: z.string(),
    roundNumber: z.number().int(),
    proposerId: z.string(),
    proposalContent: z.string(),
    responseContent: z.string(),
    status: z.string(),
    createdAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const NegotiationHistory = z
  .object({
    id: z.string(),
    documentId: z.string(),
    rounds: z.array(NegotiationRound),
    currentStatus: z.string(),
    startedAt: z.string().datetime({ offset: true }),
    lastUpdated: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const BranchMerge = z
  .object({
    id: z.string(),
    branchId: z.string(),
    sourceBranch: z.string(),
    targetBranch: z.string(),
    mergeStatus: z.string(),
    mergeStrategy: z.string(),
    conflicts: z.array(z.string()),
    mergerId: z.string(),
    createdAt: z.string().datetime({ offset: true }),
    completedAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const BranchPolicy = z
  .object({
    id: z.string(),
    branchId: z.string(),
    requiredApprovers: z.array(z.string()),
    minimumApprovals: z.number().int(),
    enforceLinearHistory: z.boolean(),
    allowForcePush: z.boolean(),
    protectedPaths: z.array(z.string()),
    mergeRules: z.record(z.string()),
    automatedChecks: z.array(z.string()),
    createdAt: z.string().datetime({ offset: true }),
    updatedAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const MergeRequest = z
  .object({
    id: z.string(),
    sourceBranchId: z.string(),
    targetBranchId: z.string(),
    title: z.string(),
    description: z.string(),
    authorId: z.string(),
    reviewers: z.array(z.string()),
    status: z.string(),
    labels: z.array(z.string()),
    hasConflicts: z.boolean(),
    automatedCheckResults: z.array(z.string()),
    createdAt: z.string().datetime({ offset: true }),
    updatedAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const DocumentBranch = z
  .object({
    id: z.string(),
    name: z.string(),
    baseVersionHash: z.string(),
    currentHeadHash: z.string(),
    purpose: z.string(),
    status: z.string(),
    reviewers: z.array(z.string()),
    branchMetadata: z.record(z.string()),
    isLocked: z.boolean(),
    lastUpdated: z.string().datetime({ offset: true }),
    merges: z.array(BranchMerge),
    policy: BranchPolicy,
    mergeRequests: z.array(MergeRequest),
  })
  .partial()
  .passthrough();
const ChangeSet = z
  .object({
    id: z.string(),
    versionId: z.string(),
    changeType: z.string(),
    contentBefore: z.string(),
    contentAfter: z.string(),
    metadata: z.record(z.string()),
    createdAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const ContextualSummary = z
  .object({
    id: z.string(),
    versionId: z.string(),
    summaryType: z.string(),
    content: z.string(),
    targetAudience: z.string(),
    keyPoints: z.array(z.string()),
    metadata: z.record(z.string()),
    createdAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const DocumentVersion = z
  .object({
    id: z.string(),
    versionHash: z.string(),
    parentHash: z.string(),
    commitMessage: z.string(),
    authorId: z.string(),
    branchName: z.string(),
    metadata: z.record(z.string()),
    tags: z.array(z.string()),
    isApproved: z.boolean(),
    approvalChain: z.string(),
    createdAt: z.string().datetime({ offset: true }),
    branches: z.array(DocumentBranch),
    changes: z.array(ChangeSet),
    summaries: z.array(ContextualSummary),
  })
  .partial()
  .passthrough();
const DocumentInstance = z
  .object({
    id: z.string(),
    templateId: z.string(),
    status: DocumentStatus.default("DOCUMENT_STATUS_UNSPECIFIED"),
    fieldValues: z.record(z.string()),
    signers: z.array(z.string()),
    language: z.string(),
    isCompleted: z.boolean(),
    dueDate: z.string().datetime({ offset: true }),
    generatedContent: z.string(),
    attachments: z.array(z.string()),
    createdAt: z.string().datetime({ offset: true }),
    updatedAt: z.string().datetime({ offset: true }),
    signatureRequests: z.array(SignatureRequest),
    aiAssistance: z.array(AIAssistanceLog),
    negotiation: NegotiationHistory,
    versions: z.array(DocumentVersion),
  })
  .partial()
  .passthrough();
const ExplanationBlock = z
  .object({
    id: z.string(),
    templateId: z.string(),
    sectionIdentifier: z.string(),
    explanationText: z.string(),
    complexityLevel: z.string(),
    relatedTerms: z.array(z.string()),
    legalReferences: z.array(z.string()),
    createdAt: z.string().datetime({ offset: true }),
    updatedAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const RiskAssessment = z
  .object({
    id: z.string(),
    intelligenceId: z.string(),
    riskType: z.string(),
    riskScore: z.number(),
    assessmentDetails: z.string(),
    mitigationSuggestions: z.array(z.string()),
    assessorId: z.string(),
    createdAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const ComplianceCheck = z
  .object({
    id: z.string(),
    intelligenceId: z.string(),
    complianceStandard: z.string(),
    checkResult: z.string(),
    violations: z.array(z.string()),
    remediationSteps: z.string(),
    checkDate: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const ContractIntelligence = z
  .object({
    id: z.string(),
    contractType: z.string(),
    riskScores: z.record(z.number()),
    detectedClauses: z.record(z.string()),
    keyObligations: z.array(z.string()),
    complianceScores: z.record(z.number()),
    jurisdiction: z.string(),
    governingLaws: z.array(z.string()),
    semanticAnalysis: z.record(z.string()),
    analysisDate: z.string().datetime({ offset: true }),
    riskAssessments: z.array(RiskAssessment),
    complianceChecks: z.array(ComplianceCheck),
  })
  .partial()
  .passthrough();
const DocumentTemplate = z
  .object({
    id: z.string(),
    name: z.string(),
    description: z.string(),
    industryType: z.string(),
    documentType: z.string(),
    baseContent: z.string(),
    isAdaptive: z.boolean(),
    metadata: z.record(z.string()),
    supportedLanguages: z.array(z.string()),
    requiredDataFields: z.array(z.string()),
    version: z.string(),
    createdAt: z.string().datetime({ offset: true }),
    updatedAt: z.string().datetime({ offset: true }),
    templateType: TemplateType.default("TEMPLATE_TYPE_UNSPECIFIED"),
    content: z.string(),
    detectedVariables: z.array(z.string()),
    variableMappings: z.record(z.string()),
    isContextAware: z.boolean(),
    supportedContexts: z.array(z.string()),
    formattingRules: z.record(z.string()),
    conditionalSections: z.array(z.string()),
    versions: z.array(TemplateVersion),
    variables: z.array(TemplateVariable),
    instances: z.array(DocumentInstance),
    explanations: z.array(ExplanationBlock),
    intelligence: z.array(ContractIntelligence),
  })
  .partial()
  .passthrough();
const WorkspaceSharing = z
  .object({
    id: z.string(),
    sharedWithEmail: z.string(),
    permissionLevel: z.string(),
    expiresAt: z.string().datetime({ offset: true }),
    createdAt: z.string().datetime({ offset: true }),
    updatedAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const WorkspaceActivity = z
  .object({
    id: z.string(),
    activityType: z.string(),
    userId: z.string(),
    description: z.string(),
    metadata: z.record(z.string()),
    createdAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const WorkspaceCompliance = z
  .object({
    id: z.string(),
    complianceType: z.string(),
    status: z.string(),
    certificationId: z.string(),
    validUntil: z.string().datetime({ offset: true }),
    createdAt: z.string().datetime({ offset: true }),
    updatedAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const AppCategory = z.enum([
  "APP_CATEGORY_UNSPECIFIED",
  "APP_CATEGORY_CONTRACT_AUTOMATION",
  "APP_CATEGORY_CONTRACT_ANALYSIS",
  "APP_CATEGORY_INTEGRATION",
  "APP_CATEGORY_AI_POWERED",
  "APP_CATEGORY_WORKFLOW",
  "APP_CATEGORY_COMPLIANCE",
]);
const PricingModel = z.enum([
  "PRICING_MODEL_UNSPECIFIED",
  "PRICING_MODEL_FREE",
  "PRICING_MODEL_SUBSCRIPTION",
  "PRICING_MODEL_USAGE_BASED",
  "PRICING_MODEL_ENTERPRISE",
]);
const AppVersion = z
  .object({
    id: z.string(),
    appId: z.string(),
    versionNumber: z.string(),
    changelog: z.string(),
    requirements: z.record(z.string()),
    isPublic: z.boolean(),
    releaseDate: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const AppInstallation = z
  .object({
    id: z.string(),
    appId: z.string(),
    workspaceId: z.string(),
    versionInstalled: z.string(),
    configuration: z.record(z.string()),
    installationStatus: z.string(),
    installedAt: z.string().datetime({ offset: true }),
    lastUsed: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const AppAnalytics = z
  .object({
    id: z.string(),
    appId: z.string(),
    metricName: z.string(),
    metricValue: z.number(),
    dimensions: z.record(z.string()),
    recordedAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const AppReview = z
  .object({
    id: z.string(),
    appId: z.string(),
    reviewerId: z.string(),
    rating: z.number(),
    reviewText: z.string(),
    createdAt: z.string().datetime({ offset: true }),
    updatedAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const AppDevelopmentInfo = z
  .object({
    id: z.string(),
    appId: z.string(),
    supportedLanguages: z.array(z.string()),
    sdkFeatures: z.array(z.string()),
    testCoverage: z.record(z.number()),
    securityScans: z.array(z.string()),
    performanceMetrics: z.record(z.string()),
    developmentStatus: z.string(),
    knownIssues: z.array(z.string()),
    roadmapUrl: z.string(),
    lastSecurityAudit: z.string().datetime({ offset: true }),
    createdAt: z.string().datetime({ offset: true }),
    updatedAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const AppWebhook = z
  .object({
    id: z.string(),
    appId: z.string(),
    url: z.string(),
    subscribedEvents: z.array(z.string()),
    secretKey: z.string(),
    retryCount: z.number().int(),
    timeoutSeconds: z.number().int(),
    isActive: z.boolean(),
    headers: z.record(z.string()),
    lastTriggered: z.string().datetime({ offset: true }),
    createdAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const AppPermission = z
  .object({
    id: z.string(),
    appId: z.string(),
    scope: z.string(),
    description: z.string(),
    isRequired: z.boolean(),
    accessLevels: z.array(z.string()),
    constraints: z.record(z.string()),
    requiresApproval: z.boolean(),
    createdAt: z.string().datetime({ offset: true }),
    updatedAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const MarketplaceApp = z
  .object({
    id: z.string(),
    name: z.string(),
    description: z.string(),
    developerId: z.string(),
    category: AppCategory.default("APP_CATEGORY_UNSPECIFIED"),
    supportedContractTypes: z.array(z.string()),
    features: z.array(z.string()),
    pricingModel: PricingModel.default("PRICING_MODEL_UNSPECIFIED"),
    pricingTiers: z.record(z.number()),
    supportedIntegrations: z.array(z.string()),
    isVerified: z.boolean(),
    rating: z.number(),
    installationCount: z.number().int(),
    requiredPermissions: z.array(z.string()),
    metadata: z.record(z.string()),
    documentationUrl: z.string(),
    supportEmail: z.string(),
    screenshots: z.array(z.string()),
    privacyPolicyUrl: z.string(),
    termsUrl: z.string(),
    publishedAt: z.string().datetime({ offset: true }),
    lastUpdated: z.string().datetime({ offset: true }),
    versions: z.array(AppVersion),
    installations: z.array(AppInstallation),
    analytics: z.array(AppAnalytics),
    reviews: z.array(AppReview),
    info: AppDevelopmentInfo,
    webhooks: z.array(AppWebhook),
    permissions: z.array(AppPermission),
  })
  .partial()
  .passthrough();
const Workspace1 = z
  .object({
    id: z.string(),
    name: z.string(),
    uniqueIdentifier: z.string(),
    s3BucketName: z.string(),
    s3FolderPath: z.string(),
    storageQuota: z.string(),
    usedStorage: z.string(),
    allowPublicSharing: z.boolean(),
    requireApproval: z.boolean(),
    gdprCompliant: z.boolean(),
    hipaaCompliant: z.boolean(),
    createdAt: z.string().datetime({ offset: true }),
    updatedAt: z.string().datetime({ offset: true }),
    deletedAt: z.string().datetime({ offset: true }),
    folders: z.array(FolderMetadata),
    templates: z.array(DocumentTemplate),
    sharing: z.array(WorkspaceSharing),
    activities: z.array(WorkspaceActivity),
    compliance: z.array(WorkspaceCompliance),
    installedApps: z.array(MarketplaceApp),
  })
  .partial()
  .passthrough();
const DataProfile = z
  .object({
    id: z.string(),
    name: z.string(),
    profileType: z.string(),
    dataFields: z.record(z.string()),
    isDefault: z.boolean(),
    createdAt: z.string().datetime({ offset: true }),
    updatedAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const Account1 = z
  .object({
    id: z.string(),
    auth0UserId: z.string(),
    email: z.string(),
    baseDirectory: z.string(),
    bucketName: z.string(),
    region: z.string(),
    orgId: z.string(),
    tenantId: z.string(),
    roles: z.array(z.string()),
    permissions: z.array(z.string()),
    mfaEnabled: z.boolean(),
    complianceLevel: ComplianceLevel.default("COMPLIANCE_LEVEL_UNSPECIFIED"),
    preferences: z.record(z.string()),
    apiKeys: z.array(z.string()),
    createdAt: z.string().datetime({ offset: true }),
    updatedAt: z.string().datetime({ offset: true }),
    deletedAt: z.string().datetime({ offset: true }),
    workspaces: z.array(Workspace1),
    dataProfiles: z.array(DataProfile),
  })
  .partial()
  .passthrough();
const UpdateAccountRequest = z
  .object({ account: Account1 })
  .partial()
  .passthrough();
const UpdateAccountResponse = z
  .object({ account: Account1 })
  .partial()
  .passthrough();
const UpdateAccountSettingsRequest = z
  .object({ settings: AccountSettings })
  .partial()
  .passthrough();
const UpdateAccountSettingsResponse = z
  .object({ settings: AccountSettings })
  .partial()
  .passthrough();
const GetAccountResponse = z
  .object({ account: Account })
  .partial()
  .passthrough();
const DeleteAccountResponse = z
  .object({ success: z.boolean() })
  .partial()
  .passthrough();
const GetAccountUsageResponse = z
  .object({
    totalJobsRun: z.number().int(),
    monthlyJobLimit: z.number().int(),
    remainingJobs: z.number().int(),
    resetTime: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const CreateAPIKeyRequest = z
  .object({
    organizationId: z.string(),
    workspaceId: z.string(),
    tenantId: z.string(),
    accountId: z.string(),
    name: z.string(),
    description: z.string(),
    scopes: z.array(z.string()),
    expiresAt: z.string().datetime({ offset: true }),
    maxUses: z.number().int(),
    allowedIps: z.array(z.string()),
    rateLimit: z.number().int(),
    enforceSigning: z.boolean(),
    allowedSignatureAlgorithms: z.array(z.string()),
    enforceMutualTls: z.boolean(),
    alertEmails: z.array(z.string()),
    alertOnQuotaThreshold: z.boolean(),
    quotaAlertThreshold: z.number(),
  })
  .partial()
  .passthrough();
const CreateAPIKeyResponse = z
  .object({ apiKey: APIKey, keyValue: z.string() })
  .partial()
  .passthrough();
const UpdateAPIKeyRequest = z
  .object({ apiKey: APIKey })
  .partial()
  .passthrough();
const UpdateAPIKeyResponse = z
  .object({ apiKey: APIKey })
  .partial()
  .passthrough();
const ListAPIKeysResponse = z
  .object({
    apiKeys: z.array(APIKey),
    nextPageNumber: z.number().int(),
    totalCount: z.number().int(),
  })
  .partial()
  .passthrough();
const RotateAPIKeyRequest = z
  .object({
    organizationId: z.string(),
    tenantId: z.string(),
    accountId: z.string(),
    workspaceId: z.string(),
    keyId: z.string(),
  })
  .partial()
  .passthrough();
const RotateAPIKeyResponse = z
  .object({
    newApiKey: APIKey,
    newKeyValue: z.string(),
    oldKeyExpiry: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const GetAPIKeyResponse = z.object({ apiKey: APIKey }).partial().passthrough();
const DeleteAPIKeyResponse = z
  .object({ success: z.boolean() })
  .partial()
  .passthrough();
const ListScrapingJobsResponse = z
  .object({ jobs: z.array(ScrapingJob) })
  .partial()
  .passthrough();
const CreateScrapingJobRequest = z
  .object({
    authPlatformUserId: z.string(),
    orgId: z.string(),
    tenantId: z.string(),
    name: z.string().optional(),
    keywords: z.array(z.string()).optional(),
    lang: z.string().optional(),
    zoom: z.number().int().optional(),
    lat: z.string().optional(),
    lon: z.string().optional(),
    fastMode: z.boolean().optional(),
    radius: z.number().int().optional(),
    depth: z.number().int().optional(),
    email: z.boolean().optional(),
    maxTime: z.number().int().optional(),
    proxies: z.array(z.string()).optional(),
  })
  .passthrough();
const CreateScrapingJobResponse = z
  .object({
    jobId: z.string(),
    status: BackgroundJobStatus.default("BACKGROUND_JOB_STATUS_UNSPECIFIED"),
  })
  .partial()
  .passthrough();
const GetScrapingJobResponse = z
  .object({ job: ScrapingJob })
  .partial()
  .passthrough();
const DeleteScrapingJobResponse = z
  .object({ success: z.boolean() })
  .partial()
  .passthrough();
const DownloadScrapingResultsResponse = z
  .object({
    content: z.string(),
    filename: z.string(),
    contentType: z.string(),
  })
  .partial()
  .passthrough();
const ListLeadsResponse = z
  .object({
    leads: z.array(Lead),
    totalCount: z.number().int(),
    nextPageNumber: z.number().int(),
  })
  .partial()
  .passthrough();
const GetLeadResponse = z.object({ lead: Lead }).partial().passthrough();
const BillingPlan = z.enum([
  "BILLING_PLAN_UNSPECIFIED",
  "BILLING_PLAN_STARTUP",
  "BILLING_PLAN_BUSINESS",
  "BILLING_PLAN_ENTERPRISE",
]);
const TenantAPIKeyScope = z.enum([
  "TENANT_API_KEY_SCOPE_UNSPECIFIED",
  "TENANT_API_KEY_SCOPE_READ_JOBS",
  "TENANT_API_KEY_SCOPE_READ_LEADS",
  "TENANT_API_KEY_SCOPE_READ_WORKFLOWS",
  "TENANT_API_KEY_SCOPE_READ_ANALYTICS",
  "TENANT_API_KEY_SCOPE_READ_SETTINGS",
  "TENANT_API_KEY_SCOPE_WRITE_JOBS",
  "TENANT_API_KEY_SCOPE_WRITE_LEADS",
  "TENANT_API_KEY_SCOPE_WRITE_WORKFLOWS",
  "TENANT_API_KEY_SCOPE_WRITE_SETTINGS",
  "TENANT_API_KEY_SCOPE_DELETE_JOBS",
  "TENANT_API_KEY_SCOPE_DELETE_LEADS",
  "TENANT_API_KEY_SCOPE_DELETE_WORKFLOWS",
  "TENANT_API_KEY_SCOPE_EXPORT_DATA",
  "TENANT_API_KEY_SCOPE_MANAGE_KEYS",
  "TENANT_API_KEY_SCOPE_BILLING_READ",
  "TENANT_API_KEY_SCOPE_BILLING_WRITE",
  "TENANT_API_KEY_SCOPE_ADMIN",
]);
const TenantAPIKey = z
  .object({
    id: z.string(),
    keyHash: z.string(),
    keyPrefix: z.string(),
    name: z.string(),
    description: z.string(),
    status: v1_Status.default("STATUS_UNSPECIFIED"),
    scopes: z.array(TenantAPIKeyScope),
    maxUses: z.number().int(),
    allowedIps: z.array(z.string()),
    useCount: z.number().int(),
    expiresAt: z.string().datetime({ offset: true }),
    createdAt: z.string().datetime({ offset: true }),
    updatedAt: z.string().datetime({ offset: true }),
    deletedAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const Tenant: z.ZodType<Tenant> = z.lazy(() =>
  z
    .object({
      id: z.string(),
      name: z.string(),
      displayName: z.string(),
      description: z.string(),
      organization: Organization,
      apiBaseUrl: z.string(),
      environmentVariables: z.record(z.string()),
      allowedOrigins: z.array(z.string()),
      storageQuota: z.string(),
      monthlyRequestLimit: z.string(),
      maxConcurrentJobs: z.number().int(),
      enableCaching: z.boolean(),
      enableRateLimiting: z.boolean(),
      enableRequestLogging: z.boolean(),
      accounts: z.array(Account),
      apiKeys: z.array(TenantAPIKey),
      totalRequests: z.string(),
      totalStorageUsed: z.string(),
      averageResponseTime: z.number(),
      createdAt: z.string().datetime({ offset: true }),
      updatedAt: z.string().datetime({ offset: true }),
      deletedAt: z.string().datetime({ offset: true }),
      status: v1_Status.default("STATUS_UNSPECIFIED"),
    })
    .partial()
    .passthrough(),
);
const PlanTier = z.enum([
  "PLAN_TIER_UNSPECIFIED",
  "PLAN_TIER_FREE",
  "PLAN_TIER_STARTER",
  "PLAN_TIER_PROFESSIONAL",
  "PLAN_TIER_ENTERPRISE",
]);
const BillingMode = z.enum([
  "BILLING_MODE_UNSPECIFIED",
  "BILLING_MODE_LICENSED",
  "BILLING_MODE_METERED",
  "BILLING_MODE_HYBRID",
]);
const Interval = z.enum([
  "INTERVAL_UNSPECIFIED",
  "INTERVAL_MONTHLY",
  "INTERVAL_YEARLY",
]);
const PaymentStatus = z.enum([
  "PAYMENT_STATUS_UNSPECIFIED",
  "PAYMENT_STATUS_PAID",
  "PAYMENT_STATUS_PAST_DUE",
  "PAYMENT_STATUS_FAILED",
  "PAYMENT_STATUS_CANCELED",
]);
const Subscription = z
  .object({
    id: z.string(),
    stripeCustomerId: z.string(),
    stripeSubscriptionId: z.string(),
    stripePriceId: z.string(),
    stripeProductId: z.string(),
    planTier: PlanTier.default("PLAN_TIER_UNSPECIFIED"),
    billingMode: BillingMode.default("BILLING_MODE_UNSPECIFIED"),
    includedJobs: z.number().int(),
    perJobRate: z.number(),
    maxConcurrentJobs: z.number().int(),
    includedStorage: z.string(),
    perGbRate: z.number(),
    advancedFilteringEnabled: z.boolean(),
    prioritySupportEnabled: z.boolean(),
    customExportsEnabled: z.boolean(),
    apiAccessEnabled: z.boolean(),
    customProxiesEnabled: z.boolean(),
    advancedAnalyticsEnabled: z.boolean(),
    retentionDays: z.number().int(),
    maxResultsPerJob: z.number().int(),
    currentPeriodStart: z.string().datetime({ offset: true }),
    currentPeriodEnd: z.string().datetime({ offset: true }),
    currentPeriodUsage: z.number(),
    currency: z.string(),
    basePrice: z.number(),
    billingInterval: Interval.default("INTERVAL_UNSPECIFIED"),
    autoRenew: z.boolean(),
    paymentStatus: PaymentStatus.default("PAYMENT_STATUS_UNSPECIFIED"),
    isTrial: z.boolean(),
    trialStart: z.string().datetime({ offset: true }),
    trialEnd: z.string().datetime({ offset: true }),
    createdAt: z.string().datetime({ offset: true }),
    updatedAt: z.string().datetime({ offset: true }),
    canceledAt: z.string().datetime({ offset: true }),
    endedAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const Organization: z.ZodType<Organization> = z.lazy(() =>
  z
    .object({
      id: z.string(),
      name: z.string(),
      displayName: z.string(),
      website: z.string(),
      description: z.string(),
      billingEmail: z.string(),
      technicalEmail: z.string(),
      phone: z.string(),
      address: z.string(),
      ssoEnabled: z.boolean(),
      ssoProvider: z.string(),
      ssoDomain: z.string(),
      allowedDomains: z.array(z.string()),
      enforce2fa: z.boolean(),
      billingPlan: BillingPlan.default("BILLING_PLAN_UNSPECIFIED"),
      billingCurrency: z.string(),
      autoBilling: z.boolean(),
      taxId: z.string(),
      complianceFrameworks: z.array(z.string()),
      dataProcessingAgreement: z.boolean(),
      dataRegion: z.string(),
      maxTenants: z.number().int(),
      totalStorageLimit: z.string(),
      maxApiKeys: z.number().int(),
      maxUsers: z.number().int(),
      tenants: z.array(Tenant),
      subscriptions: Subscription,
      createdAt: z.string().datetime({ offset: true }),
      updatedAt: z.string().datetime({ offset: true }),
      deletedAt: z.string().datetime({ offset: true }),
      createdBy: z.string(),
      status: v1_Status.default("STATUS_UNSPECIFIED"),
    })
    .partial()
    .passthrough(),
);
const ListOrganizationsResponse = z
  .object({
    organizations: z.array(Organization),
    totalCount: z.number().int(),
    nextPageNumber: z.number().int(),
  })
  .partial()
  .passthrough();
const CreateOrganizationRequest = z
  .object({ organization: Organization })
  .partial()
  .passthrough();
const CreateOrganizationResponse = z
  .object({ organization: Organization })
  .partial()
  .passthrough();
const UpdateOrganizationRequest = z
  .object({ organization: Organization })
  .partial()
  .passthrough();
const UpdateOrganizationResponse = z
  .object({ organization: Organization })
  .partial()
  .passthrough();
const ListTenantsResponse = z
  .object({ tenants: z.array(Tenant), nextPageNumber: z.number().int() })
  .partial()
  .passthrough();
const DeleteTenantResponse = z
  .object({ success: z.boolean() })
  .partial()
  .passthrough();
const GetOrganizationResponse = z
  .object({ organization: Organization })
  .partial()
  .passthrough();
const DeleteOrganizationResponse = z
  .object({ success: z.boolean() })
  .partial()
  .passthrough();
const UpdateTenantRequest = z
  .object({ tenant: Tenant })
  .partial()
  .passthrough();
const UpdateTenantResponse = z
  .object({ tenant: Tenant })
  .partial()
  .passthrough();
const CreateTenantAPIKeyRequest = z
  .object({
    organizationId: z.string(),
    tenantId: z.string(),
    apiKey: TenantAPIKey,
  })
  .partial()
  .passthrough();
const CreateTenantAPIKeyResponse = z
  .object({ keyId: z.string(), keyValue: z.string() })
  .partial()
  .passthrough();
const UpdateTenantAPIKeyRequest = z
  .object({ apiKey: TenantAPIKey })
  .partial()
  .passthrough();
const UpdateTenantAPIKeyResponse = z
  .object({ apiKey: TenantAPIKey })
  .partial()
  .passthrough();
const ListTenantAPIKeysResponse = z
  .object({
    apiKeys: z.array(TenantAPIKey),
    nextPageNumber: z.number().int(),
    totalCount: z.number().int(),
  })
  .partial()
  .passthrough();
const RotateTenantAPIKeyRequest = z
  .object({
    organizationId: z.string(),
    tenantId: z.string(),
    keyId: z.string(),
  })
  .partial()
  .passthrough();
const RotateTenantAPIKeyResponse = z
  .object({
    newApiKey: TenantAPIKey,
    newKeyValue: z.string(),
    oldKeyExpiry: z.string().datetime({ offset: true }),
    tenantId: z.string(),
    organizationId: z.string(),
  })
  .partial()
  .passthrough();
const GetTenantAPIKeyResponse = z
  .object({ apiKey: TenantAPIKey })
  .partial()
  .passthrough();
const DeleteTenantAPIKeyResponse = z
  .object({ success: z.boolean() })
  .partial()
  .passthrough();
const GetTenantResponse = z.object({ tenant: Tenant }).partial().passthrough();
const CreateTenantBody = z.object({ tenant: Tenant }).partial().passthrough();
const CreateTenantResponse = z
  .object({ tenantId: z.string() })
  .partial()
  .passthrough();
const ListWebhooksResponse = z
  .object({
    webhooks: z.array(WebhookConfig),
    nextPageNumber: z.number().int(),
    totalCount: z.number().int(),
  })
  .partial()
  .passthrough();
const CreateWebhookRequest = z
  .object({
    organizationId: z.string(),
    workspaceId: z.string(),
    tenantId: z.string(),
    accountId: z.string(),
    webhook: WebhookConfig,
  })
  .partial()
  .passthrough();
const CreateWebhookResponse = z
  .object({ webhook: WebhookConfig })
  .partial()
  .passthrough();
const UpdateWebhookRequest = z
  .object({ webhook: WebhookConfig })
  .partial()
  .passthrough();
const UpdateWebhookResponse = z
  .object({ webhook: WebhookConfig })
  .partial()
  .passthrough();
const GetWebhookResponse = z
  .object({ webhook: WebhookConfig })
  .partial()
  .passthrough();
const DeleteWebhookResponse = z
  .object({ success: z.boolean() })
  .partial()
  .passthrough();
const UpdateWorkspaceRequest = z
  .object({ workspace: Workspace1 })
  .partial()
  .passthrough();
const UpdateWorkspaceResponse = z
  .object({ workspace: Workspace1 })
  .partial()
  .passthrough();
const GetWorkspaceResponse = z
  .object({ workspace: Workspace1 })
  .partial()
  .passthrough();
const DeleteWorkspaceResponse = z
  .object({ success: z.boolean() })
  .partial()
  .passthrough();
const ListWorkspacesResponse = z
  .object({ workspaces: z.array(Workspace), nextPageNumber: z.number().int() })
  .partial()
  .passthrough();
const CreateWorkspaceRequest = z
  .object({ workspace: Workspace, accountId: z.string() })
  .partial()
  .passthrough();
const CreateWorkspaceResponse = z
  .object({ workspace: Workspace })
  .partial()
  .passthrough();
const UpdateWorkflowRequest = z
  .object({ workflow: ScrapingWorkflow })
  .partial()
  .passthrough();
const UpdateWorkflowResponse = z
  .object({ workflow: ScrapingWorkflow })
  .partial()
  .passthrough();
const JobSuccessRate = z
  .object({
    workflowId: z.string(),
    successRate: z.number(),
    totalRuns: z.number().int(),
  })
  .partial()
  .passthrough();
const GetWorkspaceAnalyticsResponse = z
  .object({
    totalLeads: z.number().int(),
    activeWorkflows: z.number().int(),
    jobsLast30Days: z.number().int(),
    successRates: z.array(JobSuccessRate),
  })
  .partial()
  .passthrough();
const ListWorkflowsResponse = z
  .object({
    workflows: z.array(ScrapingWorkflow),
    nextPageNumber: z.number().int(),
  })
  .partial()
  .passthrough();
const CreateWorkflowBody = z
  .object({ workflow: ScrapingWorkflow })
  .partial()
  .passthrough();
const CreateWorkflowResponse = z
  .object({ workflow: ScrapingWorkflow })
  .partial()
  .passthrough();
const GetWorkflowResponse = z
  .object({ workflow: ScrapingWorkflow })
  .partial()
  .passthrough();
const DeleteWorkflowResponse = z
  .object({ success: z.boolean() })
  .partial()
  .passthrough();
const PauseWorkflowBody = z
  .object({ pause: z.boolean() })
  .partial()
  .passthrough();
const PauseWorkflowResponse = z
  .object({ success: z.boolean(), workflow: ScrapingWorkflow })
  .partial()
  .passthrough();
const TriggerWorkflowBody = z
  .object({ parameters: z.record(z.string()) })
  .partial()
  .passthrough();
const TriggerWorkflowResponse = z
  .object({
    jobId: z.string(),
    status: BackgroundJobStatus.default("BACKGROUND_JOB_STATUS_UNSPECIFIED"),
  })
  .partial()
  .passthrough();
const CreateAccountRequest1 = z
  .object({
    auth0UserId: z.string(),
    email: z.string(),
    baseDirectory: z.string().optional(),
    region: z.string().optional(),
    orgId: z.string().optional(),
    tenantId: z.string().optional(),
    roles: z.array(z.string()).optional(),
    permissions: z.array(z.string()).optional(),
    mfaEnabled: z.boolean().optional(),
    complianceLevel: ComplianceLevel.optional().default(
      "COMPLIANCE_LEVEL_UNSPECIFIED",
    ),
    preferences: z.record(z.string()).optional(),
  })
  .passthrough();
const CreateAccountResponse1 = z
  .object({ account: Account1, initialWorkspaceName: z.string() })
  .partial()
  .passthrough();
const AuthenticationErrorMessageResponse1 = z
  .object({
    code: AuthErrorCode.default("NO_AUTH_ERROR"),
    message: z.string(),
    tokenInfo: TokenInfo,
    authContext: AuthContext,
    sessionInfo: SessionInfo,
    mfaInfo: MFAInfo,
    errorResponse: ErrorResponse,
  })
  .partial()
  .passthrough();
const GetAccountResponse1 = z
  .object({ account: Account1 })
  .partial()
  .passthrough();
const UpdateWorkspaceSharingRequest = z
  .object({ sharing: WorkspaceSharing })
  .partial()
  .passthrough();
const UpdateWorkspaceSharingResponse = z
  .object({ sharing: WorkspaceSharing })
  .partial()
  .passthrough();
const RemoveWorkspaceSharingResponse = z
  .object({ success: z.boolean() })
  .partial()
  .passthrough();
const CreateWorkspaceRequest1 = z
  .object({
    accountId: z.string().optional(),
    name: z.string(),
    storageQuota: z.string().optional(),
    allowPublicSharing: z.boolean().optional(),
    requireApproval: z.boolean().optional(),
    gdprCompliant: z.boolean().optional(),
    hipaaCompliant: z.boolean().optional(),
  })
  .passthrough();
const CreateWorkspaceResponse1 = z
  .object({ workspace: Workspace1 })
  .partial()
  .passthrough();
const ActivityMetrics = z
  .object({
    totalFiles: z.number().int(),
    totalFolders: z.number().int(),
    activeUsers: z.number().int(),
    storageUsed: z.string(),
    storageUsagePercentage: z.number(),
  })
  .partial()
  .passthrough();
const UserActivity = z
  .object({
    userId: z.string(),
    email: z.string(),
    fileOperations: z.number().int(),
    commentsMade: z.number().int(),
    documentsProcessed: z.number().int(),
  })
  .partial()
  .passthrough();
const ComplianceMetrics = z
  .object({
    gdprComplianceScore: z.number(),
    hipaaComplianceScore: z.number(),
    pendingApprovals: z.number().int(),
    complianceViolations: z.number().int(),
  })
  .partial()
  .passthrough();
const GetWorkspaceAnalyticsResponse1 = z
  .object({
    activity: ActivityMetrics,
    userActivities: z.array(UserActivity),
    compliance: ComplianceMetrics,
    recentActivities: z.array(WorkspaceActivity),
  })
  .partial()
  .passthrough();
const ComplianceViolation = z
  .object({
    ruleId: z.string(),
    severity: z.string(),
    description: z.string(),
    affectedResources: z.array(z.string()),
    remediationSteps: z.string(),
  })
  .partial()
  .passthrough();
const ComplianceScore = z
  .object({
    category: z.string(),
    score: z.number(),
    status: z.string(),
    improvements: z.array(z.string()),
  })
  .partial()
  .passthrough();
const GetWorkspaceComplianceReportResponse = z
  .object({
    overallScore: z.number(),
    complianceStatus: z.string(),
    violations: z.array(ComplianceViolation),
    categoryScores: z.array(ComplianceScore),
    reportGeneratedAt: z.string().datetime({ offset: true }),
    certificationId: z.string(),
  })
  .partial()
  .passthrough();
const ListWorkspacesResponse1 = z
  .object({
    workspaces: z.array(Workspace1),
    nextPageToken: z.string(),
    totalSize: z.number().int(),
  })
  .partial()
  .passthrough();
const ListWorkspaceSharingsResponse = z
  .object({
    sharings: z.array(WorkspaceSharing),
    nextPageToken: z.string(),
    totalSize: z.number().int(),
  })
  .partial()
  .passthrough();
const StorageBreakdown = z
  .object({
    fileType: z.string(),
    size: z.string(),
    fileCount: z.number().int(),
  })
  .partial()
  .passthrough();
const GetWorkspaceStorageStatsResponse = z
  .object({
    totalStorageUsed: z.string(),
    storageQuota: z.string(),
    usagePercentage: z.number(),
    storageByType: z.array(StorageBreakdown),
    totalFiles: z.number().int(),
    totalFolders: z.number().int(),
    lastUpdated: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const ShareWorkspaceBody = z
  .object({
    sharedWithEmail: z.string(),
    permissionLevel: z.string(),
    expiresAt: z.string().datetime({ offset: true }),
  })
  .partial()
  .passthrough();
const ShareWorkspaceResponse = z
  .object({ sharing: WorkspaceSharing })
  .partial()
  .passthrough();

export const schemas = {
  AccountStatus,
  Role,
  Permission,
  Timezone,
  WorkflowStatus,
  BackgroundJobStatus,
  Language,
  DayOfWeek,
  BusinessHours,
  Review,
  RevenueRange,
  EmployeeBenefit,
  Lead,
  ScrapingJob,
  OutputFormat,
  ScrapingWorkflow,
  v1_Status,
  APIKey,
  TriggerEvent,
  IncludedField,
  PayloadFormat,
  WebhookConfig,
  Workspace,
  AccountSettings,
  Account,
  ListAccountsResponse,
  ValidationErrorCode,
  FieldViolation,
  SchemaValidation,
  ResourceValidation,
  Any,
  Status,
  ErrorResponse,
  ValidationErrorMessageResponse,
  AuthErrorCode,
  TokenInfo,
  AuthContext,
  SessionInfo,
  MFAInfo,
  AuthenticationErrorMessageResponse,
  PaymentInfo,
  PaymentRequiredErrorMessageResponse,
  ForbiddenErrorMessageResponse,
  NotFoundErrorCode,
  ResourceInfo,
  Suggestions,
  APIInfo,
  NotFoundErrorMessageResponse,
  MethodNotAllowedErrorMessageResponse,
  ConflictInfo,
  ConflictErrorMessageResponse,
  GoneErrorMessageResponse,
  PreconditionFailedErrorMessageResponse,
  FieldError,
  UnprocessableEntityErrorMessageResponse,
  TooEarlyErrorMessageResponse,
  InternalErrorCode,
  LimitInfo,
  QuotaInfo,
  RateLimitContext,
  RateLimitErrorMessageResponse,
  Dependency,
  ServiceStatus,
  ResourceUtilization,
  OperationDetails,
  InternalErrorMessageResponse,
  NotImplementedErrorMessageResponse,
  BadGatewayErrorMessageResponse,
  AvailabilityInfo,
  ServiceUnavailableErrorMessageResponse,
  GatewayTimeoutErrorMessageResponse,
  rpc_Status,
  CreateAccountRequest,
  CreateAccountResponse,
  ComplianceLevel,
  FileEmbeddings,
  FileVersion,
  CommentThread,
  FileSharing,
  DocumentSnapshot,
  FileMetadata,
  FolderMetadata,
  TemplateType,
  TemplateVersion,
  TemplateVariable,
  DocumentStatus,
  SignatureStatus,
  SignatureBlock,
  SignatureWorkflow,
  SignatureRequest,
  AIAssistanceLog,
  NegotiationRound,
  NegotiationHistory,
  BranchMerge,
  BranchPolicy,
  MergeRequest,
  DocumentBranch,
  ChangeSet,
  ContextualSummary,
  DocumentVersion,
  DocumentInstance,
  ExplanationBlock,
  RiskAssessment,
  ComplianceCheck,
  ContractIntelligence,
  DocumentTemplate,
  WorkspaceSharing,
  WorkspaceActivity,
  WorkspaceCompliance,
  AppCategory,
  PricingModel,
  AppVersion,
  AppInstallation,
  AppAnalytics,
  AppReview,
  AppDevelopmentInfo,
  AppWebhook,
  AppPermission,
  MarketplaceApp,
  Workspace1,
  DataProfile,
  Account1,
  UpdateAccountRequest,
  UpdateAccountResponse,
  UpdateAccountSettingsRequest,
  UpdateAccountSettingsResponse,
  GetAccountResponse,
  DeleteAccountResponse,
  GetAccountUsageResponse,
  CreateAPIKeyRequest,
  CreateAPIKeyResponse,
  UpdateAPIKeyRequest,
  UpdateAPIKeyResponse,
  ListAPIKeysResponse,
  RotateAPIKeyRequest,
  RotateAPIKeyResponse,
  GetAPIKeyResponse,
  DeleteAPIKeyResponse,
  ListScrapingJobsResponse,
  CreateScrapingJobRequest,
  CreateScrapingJobResponse,
  GetScrapingJobResponse,
  DeleteScrapingJobResponse,
  DownloadScrapingResultsResponse,
  ListLeadsResponse,
  GetLeadResponse,
  BillingPlan,
  TenantAPIKeyScope,
  TenantAPIKey,
  Tenant,
  PlanTier,
  BillingMode,
  Interval,
  PaymentStatus,
  Subscription,
  Organization,
  ListOrganizationsResponse,
  CreateOrganizationRequest,
  CreateOrganizationResponse,
  UpdateOrganizationRequest,
  UpdateOrganizationResponse,
  ListTenantsResponse,
  DeleteTenantResponse,
  GetOrganizationResponse,
  DeleteOrganizationResponse,
  UpdateTenantRequest,
  UpdateTenantResponse,
  CreateTenantAPIKeyRequest,
  CreateTenantAPIKeyResponse,
  UpdateTenantAPIKeyRequest,
  UpdateTenantAPIKeyResponse,
  ListTenantAPIKeysResponse,
  RotateTenantAPIKeyRequest,
  RotateTenantAPIKeyResponse,
  GetTenantAPIKeyResponse,
  DeleteTenantAPIKeyResponse,
  GetTenantResponse,
  CreateTenantBody,
  CreateTenantResponse,
  ListWebhooksResponse,
  CreateWebhookRequest,
  CreateWebhookResponse,
  UpdateWebhookRequest,
  UpdateWebhookResponse,
  GetWebhookResponse,
  DeleteWebhookResponse,
  UpdateWorkspaceRequest,
  UpdateWorkspaceResponse,
  GetWorkspaceResponse,
  DeleteWorkspaceResponse,
  ListWorkspacesResponse,
  CreateWorkspaceRequest,
  CreateWorkspaceResponse,
  UpdateWorkflowRequest,
  UpdateWorkflowResponse,
  JobSuccessRate,
  GetWorkspaceAnalyticsResponse,
  ListWorkflowsResponse,
  CreateWorkflowBody,
  CreateWorkflowResponse,
  GetWorkflowResponse,
  DeleteWorkflowResponse,
  PauseWorkflowBody,
  PauseWorkflowResponse,
  TriggerWorkflowBody,
  TriggerWorkflowResponse,
  CreateAccountRequest1,
  CreateAccountResponse1,
  AuthenticationErrorMessageResponse1,
  GetAccountResponse1,
  UpdateWorkspaceSharingRequest,
  UpdateWorkspaceSharingResponse,
  RemoveWorkspaceSharingResponse,
  CreateWorkspaceRequest1,
  CreateWorkspaceResponse1,
  ActivityMetrics,
  UserActivity,
  ComplianceMetrics,
  GetWorkspaceAnalyticsResponse1,
  ComplianceViolation,
  ComplianceScore,
  GetWorkspaceComplianceReportResponse,
  ListWorkspacesResponse1,
  ListWorkspaceSharingsResponse,
  StorageBreakdown,
  GetWorkspaceStorageStatsResponse,
  ShareWorkspaceBody,
  ShareWorkspaceResponse,
};

const endpoints = makeApi([
  {
    method: "get",
    path: "/lead-scraper-microservice/api/v1/accounts",
    alias: "ListAccounts",
    description: `Retrieves a list of accounts`,
    requestFormat: "json",
    parameters: [
      {
        name: "pageSize",
        type: "Query",
        schema: z.number().int().optional(),
      },
      {
        name: "pageNumber",
        type: "Query",
        schema: z.number().int().optional(),
      },
      {
        name: "filter",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "organizationId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "tenantId",
        type: "Query",
        schema: z.string().optional(),
      },
    ],
    response: ListAccountsResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "post",
    path: "/lead-scraper-microservice/api/v1/accounts",
    alias: "CreateAccount",
    description: `Creates a new user account in the workspace service`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateAccountRequest,
      },
    ],
    response: CreateAccountResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "put",
    path: "/lead-scraper-microservice/api/v1/accounts",
    alias: "UpdateAccount",
    description: `Updates specified fields of an existing account`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: UpdateAccountRequest,
      },
    ],
    response: UpdateAccountResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "get",
    path: "/lead-scraper-microservice/api/v1/accounts/:id",
    alias: "GetAccount",
    description: `Retrieves details of a specific account`,
    requestFormat: "json",
    parameters: [
      {
        name: "id",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "organizationId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "tenantId",
        type: "Query",
        schema: z.string().optional(),
      },
    ],
    response: GetAccountResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "delete",
    path: "/lead-scraper-microservice/api/v1/accounts/:id",
    alias: "DeleteAccount",
    description: `Permanently deletes an account and associated resources`,
    requestFormat: "json",
    parameters: [
      {
        name: "id",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "organizationId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "tenantId",
        type: "Query",
        schema: z.string().optional(),
      },
    ],
    response: z.object({ success: z.boolean() }).partial().passthrough(),
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "get",
    path: "/lead-scraper-microservice/api/v1/accounts/:id/usage",
    alias: "GetAccountUsage",
    description: `Retrieves usage details for a given account`,
    requestFormat: "json",
    parameters: [
      {
        name: "id",
        type: "Path",
        schema: z.string(),
      },
    ],
    response: GetAccountUsageResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "put",
    path: "/lead-scraper-microservice/api/v1/accounts/settings",
    alias: "UpdateAccountSettings",
    description: `Updates the settings for a given account`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: UpdateAccountSettingsRequest,
      },
    ],
    response: UpdateAccountSettingsResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "post",
    path: "/lead-scraper-microservice/api/v1/api-keys",
    alias: "CreateAPIKey",
    description: `Creates a new API key with specified permissions and settings`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateAPIKeyRequest,
      },
    ],
    response: CreateAPIKeyResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "put",
    path: "/lead-scraper-microservice/api/v1/api-keys",
    alias: "UpdateAPIKey",
    description: `Updates an existing API key&#x27;s configuration`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: UpdateAPIKeyRequest,
      },
    ],
    response: UpdateAPIKeyResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "get",
    path: "/lead-scraper-microservice/api/v1/api-keys/:keyId",
    alias: "GetAPIKey",
    description: `Retrieves details of a specific API key`,
    requestFormat: "json",
    parameters: [
      {
        name: "keyId",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "organizationId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "tenantId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "accountId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "workspaceId",
        type: "Query",
        schema: z.string().optional(),
      },
    ],
    response: GetAPIKeyResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "delete",
    path: "/lead-scraper-microservice/api/v1/api-keys/:keyId",
    alias: "DeleteAPIKey",
    description: `Permanently deletes an API key`,
    requestFormat: "json",
    parameters: [
      {
        name: "keyId",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "organizationId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "workspaceId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "tenantId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "accountId",
        type: "Query",
        schema: z.string().optional(),
      },
    ],
    response: z.object({ success: z.boolean() }).partial().passthrough(),
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "get",
    path: "/lead-scraper-microservice/api/v1/api-keys/list",
    alias: "ListAPIKeys",
    description: `Retrieves a list of API keys with optional filtering`,
    requestFormat: "json",
    parameters: [
      {
        name: "organizationId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "tenantId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "accountId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "workspaceId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "pageSize",
        type: "Query",
        schema: z.number().int().optional(),
      },
      {
        name: "pageNumber",
        type: "Query",
        schema: z.number().int().optional(),
      },
      {
        name: "status",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "search",
        type: "Query",
        schema: z.string().optional(),
      },
    ],
    response: ListAPIKeysResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "post",
    path: "/lead-scraper-microservice/api/v1/api-keys/rotate",
    alias: "RotateAPIKey",
    description: `Rotates an API key while maintaining its configuration`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: RotateAPIKeyRequest,
      },
    ],
    response: RotateAPIKeyResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "get",
    path: "/lead-scraper-microservice/api/v1/jobs",
    alias: "ListScrapingJobs",
    description: `This endpoint retrieves all Google Maps scraping jobs`,
    requestFormat: "json",
    parameters: [
      {
        name: "authPlatformUserId",
        type: "Query",
        schema: z.string(),
      },
      {
        name: "orgId",
        type: "Query",
        schema: z.string(),
      },
      {
        name: "tenantId",
        type: "Query",
        schema: z.string(),
      },
    ],
    response: ListScrapingJobsResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "post",
    path: "/lead-scraper-microservice/api/v1/jobs",
    alias: "CreateScrapingJob",
    description: `This endpoint creates a new Google Maps scraping job`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateScrapingJobRequest,
      },
    ],
    response: CreateScrapingJobResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "get",
    path: "/lead-scraper-microservice/api/v1/jobs/:jobId",
    alias: "GetScrapingJob",
    description: `This endpoint retrieves a specific Google Maps scraping job`,
    requestFormat: "json",
    parameters: [
      {
        name: "jobId",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "userId",
        type: "Query",
        schema: z.string(),
      },
      {
        name: "orgId",
        type: "Query",
        schema: z.string(),
      },
      {
        name: "tenantId",
        type: "Query",
        schema: z.string(),
      },
    ],
    response: GetScrapingJobResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "delete",
    path: "/lead-scraper-microservice/api/v1/jobs/:jobId",
    alias: "DeleteScrapingJob",
    description: `This endpoint deletes a specific Google Maps scraping job`,
    requestFormat: "json",
    parameters: [
      {
        name: "jobId",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "userId",
        type: "Query",
        schema: z.string(),
      },
      {
        name: "orgId",
        type: "Query",
        schema: z.string(),
      },
      {
        name: "tenantId",
        type: "Query",
        schema: z.string(),
      },
    ],
    response: z.object({ success: z.boolean() }).partial().passthrough(),
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "get",
    path: "/lead-scraper-microservice/api/v1/jobs/:jobId/download",
    alias: "DownloadScrapingResults",
    description: `This endpoint downloads the results of a Google Maps scraping job as CSV`,
    requestFormat: "json",
    parameters: [
      {
        name: "jobId",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "userId",
        type: "Query",
        schema: z.string(),
      },
      {
        name: "orgId",
        type: "Query",
        schema: z.string(),
      },
      {
        name: "tenantId",
        type: "Query",
        schema: z.string(),
      },
    ],
    response: DownloadScrapingResultsResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "get",
    path: "/lead-scraper-microservice/api/v1/leads",
    alias: "ListLeads",
    description: `Retrieves a paginated list of leads with comprehensive filtering options`,
    requestFormat: "json",
    parameters: [
      {
        name: "organizationId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "workspaceId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "tenantId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "accountId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "pageSize",
        type: "Query",
        schema: z.number().int().optional(),
      },
      {
        name: "pageNumber",
        type: "Query",
        schema: z.number().int().optional(),
      },
    ],
    response: ListLeadsResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "get",
    path: "/lead-scraper-microservice/api/v1/leads/:leadId",
    alias: "GetLead",
    description: `Retrieves detailed information about a specific lead`,
    requestFormat: "json",
    parameters: [
      {
        name: "leadId",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "organizationId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "workspaceId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "tenantId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "accountId",
        type: "Query",
        schema: z.string().optional(),
      },
    ],
    response: GetLeadResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "get",
    path: "/lead-scraper-microservice/api/v1/organization",
    alias: "ListOrganizations",
    description: `Retrieves a list of all organizations in a tenant`,
    requestFormat: "json",
    parameters: [
      {
        name: "pageSize",
        type: "Query",
        schema: z.number().int().optional(),
      },
      {
        name: "pageNumber",
        type: "Query",
        schema: z.number().int().optional(),
      },
    ],
    response: ListOrganizationsResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "post",
    path: "/lead-scraper-microservice/api/v1/organization",
    alias: "CreateOrganization",
    description: `Creates a new organization within a tenant`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateOrganizationRequest,
      },
    ],
    response: CreateOrganizationResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "put",
    path: "/lead-scraper-microservice/api/v1/organization",
    alias: "UpdateOrganization",
    description: `Updates the configuration of a specific organization`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: UpdateOrganizationRequest,
      },
    ],
    response: UpdateOrganizationResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "get",
    path: "/lead-scraper-microservice/api/v1/organization/:id",
    alias: "GetOrganization",
    description: `Retrieves details of a specific organization`,
    requestFormat: "json",
    parameters: [
      {
        name: "id",
        type: "Path",
        schema: z.string(),
      },
    ],
    response: GetOrganizationResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "delete",
    path: "/lead-scraper-microservice/api/v1/organization/:id",
    alias: "DeleteOrganization",
    description: `Permanently deletes an organization and all associated resources`,
    requestFormat: "json",
    parameters: [
      {
        name: "id",
        type: "Path",
        schema: z.string(),
      },
    ],
    response: z.object({ success: z.boolean() }).partial().passthrough(),
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "get",
    path: "/lead-scraper-microservice/api/v1/organization/tenants/:organizationId",
    alias: "ListTenants",
    description: `Retrieves a list of all tenants in the system`,
    requestFormat: "json",
    parameters: [
      {
        name: "organizationId",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "pageSize",
        type: "Query",
        schema: z.number().int().optional(),
      },
      {
        name: "pageNumber",
        type: "Query",
        schema: z.number().int().optional(),
      },
    ],
    response: ListTenantsResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "delete",
    path: "/lead-scraper-microservice/api/v1/organization/tenants/:organizationId/:tenantId",
    alias: "DeleteTenant",
    description: `Permanently deletes a tenant and all associated resources`,
    requestFormat: "json",
    parameters: [
      {
        name: "organizationId",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "tenantId",
        type: "Path",
        schema: z.string(),
      },
    ],
    response: z.object({ success: z.boolean() }).partial().passthrough(),
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "post",
    path: "/lead-scraper-microservice/api/v1/organizations/:organizationId/tenants",
    alias: "CreateTenant",
    description: `Creates a new tenant in the system`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateTenantBody,
      },
      {
        name: "organizationId",
        type: "Path",
        schema: z.string(),
      },
    ],
    response: z.object({ tenantId: z.string() }).partial().passthrough(),
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "put",
    path: "/lead-scraper-microservice/api/v1/organizations/tenants",
    alias: "UpdateTenant",
    description: `Updates the configuration of a specific tenant`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: UpdateTenantRequest,
      },
    ],
    response: UpdateTenantResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "get",
    path: "/lead-scraper-microservice/api/v1/organizations/tenants/:organizationId/:tenantId",
    alias: "GetTenant",
    description: `Retrieves details of a specific tenant`,
    requestFormat: "json",
    parameters: [
      {
        name: "organizationId",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "tenantId",
        type: "Path",
        schema: z.string(),
      },
    ],
    response: GetTenantResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "post",
    path: "/lead-scraper-microservice/api/v1/organizations/tenants/api-keys",
    alias: "CreateTenantAPIKey",
    description: `Creates a new API key for a tenant with specified permissions`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateTenantAPIKeyRequest,
      },
    ],
    response: CreateTenantAPIKeyResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "put",
    path: "/lead-scraper-microservice/api/v1/organizations/tenants/api-keys",
    alias: "UpdateTenantAPIKey",
    description: `Updates an existing tenant API key&#x27;s configuration`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: UpdateTenantAPIKeyRequest,
      },
    ],
    response: UpdateTenantAPIKeyResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "get",
    path: "/lead-scraper-microservice/api/v1/organizations/tenants/api-keys/:keyId",
    alias: "GetTenantAPIKey",
    description: `Retrieves details of a specific tenant API key`,
    requestFormat: "json",
    parameters: [
      {
        name: "keyId",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "organizationId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "tenantId",
        type: "Query",
        schema: z.string().optional(),
      },
    ],
    response: GetTenantAPIKeyResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "delete",
    path: "/lead-scraper-microservice/api/v1/organizations/tenants/api-keys/:keyId",
    alias: "DeleteTenantAPIKey",
    description: `Permanently deletes a tenant API key`,
    requestFormat: "json",
    parameters: [
      {
        name: "keyId",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "organizationId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "tenantId",
        type: "Query",
        schema: z.string().optional(),
      },
    ],
    response: z.object({ success: z.boolean() }).partial().passthrough(),
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "get",
    path: "/lead-scraper-microservice/api/v1/organizations/tenants/api-keys/list",
    alias: "ListTenantAPIKeys",
    description: `Retrieves all API keys for a specific tenant`,
    requestFormat: "json",
    parameters: [
      {
        name: "organizationId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "tenantId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "pageSize",
        type: "Query",
        schema: z.number().int().optional(),
      },
      {
        name: "pageNumber",
        type: "Query",
        schema: z.number().int().optional(),
      },
      {
        name: "status",
        type: "Query",
        schema: z.string().optional(),
      },
    ],
    response: ListTenantAPIKeysResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "post",
    path: "/lead-scraper-microservice/api/v1/organizations/tenants/api-keys/rotate",
    alias: "RotateTenantAPIKey",
    description: `Rotates a tenant API key while maintaining its configuration`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: RotateTenantAPIKeyRequest,
      },
    ],
    response: RotateTenantAPIKeyResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "get",
    path: "/lead-scraper-microservice/api/v1/webhooks",
    alias: "ListWebhooks",
    description: `Lists all webhook configurations with pagination`,
    requestFormat: "json",
    parameters: [
      {
        name: "organizationId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "workspaceId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "tenantId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "accountId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "pageSize",
        type: "Query",
        schema: z.number().int().optional(),
      },
      {
        name: "pageNumber",
        type: "Query",
        schema: z.number().int().optional(),
      },
      {
        name: "status",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "search",
        type: "Query",
        schema: z.string().optional(),
      },
    ],
    response: ListWebhooksResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "post",
    path: "/lead-scraper-microservice/api/v1/webhooks",
    alias: "CreateWebhook",
    description: `Creates a new webhook configuration for event notifications`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateWebhookRequest,
      },
    ],
    response: CreateWebhookResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "put",
    path: "/lead-scraper-microservice/api/v1/webhooks",
    alias: "UpdateWebhook",
    description: `Updates an existing webhook configuration`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: UpdateWebhookRequest,
      },
    ],
    response: UpdateWebhookResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "get",
    path: "/lead-scraper-microservice/api/v1/webhooks/:webhookId",
    alias: "GetWebhook",
    description: `Retrieves details of a specific webhook configuration`,
    requestFormat: "json",
    parameters: [
      {
        name: "webhookId",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "organizationId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "workspaceId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "tenantId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "accountId",
        type: "Query",
        schema: z.string().optional(),
      },
    ],
    response: GetWebhookResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "delete",
    path: "/lead-scraper-microservice/api/v1/webhooks/:webhookId",
    alias: "DeleteWebhook",
    description: `Deletes a webhook configuration`,
    requestFormat: "json",
    parameters: [
      {
        name: "webhookId",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "organizationId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "workspaceId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "tenantId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "accountId",
        type: "Query",
        schema: z.string().optional(),
      },
    ],
    response: z.object({ success: z.boolean() }).partial().passthrough(),
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "put",
    path: "/lead-scraper-microservice/api/v1/workspace",
    alias: "UpdateWorkspace",
    description: `Updates the details of a specific workspace`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: UpdateWorkspaceRequest,
      },
    ],
    response: UpdateWorkspaceResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "get",
    path: "/lead-scraper-microservice/api/v1/workspace/:id",
    alias: "GetWorkspace",
    description: `Retrieves details of a specific workspace`,
    requestFormat: "json",
    parameters: [
      {
        name: "id",
        type: "Path",
        schema: z.string(),
      },
    ],
    response: GetWorkspaceResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "delete",
    path: "/lead-scraper-microservice/api/v1/workspace/:id",
    alias: "DeleteWorkspace",
    description: `Deletes a specific workspace`,
    requestFormat: "json",
    parameters: [
      {
        name: "id",
        type: "Path",
        schema: z.string(),
      },
    ],
    response: z.object({ success: z.boolean() }).partial().passthrough(),
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "get",
    path: "/lead-scraper-microservice/api/v1/workspaces",
    alias: "ListWorkspaces",
    description: `Retrieves a list of workspaces for a given account`,
    requestFormat: "json",
    parameters: [
      {
        name: "accountId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "pageSize",
        type: "Query",
        schema: z.number().int().optional(),
      },
      {
        name: "pageNumber",
        type: "Query",
        schema: z.number().int().optional(),
      },
    ],
    response: ListWorkspacesResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "post",
    path: "/lead-scraper-microservice/api/v1/workspaces",
    alias: "CreateWorkspace",
    description: `Creates a new workspace for a given account`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateWorkspaceRequest,
      },
    ],
    response: CreateWorkspaceResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "get",
    path: "/lead-scraper-microservice/api/v1/workspaces/:workspaceId/analytics",
    alias: "GetWorkspaceAnalytics",
    description: `Retrieves analytics data for a specific workspace`,
    requestFormat: "json",
    parameters: [
      {
        name: "workspaceId",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "startTime",
        type: "Query",
        schema: z.string().datetime({ offset: true }).optional(),
      },
      {
        name: "endTime",
        type: "Query",
        schema: z.string().datetime({ offset: true }).optional(),
      },
    ],
    response: GetWorkspaceAnalyticsResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "get",
    path: "/lead-scraper-microservice/api/v1/workspaces/:workspaceId/workflows",
    alias: "ListWorkflows",
    description: `Retrieves a list of workflows for a given workspace`,
    requestFormat: "json",
    parameters: [
      {
        name: "workspaceId",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "pageSize",
        type: "Query",
        schema: z.number().int().optional(),
      },
      {
        name: "pageNumber",
        type: "Query",
        schema: z.number().int().optional(),
      },
      {
        name: "filter",
        type: "Query",
        schema: z.string().optional(),
      },
    ],
    response: ListWorkflowsResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "post",
    path: "/lead-scraper-microservice/api/v1/workspaces/:workspaceId/workflows",
    alias: "LeadScraperService_CreateWorkflow",
    description: `This endpoint creates a new workflow configuration for a specific workspace

Required parameters:
- workspace_id: Parent workspace ID
- name: Human-readable workflow name
- description: Optional workflow description
- parameters: Job parameters for the workflow

Example:
&#x60;&#x60;&#x60;json`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateWorkflowBody,
      },
      {
        name: "workspaceId",
        type: "Path",
        schema: z.string(),
      },
    ],
    response: CreateWorkflowResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "get",
    path: "/lead-scraper-microservice/api/v1/workspaces/:workspaceId/workflows/:id",
    alias: "GetWorkflow",
    description: `Retrieves details of a specific workflow`,
    requestFormat: "json",
    parameters: [
      {
        name: "workspaceId",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "id",
        type: "Path",
        schema: z.string(),
      },
    ],
    response: GetWorkflowResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "delete",
    path: "/lead-scraper-microservice/api/v1/workspaces/:workspaceId/workflows/:id",
    alias: "DeleteWorkflow",
    description: `Deletes a specific workflow`,
    requestFormat: "json",
    parameters: [
      {
        name: "workspaceId",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "id",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "orgId",
        type: "Query",
        schema: z.string(),
      },
      {
        name: "tenantId",
        type: "Query",
        schema: z.string(),
      },
      {
        name: "accountId",
        type: "Query",
        schema: z.string().optional(),
      },
    ],
    response: z.object({ success: z.boolean() }).partial().passthrough(),
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "post",
    path: "/lead-scraper-microservice/api/v1/workspaces/:workspaceId/workflows/:id/pause",
    alias: "PauseWorkflow",
    description: `Pauses the execution of a specific workflow`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: z.object({ pause: z.boolean() }).partial().passthrough(),
      },
      {
        name: "workspaceId",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "id",
        type: "Path",
        schema: z.string(),
      },
    ],
    response: PauseWorkflowResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "post",
    path: "/lead-scraper-microservice/api/v1/workspaces/:workspaceId/workflows/:id/trigger",
    alias: "TriggerWorkflow",
    description: `Triggers the execution of a specific workflow`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: TriggerWorkflowBody,
      },
      {
        name: "workspaceId",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "id",
        type: "Path",
        schema: z.string(),
      },
    ],
    response: TriggerWorkflowResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "put",
    path: "/lead-scraper-microservice/api/v1/workspaces/workflow",
    alias: "UpdateWorkflow",
    description: `Updates the details of a specific workflow`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: UpdateWorkflowRequest,
      },
    ],
    response: UpdateWorkflowResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse,
      },
      {
        status: 402,
        description: `Payment Required - Payment is necessary to proceed`,
        schema: PaymentRequiredErrorMessageResponse,
      },
      {
        status: 403,
        description: `Forbidden - Access denied`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 405,
        description: `Method Not Allowed - HTTP method not supported`,
        schema: MethodNotAllowedErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 410,
        description: `Gone - Resource is no longer available`,
        schema: GoneErrorMessageResponse,
      },
      {
        status: 412,
        description: `Precondition Failed - Preconditions in headers did not match`,
        schema: PreconditionFailedErrorMessageResponse,
      },
      {
        status: 422,
        description: `Unprocessable Entity - Semantic errors in the request`,
        schema: UnprocessableEntityErrorMessageResponse,
      },
      {
        status: 425,
        description: `Too Early - Request is being replayed`,
        schema: TooEarlyErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: 501,
        description: `Not Implemented - Functionality not supported`,
        schema: NotImplementedErrorMessageResponse,
      },
      {
        status: 502,
        description: `Bad Gateway - Invalid response from upstream server`,
        schema: BadGatewayErrorMessageResponse,
      },
      {
        status: 503,
        description: `Service Unavailable - Try again later`,
        schema: ServiceUnavailableErrorMessageResponse,
      },
      {
        status: 504,
        description: `Gateway Timeout - Upstream server timed out`,
        schema: GatewayTimeoutErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: rpc_Status,
      },
    ],
  },
  {
    method: "post",
    path: "/workspace-service/v1/accounts",
    alias: "CreateAccount1",
    description: `Creates a new user account with initial workspace`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateAccountRequest1,
      },
    ],
    response: CreateAccountResponse1,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse1,
      },
      {
        status: 403,
        description: `Forbidden - Insufficient permissions`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: Status,
      },
    ],
  },
  {
    method: "put",
    path: "/workspace-service/v1/accounts",
    alias: "UpdateAccount1",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: UpdateAccountRequest,
      },
    ],
    response: UpdateAccountResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse1,
      },
      {
        status: 403,
        description: `Forbidden - Insufficient permissions`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: Status,
      },
    ],
  },
  {
    method: "get",
    path: "/workspace-service/v1/accounts/:id",
    alias: "GetAccount1",
    requestFormat: "json",
    parameters: [
      {
        name: "id",
        type: "Path",
        schema: z.string(),
      },
    ],
    response: GetAccountResponse1,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse1,
      },
      {
        status: 403,
        description: `Forbidden - Insufficient permissions`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: Status,
      },
    ],
  },
  {
    method: "delete",
    path: "/workspace-service/v1/accounts/:id",
    alias: "DeleteAccount1",
    requestFormat: "json",
    parameters: [
      {
        name: "id",
        type: "Path",
        schema: z.string(),
      },
    ],
    response: z.object({ success: z.boolean() }).partial().passthrough(),
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse1,
      },
      {
        status: 403,
        description: `Forbidden - Insufficient permissions`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: Status,
      },
    ],
  },
  {
    method: "put",
    path: "/workspace-service/v1/workspace-sharings",
    alias: "UpdateWorkspaceSharing",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: UpdateWorkspaceSharingRequest,
      },
    ],
    response: UpdateWorkspaceSharingResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse1,
      },
      {
        status: 403,
        description: `Forbidden - Insufficient permissions`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: Status,
      },
    ],
  },
  {
    method: "delete",
    path: "/workspace-service/v1/workspace-sharings/:sharingId",
    alias: "RemoveWorkspaceSharing",
    requestFormat: "json",
    parameters: [
      {
        name: "sharingId",
        type: "Path",
        schema: z.string(),
      },
    ],
    response: z.object({ success: z.boolean() }).partial().passthrough(),
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse1,
      },
      {
        status: 403,
        description: `Forbidden - Insufficient permissions`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: Status,
      },
    ],
  },
  {
    method: "post",
    path: "/workspace-service/v1/workspaces",
    alias: "CreateWorkspace1",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateWorkspaceRequest1,
      },
    ],
    response: CreateWorkspaceResponse1,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse1,
      },
      {
        status: 403,
        description: `Forbidden - Insufficient permissions`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: Status,
      },
    ],
  },
  {
    method: "put",
    path: "/workspace-service/v1/workspaces",
    alias: "UpdateWorkspace1",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: UpdateWorkspaceRequest,
      },
    ],
    response: UpdateWorkspaceResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse1,
      },
      {
        status: 403,
        description: `Forbidden - Insufficient permissions`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: Status,
      },
    ],
  },
  {
    method: "get",
    path: "/workspace-service/v1/workspaces/:id",
    alias: "GetWorkspace1",
    requestFormat: "json",
    parameters: [
      {
        name: "id",
        type: "Path",
        schema: z.string(),
      },
    ],
    response: GetWorkspaceResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse1,
      },
      {
        status: 403,
        description: `Forbidden - Insufficient permissions`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: Status,
      },
    ],
  },
  {
    method: "delete",
    path: "/workspace-service/v1/workspaces/:id",
    alias: "DeleteWorkspace1",
    requestFormat: "json",
    parameters: [
      {
        name: "id",
        type: "Path",
        schema: z.string(),
      },
    ],
    response: z.object({ success: z.boolean() }).partial().passthrough(),
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse1,
      },
      {
        status: 403,
        description: `Forbidden - Insufficient permissions`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: Status,
      },
    ],
  },
  {
    method: "post",
    path: "/workspace-service/v1/workspaces/:workspaceId/share",
    alias: "ShareWorkspace",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: ShareWorkspaceBody,
      },
      {
        name: "workspaceId",
        type: "Path",
        schema: z.string(),
      },
    ],
    response: ShareWorkspaceResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse1,
      },
      {
        status: 403,
        description: `Forbidden - Insufficient permissions`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: Status,
      },
    ],
  },
  {
    method: "get",
    path: "/workspace-service/v1/workspaces/analytics/:workspaceId",
    alias: "GetWorkspaceAnalytics1",
    requestFormat: "json",
    parameters: [
      {
        name: "workspaceId",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "startTime",
        type: "Query",
        schema: z.string().datetime({ offset: true }).optional(),
      },
      {
        name: "endTime",
        type: "Query",
        schema: z.string().datetime({ offset: true }).optional(),
      },
    ],
    response: GetWorkspaceAnalyticsResponse1,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse1,
      },
      {
        status: 403,
        description: `Forbidden - Insufficient permissions`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: Status,
      },
    ],
  },
  {
    method: "get",
    path: "/workspace-service/v1/workspaces/compliance-report/:workspaceId",
    alias: "GetWorkspaceComplianceReport",
    requestFormat: "json",
    parameters: [
      {
        name: "workspaceId",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "complianceType",
        type: "Query",
        schema: z.string().optional(),
      },
    ],
    response: GetWorkspaceComplianceReportResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse1,
      },
      {
        status: 403,
        description: `Forbidden - Insufficient permissions`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: Status,
      },
    ],
  },
  {
    method: "get",
    path: "/workspace-service/v1/workspaces/list",
    alias: "ListWorkspaces1",
    requestFormat: "json",
    parameters: [
      {
        name: "accountId",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "pageSize",
        type: "Query",
        schema: z.number().int().optional(),
      },
      {
        name: "pageToken",
        type: "Query",
        schema: z.string().optional(),
      },
      {
        name: "filter",
        type: "Query",
        schema: z.string().optional(),
      },
    ],
    response: ListWorkspacesResponse1,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse1,
      },
      {
        status: 403,
        description: `Forbidden - Insufficient permissions`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: Status,
      },
    ],
  },
  {
    method: "get",
    path: "/workspace-service/v1/workspaces/sharings/:workspaceId",
    alias: "ListWorkspaceSharings",
    requestFormat: "json",
    parameters: [
      {
        name: "workspaceId",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "pageSize",
        type: "Query",
        schema: z.number().int().optional(),
      },
      {
        name: "pageToken",
        type: "Query",
        schema: z.string().optional(),
      },
    ],
    response: ListWorkspaceSharingsResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse1,
      },
      {
        status: 403,
        description: `Forbidden - Insufficient permissions`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: Status,
      },
    ],
  },
  {
    method: "get",
    path: "/workspace-service/v1/workspaces/storage-stats/:workspaceId",
    alias: "GetWorkspaceStorageStats",
    requestFormat: "json",
    parameters: [
      {
        name: "workspaceId",
        type: "Path",
        schema: z.string(),
      },
    ],
    response: GetWorkspaceStorageStatsResponse,
    errors: [
      {
        status: 400,
        description: `Bad Request - Invalid input parameters`,
        schema: ValidationErrorMessageResponse,
      },
      {
        status: 401,
        description: `Unauthorized - Authentication required`,
        schema: AuthenticationErrorMessageResponse1,
      },
      {
        status: 403,
        description: `Forbidden - Insufficient permissions`,
        schema: ForbiddenErrorMessageResponse,
      },
      {
        status: 404,
        description: `Not Found - Resource not found`,
        schema: NotFoundErrorMessageResponse,
      },
      {
        status: 409,
        description: `Conflict - Resource already exists`,
        schema: ConflictErrorMessageResponse,
      },
      {
        status: 429,
        description: `Too Many Requests - Rate limit exceeded`,
        schema: RateLimitErrorMessageResponse,
      },
      {
        status: 500,
        description: `Internal Server Error`,
        schema: InternalErrorMessageResponse,
      },
      {
        status: "default",
        description: `An unexpected error response.`,
        schema: Status,
      },
    ],
  },
]);

export const api = new Zodios(endpoints);

export function createApiClient(baseUrl: string, options?: ZodiosOptions) {
  return new Zodios(baseUrl, endpoints, options);
}

export class ApiClient {
  private readonly client: ZodiosInstance<typeof endpoints>;

  constructor(baseUrl: string, options?: ZodiosOptions) {
    this.client = new Zodios(baseUrl, endpoints, options);
  }

  async getLeadScraperMicroserviceApiV1Accounts(params: {
    pageSize: number | undefined;
    pageNumber: number | undefined;
    filter: string | undefined;
    organizationId: string | undefined;
    tenantId: string | undefined;
  }) {
    return this.client.get("/lead-scraper-microservice/api/v1/accounts", {
      params: {},
      queries: {
        pageSize: params.pageSize ? Number(params.pageSize) : undefined,
        pageNumber: params.pageNumber ? Number(params.pageNumber) : undefined,
        filter: params.filter,
        organizationId: params.organizationId,
        tenantId: params.tenantId,
      },
    });
  }

  async createLeadScraperMicroserviceApiV1Accounts(
    data: z.infer<typeof CreateAccountRequest>,
  ) {
    return this.client.post(
      "/lead-scraper-microservice/api/v1/accounts",
      data,
      {},
    );
  }

  async updateLeadScraperMicroserviceApiV1Accounts(
    data: z.infer<typeof schemas.UpdateAccountRequest>,
  ) {
    return this.client.put(
      "/lead-scraper-microservice/api/v1/accounts",
      data,
      {},
    );
  }

  async updateLeadScraperMicroserviceApiV1AccountsSettings(
    data: z.infer<typeof schemas.UpdateAccountSettingsRequest>,
  ) {
    return this.client.put(
      "/lead-scraper-microservice/api/v1/accounts/settings",
      data,
      {},
    );
  }

  async getLeadScraperMicroserviceApiV1AccountsId(params: {
    id: string;
    organizationId: string | undefined;
    tenantId: string | undefined;
  }) {
    return this.client.get("/lead-scraper-microservice/api/v1/accounts/:id", {
      params: {
        id: params.id,
      },
      queries: {
        organizationId: params.organizationId,
        tenantId: params.tenantId,
      },
    });
  }

  async deleteLeadScraperMicroserviceApiV1AccountsId(params: {
    id: string;
    organizationId: string | undefined;
    tenantId: string | undefined;
  }) {
    return this.client.delete(
      "/lead-scraper-microservice/api/v1/accounts/:id",
      undefined,
      {
        params: {
          id: params.id,
        },
        queries: {
          organizationId: params.organizationId,
          tenantId: params.tenantId,
        },
      },
    );
  }
  async getLeadScraperMicroserviceApiV1AccountsIdUsage(params: { id: string }) {
    return this.client.get(
      "/lead-scraper-microservice/api/v1/accounts/:id/usage",
      {
        params: {
          id: params.id,
        },
      },
    );
  }

  async createLeadScraperMicroserviceApiV1ApiKeys(
    data: z.infer<typeof CreateAPIKeyRequest>,
  ) {
    return this.client.post(
      "/lead-scraper-microservice/api/v1/api-keys",
      data,
      {},
    );
  }

  async updateLeadScraperMicroserviceApiV1ApiKeys(
    data: z.infer<typeof schemas.UpdateAPIKeyRequest>,
  ) {
    return this.client.put(
      "/lead-scraper-microservice/api/v1/api-keys",
      data,
      {},
    );
  }

  async getLeadScraperMicroserviceApiV1ApiKeysList(params: {
    organizationId: string | undefined;
    tenantId: string | undefined;
    accountId: string | undefined;
    workspaceId: string | undefined;
    pageSize: number | undefined;
    pageNumber: number | undefined;
    status: string | undefined;
    search: string | undefined;
  }) {
    return this.client.get("/lead-scraper-microservice/api/v1/api-keys/list", {
      params: {},
      queries: {
        organizationId: params.organizationId,
        tenantId: params.tenantId,
        accountId: params.accountId,
        workspaceId: params.workspaceId,
        pageSize: params.pageSize ? Number(params.pageSize) : undefined,
        pageNumber: params.pageNumber ? Number(params.pageNumber) : undefined,
        status: params.status,
        search: params.search,
      },
    });
  }

  async createLeadScraperMicroserviceApiV1ApiKeysRotate(
    data: z.infer<typeof RotateAPIKeyRequest>,
  ) {
    return this.client.post(
      "/lead-scraper-microservice/api/v1/api-keys/rotate",
      data,
      {},
    );
  }

  async getLeadScraperMicroserviceApiV1ApiKeysKeyId(params: {
    keyId: string;
    organizationId: string | undefined;
    tenantId: string | undefined;
    accountId: string | undefined;
    workspaceId: string | undefined;
  }) {
    return this.client.get(
      "/lead-scraper-microservice/api/v1/api-keys/:keyId",
      {
        params: {
          keyId: params.keyId,
        },
        queries: {
          organizationId: params.organizationId,
          tenantId: params.tenantId,
          accountId: params.accountId,
          workspaceId: params.workspaceId,
        },
      },
    );
  }

  async deleteLeadScraperMicroserviceApiV1ApiKeysKeyId(params: {
    keyId: string;
    organizationId: string | undefined;
    workspaceId: string | undefined;
    tenantId: string | undefined;
    accountId: string | undefined;
  }) {
    return this.client.delete(
      "/lead-scraper-microservice/api/v1/api-keys/:keyId",
      undefined,
      {
        params: {
          keyId: params.keyId,
        },
        queries: {
          organizationId: params.organizationId,
          workspaceId: params.workspaceId,
          tenantId: params.tenantId,
          accountId: params.accountId,
        },
      },
    );
  }
  async getLeadScraperMicroserviceApiV1Jobs(params: {
    authPlatformUserId: string;
    orgId: string;
    tenantId: string;
  }) {
    return this.client.get("/lead-scraper-microservice/api/v1/jobs", {
      params: {},
      queries: {
        authPlatformUserId: params.authPlatformUserId,
        orgId: params.orgId,
        tenantId: params.tenantId,
      },
    });
  }

  async createLeadScraperMicroserviceApiV1Jobs(
    data: z.infer<typeof CreateScrapingJobRequest>,
  ) {
    return this.client.post("/lead-scraper-microservice/api/v1/jobs", data, {});
  }

  async getLeadScraperMicroserviceApiV1JobsJobId(params: {
    jobId: string;
    userId: string;
    orgId: string;
    tenantId: string;
  }) {
    return this.client.get("/lead-scraper-microservice/api/v1/jobs/:jobId", {
      params: {
        jobId: params.jobId,
      },
      queries: {
        userId: params.userId,
        orgId: params.orgId,
        tenantId: params.tenantId,
      },
    });
  }

  async deleteLeadScraperMicroserviceApiV1JobsJobId(params: {
    jobId: string;
    userId: string;
    orgId: string;
    tenantId: string;
  }) {
    return this.client.delete(
      "/lead-scraper-microservice/api/v1/jobs/:jobId",
      undefined,
      {
        params: {
          jobId: params.jobId,
        },
        queries: {
          userId: params.userId,
          orgId: params.orgId,
          tenantId: params.tenantId,
        },
      },
    );
  }
  async getLeadScraperMicroserviceApiV1JobsJobIdDownload(params: {
    jobId: string;
    userId: string;
    orgId: string;
    tenantId: string;
  }) {
    return this.client.get(
      "/lead-scraper-microservice/api/v1/jobs/:jobId/download",
      {
        params: {
          jobId: params.jobId,
        },
        queries: {
          userId: params.userId,
          orgId: params.orgId,
          tenantId: params.tenantId,
        },
      },
    );
  }

  async getLeadScraperMicroserviceApiV1Leads(params: {
    organizationId: string | undefined;
    workspaceId: string | undefined;
    tenantId: string | undefined;
    accountId: string | undefined;
    pageSize: number | undefined;
    pageNumber: number | undefined;
  }) {
    return this.client.get("/lead-scraper-microservice/api/v1/leads", {
      params: {},
      queries: {
        organizationId: params.organizationId,
        workspaceId: params.workspaceId,
        tenantId: params.tenantId,
        accountId: params.accountId,
        pageSize: params.pageSize ? Number(params.pageSize) : undefined,
        pageNumber: params.pageNumber ? Number(params.pageNumber) : undefined,
      },
    });
  }

  async getLeadScraperMicroserviceApiV1LeadsLeadId(params: {
    leadId: string;
    organizationId: string | undefined;
    workspaceId: string | undefined;
    tenantId: string | undefined;
    accountId: string | undefined;
  }) {
    return this.client.get("/lead-scraper-microservice/api/v1/leads/:leadId", {
      params: {
        leadId: params.leadId,
      },
      queries: {
        organizationId: params.organizationId,
        workspaceId: params.workspaceId,
        tenantId: params.tenantId,
        accountId: params.accountId,
      },
    });
  }

  async getLeadScraperMicroserviceApiV1Organization(params: {
    pageSize: number | undefined;
    pageNumber: number | undefined;
  }) {
    return this.client.get("/lead-scraper-microservice/api/v1/organization", {
      params: {},
      queries: {
        pageSize: params.pageSize ? Number(params.pageSize) : undefined,
        pageNumber: params.pageNumber ? Number(params.pageNumber) : undefined,
      },
    });
  }

  async createLeadScraperMicroserviceApiV1Organization(
    data: z.infer<typeof CreateOrganizationRequest>,
  ) {
    return this.client.post(
      "/lead-scraper-microservice/api/v1/organization",
      data,
      {},
    );
  }

  async updateLeadScraperMicroserviceApiV1Organization(
    data: z.infer<typeof schemas.UpdateOrganizationRequest>,
  ) {
    return this.client.put(
      "/lead-scraper-microservice/api/v1/organization",
      data,
      {},
    );
  }

  async getLeadScraperMicroserviceApiV1OrganizationTenantsOrganizationId(params: {
    organizationId: string;
    pageSize: number | undefined;
    pageNumber: number | undefined;
  }) {
    return this.client.get(
      "/lead-scraper-microservice/api/v1/organization/tenants/:organizationId",
      {
        params: {
          organizationId: params.organizationId,
        },
        queries: {
          pageSize: params.pageSize ? Number(params.pageSize) : undefined,
          pageNumber: params.pageNumber ? Number(params.pageNumber) : undefined,
        },
      },
    );
  }

  async deleteLeadScraperMicroserviceApiV1OrganizationTenantsOrganizationIdTenantId(params: {
    organizationId: string;
    tenantId: string;
  }) {
    return this.client.delete(
      "/lead-scraper-microservice/api/v1/organization/tenants/:organizationId/:tenantId",
      undefined,
      {
        params: {
          organizationId: params.organizationId,
          tenantId: params.tenantId,
        },
      },
    );
  }
  async getLeadScraperMicroserviceApiV1OrganizationId(params: { id: string }) {
    return this.client.get(
      "/lead-scraper-microservice/api/v1/organization/:id",
      {
        params: {
          id: params.id,
        },
      },
    );
  }

  async deleteLeadScraperMicroserviceApiV1OrganizationId(params: {
    id: string;
  }) {
    return this.client.delete(
      "/lead-scraper-microservice/api/v1/organization/:id",
      undefined,
      {
        params: {
          id: params.id,
        },
      },
    );
  }

  async updateLeadScraperMicroserviceApiV1OrganizationsTenants(
    data: z.infer<typeof schemas.UpdateTenantRequest>,
  ) {
    return this.client.put(
      "/lead-scraper-microservice/api/v1/organizations/tenants",
      data,
      {},
    );
  }

  async createLeadScraperMicroserviceApiV1OrganizationsTenantsApiKeys(
    data: z.infer<typeof CreateTenantAPIKeyRequest>,
  ) {
    return this.client.post(
      "/lead-scraper-microservice/api/v1/organizations/tenants/api-keys",
      data,
      {},
    );
  }

  async updateLeadScraperMicroserviceApiV1OrganizationsTenantsApiKeys(
    data: z.infer<typeof schemas.UpdateTenantAPIKeyRequest>,
  ) {
    return this.client.put(
      "/lead-scraper-microservice/api/v1/organizations/tenants/api-keys",
      data,
      {},
    );
  }

  async getLeadScraperMicroserviceApiV1OrganizationsTenantsApiKeysList(params: {
    organizationId: string | undefined;
    tenantId: string | undefined;
    pageSize: number | undefined;
    pageNumber: number | undefined;
    status: string | undefined;
  }) {
    return this.client.get(
      "/lead-scraper-microservice/api/v1/organizations/tenants/api-keys/list",
      {
        params: {},
        queries: {
          organizationId: params.organizationId,
          tenantId: params.tenantId,
          pageSize: params.pageSize ? Number(params.pageSize) : undefined,
          pageNumber: params.pageNumber ? Number(params.pageNumber) : undefined,
          status: params.status,
        },
      },
    );
  }

  async createLeadScraperMicroserviceApiV1OrganizationsTenantsApiKeysRotate(
    data: z.infer<typeof RotateTenantAPIKeyRequest>,
  ) {
    return this.client.post(
      "/lead-scraper-microservice/api/v1/organizations/tenants/api-keys/rotate",
      data,
      {},
    );
  }

  async getLeadScraperMicroserviceApiV1OrganizationsTenantsApiKeysKeyId(params: {
    keyId: string;
    organizationId: string | undefined;
    tenantId: string | undefined;
  }) {
    return this.client.get(
      "/lead-scraper-microservice/api/v1/organizations/tenants/api-keys/:keyId",
      {
        params: {
          keyId: params.keyId,
        },
        queries: {
          organizationId: params.organizationId,
          tenantId: params.tenantId,
        },
      },
    );
  }

  async deleteLeadScraperMicroserviceApiV1OrganizationsTenantsApiKeysKeyId(params: {
    keyId: string;
    organizationId: string | undefined;
    tenantId: string | undefined;
  }) {
    return this.client.delete(
      "/lead-scraper-microservice/api/v1/organizations/tenants/api-keys/:keyId",
      undefined,
      {
        params: {
          keyId: params.keyId,
        },
        queries: {
          organizationId: params.organizationId,
          tenantId: params.tenantId,
        },
      },
    );
  }
  async getLeadScraperMicroserviceApiV1OrganizationsTenantsOrganizationIdTenantId(params: {
    organizationId: string;
    tenantId: string;
  }) {
    return this.client.get(
      "/lead-scraper-microservice/api/v1/organizations/tenants/:organizationId/:tenantId",
      {
        params: {
          organizationId: params.organizationId,
          tenantId: params.tenantId,
        },
      },
    );
  }

  async createLeadScraperMicroserviceApiV1OrganizationsOrganizationIdTenants(
    data: z.infer<typeof CreateTenantBody>,
    params: {
      organizationId: string;
    },
  ) {
    return this.client.post(
      "/lead-scraper-microservice/api/v1/organizations/:organizationId/tenants",
      data,
      {
        params: {
          organizationId: params.organizationId,
        },
      },
    );
  }

  async getLeadScraperMicroserviceApiV1Webhooks(params: {
    organizationId: string | undefined;
    workspaceId: string | undefined;
    tenantId: string | undefined;
    accountId: string | undefined;
    pageSize: number | undefined;
    pageNumber: number | undefined;
    status: string | undefined;
    search: string | undefined;
  }) {
    return this.client.get("/lead-scraper-microservice/api/v1/webhooks", {
      params: {},
      queries: {
        organizationId: params.organizationId,
        workspaceId: params.workspaceId,
        tenantId: params.tenantId,
        accountId: params.accountId,
        pageSize: params.pageSize ? Number(params.pageSize) : undefined,
        pageNumber: params.pageNumber ? Number(params.pageNumber) : undefined,
        status: params.status,
        search: params.search,
      },
    });
  }

  async createLeadScraperMicroserviceApiV1Webhooks(
    data: z.infer<typeof CreateWebhookRequest>,
  ) {
    return this.client.post(
      "/lead-scraper-microservice/api/v1/webhooks",
      data,
      {},
    );
  }

  async updateLeadScraperMicroserviceApiV1Webhooks(
    data: z.infer<typeof schemas.UpdateWebhookRequest>,
  ) {
    return this.client.put(
      "/lead-scraper-microservice/api/v1/webhooks",
      data,
      {},
    );
  }

  async getLeadScraperMicroserviceApiV1WebhooksWebhookId(params: {
    webhookId: string;
    organizationId: string | undefined;
    workspaceId: string | undefined;
    tenantId: string | undefined;
    accountId: string | undefined;
  }) {
    return this.client.get(
      "/lead-scraper-microservice/api/v1/webhooks/:webhookId",
      {
        params: {
          webhookId: params.webhookId,
        },
        queries: {
          organizationId: params.organizationId,
          workspaceId: params.workspaceId,
          tenantId: params.tenantId,
          accountId: params.accountId,
        },
      },
    );
  }

  async deleteLeadScraperMicroserviceApiV1WebhooksWebhookId(params: {
    webhookId: string;
    organizationId: string | undefined;
    workspaceId: string | undefined;
    tenantId: string | undefined;
    accountId: string | undefined;
  }) {
    return this.client.delete(
      "/lead-scraper-microservice/api/v1/webhooks/:webhookId",
      undefined,
      {
        params: {
          webhookId: params.webhookId,
        },
        queries: {
          organizationId: params.organizationId,
          workspaceId: params.workspaceId,
          tenantId: params.tenantId,
          accountId: params.accountId,
        },
      },
    );
  }

  async updateLeadScraperMicroserviceApiV1Workspace(
    data: z.infer<typeof schemas.UpdateWorkspaceRequest>,
  ) {
    return this.client.put(
      "/lead-scraper-microservice/api/v1/workspace",
      data,
      {},
    );
  }

  async getLeadScraperMicroserviceApiV1WorkspaceId(params: { id: string }) {
    return this.client.get("/lead-scraper-microservice/api/v1/workspace/:id", {
      params: {
        id: params.id,
      },
    });
  }

  async deleteLeadScraperMicroserviceApiV1WorkspaceId(params: { id: string }) {
    return this.client.delete(
      "/lead-scraper-microservice/api/v1/workspace/:id",
      undefined,
      {
        params: {
          id: params.id,
        },
      },
    );
  }
  async getLeadScraperMicroserviceApiV1Workspaces(params: {
    accountId: string | undefined;
    pageSize: number | undefined;
    pageNumber: number | undefined;
  }) {
    return this.client.get("/lead-scraper-microservice/api/v1/workspaces", {
      params: {},
      queries: {
        accountId: params.accountId,
        pageSize: params.pageSize ? Number(params.pageSize) : undefined,
        pageNumber: params.pageNumber ? Number(params.pageNumber) : undefined,
      },
    });
  }

  async createLeadScraperMicroserviceApiV1Workspaces(
    data: z.infer<typeof CreateWorkspaceRequest>,
  ) {
    return this.client.post(
      "/lead-scraper-microservice/api/v1/workspaces",
      data,
      {},
    );
  }

  async updateLeadScraperMicroserviceApiV1WorkspacesWorkflow(
    data: z.infer<typeof schemas.UpdateWorkflowRequest>,
  ) {
    return this.client.put(
      "/lead-scraper-microservice/api/v1/workspaces/workflow",
      data,
      {},
    );
  }

  async getLeadScraperMicroserviceApiV1WorkspacesWorkspaceIdAnalytics(params: {
    workspaceId: string;
    startTime: string | undefined;
    endTime: string | undefined;
  }) {
    return this.client.get(
      "/lead-scraper-microservice/api/v1/workspaces/:workspaceId/analytics",
      {
        params: {
          workspaceId: params.workspaceId,
        },
        queries: {
          startTime: params.startTime,
          endTime: params.endTime,
        },
      },
    );
  }

  async getLeadScraperMicroserviceApiV1WorkspacesWorkspaceIdWorkflows(params: {
    workspaceId: string;
    pageSize: number | undefined;
    pageNumber: number | undefined;
    filter: string | undefined;
  }) {
    return this.client.get(
      "/lead-scraper-microservice/api/v1/workspaces/:workspaceId/workflows",
      {
        params: {
          workspaceId: params.workspaceId,
        },
        queries: {
          pageSize: params.pageSize ? Number(params.pageSize) : undefined,
          pageNumber: params.pageNumber ? Number(params.pageNumber) : undefined,
          filter: params.filter,
        },
      },
    );
  }

  async createLeadScraperMicroserviceApiV1WorkspacesWorkspaceIdWorkflows(
    data: z.infer<typeof CreateWorkflowBody>,
    params: {
      workspaceId: string;
    },
  ) {
    return this.client.post(
      "/lead-scraper-microservice/api/v1/workspaces/:workspaceId/workflows",
      data,
      {
        params: {
          workspaceId: params.workspaceId,
        },
      },
    );
  }

  async getLeadScraperMicroserviceApiV1WorkspacesWorkspaceIdWorkflowsId(params: {
    workspaceId: string;
    id: string;
  }) {
    return this.client.get(
      "/lead-scraper-microservice/api/v1/workspaces/:workspaceId/workflows/:id",
      {
        params: {
          workspaceId: params.workspaceId,
          id: params.id,
        },
      },
    );
  }

  async deleteLeadScraperMicroserviceApiV1WorkspacesWorkspaceIdWorkflowsId(params: {
    workspaceId: string;
    id: string;
    orgId: string;
    tenantId: string;
    accountId: string | undefined;
  }) {
    return this.client.delete(
      "/lead-scraper-microservice/api/v1/workspaces/:workspaceId/workflows/:id",
      undefined,
      {
        params: {
          workspaceId: params.workspaceId,
          id: params.id,
        },
        queries: {
          orgId: params.orgId,
          tenantId: params.tenantId,
          accountId: params.accountId,
        },
      },
    );
  }

  async createLeadScraperMicroserviceApiV1WorkspacesWorkspaceIdWorkflowsIdPause(
    data: z.infer<typeof PauseWorkflowBody>,
    params: {
      workspaceId: string;
      id: string;
    },
  ) {
    return this.client.post(
      "/lead-scraper-microservice/api/v1/workspaces/:workspaceId/workflows/:id/pause",
      data,
      {
        params: {
          workspaceId: params.workspaceId,
          id: params.id,
        },
      },
    );
  }

  async createLeadScraperMicroserviceApiV1WorkspacesWorkspaceIdWorkflowsIdTrigger(
    data: z.infer<typeof TriggerWorkflowBody>,
    params: {
      workspaceId: string;
      id: string;
    },
  ) {
    return this.client.post(
      "/lead-scraper-microservice/api/v1/workspaces/:workspaceId/workflows/:id/trigger",
      data,
      {
        params: {
          workspaceId: params.workspaceId,
          id: params.id,
        },
      },
    );
  }

  async createWorkspaceServiceV1Accounts(
    data: z.infer<typeof CreateAccountRequest1>,
  ) {
    return this.client.post("/workspace-service/v1/accounts", data, {});
  }

  async updateWorkspaceServiceV1Accounts(
    data: z.infer<typeof schemas.UpdateAccountRequest>,
  ) {
    return this.client.put("/workspace-service/v1/accounts", data, {});
  }

  async getWorkspaceServiceV1AccountsId(params: { id: string }) {
    return this.client.get("/workspace-service/v1/accounts/:id", {
      params: {
        id: params.id,
      },
    });
  }

  async deleteWorkspaceServiceV1AccountsId(params: { id: string }) {
    return this.client.delete("/workspace-service/v1/accounts/:id", undefined, {
      params: {
        id: params.id,
      },
    });
  }

  async updateWorkspaceServiceV1WorkspaceSharings(
    data: z.infer<typeof schemas.UpdateWorkspaceSharingRequest>,
  ) {
    return this.client.put(
      "/workspace-service/v1/workspace-sharings",
      data,
      {},
    );
  }

  async deleteWorkspaceServiceV1WorkspaceSharingsSharingId(params: {
    sharingId: string;
  }) {
    return this.client.delete(
      "/workspace-service/v1/workspace-sharings/:sharingId",
      undefined,
      {
        params: {
          sharingId: params.sharingId,
        },
      },
    );
  }

  async createWorkspaceServiceV1Workspaces(
    data: z.infer<typeof CreateWorkspaceRequest1>,
  ) {
    return this.client.post("/workspace-service/v1/workspaces", data, {});
  }

  async updateWorkspaceServiceV1Workspaces(
    data: z.infer<typeof schemas.UpdateWorkspaceRequest>,
  ) {
    return this.client.put("/workspace-service/v1/workspaces", data, {});
  }

  async getWorkspaceServiceV1WorkspacesAnalyticsWorkspaceId(params: {
    workspaceId: string;
    startTime: string | undefined;
    endTime: string | undefined;
  }) {
    return this.client.get(
      "/workspace-service/v1/workspaces/analytics/:workspaceId",
      {
        params: {
          workspaceId: params.workspaceId,
        },
        queries: {
          startTime: params.startTime,
          endTime: params.endTime,
        },
      },
    );
  }

  async getWorkspaceServiceV1WorkspacesComplianceReportWorkspaceId(params: {
    workspaceId: string;
    complianceType: string | undefined;
  }) {
    return this.client.get(
      "/workspace-service/v1/workspaces/compliance-report/:workspaceId",
      {
        params: {
          workspaceId: params.workspaceId,
        },
        queries: {
          complianceType: params.complianceType,
        },
      },
    );
  }

  async getWorkspaceServiceV1WorkspacesList(params: {
    accountId: string | undefined;
    pageSize: number | undefined;
    pageToken: string | undefined;
    filter: string | undefined;
  }) {
    return this.client.get("/workspace-service/v1/workspaces/list", {
      params: {},
      queries: {
        accountId: params.accountId,
        pageSize: params.pageSize ? Number(params.pageSize) : undefined,
        pageToken: params.pageToken,
        filter: params.filter,
      },
    });
  }

  async getWorkspaceServiceV1WorkspacesSharingsWorkspaceId(params: {
    workspaceId: string;
    pageSize: number | undefined;
    pageToken: string | undefined;
  }) {
    return this.client.get(
      "/workspace-service/v1/workspaces/sharings/:workspaceId",
      {
        params: {
          workspaceId: params.workspaceId,
        },
        queries: {
          pageSize: params.pageSize ? Number(params.pageSize) : undefined,
          pageToken: params.pageToken,
        },
      },
    );
  }

  async getWorkspaceServiceV1WorkspacesStorageStatsWorkspaceId(params: {
    workspaceId: string;
  }) {
    return this.client.get(
      "/workspace-service/v1/workspaces/storage-stats/:workspaceId",
      {
        params: {
          workspaceId: params.workspaceId,
        },
      },
    );
  }

  async getWorkspaceServiceV1WorkspacesId(params: { id: string }) {
    return this.client.get("/workspace-service/v1/workspaces/:id", {
      params: {
        id: params.id,
      },
    });
  }

  async deleteWorkspaceServiceV1WorkspacesId(params: { id: string }) {
    return this.client.delete(
      "/workspace-service/v1/workspaces/:id",
      undefined,
      {
        params: {
          id: params.id,
        },
      },
    );
  }

  async createWorkspaceServiceV1WorkspacesWorkspaceIdShare(
    data: z.infer<typeof ShareWorkspaceBody>,
    params: {
      workspaceId: string;
    },
  ) {
    return this.client.post(
      "/workspace-service/v1/workspaces/:workspaceId/share",
      data,
      {
        params: {
          workspaceId: params.workspaceId,
        },
      },
    );
  }
}
