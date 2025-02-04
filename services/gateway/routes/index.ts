import { Hono } from "hono";
import type { Env } from "../types";

const router = new Hono<{ Bindings: Env }>();

import { leadScraperMicroserviceapiv1accountsRouter } from "./leadScraperMicroserviceapiv1accounts";
router.route(
  "/lead-scraper-microservice/api/v1/accounts",
  leadScraperMicroserviceapiv1accountsRouter,
);

import { leadScraperMicroserviceapiv1accountssettingsRouter } from "./leadScraperMicroserviceapiv1accountssettings";
router.route(
  "/lead-scraper-microservice/api/v1/accounts/settings",
  leadScraperMicroserviceapiv1accountssettingsRouter,
);

import { leadScraperMicroserviceapiv1accountsidRouter } from "./leadScraperMicroserviceapiv1accountsid";
router.route(
  "/lead-scraper-microservice/api/v1/accounts/:id",
  leadScraperMicroserviceapiv1accountsidRouter,
);

import { leadScraperMicroserviceapiv1accountsidusageRouter } from "./leadScraperMicroserviceapiv1accountsidusage";
router.route(
  "/lead-scraper-microservice/api/v1/accounts/:id/usage",
  leadScraperMicroserviceapiv1accountsidusageRouter,
);

import { leadScraperMicroserviceapiv1jobsRouter } from "./leadScraperMicroserviceapiv1jobs";
router.route(
  "/lead-scraper-microservice/api/v1/jobs",
  leadScraperMicroserviceapiv1jobsRouter,
);

import { leadScraperMicroserviceapiv1jobsjobidRouter } from "./leadScraperMicroserviceapiv1jobsjobid";
router.route(
  "/lead-scraper-microservice/api/v1/jobs/:jobId",
  leadScraperMicroserviceapiv1jobsjobidRouter,
);

import { leadScraperMicroserviceapiv1jobsjobiddownloadRouter } from "./leadScraperMicroserviceapiv1jobsjobiddownload";
router.route(
  "/lead-scraper-microservice/api/v1/jobs/:jobId/download",
  leadScraperMicroserviceapiv1jobsjobiddownloadRouter,
);

import { leadScraperMicroserviceapiv1workspaceRouter } from "./leadScraperMicroserviceapiv1workspace";
router.route(
  "/lead-scraper-microservice/api/v1/workspace",
  leadScraperMicroserviceapiv1workspaceRouter,
);

import { leadScraperMicroserviceapiv1workspaceidRouter } from "./leadScraperMicroserviceapiv1workspaceid";
router.route(
  "/lead-scraper-microservice/api/v1/workspace/:id",
  leadScraperMicroserviceapiv1workspaceidRouter,
);

import { leadScraperMicroserviceapiv1workspacesRouter } from "./leadScraperMicroserviceapiv1workspaces";
router.route(
  "/lead-scraper-microservice/api/v1/workspaces",
  leadScraperMicroserviceapiv1workspacesRouter,
);

import { leadScraperMicroserviceapiv1workspacesworkflowRouter } from "./leadScraperMicroserviceapiv1workspacesworkflow";
router.route(
  "/lead-scraper-microservice/api/v1/workspaces/workflow",
  leadScraperMicroserviceapiv1workspacesworkflowRouter,
);

import { leadScraperMicroserviceapiv1workspacesworkspaceidanalyticsRouter } from "./leadScraperMicroserviceapiv1workspacesworkspaceidanalytics";
router.route(
  "/lead-scraper-microservice/api/v1/workspaces/:workspaceId/analytics",
  leadScraperMicroserviceapiv1workspacesworkspaceidanalyticsRouter,
);

import { leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsRouter } from "./leadScraperMicroserviceapiv1workspacesworkspaceidworkflows";
router.route(
  "/lead-scraper-microservice/api/v1/workspaces/:workspaceId/workflows",
  leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsRouter,
);

import { leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsidRouter } from "./leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsid";
router.route(
  "/lead-scraper-microservice/api/v1/workspaces/:workspaceId/workflows/:id",
  leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsidRouter,
);

import { leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsidpauseRouter } from "./leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsidpause";
router.route(
  "/lead-scraper-microservice/api/v1/workspaces/:workspaceId/workflows/:id/pause",
  leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsidpauseRouter,
);

import { leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsidtriggerRouter } from "./leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsidtrigger";
router.route(
  "/lead-scraper-microservice/api/v1/workspaces/:workspaceId/workflows/:id/trigger",
  leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsidtriggerRouter,
);

import { workspaceServicev1accountsRouter } from "./workspaceServicev1accounts";
router.route(
  "/workspace-service/v1/accounts",
  workspaceServicev1accountsRouter,
);

import { workspaceServicev1accountsidRouter } from "./workspaceServicev1accountsid";
router.route(
  "/workspace-service/v1/accounts/:id",
  workspaceServicev1accountsidRouter,
);

import { workspaceServicev1workspaceSharingsRouter } from "./workspaceServicev1workspaceSharings";
router.route(
  "/workspace-service/v1/workspace-sharings",
  workspaceServicev1workspaceSharingsRouter,
);

import { workspaceServicev1workspaceSharingssharingidRouter } from "./workspaceServicev1workspaceSharingssharingid";
router.route(
  "/workspace-service/v1/workspace-sharings/:sharingId",
  workspaceServicev1workspaceSharingssharingidRouter,
);

import { workspaceServicev1workspacesRouter } from "./workspaceServicev1workspaces";
router.route(
  "/workspace-service/v1/workspaces",
  workspaceServicev1workspacesRouter,
);

import { workspaceServicev1workspacesanalyticsworkspaceidRouter } from "./workspaceServicev1workspacesanalyticsworkspaceid";
router.route(
  "/workspace-service/v1/workspaces/analytics/:workspaceId",
  workspaceServicev1workspacesanalyticsworkspaceidRouter,
);

import { workspaceServicev1workspacescomplianceReportworkspaceidRouter } from "./workspaceServicev1workspacescomplianceReportworkspaceid";
router.route(
  "/workspace-service/v1/workspaces/compliance-report/:workspaceId",
  workspaceServicev1workspacescomplianceReportworkspaceidRouter,
);

import { workspaceServicev1workspaceslistRouter } from "./workspaceServicev1workspaceslist";
router.route(
  "/workspace-service/v1/workspaces/list",
  workspaceServicev1workspaceslistRouter,
);

import { workspaceServicev1workspacessharingsworkspaceidRouter } from "./workspaceServicev1workspacessharingsworkspaceid";
router.route(
  "/workspace-service/v1/workspaces/sharings/:workspaceId",
  workspaceServicev1workspacessharingsworkspaceidRouter,
);

import { workspaceServicev1workspacesstorageStatsworkspaceidRouter } from "./workspaceServicev1workspacesstorageStatsworkspaceid";
router.route(
  "/workspace-service/v1/workspaces/storage-stats/:workspaceId",
  workspaceServicev1workspacesstorageStatsworkspaceidRouter,
);

import { workspaceServicev1workspacesidRouter } from "./workspaceServicev1workspacesid";
router.route(
  "/workspace-service/v1/workspaces/:id",
  workspaceServicev1workspacesidRouter,
);

import { workspaceServicev1workspacesworkspaceidshareRouter } from "./workspaceServicev1workspacesworkspaceidshare";
router.route(
  "/workspace-service/v1/workspaces/:workspaceId/share",
  workspaceServicev1workspacesworkspaceidshareRouter,
);

export const LeadScrapingServiceAPIRouter = router;
