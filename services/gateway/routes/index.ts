import type { Env } from "../types";
import { Hono } from "hono";
import { leadScraperMicroserviceapiv1accountsRouter } from "./leadScraperMicroserviceapiv1accounts";
import { leadScraperMicroserviceapiv1accountsidRouter } from "./leadScraperMicroserviceapiv1accountsid";
import { leadScraperMicroserviceapiv1accountsidusageRouter } from "./leadScraperMicroserviceapiv1accountsidusage";
import { leadScraperMicroserviceapiv1accountssettingsRouter } from "./leadScraperMicroserviceapiv1accountssettings";
import { leadScraperMicroserviceapiv1jobsRouter } from "./leadScraperMicroserviceapiv1jobs";
import { leadScraperMicroserviceapiv1jobsjobidRouter } from "./leadScraperMicroserviceapiv1jobsjobid";
import { leadScraperMicroserviceapiv1jobsjobiddownloadRouter } from "./leadScraperMicroserviceapiv1jobsjobiddownload";
import { leadScraperMicroserviceapiv1organizationRouter } from "./leadScraperMicroserviceapiv1organization";
import { leadScraperMicroserviceapiv1organizationidRouter } from "./leadScraperMicroserviceapiv1organizationid";
import { leadScraperMicroserviceapiv1organizationsorganizationidtenantsRouter } from "./leadScraperMicroserviceapiv1organizationsorganizationidtenants";
import { leadScraperMicroserviceapiv1organizationstenantsRouter } from "./leadScraperMicroserviceapiv1organizationstenants";
import { leadScraperMicroserviceapiv1organizationstenantsorganizationidtenantidRouter } from "./leadScraperMicroserviceapiv1organizationstenantsorganizationidtenantid";
import { leadScraperMicroserviceapiv1organizationtenantsorganizationidRouter } from "./leadScraperMicroserviceapiv1organizationtenantsorganizationid";
import { leadScraperMicroserviceapiv1organizationtenantsorganizationidtenantidRouter } from "./leadScraperMicroserviceapiv1organizationtenantsorganizationidtenantid";
import { leadScraperMicroserviceapiv1workspaceRouter } from "./leadScraperMicroserviceapiv1workspace";
import { leadScraperMicroserviceapiv1workspaceidRouter } from "./leadScraperMicroserviceapiv1workspaceid";
import { leadScraperMicroserviceapiv1workspacesRouter } from "./leadScraperMicroserviceapiv1workspaces";
import { leadScraperMicroserviceapiv1workspacesworkflowRouter } from "./leadScraperMicroserviceapiv1workspacesworkflow";
import { leadScraperMicroserviceapiv1workspacesworkspaceidanalyticsRouter } from "./leadScraperMicroserviceapiv1workspacesworkspaceidanalytics";
import { leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsRouter } from "./leadScraperMicroserviceapiv1workspacesworkspaceidworkflows";
import { leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsidRouter } from "./leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsid";
import { leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsidpauseRouter } from "./leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsidpause";
import { leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsidtriggerRouter } from "./leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsidtrigger";
import { workspaceServicev1accountsRouter } from "./workspaceServicev1accounts";
import { workspaceServicev1accountsidRouter } from "./workspaceServicev1accountsid";
import { workspaceServicev1workspaceSharingsRouter } from "./workspaceServicev1workspaceSharings";
import { workspaceServicev1workspaceSharingssharingidRouter } from "./workspaceServicev1workspaceSharingssharingid";
import { workspaceServicev1workspacesRouter } from "./workspaceServicev1workspaces";
import { workspaceServicev1workspacesanalyticsworkspaceidRouter } from "./workspaceServicev1workspacesanalyticsworkspaceid";
import { workspaceServicev1workspacescomplianceReportworkspaceidRouter } from "./workspaceServicev1workspacescomplianceReportworkspaceid";
import { workspaceServicev1workspacesidRouter } from "./workspaceServicev1workspacesid";
import { workspaceServicev1workspaceslistRouter } from "./workspaceServicev1workspaceslist";
import { workspaceServicev1workspacessharingsworkspaceidRouter } from "./workspaceServicev1workspacessharingsworkspaceid";
import { workspaceServicev1workspacesstorageStatsworkspaceidRouter } from "./workspaceServicev1workspacesstorageStatsworkspaceid";
import { workspaceServicev1workspacesworkspaceidshareRouter } from "./workspaceServicev1workspacesworkspaceidshare";

const router = new Hono<{ Bindings: Env }>();

router.route(
  "/lead-scraper-microservice/api/v1/accounts",
  leadScraperMicroserviceapiv1accountsRouter,
);

router.route(
  "/lead-scraper-microservice/api/v1/accounts/settings",
  leadScraperMicroserviceapiv1accountssettingsRouter,
);

router.route(
  "/lead-scraper-microservice/api/v1/accounts/:id",
  leadScraperMicroserviceapiv1accountsidRouter,
);

router.route(
  "/lead-scraper-microservice/api/v1/accounts/:id/usage",
  leadScraperMicroserviceapiv1accountsidusageRouter,
);

router.route(
  "/lead-scraper-microservice/api/v1/jobs",
  leadScraperMicroserviceapiv1jobsRouter,
);

router.route(
  "/lead-scraper-microservice/api/v1/jobs/:jobId",
  leadScraperMicroserviceapiv1jobsjobidRouter,
);

router.route(
  "/lead-scraper-microservice/api/v1/jobs/:jobId/download",
  leadScraperMicroserviceapiv1jobsjobiddownloadRouter,
);

router.route(
  "/lead-scraper-microservice/api/v1/organization",
  leadScraperMicroserviceapiv1organizationRouter,
);

router.route(
  "/lead-scraper-microservice/api/v1/organization/tenants/:organizationId",
  leadScraperMicroserviceapiv1organizationtenantsorganizationidRouter,
);

router.route(
  "/lead-scraper-microservice/api/v1/organization/tenants/:organizationId/:tenantId",
  leadScraperMicroserviceapiv1organizationtenantsorganizationidtenantidRouter,
);

router.route(
  "/lead-scraper-microservice/api/v1/organization/:id",
  leadScraperMicroserviceapiv1organizationidRouter,
);

router.route(
  "/lead-scraper-microservice/api/v1/organizations/tenants",
  leadScraperMicroserviceapiv1organizationstenantsRouter,
);

router.route(
  "/lead-scraper-microservice/api/v1/organizations/tenants/:organizationId/:tenantId",
  leadScraperMicroserviceapiv1organizationstenantsorganizationidtenantidRouter,
);

router.route(
  "/lead-scraper-microservice/api/v1/organizations/:organizationId/tenants",
  leadScraperMicroserviceapiv1organizationsorganizationidtenantsRouter,
);

router.route(
  "/lead-scraper-microservice/api/v1/workspace",
  leadScraperMicroserviceapiv1workspaceRouter,
);

router.route(
  "/lead-scraper-microservice/api/v1/workspace/:id",
  leadScraperMicroserviceapiv1workspaceidRouter,
);

router.route(
  "/lead-scraper-microservice/api/v1/workspaces",
  leadScraperMicroserviceapiv1workspacesRouter,
);

router.route(
  "/lead-scraper-microservice/api/v1/workspaces/workflow",
  leadScraperMicroserviceapiv1workspacesworkflowRouter,
);

router.route(
  "/lead-scraper-microservice/api/v1/workspaces/:workspaceId/analytics",
  leadScraperMicroserviceapiv1workspacesworkspaceidanalyticsRouter,
);

router.route(
  "/lead-scraper-microservice/api/v1/workspaces/:workspaceId/workflows",
  leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsRouter,
);

router.route(
  "/lead-scraper-microservice/api/v1/workspaces/:workspaceId/workflows/:id",
  leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsidRouter,
);

router.route(
  "/lead-scraper-microservice/api/v1/workspaces/:workspaceId/workflows/:id/pause",
  leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsidpauseRouter,
);

router.route(
  "/lead-scraper-microservice/api/v1/workspaces/:workspaceId/workflows/:id/trigger",
  leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsidtriggerRouter,
);

router.route(
  "/workspace-service/v1/accounts",
  workspaceServicev1accountsRouter,
);

router.route(
  "/workspace-service/v1/accounts/:id",
  workspaceServicev1accountsidRouter,
);

router.route(
  "/workspace-service/v1/workspace-sharings",
  workspaceServicev1workspaceSharingsRouter,
);

router.route(
  "/workspace-service/v1/workspace-sharings/:sharingId",
  workspaceServicev1workspaceSharingssharingidRouter,
);

router.route(
  "/workspace-service/v1/workspaces",
  workspaceServicev1workspacesRouter,
);

router.route(
  "/workspace-service/v1/workspaces/analytics/:workspaceId",
  workspaceServicev1workspacesanalyticsworkspaceidRouter,
);

router.route(
  "/workspace-service/v1/workspaces/compliance-report/:workspaceId",
  workspaceServicev1workspacescomplianceReportworkspaceidRouter,
);

router.route(
  "/workspace-service/v1/workspaces/list",
  workspaceServicev1workspaceslistRouter,
);

router.route(
  "/workspace-service/v1/workspaces/sharings/:workspaceId",
  workspaceServicev1workspacessharingsworkspaceidRouter,
);

router.route(
  "/workspace-service/v1/workspaces/storage-stats/:workspaceId",
  workspaceServicev1workspacesstorageStatsworkspaceidRouter,
);

router.route(
  "/workspace-service/v1/workspaces/:id",
  workspaceServicev1workspacesidRouter,
);

router.route(
  "/workspace-service/v1/workspaces/:workspaceId/share",
  workspaceServicev1workspacesworkspaceidshareRouter,
);

export const LeadScrapingServiceAPIRouter = router;
