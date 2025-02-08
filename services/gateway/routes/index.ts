// Generated index file
import { OpenAPIHono as Hono } from "@hono/zod-openapi";
import type { Env } from "../types";

// Import all route handlers
import { leadScraperMicroserviceapiv1organizationRouter } from "./leadScraperMicroserviceapiv1organization";
import { leadScraperMicroserviceapiv1organizationtenantsorganizationidRouter } from "./leadScraperMicroserviceapiv1organizationtenantsorganizationid";
import { leadScraperMicroserviceapiv1organizationtenantsorganizationidtenantidRouter } from "./leadScraperMicroserviceapiv1organizationtenantsorganizationidtenantid";
import { leadScraperMicroserviceapiv1organizationidRouter } from "./leadScraperMicroserviceapiv1organizationid";
import { leadScraperMicroserviceapiv1organizationstenantsRouter } from "./leadScraperMicroserviceapiv1organizationstenants";
import { leadScraperMicroserviceapiv1organizationstenantsorganizationidtenantidRouter } from "./leadScraperMicroserviceapiv1organizationstenantsorganizationidtenantid";
import { leadScraperMicroserviceapiv1organizationsorganizationidtenantsRouter } from "./leadScraperMicroserviceapiv1organizationsorganizationidtenants";
import { workspaceServicev1accountsRouter } from "./workspaceServicev1accounts";
import { workspaceServicev1accountsidRouter } from "./workspaceServicev1accountsid";
import { workspaceServicev1workspaceSharingsRouter } from "./workspaceServicev1workspaceSharings";
import { workspaceServicev1workspaceSharingssharingidRouter } from "./workspaceServicev1workspaceSharingssharingid";
import { workspaceServicev1workspacesRouter } from "./workspaceServicev1workspaces";
import { workspaceServicev1workspacesanalyticsworkspaceidRouter } from "./workspaceServicev1workspacesanalyticsworkspaceid";
import { workspaceServicev1workspacescomplianceReportworkspaceidRouter } from "./workspaceServicev1workspacescomplianceReportworkspaceid";
import { workspaceServicev1workspaceslistRouter } from "./workspaceServicev1workspaceslist";
import { workspaceServicev1workspacessharingsworkspaceidRouter } from "./workspaceServicev1workspacessharingsworkspaceid";
import { workspaceServicev1workspacesstorageStatsworkspaceidRouter } from "./workspaceServicev1workspacesstorageStatsworkspaceid";
import { workspaceServicev1workspacesidRouter } from "./workspaceServicev1workspacesid";
import { workspaceServicev1workspacesworkspaceidshareRouter } from "./workspaceServicev1workspacesworkspaceidshare";

const router = new Hono<{ Bindings: Env }>();

// Lead Scraper Microservice Routes
router.route(
  "/lead-scraper-microservice/api/v1/organization",
  leadScraperMicroserviceapiv1organizationRouter,
);
router.route(
  "/lead-scraper-microservice/api/v1/organization/tenants/{organizationId}",
  leadScraperMicroserviceapiv1organizationtenantsorganizationidRouter,
);
router.route(
  "/lead-scraper-microservice/api/v1/organization/tenants/{organizationId}/{tenantId}",
  leadScraperMicroserviceapiv1organizationtenantsorganizationidtenantidRouter,
);
router.route(
  "/lead-scraper-microservice/api/v1/organization/{id}",
  leadScraperMicroserviceapiv1organizationidRouter,
);
router.route(
  "/lead-scraper-microservice/api/v1/organizations/tenants",
  leadScraperMicroserviceapiv1organizationstenantsRouter,
);
router.route(
  "/lead-scraper-microservice/api/v1/organizations/tenants/{organizationId}/{tenantId}",
  leadScraperMicroserviceapiv1organizationstenantsorganizationidtenantidRouter,
);
router.route(
  "/lead-scraper-microservice/api/v1/organizations/{organizationId}/tenants",
  leadScraperMicroserviceapiv1organizationsorganizationidtenantsRouter,
);

// Workspace Service Routes
router.route(
  "/workspace-service/v1/accounts",
  workspaceServicev1accountsRouter,
);
router.route(
  "/workspace-service/v1/accounts/{id}",
  workspaceServicev1accountsidRouter,
);
router.route(
  "/workspace-service/v1/workspace-sharings",
  workspaceServicev1workspaceSharingsRouter,
);
router.route(
  "/workspace-service/v1/workspace-sharings/{sharingId}",
  workspaceServicev1workspaceSharingssharingidRouter,
);
router.route(
  "/workspace-service/v1/workspaces",
  workspaceServicev1workspacesRouter,
);
router.route(
  "/workspace-service/v1/workspaces/analytics/{workspaceId}",
  workspaceServicev1workspacesanalyticsworkspaceidRouter,
);
router.route(
  "/workspace-service/v1/workspaces/compliance-report/{workspaceId}",
  workspaceServicev1workspacescomplianceReportworkspaceidRouter,
);
router.route(
  "/workspace-service/v1/workspaces/list",
  workspaceServicev1workspaceslistRouter,
);
router.route(
  "/workspace-service/v1/workspaces/sharings/{workspaceId}",
  workspaceServicev1workspacessharingsworkspaceidRouter,
);
router.route(
  "/workspace-service/v1/workspaces/storage-stats/{workspaceId}",
  workspaceServicev1workspacesstorageStatsworkspaceidRouter,
);
router.route(
  "/workspace-service/v1/workspaces/{id}",
  workspaceServicev1workspacesidRouter,
);
router.route(
  "/workspace-service/v1/workspaces/{workspaceId}/share",
  workspaceServicev1workspacesworkspaceidshareRouter,
);

export const LeadScrapingServiceAPIRouter = router;
