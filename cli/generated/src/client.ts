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
  status: APIKey_Status;
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
type APIKey_Status =
  | "STATUS_UNSPECIFIED"
  | "STATUS_ACTIVE"
  | "STATUS_REVOKED"
  | "STATUS_EXPIRED"
  | "STATUS_RATE_LIMITED"
  | "STATUS_SUSPENDED"
  | "STATUS_PENDING_REVIEW"
  | "STATUS_DEPRECATED";
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
    .passthrough()
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
    .passthrough()
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
    .passthrough()
);
const APIKey_Status = z.enum([
  "STATUS_UNSPECIFIED",
  "STATUS_ACTIVE",
  "STATUS_REVOKED",
  "STATUS_EXPIRED",
  "STATUS_RATE_LIMITED",
  "STATUS_SUSPENDED",
  "STATUS_PENDING_REVIEW",
  "STATUS_DEPRECATED",
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
      status: APIKey_Status.default("STATUS_UNSPECIFIED"),
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
    .passthrough()
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
    .passthrough()
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
    .passthrough()
);
const CreateAccountRequest = z
  .object({ account: Account, initialWorkspaceName: z.string() })
  .partial()
  .passthrough();
const CreateAccountResponse = z
  .object({ account: Account, initialWorkspaceId: z.string() })
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
const rpc_Status = z
  .object({
    code: z.number().int(),
    message: z.string(),
    details: z.array(Any),
  })
  .partial()
  .passthrough();
const ErrorResponse = z.object({ status: rpc_Status }).partial().passthrough();
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
const UpdateAccountRequest = z
  .object({ account: Account })
  .partial()
  .passthrough();
const UpdateAccountResponse = z
  .object({ account: Account })
  .partial()
  .passthrough();
const DeleteAccountResponse = z
  .object({ success: z.boolean() })
  .partial()
  .passthrough();
const GetAccountResponse = z
  .object({ account: Account })
  .partial()
  .passthrough();
const ListScrapingJobsResponse = z
  .object({ jobs: z.array(ScrapingJob) })
  .partial()
  .passthrough();
const CreateScrapingJobRequest = z
  .object({
    userId: z.string(),
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
const DeleteScrapingJobResponse = z
  .object({ success: z.boolean() })
  .partial()
  .passthrough();
const GetScrapingJobResponse = z
  .object({ job: ScrapingJob })
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
  APIKey_Status,
  APIKey,
  TriggerEvent,
  IncludedField,
  PayloadFormat,
  WebhookConfig,
  Workspace,
  AccountSettings,
  Account,
  CreateAccountRequest,
  CreateAccountResponse,
  ValidationErrorCode,
  FieldViolation,
  SchemaValidation,
  ResourceValidation,
  Any,
  rpc_Status,
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
  UpdateAccountRequest,
  UpdateAccountResponse,
  DeleteAccountResponse,
  GetAccountResponse,
  ListScrapingJobsResponse,
  CreateScrapingJobRequest,
  CreateScrapingJobResponse,
  DeleteScrapingJobResponse,
  GetScrapingJobResponse,
  DownloadScrapingResultsResponse,
};

const endpoints = makeApi([
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
    method: "get",
    path: "/lead-scraper-microservice/api/v1/jobs",
    alias: "ListScrapingJobs",
    description: `This endpoint retrieves all Google Maps scraping jobs`,
    requestFormat: "json",
    parameters: [
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


    async createLeadScraperMicroserviceApiV1Accounts(data: z.infer<typeof CreateAccountRequest>, ) {
      return this.client.post("/lead-scraper-microservice/api/v1/accounts", data, { 
        
      });
    }

    async updateLeadScraperMicroserviceApiV1Accounts(data: z.infer<typeof schemas.UpdateAccountRequest>, ) {
      return this.client.put("/lead-scraper-microservice/api/v1/accounts", data, { 
        
      });
    }

    async getLeadScraperMicroserviceApiV1AccountsId(params: { id: string;  }) {
      return this.client.get("/lead-scraper-microservice/api/v1/accounts/:id", { 
        params: { id: params.id, },
      });
    }



    async deleteLeadScraperMicroserviceApiV1AccountsId(params: { id: string;  }) {
      return this.client.delete("/lead-scraper-microservice/api/v1/accounts/:id", undefined, { 
        params: { id: params.id, },
      });
    }
    async getLeadScraperMicroserviceApiV1Jobs(params: { userId: string; orgId: string; tenantId: string;  }) {
      return this.client.get("/lead-scraper-microservice/api/v1/jobs", { 
        
        queries: { userId: params.userId, orgId: params.orgId, tenantId: params.tenantId }
      });
    }

    async createLeadScraperMicroserviceApiV1Jobs(data: z.infer<typeof CreateScrapingJobRequest>, ) {
      return this.client.post("/lead-scraper-microservice/api/v1/jobs", data, { 
        
      });
    }


    async getLeadScraperMicroserviceApiV1JobsJobId(params: { jobId: string; userId: string; orgId: string; tenantId: string;  }) {
      return this.client.get("/lead-scraper-microservice/api/v1/jobs/:jobId", { 
        params: { jobId: params.jobId, },
        queries: { userId: params.userId, orgId: params.orgId, tenantId: params.tenantId }
      });
    }



    async deleteLeadScraperMicroserviceApiV1JobsJobId(params: { jobId: string; userId: string; orgId: string; tenantId: string;  }) {
      return this.client.delete("/lead-scraper-microservice/api/v1/jobs/:jobId", undefined, { 
        params: { jobId: params.jobId, },
        queries: { userId: params.userId, orgId: params.orgId, tenantId: params.tenantId }
      });
    }
    async getLeadScraperMicroserviceApiV1JobsJobIdDownload(params: { jobId: string; userId: string; orgId: string; tenantId: string;  }) {
      return this.client.get("/lead-scraper-microservice/api/v1/jobs/:jobId/download", { 
        params: { jobId: params.jobId, },
        queries: { userId: params.userId, orgId: params.orgId, tenantId: params.tenantId }
      });
    }



}